use scenekit::{Technique, View};

#[test]
fn test_technique_symbol_round_trip() {
    let view = View::new(32.0, 32.0).expect("view");
    let technique = Technique::minimal_draw_scene().expect("technique");
    technique.set_float_symbol("amplitude", 0.5);
    view.set_technique(Some(&technique));
    let attached = view.technique().expect("attached technique");
    assert_eq!(attached.dictionary_key_count(), 3);
    let amplitude = attached.float_symbol("amplitude").expect("amplitude");
    assert!((amplitude - 0.5).abs() < f64::EPSILON);
}
