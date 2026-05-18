use crate::ffi;

/// Wraps `SCNTransaction`.
#[derive(Debug, Clone, Copy, Default)]
pub struct Transaction;

impl Transaction {
    /// Mirrors `SCNTransaction.begin`.
    pub fn begin() {
        unsafe { ffi::scn_transaction_begin() };
    }

    /// Mirrors `SCNTransaction.commit`.
    pub fn commit() {
        unsafe { ffi::scn_transaction_commit() };
    }

    /// Mirrors `SCNTransaction.flush`.
    pub fn flush() {
        unsafe { ffi::scn_transaction_flush() };
    }

    /// Mirrors `SCNTransaction.animationDuration`.
    #[must_use]
    pub fn animation_duration() -> f64 {
        unsafe { ffi::scn_transaction_get_animation_duration() }
    }

    /// Sets the `SCNTransaction.animationDuration` member.
    pub fn set_animation_duration(animation_duration: f64) {
        unsafe { ffi::scn_transaction_set_animation_duration(animation_duration) };
    }

    /// Mirrors `SCNTransaction.disableActions`.
    #[must_use]
    pub fn disable_actions() -> bool {
        unsafe { ffi::scn_transaction_get_disable_actions() }
    }

    /// Sets the `SCNTransaction.disableActions` member.
    pub fn set_disable_actions(disable_actions: bool) {
        unsafe { ffi::scn_transaction_set_disable_actions(disable_actions) };
    }
}
