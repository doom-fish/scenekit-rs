use std::sync::{Arc, Barrier};
use std::thread;

use apple_metal::{pixel_format, MetalDevice};
use scenekit::{Geometry, Node, Program, Renderer, Shadable};

mod common;

const SHADER: &str = r"
#include <metal_stdlib>
using namespace metal;

struct NodeBuffer {
    float4x4 modelViewProjectionTransform;
};

struct VertexIn {
    float3 position [[attribute(0)]];
};

struct VertexOut {
    float4 position [[position]];
};

vertex VertexOut solid_vertex(VertexIn in [[stage_in]], constant NodeBuffer& scn_node [[buffer(1)]]) {
    VertexOut out;
    out.position = scn_node.modelViewProjectionTransform * float4(in.position, 1.0);
    return out;
}

fragment half4 solid_fragment(VertexOut in [[stage_in]]) {
    return half4(1.0);
}
";

#[test]
fn renderers_can_be_created_on_many_threads_at_once() {
    let threads = 8;
    let barrier = Arc::new(Barrier::new(threads));
    let workers: Vec<_> = (0..threads)
        .map(|_| {
            let barrier = Arc::clone(&barrier);
            thread::spawn(move || {
                let device = MetalDevice::system_default().expect("device");
                let library = device.new_library_with_source(SHADER).expect("library");
                let program = Program::new().expect("program");
                program.set_library(Some(&library));
                program.set_vertex_function_name(Some("solid_vertex"));
                program.set_fragment_function_name(Some("solid_fragment"));
                let (scene, root, camera_node) = common::scene_with_camera().expect("scene");
                let geometry = Geometry::box_geometry(1.0, 1.0, 1.0, 0.0).expect("box");
                geometry
                    .first_material()
                    .expect("material")
                    .set_program(Some(&program));
                let node = Node::with_geometry(Some(&geometry)).expect("node");
                root.add_child_node(&node);
                let texture =
                    common::render_target(&device, 8, pixel_format::BGRA8UNORM).expect("texture");
                barrier.wait();
                let renderer = Renderer::new(Some(&device)).expect("renderer");
                renderer.set_scene(Some(&scene));
                renderer.set_point_of_view(Some(&camera_node));
                common::render_frame(&device, &renderer, &texture, 0.0).expect("render");
            })
        })
        .collect();
    for worker in workers {
        worker.join().expect("worker thread");
    }
}
