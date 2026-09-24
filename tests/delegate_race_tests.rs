use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use apple_metal::{pixel_format, MetalDevice};
use scenekit::{NodeRendererDelegate, NodeRendererDelegateCallbacks, Renderer, Transaction};

mod common;

const SWAPS: usize = 50_000;

extern "C" {
    fn scn_node_set_renderer_delegate(node: *mut c_void, delegate: *mut c_void);
}

#[test]
fn clearing_a_renderer_delegate_while_another_thread_renders_is_safe() {
    let device = MetalDevice::system_default().expect("device");
    let texture = common::render_target(&device, 16, pixel_format::BGRA8UNORM).expect("texture");
    let queue = device.new_command_queue().expect("queue");
    let (scene, cube_node, camera_node) = common::green_cube_scene().expect("scene");
    let renderer = Renderer::new(Some(&device)).expect("renderer");
    renderer.set_scene(Some(&scene));
    renderer.set_point_of_view(Some(&camera_node));

    let node_address = cube_node.as_ptr() as usize;
    let stop = Arc::new(AtomicBool::new(false));
    let installed_calls = Arc::new(AtomicUsize::new(0));
    let calls = Arc::new(AtomicUsize::new(0));
    let swaps = Arc::new(AtomicUsize::new(0));
    let installed = NodeRendererDelegate::new(NodeRendererDelegateCallbacks::new().on_render({
        let installed_calls = Arc::clone(&installed_calls);
        move |_, _| {
            installed_calls.fetch_add(1, Ordering::Relaxed);
        }
    }))
    .expect("installed delegate");
    cube_node.set_renderer_delegate(Some(&installed));
    Transaction::flush();
    for frame in 0..2 {
        common::render_frame_on(&queue, &renderer, &texture, f64::from(frame) / 60.0)
            .expect("first frames");
    }
    assert!(
        installed_calls.load(Ordering::Relaxed) > 0,
        "SceneKit installs the renderer delegate on the rendering thread"
    );
    let swapper = thread::spawn({
        let stop = Arc::clone(&stop);
        let calls = Arc::clone(&calls);
        let swaps = Arc::clone(&swaps);
        move || {
            let node = node_address as *mut c_void;
            while !stop.load(Ordering::Acquire) && swaps.load(Ordering::Relaxed) < SWAPS {
                common::autoreleasepool(|| {
                    let calls = Arc::clone(&calls);
                    let delegate = NodeRendererDelegate::new(
                        NodeRendererDelegateCallbacks::new().on_render(move |_, _| {
                            calls.fetch_add(1, Ordering::Relaxed);
                        }),
                    )
                    .expect("delegate");
                    unsafe { scn_node_set_renderer_delegate(node, delegate.as_ptr()) };
                    thread::yield_now();
                    unsafe { scn_node_set_renderer_delegate(node, core::ptr::null_mut()) };
                    drop(delegate);
                });
                swaps.fetch_add(1, Ordering::Relaxed);
            }
        }
    });

    let started = Instant::now();
    let mut frame = 0_u32;
    while !swapper.is_finished() && started.elapsed() < Duration::from_secs(30) {
        common::autoreleasepool(|| {
            common::render_frame_on(&queue, &renderer, &texture, f64::from(frame) / 60.0)
                .expect("render");
        });
        frame += 1;
    }
    stop.store(true, Ordering::Release);
    swapper.join().expect("swapper thread");
    assert!(frame > 0);
    assert_eq!(swaps.load(Ordering::Relaxed), SWAPS);
    assert!(
        calls.load(Ordering::Relaxed) > 0,
        "the render thread never called a delegate that another thread was swapping"
    );
    drop(installed);
    drop(cube_node);
}
