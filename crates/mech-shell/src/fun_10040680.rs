// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10098224: *mut c_void;
}

#[allow(non_snake_case)]
unsafe extern "fastcall" {
    fn FUN_10040640(owner_object: i32) -> u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapFree(h_heap: *mut c_void, dw_flags: u32, lp_mem: *mut c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "fastcall" fn FUN_10040680(owner_object: i32) {
    mech_trace::enter!("10040680", owner_object);
    // SAFETY: the callees and the global are the game's own; owner_object
    // points at the caller's object, whose field at +0x19 is read and
    // cleared where the original does.
    unsafe {
        mech_trace::cr!("10040680", "10040640", FUN_10040640(owner_object));
        let field = (owner_object as u32).wrapping_add(0x19) as *mut *mut c_void;
        let block = mech_trace::rd!("10040680", field, read_unaligned);
        if !block.is_null() {
            mech_trace::cr!(
                "10040680",
                "HeapFree",
                HeapFree(
                    mech_trace::gr!("10040680", "10098224", DAT_10098224),
                    1,
                    mech_trace::rd!("10040680", field, read_unaligned)
                )
            );
            mech_trace::wr!("10040680", field, write_unaligned, core::ptr::null_mut());
        }
    }
}
