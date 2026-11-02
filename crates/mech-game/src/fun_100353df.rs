// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aaf68: *mut i32;
    static mut DAT_10160c00: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100186c5(param_1: i32);
    fn FUN_1003cd85(param_1: *mut i32);
    fn FUN_10054925(param_1: i32);
    fn FUN_1005f8cf();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100353df(param_1: u8) {
    mech_trace::enter!("100353df", param_1);
    // SAFETY: the callees and globals are the game's own memory; the table at
    // DAT_10160c00 is read by byte offset as the original does.
    unsafe {
        if (param_1 & 1) != 0 {
            FUN_100186c5(mech_trace::gr!("100353df", "100aaf68", DAT_100aaf68) as i32);
        }
        FUN_1003cd85(mech_trace::gr!("100353df", "100aaf68", DAT_100aaf68));
        let mut local_c: i32 = 0;
        let mut local_8: i32 = mech_trace::gr!("100353df", "10160c00", DAT_10160c00);
        while local_8 != 0 {
            FUN_10054925(local_8);
            local_c = local_c.wrapping_add(1);
            local_8 = mech_trace::rd!(
                "100353df",
                (&raw const DAT_10160c00).byte_add(local_c.wrapping_mul(4) as usize),
                read_unaligned
            );
        }
        FUN_1005f8cf();
    }
}
