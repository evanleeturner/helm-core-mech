// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100af17c: u32;
    static mut DAT_100f3120: [u8; 0];
    static mut PTR_DAT_100af0c8: *mut i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1008668b(param_1: i32, param_2: u32, param_3: *mut i32) -> *mut u16;
    fn FUN_1008664f(param_1: u32, param_2: *mut i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1006099e(param_1: u32, param_2: i32) -> u32 {
    mech_trace::enter!("1006099e", param_1, param_2);
    // SAFETY: the globals and callees are the game's own; the slot at
    // DAT_100f3120 + param_2 * 4 is read and written as the original does.
    unsafe {
        let slot = (&raw mut DAT_100f3120)
            .cast::<u8>()
            .byte_offset(param_2.wrapping_mul(4) as isize)
            .cast::<u32>();
        let u_var1 = mech_trace::rd!("1006099e", slot, read_unaligned);
        mech_trace::wr!("1006099e", slot, write_unaligned, param_1);
        mech_trace::cr!(
            "1006099e",
            "1008668b",
            FUN_1008668b(
                mech_trace::gr!("1006099e", "100af17c", DAT_100af17c) as i32,
                param_1,
                mech_trace::gr!("1006099e", "PTR_DAT_100af0c8", PTR_DAT_100af0c8)
            )
        );
        mech_trace::cr!(
            "1006099e",
            "1008664f",
            FUN_1008664f(
                param_1,
                mech_trace::gr!("1006099e", "PTR_DAT_100af0c8", PTR_DAT_100af0c8)
            )
        );
        mech_trace::rt!("1006099e", u_var1)
    }
}
