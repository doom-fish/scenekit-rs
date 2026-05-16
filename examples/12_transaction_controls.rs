use scenekit::Transaction;

fn main() {
    Transaction::begin();
    Transaction::set_animation_duration(0.2);
    Transaction::set_disable_actions(true);
    assert!((Transaction::animation_duration() - 0.2).abs() < f64::EPSILON);
    assert!(Transaction::disable_actions());
    Transaction::commit();
    println!("✅ transaction configured");
}
