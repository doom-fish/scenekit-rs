use scenekit::{
    program, BufferFrequency, Material, Program, ProgramBufferBinding, ProgramDelegate, Shadable,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program = Program::new().ok_or("missing program")?;
    program.set_vertex_shader(Some("void main() {}"));
    program.set_fragment_shader(Some("void main() {}"));
    program.set_vertex_function_name(Some("vertex_main"));
    program.set_fragment_function_name(Some("fragment_main"));
    program.set_semantic(Some("TEXCOORD0"), "a_texcoord", Some(1));

    let delegate = ProgramDelegate::new(|error| eprintln!("program error: {error}"))
        .ok_or("missing program delegate")?;
    program.set_delegate(Some(&delegate));

    let binding =
        ProgramBufferBinding::new(|buffer_stream| buffer_stream.write_bytes(&[1, 2, 3, 4]))
            .ok_or("missing program buffer binding")?;
    program.set_buffer_binding("u_payload", BufferFrequency::PerFrame, Some(&binding));

    let material = Material::new().ok_or("missing material")?;
    material.set_program(Some(&program));
    material.set_shader_modifier(
        &program::shader_modifier_entry_point_surface(),
        Some("#pragma body\n_output.color.rgb = 1.0 - _surface.diffuse.rgb;"),
    );
    println!("✅ configured program and shader modifier");
    Ok(())
}
