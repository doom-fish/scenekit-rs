use std::path::Path;

use crate::error::SceneKitError;
use crate::ffi;
use crate::node::Node;
use crate::private::{cstring_from_path, handle_type};

handle_type!(AudioSource);
handle_type!(AudioPlayer);

impl AudioSource {
    /// Mirrors `SCNAudioSource.fromUrl`.
    pub fn from_url(path: impl AsRef<Path>) -> Result<Self, SceneKitError> {
        let path = cstring_from_path(path.as_ref())
            .ok_or_else(|| SceneKitError::new("path contains an interior NUL byte"))?;
        let ptr = unsafe { ffi::scn_audio_source_new_url(path.as_ptr()) };
        if ptr.is_null() {
            Err(SceneKitError::new("SCNAudioSource(url:) returned nil"))
        } else {
            Ok(unsafe { Self::from_raw_unchecked(ptr) })
        }
    }

    /// Mirrors `SCNAudioSource.volume`.
    #[must_use]
    pub fn volume(&self) -> f32 {
        unsafe { ffi::scn_audio_source_get_volume(self.ptr) }
    }

    /// Sets the `SCNAudioSource.volume` member.
    pub fn set_volume(&self, volume: f32) {
        unsafe { ffi::scn_audio_source_set_volume(self.ptr, volume) };
    }

    /// Returns the `SCNAudioSource.isPositional` value.
    #[must_use]
    pub fn is_positional(&self) -> bool {
        unsafe { ffi::scn_audio_source_get_positional(self.ptr) }
    }

    /// Sets the `SCNAudioSource.positional` member.
    pub fn set_positional(&self, positional: bool) {
        unsafe { ffi::scn_audio_source_set_positional(self.ptr, positional) };
    }

    /// Mirrors `SCNAudioSource.loops`.
    #[must_use]
    pub fn loops(&self) -> bool {
        unsafe { ffi::scn_audio_source_get_loops(self.ptr) }
    }

    /// Sets the `SCNAudioSource.loops` member.
    pub fn set_loops(&self, loops: bool) {
        unsafe { ffi::scn_audio_source_set_loops(self.ptr, loops) };
    }

    /// Mirrors `SCNAudioSource.load`.
    pub fn load(&self) {
        unsafe { ffi::scn_audio_source_load(self.ptr) };
    }
}

impl AudioPlayer {
    /// Mirrors `SCNAudioPlayer.withSource`.
    #[must_use]
    pub fn with_source(source: &AudioSource) -> Option<Self> {
        unsafe { Self::from_raw(ffi::scn_audio_player_new(source.ptr)) }
    }

    /// Mirrors `SCNAudioPlayer.source`.
    #[must_use]
    pub fn source(&self) -> Option<AudioSource> {
        unsafe { AudioSource::from_raw(ffi::scn_audio_player_source(self.ptr)) }
    }
}

impl Node {
    /// Mirrors `SCNNode.addAudioPlayer`.
    pub fn add_audio_player(&self, player: &AudioPlayer) {
        unsafe { ffi::scn_node_add_audio_player(self.ptr, player.ptr) };
    }

    /// Mirrors `SCNNode.removeAllAudioPlayers`.
    pub fn remove_all_audio_players(&self) {
        unsafe { ffi::scn_node_remove_all_audio_players(self.ptr) };
    }

    /// Mirrors `SCNNode.audioPlayerCount`.
    #[must_use]
    pub fn audio_player_count(&self) -> usize {
        unsafe { ffi::scn_node_audio_player_count(self.ptr) }
    }
}
