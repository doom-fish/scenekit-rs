use crate::ffi;

#[derive(Debug, Clone, Copy, Default)]
pub struct Transaction;

impl Transaction {
    pub fn begin() {
        unsafe { ffi::scn_transaction_begin() };
    }

    pub fn commit() {
        unsafe { ffi::scn_transaction_commit() };
    }

    pub fn flush() {
        unsafe { ffi::scn_transaction_flush() };
    }

    #[must_use]
    pub fn animation_duration() -> f64 {
        unsafe { ffi::scn_transaction_get_animation_duration() }
    }

    pub fn set_animation_duration(animation_duration: f64) {
        unsafe { ffi::scn_transaction_set_animation_duration(animation_duration) };
    }

    #[must_use]
    pub fn disable_actions() -> bool {
        unsafe { ffi::scn_transaction_get_disable_actions() }
    }

    pub fn set_disable_actions(disable_actions: bool) {
        unsafe { ffi::scn_transaction_set_disable_actions(disable_actions) };
    }
}
