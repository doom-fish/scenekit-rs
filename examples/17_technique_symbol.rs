use scenekit::{Technique, View};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let view = View::new(64.0, 64.0).ok_or("missing view")?;
    let technique = Technique::minimal_draw_scene().ok_or("missing technique")?;
    technique.set_float_symbol("amplitude", 0.75);
    view.set_technique(Some(&technique));
    let attached = view.technique().ok_or("missing attached technique")?;
    let amplitude = attached
        .float_symbol("amplitude")
        .ok_or("missing amplitude")?;
    assert!((amplitude - 0.75).abs() < f64::EPSILON);
    println!("✅ technique attached");
    Ok(())
}
