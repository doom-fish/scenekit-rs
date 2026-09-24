use std::sync::{Arc, Mutex};

use scenekit::{program, BufferFrequency, Program, ProgramBufferBinding, ProgramDelegate};

mod common;

#[test]
fn test_program_area_round_trip() {
    let program = Program::new().expect("program");
    program.set_vertex_shader(Some("void main() {}"));
    program.set_fragment_shader(Some("void main() {}"));
    program.set_geometry_shader(Some("void main() {}"));
    program.set_tessellation_control_shader(Some("void main() {}"));
    program.set_tessellation_evaluation_shader(Some("void main() {}"));
    program.set_vertex_function_name(Some("vertex_main"));
    program.set_fragment_function_name(Some("fragment_main"));
    program.set_opaque(false);
    program.set_semantic(Some("TEXCOORD0"), "a_texcoord", Some(1));

    assert_eq!(program.vertex_shader().as_deref(), Some("void main() {}"));
    assert_eq!(program.fragment_shader().as_deref(), Some("void main() {}"));
    assert_eq!(program.geometry_shader().as_deref(), Some("void main() {}"));
    assert_eq!(
        program.tessellation_control_shader().as_deref(),
        Some("void main() {}")
    );
    assert_eq!(
        program.tessellation_evaluation_shader().as_deref(),
        Some("void main() {}")
    );
    assert_eq!(
        program.vertex_function_name().as_deref(),
        Some("vertex_main")
    );
    assert_eq!(
        program.fragment_function_name().as_deref(),
        Some("fragment_main")
    );
    assert!(!program.opaque());
    assert_eq!(
        program.semantic_for_symbol("a_texcoord").as_deref(),
        Some("TEXCOORD0")
    );
    assert!(!program::program_mapping_channel_key().is_empty());
    assert!(!program::shader_modifier_entry_point_surface().is_empty());

    let delegate_errors = Arc::new(Mutex::new(Vec::new()));
    let delegate = ProgramDelegate::new({
        let delegate_errors = Arc::clone(&delegate_errors);
        move |error| {
            delegate_errors
                .lock()
                .expect("errors")
                .push(error.to_string());
        }
    })
    .expect("program delegate");
    program.set_delegate(Some(&delegate));
    common::autoreleasepool(|| {
        let delegate = common::send_object(program.as_ptr(), c"delegate");
        assert!(!delegate.is_null());
        common::send_with_two_objects(
            delegate,
            c"program:handleError:",
            program.as_ptr(),
            common::ns_error(c"scenekit-rs-program-tests"),
        );
    });
    let errors = delegate_errors.lock().expect("errors").clone();
    assert_eq!(errors.len(), 1);
    assert!(
        errors[0].contains("scenekit-rs-program-tests"),
        "{errors:?}"
    );

    let binding = ProgramBufferBinding::new(|buffer_stream| {
        let _ = buffer_stream.write_bytes(&[1_u8, 2, 3, 4]);
    })
    .expect("program buffer binding");
    program.set_buffer_binding("u_payload", BufferFrequency::PerFrame, Some(&binding));
    drop(binding);
    program.set_buffer_binding("u_payload", BufferFrequency::PerFrame, None);
    program.set_delegate(None);
}
