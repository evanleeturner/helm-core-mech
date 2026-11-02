// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3c9c: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100240be(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10081023(_param_1: u32, param_2: i32) {
    mech_trace::enter!("10081023", _param_1, param_2);
    // SAFETY: the callee and the global are the game's own; DAT_100b3c9c holds
    // an address of a structure whose field at offset 0x28 is written.
    unsafe {
        FUN_100240be(param_2);
        mech_trace::wr!(
            "10081023",
            (mech_trace::gr!("10081023", "100b3c9c", DAT_100b3c9c).wrapping_add(0x28) as *mut i32),
            write_unaligned,
            param_2
        );
    }
}
