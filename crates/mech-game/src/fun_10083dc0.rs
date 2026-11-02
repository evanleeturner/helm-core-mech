// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b9f70: *mut i32;
    static mut DAT_100b9f88: *mut i32;
    static mut DAT_100b9f90: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10083e28();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10083dc0() {
    // SAFETY: the globals and callee are the game's own; the objects are COM-style
    // with a vtable pointer in their first word and the function at vtable offset 8.
    unsafe {
        DAT_100b9f90 = 0;
        if !DAT_100b9f70.is_null() {
            FUN_10083e28();
            if !DAT_100b9f88.is_null() {
                let obj = DAT_100b9f88;
                let vt = obj.read_unaligned();
                let addr = vt.wrapping_add(8) as usize as *const u32;
                let f: unsafe extern "C" fn(*mut i32) =
                    core::mem::transmute(addr.read_unaligned() as usize);
                f(obj);
                DAT_100b9f88 = core::ptr::null_mut();
            }
            let obj = DAT_100b9f70;
            let vt = obj.read_unaligned();
            let addr = vt.wrapping_add(8) as usize as *const u32;
            let f: unsafe extern "C" fn(*mut i32) =
                core::mem::transmute(addr.read_unaligned() as usize);
            f(obj);
            DAT_100b9f70 = core::ptr::null_mut();
        }
    }
}
