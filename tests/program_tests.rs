use std::cell::RefCell;
use std::ffi::CString;
use std::rc::Rc;

use scenekit::{ffi, program, BufferFrequency, Program, ProgramBufferBinding, ProgramDelegate};

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

    let delegate_errors = Rc::new(RefCell::new(Vec::new()));
    let delegate = ProgramDelegate::new({
        let delegate_errors = Rc::clone(&delegate_errors);
        move |error| delegate_errors.borrow_mut().push(error.to_string())
    })
    .expect("program delegate");
    program.set_delegate(Some(&delegate));
    let error_message = CString::new("shader compilation failed").expect("error message cstring");
    unsafe {
        ffi::scn_program_test_invoke_delegate_handle_error(
            program.as_ptr(),
            error_message.as_ptr(),
        );
    }
    assert_eq!(
        delegate_errors.borrow().as_slice(),
        ["shader compilation failed"]
    );

    let bytes_written = Rc::new(RefCell::new(0_usize));
    let binding = ProgramBufferBinding::new({
        let bytes_written = Rc::clone(&bytes_written);
        move |buffer_stream| {
            let bytes = [1_u8, 2, 3, 4];
            buffer_stream.write_bytes(&bytes);
            *bytes_written.borrow_mut() = bytes.len();
        }
    })
    .expect("program buffer binding");
    program.set_buffer_binding("u_payload", BufferFrequency::PerFrame, Some(&binding));
    let binding_name = CString::new("u_payload").expect("binding name cstring");
    let bound_len = unsafe {
        ffi::scn_program_test_invoke_buffer_binding(program.as_ptr(), binding_name.as_ptr())
    };
    assert_eq!(bound_len, 4);
    assert_eq!(*bytes_written.borrow(), 4);

    program.set_buffer_binding("u_payload", BufferFrequency::PerFrame, None);
    program.set_delegate(None);
}
