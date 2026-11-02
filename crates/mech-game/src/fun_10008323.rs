// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a520c: u8;
    static mut DAT_100a5360: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10007440(param_1: *mut i32, param_2: *mut i32, param_3: i32) -> bool;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10008323(param_1: i32) {
    mech_trace::enter!("10008323", param_1);
    // SAFETY: the globals and the callee are the game's own; only addresses
    // of the globals are taken, and the callee is called as the original does.
    unsafe {
        let a = (&raw mut DAT_100a520c).cast::<i32>();
        let b = (&raw mut DAT_100a5360)
            .wrapping_byte_offset(param_1.wrapping_mul(0xc) as isize)
            .cast::<i32>();
        mech_trace::cr!("10008323", "10007440", FUN_10007440(a, b, -1));
    }
}
