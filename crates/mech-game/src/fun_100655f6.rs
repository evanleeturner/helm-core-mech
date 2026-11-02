// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38e0: u32;
    static mut DAT_100b38e4: u32;
    static mut DAT_100b38e8: u32;
    static mut DAT_100c7fa8: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100655f6(param_1: u32, param_2: u8) -> u32 {
    mech_trace::enter!("100655f6", param_1, param_2);
    // SAFETY: the globals are the game's own; the tables they point at are the
    // game's buffers, accessed at the same offsets as the original.
    unsafe {
        mech_trace::rt!(
            "100655f6",
            if mech_trace::gr!("100655f6", "100c7fa8", DAT_100c7fa8) < 0x1000 {
                let mut local_8: u32 = (u32::from(param_2) << 8) ^ param_1;
                loop {
                    local_8 = local_8 % 0x26f5;
                    let p = mech_trace::gr!("100655f6", "100b38e8", DAT_100b38e8)
                        .wrapping_add(local_8.wrapping_mul(4))
                        as *const i32;
                    if mech_trace::rd!("100655f6", p, read_unaligned) == 0xffff {
                        break;
                    }
                    local_8 = local_8.wrapping_add(0x7f7);
                }
                let p = mech_trace::gr!("100655f6", "100b38e8", DAT_100b38e8)
                    .wrapping_add(local_8.wrapping_mul(4)) as *mut u32;
                mech_trace::wr!(
                    "100655f6",
                    p,
                    write_unaligned,
                    mech_trace::gr!("100655f6", "100c7fa8", DAT_100c7fa8)
                );
                let b = mech_trace::gr!("100655f6", "100c7fa8", DAT_100c7fa8)
                    .wrapping_add(mech_trace::gr!("100655f6", "100b38e0", DAT_100b38e0))
                    as *mut u8;
                mech_trace::wr!("100655f6", b, write, param_2);
                let q = mech_trace::gr!("100655f6", "100b38e4", DAT_100b38e4).wrapping_add(
                    mech_trace::gr!("100655f6", "100c7fa8", DAT_100c7fa8).wrapping_mul(4),
                ) as *mut u32;
                if param_1 == 0xffff {
                    mech_trace::wr!("100655f6", q, write_unaligned, 0xffff);
                } else {
                    mech_trace::wr!("100655f6", q, write_unaligned, param_1);
                }
                let u_var1 = mech_trace::gr!("100655f6", "100c7fa8", DAT_100c7fa8);
                mech_trace::gw!(
                    "100655f6",
                    "100c7fa8",
                    DAT_100c7fa8,
                    mech_trace::gr!("100655f6", "100c7fa8", DAT_100c7fa8).wrapping_add(1)
                );
                u_var1
            } else {
                0xffff
            }
        )
    }
}
