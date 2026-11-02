// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ada50: i32;
    static mut DAT_10168930: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002f727(param_1: i32) -> u8;
    fn FUN_1003052d(param_1: i32) -> *mut u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10030fd2(param_1: i32) {
    mech_trace::enter!("10030fd2", param_1);
    // SAFETY: the globals and callees are the game's own; the table entry at
    // param_1 * 0x34 is in range because param_1 was checked against the count.
    unsafe {
        if -1 < param_1 && param_1 < mech_trace::gr!("10030fd2", "100ada50", DAT_100ada50) {
            let p = (&raw mut DAT_10168930)
                .cast::<u8>()
                .byte_add(param_1.wrapping_mul(0x34) as usize);
            mech_trace::cr!(
                "10030fd2",
                "1002f727",
                FUN_1002f727(mech_trace::rd!("10030fd2", p.cast::<i32>(), read_unaligned))
            );
            mech_trace::cr!("10030fd2", "1003052d", FUN_1003052d(param_1));
        }
    }
}
