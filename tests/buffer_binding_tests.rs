use std::sync::{Arc, Mutex};

use apple_metal::{pixel_format, MetalDevice, MetalTexture};
use scenekit::{
    read_texture_bytes, BufferFrequency, Geometry, Node, Program, ProgramBufferBinding, Renderer,
    Scene, Shadable, Transaction,
};

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

struct Tint {
    float4 color;
    float4 scale;
};

vertex VertexOut tint_vertex(VertexIn in [[stage_in]], constant NodeBuffer& scn_node [[buffer(1)]]) {
    VertexOut out;
    out.position = scn_node.modelViewProjectionTransform * float4(in.position, 1.0);
    return out;
}

fragment half4 tint_fragment(VertexOut in [[stage_in]], constant Tint& tint [[buffer(2)]]) {
    return half4(tint.color * tint.scale);
}
";

const TINT_LEN: usize = 32;
const TRANSPARENT: [u8; 4] = [0, 0, 0, 0];
const RED: [u8; 4] = [0, 0, 255, 255];
const GREEN: [u8; 4] = [0, 255, 0, 255];

type WriteResults = Arc<Mutex<Vec<Result<(), String>>>>;

struct Fixture {
    device: MetalDevice,
    renderer: Renderer,
    texture: MetalTexture,
    program: Program,
    _scene: Scene,
    _node: Node,
    _camera_node: Node,
}

impl Fixture {
    fn new() -> Self {
        let device = MetalDevice::system_default().expect("device");
        let library = device.new_library_with_source(SHADER).expect("library");
        let program = Program::new().expect("program");
        program.set_library(Some(&library));
        program.set_vertex_function_name(Some("tint_vertex"));
        program.set_fragment_function_name(Some("tint_fragment"));
        let (scene, root, camera_node) = common::scene_with_camera().expect("scene");
        let geometry = Geometry::box_geometry(2.0, 2.0, 2.0, 0.0).expect("box");
        geometry
            .first_material()
            .expect("material")
            .set_program(Some(&program));
        let node = Node::with_geometry(Some(&geometry)).expect("node");
        root.add_child_node(&node);
        let renderer = Renderer::new(Some(&device)).expect("renderer");
        renderer.set_scene(Some(&scene));
        renderer.set_point_of_view(Some(&camera_node));
        let texture =
            common::render_target(&device, 32, pixel_format::BGRA8UNORM).expect("texture");
        Self {
            device,
            renderer,
            texture,
            program,
            _scene: scene,
            _node: node,
            _camera_node: camera_node,
        }
    }

    fn bind(&self, binding: Option<&ProgramBufferBinding>) {
        self.program
            .set_buffer_binding("tint", BufferFrequency::PerFrame, binding);
        Transaction::flush();
    }

    fn center_pixel(&self) -> [u8; 4] {
        common::autoreleasepool(|| {
            common::render_frame(&self.device, &self.renderer, &self.texture, 0.0).expect("render");
        });
        let bytes = read_texture_bytes(&self.texture).expect("read texture");
        let offset = (16 * 32 + 16) * 4;
        bytes[offset..offset + 4].try_into().expect("pixel")
    }
}

fn tint(color: [f32; 4]) -> Vec<u8> {
    color
        .iter()
        .chain([1.0_f32; 4].iter())
        .flat_map(|component| component.to_ne_bytes())
        .collect()
}

fn recording_binding(results: &WriteResults, bytes: Vec<u8>) -> ProgramBufferBinding {
    let results = Arc::clone(results);
    ProgramBufferBinding::new(move |stream| {
        let result = stream
            .write_bytes(&bytes)
            .map_err(|error| error.to_string());
        results.lock().expect("results").push(result);
    })
    .expect("binding")
}

#[test]
fn exact_writes_reach_the_shader() {
    let fixture = Fixture::new();
    let required = Arc::new(Mutex::new(Vec::new()));
    let binding = ProgramBufferBinding::new({
        let required = Arc::clone(&required);
        move |stream| {
            required
                .lock()
                .expect("required")
                .push(stream.required_length());
            stream
                .write_bytes(&tint([1.0, 0.0, 0.0, 1.0]))
                .expect("exact write");
        }
    })
    .expect("binding");
    fixture.bind(Some(&binding));

    assert_eq!(fixture.center_pixel(), RED);
    let required = required.lock().expect("required").clone();
    assert!(!required.is_empty(), "SceneKit never asked for the buffer");
    assert!(required.iter().all(|length| *length == Some(TINT_LEN)));
}

#[test]
fn short_writes_are_rejected_and_the_buffer_is_zero_filled() {
    let fixture = Fixture::new();
    let results = WriteResults::default();
    let binding = recording_binding(&results, tint([1.0, 0.0, 0.0, 1.0])[..16].to_vec());
    fixture.bind(Some(&binding));

    assert_eq!(fixture.center_pixel(), TRANSPARENT);
    let results = results.lock().expect("results").clone();
    assert!(!results.is_empty());
    for result in &results {
        let message = result
            .as_ref()
            .expect_err("a 16-byte write must be rejected");
        assert!(message.contains("needs 32 bytes, got 16"), "{message}");
    }
}

