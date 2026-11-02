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
    mech_trace::enter!("10083dc0");
    // SAFETY: the globals and callee are the game's own; the objects are COM-style
    // with a vtable pointer in their first word and the function at vtable offset 8.
    unsafe {
        mech_trace::gw!("10083dc0", "100b9f90", DAT_100b9f90, 0);
        if !mech_trace::gr!("10083dc0", "100b9f70", DAT_100b9f70).is_null() {
            FUN_10083e28();
            if !mech_trace::gr!("10083dc0", "100b9f88", DAT_100b9f88).is_null() {
                let obj = mech_trace::gr!("10083dc0", "100b9f88", DAT_100b9f88);
                let vt = mech_trace::rd!("10083dc0", obj, read_unaligned);
                let addr = vt.wrapping_add(8) as usize as *const u32;
                let f: unsafe extern "C" fn(*mut i32) = core::mem::transmute(mech_trace::rd!(
                    "10083dc0",
                    addr,
                    read_unaligned
                )
                    as usize);
                mech_trace::cr!("10083dc0", "ind", f(obj));
                mech_trace::gw!("10083dc0", "100b9f88", DAT_100b9f88, core::ptr::null_mut());
            }
            let obj = mech_trace::gr!("10083dc0", "100b9f70", DAT_100b9f70);
            let vt = mech_trace::rd!("10083dc0", obj, read_unaligned);
            let addr = vt.wrapping_add(8) as usize as *const u32;
            let f: unsafe extern "C" fn(*mut i32) =
                core::mem::transmute(mech_trace::rd!("10083dc0", addr, read_unaligned) as usize);
            mech_trace::cr!("10083dc0", "ind", f(obj));
            mech_trace::gw!("10083dc0", "100b9f70", DAT_100b9f70, core::ptr::null_mut());
        }
    }
}
