//! Process-lifetime Android context for this core's CPAL/Oboe instance.

use std::ffi::c_void;
use std::sync::OnceLock;

static BINDING: OnceLock<(usize, usize)> = OnceLock::new();

/// Bind the audio backend to Android. Repeating the same binding succeeds.
///
/// # Safety
/// `java_vm` must be the process JavaVM and `application` a global JNI
/// reference to the Application context. Both must remain valid for the life
/// of the process. The caller must not initialize or release `ndk_context`
/// separately in this library.
pub unsafe fn initialize(java_vm: *mut c_void, application: *mut c_void) -> bool {
    if java_vm.is_null() || application.is_null() {
        return false;
    }
    let binding = (java_vm.addr(), application.addr());
    *BINDING.get_or_init(|| {
        unsafe { ndk_context::initialize_android_context(java_vm, application) };
        binding
    }) == binding
}

pub fn is_initialized() -> bool {
    BINDING.get().is_some()
}
