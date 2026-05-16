use scenekit::{Animation, AnimationPlayer, Node};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let animation = Animation::opacity(0.0, 1.0, 0.25).ok_or("missing animation")?;
    animation.set_repeat_count(2.0);
    animation.set_autoreverses(true);
    animation.set_uses_scene_time_base(true);
    let player = AnimationPlayer::new(&animation).ok_or("missing animation player")?;
    player.set_speed(1.5);
    player.set_paused(true);
    let node = Node::new().ok_or("missing node")?;
    node.add_animation_player(&player, "fade");
    let fetched = node
        .animation_player("fade")
        .ok_or("missing fetched player")?;
    assert!(fetched.is_paused());
    println!("✅ animation player configured");
    Ok(())
}
