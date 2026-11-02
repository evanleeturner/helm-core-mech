// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10059b2b: u8;
    static mut DAT_1005a93b: u8;
}

unsafe fn rd_a(i: u32) -> i32 {
    // SAFETY: the table is the game's own memory; the address is read as the original does.
    unsafe {
        mech_trace::rd!(
            "1005a93f",
            (&raw mut DAT_10059b2b)
                .wrapping_byte_add(i.wrapping_mul(4) as usize)
                .cast::<i32>(),
            read_unaligned
        )
    }
}

unsafe fn rd_b(i: u32) -> i32 {
    // SAFETY: the table is the game's own memory; the address is read as the original does.
    unsafe {
        mech_trace::rd!(
            "1005a93f",
            (&raw mut DAT_1005a93b)
                .wrapping_byte_add(i.wrapping_mul(4).wrapping_neg() as usize)
                .cast::<i32>(),
            read_unaligned
        )
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005a93f(mut param_1: u32, param_2: *mut i32, param_3: *mut i32) {
    mech_trace::enter!("1005a93f", param_1, param_2, param_3);
    while (param_1 as i32) < 0 {
        param_1 = param_1.wrapping_add(0xe10);
    }
    while 0xe10 < (param_1 as i32) {
        param_1 = param_1.wrapping_sub(0xe10);
    }
    let i_var1: i32;
    let i_var2: i32;
    // SAFETY: table reads are from the game's own memory; param_2 and param_3
    // point at the caller's writable ints.
    unsafe {
        if param_1 < 0x709 {
            if param_1 < 0x385 {
                i_var1 = rd_a(param_1);
                i_var2 = rd_b(param_1);
            } else {
                i_var1 = rd_a(0x708u32.wrapping_sub(param_1)).wrapping_neg();
                i_var2 = rd_b(0x708u32.wrapping_sub(param_1));
            }
        } else {
            let u_var3 = 0xe10u32.wrapping_sub(param_1);
            if u_var3 < 0x385 {
                i_var1 = rd_a(u_var3);
                i_var2 = rd_b(u_var3).wrapping_neg();
            } else {
                i_var1 = rd_a(0x708u32.wrapping_sub(u_var3)).wrapping_neg();
                i_var2 = rd_b(0x708u32.wrapping_sub(u_var3)).wrapping_neg();
            }
        }
        mech_trace::wr!("1005a93f", param_2, write_unaligned, i_var1);
        mech_trace::wr!("1005a93f", param_3, write_unaligned, i_var2);
    }
}
