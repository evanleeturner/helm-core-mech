// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a5534: *mut u32;
    static mut DAT_100a5538: u32;
    static mut DAT_100a553c: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn _AIL_end_sample_4(param_1: u32) -> u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10007dc0(param_1: *mut u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10007c93(param_1: i32) {
    mech_trace::enter!("10007c93", param_1);
    // SAFETY: the globals and callees are the game's own; the queue entries
    // are read and written at the same offsets as the original.
    unsafe {
        if !mech_trace::gr!("10007c93", "100a5534", DAT_100a5534).is_null()
            && mech_trace::gr!("10007c93", "100a553c", DAT_100a553c) == 0
        {
            let head = mech_trace::gr!("10007c93", "100a5534", DAT_100a5534);
            if 0 < mech_trace::rd!("10007c93", head.byte_add(4), read_unaligned) as i32
                && (mech_trace::rd!("10007c93", head.byte_add(24), read_unaligned) as i32) < 0x50
            {
                if mech_trace::gr!("10007c93", "100a5538", DAT_100a5538) != 0 {
                    mech_trace::cr!(
                        "10007c93",
                        "_AIL_end_sample_4",
                        _AIL_end_sample_4(mech_trace::gr!("10007c93", "100a5538", DAT_100a5538))
                    );
                }
                mech_trace::gw!(
                    "10007c93",
                    "100a5534",
                    DAT_100a5534,
                    mech_trace::cr!(
                        "10007c93",
                        "10007dc0",
                        FUN_10007dc0(mech_trace::gr!("10007c93", "100a5534", DAT_100a5534))
                    ) as *mut u32
                );
                if mech_trace::gr!("10007c93", "100a5534", DAT_100a5534).is_null() {
                    return;
                }
            }
            let mut local_8: *mut u32 = mech_trace::gr!("10007c93", "100a5534", DAT_100a5534);
            let mut local_10: *mut u32 = mech_trace::rd!(
                "10007c93",
                local_8.byte_add(0x4f).cast::<*mut u32>(),
                read_unaligned
            );
            while !local_10.is_null() {
                let mut b_var1 = true;
                if param_1 != 0
                    && (mech_trace::rd!("10007c93", local_10.byte_add(12), read_unaligned) != 0
                        || (0 < mech_trace::rd!("10007c93", local_10.byte_add(4), read_unaligned)
                            as i32
                            && 0x4f
                                < mech_trace::rd!("10007c93", local_10.byte_add(24), read_unaligned)
                                    as i32))
                {
                    b_var1 = false;
                }
                if b_var1 {
                    let u_var2 = mech_trace::cr!("10007c93", "10007dc0", FUN_10007dc0(local_10));
                    mech_trace::wr!(
                        "10007c93",
                        local_8.byte_add(0x4f).cast::<u32>(),
                        write_unaligned,
                        u_var2
                    );
                    local_10 = mech_trace::rd!(
                        "10007c93",
                        local_8.byte_add(0x4f).cast::<*mut u32>(),
                        read_unaligned
                    );
                } else {
                    local_8 = local_10;
                    local_10 = mech_trace::rd!(
                        "10007c93",
                        local_10.byte_add(0x4f).cast::<*mut u32>(),
                        read_unaligned
                    );
                }
            }
            mech_trace::gw!("10007c93", "100a553c", DAT_100a553c, 1);
        }
    }
}
