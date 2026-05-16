use std::path::Path;

use scenekit::{AudioPlayer, AudioSource, Node};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sound = Path::new("/System/Library/Sounds/Glass.aiff");
    if !sound.exists() {
        println!("ℹ️ system sound missing, skipping audio example");
        return Ok(());
    }

    let source = AudioSource::from_url(sound)?;
    source.set_volume(0.25);
    source.set_positional(false);
    source.load();
    let player = AudioPlayer::with_source(&source).ok_or("missing player")?;
    let node = Node::new().ok_or("missing node")?;
    node.add_audio_player(&player);
    assert!(player.source().is_some());
    println!("✅ audio player attached");
    Ok(())
}