#[test]
fn split_writes_are_rejected_instead_of_rebinding_part_of_the_buffer() {
    let fixture = Fixture::new();
    let results = WriteResults::default();
    let binding = ProgramBufferBinding::new({
        let results = Arc::clone(&results);
        move |stream| {
            let bytes = tint([0.0, 1.0, 0.0, 1.0]);
            for half in bytes.chunks(TINT_LEN / 2) {
                let result = stream.write_bytes(half).map_err(|error| error.to_string());
                results.lock().expect("results").push(result);
            }
        }
    })
    .expect("binding");
    fixture.bind(Some(&binding));

    assert_eq!(fixture.center_pixel(), TRANSPARENT);
    assert!(results.lock().expect("results").iter().all(Result::is_err));
}

#[test]
fn longer_writes_are_accepted() {
    let fixture = Fixture::new();
    let results = WriteResults::default();
    let mut bytes = tint([0.0, 1.0, 0.0, 1.0]);
    bytes.resize(4096, 0xAB);
    let binding = recording_binding(&results, bytes);
    fixture.bind(Some(&binding));

    assert_eq!(fixture.center_pixel(), GREEN);
    assert!(results.lock().expect("results").iter().all(Result::is_ok));
}

#[test]
fn empty_writes_are_rejected() {
    let fixture = Fixture::new();
    let results = WriteResults::default();
    let binding = recording_binding(&results, Vec::new());
    fixture.bind(Some(&binding));

    assert_eq!(fixture.center_pixel(), TRANSPARENT);
    let results = results.lock().expect("results").clone();
    assert!(!results.is_empty());
    assert!(results.iter().all(Result::is_err));
}

#[test]
fn unchecked_writes_still_reach_the_shader() {
    let fixture = Fixture::new();
    let results = WriteResults::default();
    let binding = ProgramBufferBinding::new({
        let results = Arc::clone(&results);
        move |stream| {
            let exact = unsafe { stream.write_bytes_unchecked(&tint([1.0, 0.0, 0.0, 1.0])) };
            let empty = unsafe { stream.write_bytes_unchecked(&[]) };
            let mut results = results.lock().expect("results");
            results.push(exact.map_err(|error| error.to_string()));
            results.push(empty.map_err(|error| error.to_string()));
        }
    })
    .expect("binding");
    fixture.bind(Some(&binding));

    assert_eq!(fixture.center_pixel(), RED);
    let results = results.lock().expect("results").clone();
    assert!(results
        .chunks(2)
        .all(|pair| pair[0].is_ok() && pair[1].is_err()));
}

#[test]
fn bindings_that_write_nothing_or_were_dropped_bind_zeros() {
    let fixture = Fixture::new();
    let silent = ProgramBufferBinding::new(|_| {}).expect("binding");
    fixture.bind(Some(&silent));
    assert_eq!(fixture.center_pixel(), TRANSPARENT);

    let results = WriteResults::default();
    let binding = recording_binding(&results, tint([0.0, 1.0, 0.0, 1.0]));
    fixture.bind(Some(&binding));
    assert_eq!(fixture.center_pixel(), GREEN);
    drop(binding);
    assert_eq!(fixture.center_pixel(), TRANSPARENT);

    fixture.bind(None);
    assert_eq!(fixture.center_pixel(), TRANSPARENT);
}

#[test]
fn writes_beyond_the_device_limit_are_rejected_without_reading() {
    let fixture = Fixture::new();
    let maximum = Arc::new(Mutex::new(None));
    let probe = ProgramBufferBinding::new({
        let maximum = Arc::clone(&maximum);
        move |stream| {
            *maximum.lock().expect("maximum") = Some(stream.maximum_length());
            let _ = stream.write_bytes(&tint([0.0, 1.0, 0.0, 1.0]));
        }
    })
    .expect("binding");
    fixture.bind(Some(&probe));
    assert_eq!(fixture.center_pixel(), GREEN);
    let maximum = maximum
        .lock()
        .expect("maximum")
        .expect("SceneKit never asked for the buffer");
    assert!(maximum >= TINT_LEN);

    let length = maximum + 1;
    let mapping = unsafe {
        libc::mmap(
            core::ptr::null_mut(),
            length,
            libc::PROT_READ,
            libc::MAP_PRIVATE | libc::MAP_ANON,
            -1,
            0,
        )
    };
    assert_ne!(
        mapping,
        libc::MAP_FAILED,
        "could not reserve {length} bytes"
    );
    let address = mapping as usize;
    let results = WriteResults::default();
    let huge = ProgramBufferBinding::new({
        let results = Arc::clone(&results);
        move |stream| {
            let bytes = unsafe { std::slice::from_raw_parts(address as *const u8, length) };
            let result = stream.write_bytes(bytes).map_err(|error| error.to_string());
            results.lock().expect("results").push(result);
        }
    })
    .expect("binding");
    fixture.bind(Some(&huge));
    assert_eq!(fixture.center_pixel(), TRANSPARENT);
    unsafe { libc::munmap(mapping, length) };

    let results = results.lock().expect("results").clone();
    assert!(!results.is_empty());
    for result in &results {
        let message = result
            .as_ref()
            .expect_err("an oversized write must be rejected");
        assert!(message.contains("exceed"), "{message}");
    }
}
