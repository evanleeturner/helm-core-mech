// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38e8: *mut u32;
    static mut DAT_100c7fa8: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100655f6(param_1: u32, param_2: u8) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10065786(param_1: u8) {
    mech_trace::enter!("10065786", param_1);
    // SAFETY: the globals and the callee are the game's own; the table
    // pointer addresses at least 0x26f5 dwords, written as the original does.
    unsafe {
        mech_trace::gw!("10065786", "100c7fa8", DAT_100c7fa8, 0);
        let mut local_c = mech_trace::gr!("10065786", "100b38e8", DAT_100b38e8);
        let mut local_8: i32 = 0;
        while local_8 < 0x26f5 {
            mech_trace::wr!("10065786", local_c, write_unaligned, 0xffff);
            local_c = local_c.wrapping_add(1);
            local_8 = local_8.wrapping_add(1);
        }
        local_8 = 0;
        while local_8 < (1u32.wrapping_shl((param_1 & 0x1f) as u32) as i32).wrapping_add(2) {
            mech_trace::cr!("10065786", "100655f6", FUN_100655f6(0xffff, local_8 as u8));
            local_8 = local_8.wrapping_add(1);
        }
    }
}
