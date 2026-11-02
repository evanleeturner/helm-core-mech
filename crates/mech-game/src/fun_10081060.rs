// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3c9c: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1007d0a4(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10081060(_param_1: u32, param_2: i32) {
    mech_trace::enter!("10081060", _param_1, param_2);
    // SAFETY: the callee and the global are the game's own; the global holds
    // an address of a structure whose field at offset 0x2c is written.
    unsafe {
        FUN_1007d0a4(param_2);
        mech_trace::wr!(
            "10081060",
            ((mech_trace::gr!("10081060", "100b3c9c", DAT_100b3c9c) as usize).wrapping_add(0x2c)
                as *mut i32),
            write_unaligned,
            param_2
        );
    }
}
