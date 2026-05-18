use crate::ffi;
use crate::node::Node;
use crate::private::handle_type;

handle_type!(ParticleSystem);

impl ParticleSystem {
    /// Creates a wrapped `SCNParticleSystem` instance.
    #[must_use]
    pub fn new() -> Option<Self> {
        unsafe { Self::from_raw(ffi::scn_particle_system_new()) }
    }

    /// Mirrors `SCNParticleSystem.birthRate`.
    #[must_use]
    pub fn birth_rate(&self) -> f64 {
        unsafe { ffi::scn_particle_system_get_birth_rate(self.ptr) }
    }

    /// Sets the `SCNParticleSystem.birthRate` member.
    pub fn set_birth_rate(&self, birth_rate: f64) {
        unsafe { ffi::scn_particle_system_set_birth_rate(self.ptr, birth_rate) };
    }

    /// Mirrors `SCNParticleSystem.lifeSpan`.
    #[must_use]
    pub fn life_span(&self) -> f64 {
        unsafe { ffi::scn_particle_system_get_life_span(self.ptr) }
    }

    /// Sets the `SCNParticleSystem.lifeSpan` member.
    pub fn set_life_span(&self, life_span: f64) {
        unsafe { ffi::scn_particle_system_set_life_span(self.ptr, life_span) };
    }

    /// Mirrors `SCNParticleSystem.loops`.
    #[must_use]
    pub fn loops(&self) -> bool {
        unsafe { ffi::scn_particle_system_get_loops(self.ptr) }
    }

    /// Sets the `SCNParticleSystem.loops` member.
    pub fn set_loops(&self, loops: bool) {
        unsafe { ffi::scn_particle_system_set_loops(self.ptr, loops) };
    }
}

impl Node {
    /// Mirrors `SCNNode.addParticleSystem`.
    pub fn add_particle_system(&self, particle_system: &ParticleSystem) {
        unsafe { ffi::scn_node_add_particle_system(self.ptr, particle_system.ptr) };
    }

    /// Mirrors `SCNNode.removeAllParticleSystems`.
    pub fn remove_all_particle_systems(&self) {
        unsafe { ffi::scn_node_remove_all_particle_systems(self.ptr) };
    }

    /// Mirrors `SCNNode.particleSystemCount`.
    #[must_use]
    pub fn particle_system_count(&self) -> usize {
        unsafe { ffi::scn_node_particle_system_count(self.ptr) }
    }
}
