// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10160c00: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1003b2f0(param_1: *mut core::ffi::c_void);
    fn FUN_1003cd85(param_1: *mut i32);
    fn FUN_10054925(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10054d99(param_1: u32, param_2: *mut i32) {
    let _ = param_1;
    // SAFETY: the callees and the global table are the game's own; the table
    // is read by byte offset from DAT_10160c00 until a zero entry, as the
    // original does.
    unsafe {
        FUN_1003b2f0(param_2.cast());
        FUN_1003cd85(param_2);
        let mut local_c: i32 = 0;
        let mut local_8: i32 = DAT_10160c00;
        while local_8 != 0 {
            FUN_10054925(local_8);
            local_c = local_c.wrapping_add(1);
            local_8 = (&raw mut DAT_10160c00)
                .byte_add((local_c.wrapping_mul(4)) as usize)
                .read_unaligned();
        }
    }
}
