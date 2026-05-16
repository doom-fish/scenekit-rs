use scenekit::Transaction;

#[test]
fn test_transaction_global_properties() {
    Transaction::begin();
    Transaction::set_animation_duration(0.15);
    Transaction::set_disable_actions(true);
    assert!((Transaction::animation_duration() - 0.15).abs() < f64::EPSILON);
    assert!(Transaction::disable_actions());
    Transaction::commit();
    Transaction::flush();
}
