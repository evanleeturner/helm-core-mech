// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3c9c: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1003106f(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1008109d(_param_1: u32, param_2: i32) {
    mech_trace::enter!("1008109d", _param_1, param_2);
    // SAFETY: the callee and the global are the game's own; DAT_100b3c9c holds
    // a pointer to a structure whose field at offset 0x30 is written.
    unsafe {
        FUN_1003106f(param_2);
        mech_trace::wr!(
            "1008109d",
            (mech_trace::gr!("1008109d", "100b3c9c", DAT_100b3c9c).wrapping_add(0x30) as *mut i32),
            write_unaligned,
            param_2
        );
    }
}
