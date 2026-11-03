// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10017ec0(forwarded_arg: i32, param_2: i32, param_3: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "thiscall" fn FUN_1005ed70(object: *mut core::ffi::c_void, forwarded_arg: i32) {
    mech_trace::enter!("1005ed70", object, forwarded_arg);
    // SAFETY: object points at the caller's object, whose first field is read
    // as an i32; the callee is the game's own.
    unsafe {
        let first = mech_trace::rd!("1005ed70", object.cast::<i32>(), read_unaligned);
        FUN_10017ec0(first, forwarded_arg, 0);
    }
}
