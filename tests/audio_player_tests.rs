use scenekit::{AudioPlayer, AudioSource, Node};

mod common;

#[test]
fn test_audio_source_and_player_attach_to_node() {
    let sound = common::system_sound_path();
    if !sound.exists() {
        return;
    }

    let source = AudioSource::from_url(sound).expect("audio source");
    source.set_volume(0.5);
    source.set_positional(false);
    source.set_loops(false);
    source.load();
    let player = AudioPlayer::with_source(&source).expect("audio player");
    let node = Node::new().expect("node");
    node.add_audio_player(&player);
    assert!((source.volume() - 0.5).abs() < f32::EPSILON);
    assert!(!source.is_positional());
    assert!(!source.loops());
    assert!(player.source().is_some());
    node.remove_all_audio_players();
}
