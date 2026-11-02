// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38e0: u32;
    static mut DAT_100b38e4: u32;
    static mut DAT_100b38e8: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100656d3(param_1: u32, param_2: u8) -> u32 {
    mech_trace::enter!("100656d3", param_1, param_2);
    if param_1 == 0xffff {
        return mech_trace::rt!("100656d3", param_2 as u32);
    }
    let mut local_8: u32 = ((param_2 as u32).wrapping_shl(8)) ^ param_1;
    loop {
        local_8 %= 0x26f5;
        // SAFETY: the tables are the game's own memory; the indices are
        // read where the original reads them.
        unsafe {
            let u_var1 = mech_trace::rd!(
                "100656d3",
                (mech_trace::gr!("100656d3", "100b38e8", DAT_100b38e8)
                    .wrapping_add(local_8.wrapping_mul(4)) as *const u32),
                read_unaligned
            );
            if u_var1 == 0xffff {
                break;
            }
            if mech_trace::rd!(
                "100656d3",
                (mech_trace::gr!("100656d3", "100b38e4", DAT_100b38e4)
                    .wrapping_add(u_var1.wrapping_mul(4)) as *const u32),
                read_unaligned
            ) == param_1
                && mech_trace::rd!(
                    "100656d3",
                    (mech_trace::gr!("100656d3", "100b38e0", DAT_100b38e0).wrapping_add(u_var1)
                        as *const u8),
                    read
                ) == param_2
            {
                return mech_trace::rt!("100656d3", u_var1);
            }
        }
        local_8 = local_8.wrapping_add(0x7f7);
    }
    mech_trace::rt!("100656d3", 0xffff)
}
