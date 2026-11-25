use super::{boundary, failure};
use shinki_emu_abi::{Bytes, Error, Header, Status, INVALID_ARGUMENT, INVALID_STATE, OK};
use std::ffi::c_void;

const ID: &[u8] = b"shinki.emu.android_context";
const VERSION: u32 = 1;
const CODE: &str = "RFVP_ANDROID_CONTEXT";

// Layout of the additive ABI v3 table, which postdates the standalone ABI pin.
#[repr(C)]
struct AndroidContextV1 {
    header: Header,
    initialize: Option<unsafe extern "C" fn(*mut c_void, *mut c_void, *mut Error) -> Status>,
}

static API: AndroidContextV1 = AndroidContextV1 {
    header: Header::of::<AndroidContextV1>(),
    initialize: Some(initialize),
};

pub(super) unsafe fn extension(name: Bytes, version: u32) -> Option<*const c_void> {
    (version == VERSION && unsafe { name.as_slice(ID.len()) } == Ok(ID))
        .then_some((&raw const API).cast())
}

unsafe extern "C" fn initialize(
    java_vm: *mut c_void,
    application: *mut c_void,
    error: *mut Error,
) -> Status {
    boundary(error, CODE, || {
        if java_vm.is_null() || application.is_null() {
            return Ok(failure(
                error,
                INVALID_ARGUMENT,
                CODE,
                "JavaVM and Application global reference are required",
            ));
        }
        if !unsafe { rfvp::android_context::initialize(java_vm, application) } {
            return Ok(failure(
                error,
                INVALID_STATE,
                CODE,
                "Android audio is already bound to a different context",
            ));
        }
        Ok(OK)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use shinki_emu_abi::ABI_VERSION;
    use std::ptr;

    #[test]
    fn context_negotiation_and_null_activation_leave_core_uninitialized() {
        let mut core = ptr::null();
        assert_eq!(unsafe { super::super::shinki_emu_query(ABI_VERSION, &mut core) }, OK);
        let core = unsafe { &*core };
        let mut table = ptr::null();
        assert_eq!(unsafe { core.extension.unwrap()(Bytes::new(ID), VERSION, &mut table) }, OK);
        let context = unsafe { &*table.cast::<AndroidContextV1>() };
        let mut error = Error::default();
        assert_eq!(
            unsafe { context.initialize.unwrap()(ptr::null_mut(), ptr::null_mut(), &mut error) },
            INVALID_ARGUMENT,
        );
        assert_eq!(
            unsafe { core.initialize.unwrap()(ptr::null(), &mut error) },
            INVALID_STATE,
        );
    }
}
