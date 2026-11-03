// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1009c078: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1006f200(param_1: u32) -> *mut c_void;
}

#[allow(non_snake_case)]
unsafe extern "thiscall" {
    fn FUN_10040590(
        this: *mut c_void,
        param_1: *mut i8,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
    ) -> *mut i32;
    fn FUN_10013330(this: *mut c_void, param_1: u32, param_2: i32);
}

#[allow(non_snake_case)]
unsafe extern "fastcall" {
    fn FUN_100402b0(param_1: *mut u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "thiscall" fn FUN_10050b80(
    this: *mut c_void,
    param_1: i32,
    param_2: i32,
    mut param_3: *mut i8,
    param_4: i32,
) -> *mut i32 {
    mech_trace::enter!("10050b80", this, param_1, param_2, param_3, param_4);
    // SAFETY: the callees and the global are the game's own; `this` points at
    // a live object whose field at 0x410 is read where the original does.
    unsafe {
        if param_3.is_null() {
            param_3 = &raw mut DAT_1009c078;
        }
        let this_00 = mech_trace::cr!("10050b80", "1006f200", FUN_1006f200(0x3e));
        let pi_var1: *mut i32 = if this_00.is_null() {
            core::ptr::null_mut()
        } else {
            mech_trace::cr!(
                "10050b80",
                "10040590",
                FUN_10040590(this_00, param_3, param_1, param_2, param_4, this as i32)
            )
        };
        let inner = mech_trace::rd!(
            "10050b80",
            this.byte_add(0x410).cast::<*mut c_void>(),
            read_unaligned
        );
        FUN_10013330(inner, pi_var1 as u32, 0);
        FUN_100402b0(pi_var1 as *mut u32);
        mech_trace::rt!("10050b80", pi_var1)
    }
}
