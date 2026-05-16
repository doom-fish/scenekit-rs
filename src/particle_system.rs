use crate::ffi;
use crate::node::Node;
use crate::private::handle_type;

handle_type!(ParticleSystem);

impl ParticleSystem {
    #[must_use]
    pub fn new() -> Option<Self> {
        unsafe { Self::from_raw(ffi::scn_particle_system_new()) }
    }

    #[must_use]
    pub fn birth_rate(&self) -> f64 {
        unsafe { ffi::scn_particle_system_get_birth_rate(self.ptr) }
    }

    pub fn set_birth_rate(&self, birth_rate: f64) {
        unsafe { ffi::scn_particle_system_set_birth_rate(self.ptr, birth_rate) };
    }

    #[must_use]
    pub fn life_span(&self) -> f64 {
        unsafe { ffi::scn_particle_system_get_life_span(self.ptr) }
    }

    pub fn set_life_span(&self, life_span: f64) {
        unsafe { ffi::scn_particle_system_set_life_span(self.ptr, life_span) };
    }

    #[must_use]
    pub fn loops(&self) -> bool {
        unsafe { ffi::scn_particle_system_get_loops(self.ptr) }
    }

    pub fn set_loops(&self, loops: bool) {
        unsafe { ffi::scn_particle_system_set_loops(self.ptr, loops) };
    }
}

impl Node {
    pub fn add_particle_system(&self, particle_system: &ParticleSystem) {
        unsafe { ffi::scn_node_add_particle_system(self.ptr, particle_system.ptr) };
    }

    pub fn remove_all_particle_systems(&self) {
        unsafe { ffi::scn_node_remove_all_particle_systems(self.ptr) };
    }

    #[must_use]
    pub fn particle_system_count(&self) -> usize {
        unsafe { ffi::scn_node_particle_system_count(self.ptr) }
    }
}
