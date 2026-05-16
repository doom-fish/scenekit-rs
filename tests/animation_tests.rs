use scenekit::{Animation, AnimationPlayer, Node};

#[test]
fn test_animation_player_round_trip() {
    let animation = Animation::opacity(0.0, 1.0, 0.5).expect("animation");
    animation.set_repeat_count(3.0);
    animation.set_autoreverses(true);
    animation.set_uses_scene_time_base(true);
    let player = AnimationPlayer::new(&animation).expect("player");
    player.set_speed(1.25);
    player.set_paused(true);
    let node = Node::new().expect("node");
    node.add_animation_player(&player, "fade");
    let fetched = node.animation_player("fade").expect("fetched player");
    assert!(fetched.is_paused());
    assert!((fetched.speed() - 1.25).abs() < f64::EPSILON);
    assert!((fetched.animation().expect("animation").duration() - 0.5).abs() < f64::EPSILON);
}
