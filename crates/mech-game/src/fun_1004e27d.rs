// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa1ac: i32;
    static mut DAT_100adf34: i32;
    static mut DAT_100b0710: i32;
    static mut DAT_10123240: [u8; 0];
    static mut DAT_1012327b: [u8; 0];
    static mut DAT_101232ee: [u8; 0];
    static mut DAT_101658a2: [u16; 0];
    static mut DAT_10165922: [u8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1004e27d() {
    mech_trace::enter!("1004e27d");
    // SAFETY: all globals and the memory they address belong to the game;
    // reads and writes happen at the same offsets and in the same order as the original.
    unsafe {
        let mut local_c: i32 = -0x7fff;
        let mut b_var2 = false;
        if mech_trace::gr!("1004e27d", "100aa1ac", DAT_100aa1ac) != 0
            && mech_trace::rd!(
                "1004e27d",
                (mech_trace::gr!("1004e27d", "100adf34", DAT_100adf34).wrapping_add(10)
                    as *const u8),
                read
            ) == 0
        {
            let mut local_34: i32 = 0;
            let mut b_var1 = false;
            let mut local_18: i32 = 0;
            loop {
                let base = (mech_trace::gr!("1004e27d", "100b0710", DAT_100b0710) as u32)
                    .wrapping_mul(0x3c0a);
                let cnt = mech_trace::rd!(
                    "1004e27d",
                    (&raw mut DAT_10123240)
                        .cast::<u8>()
                        .byte_add(base as usize)
                        .cast::<i32>(),
                    read_unaligned
                );
                if !(local_18 < cnt) {
                    break;
                }
                let i_var3 = base.wrapping_add((local_18 as u32).wrapping_mul(0x13f)) as usize;
                let a = mech_trace::rd!(
                    "1004e27d",
                    (&raw mut DAT_101232ee).cast::<u8>().byte_add(i_var3),
                    read
                );
                local_34 = (local_34 as u32).wrapping_add(a as u32) as i32;
                let b = mech_trace::rd!(
                    "1004e27d",
                    (&raw mut DAT_1012327b).cast::<u8>().byte_add(i_var3),
                    read
                );
                if a != 0 && b == 2 {
                    b_var1 = true;
                }
                local_18 = local_18.wrapping_add(1);
            }
            if b_var1 && local_34 == 1 {
                b_var2 = true;
            }
            if b_var2 {
                let mut local_14: i32 = 0;
                let mut local_1c: i32 = 0;
                while local_1c < 8 {
                    let mut local_28: i32 = 0;
                    let mut local_24: i32 = 0;
                    while local_24 < 8 {
                        let idx = (local_1c * 8 + local_24) as usize;
                        let v = mech_trace::rd!(
                            "1004e27d",
                            (&raw mut DAT_101658a2)
                                .cast::<u8>()
                                .byte_add(idx * 2)
                                .cast::<u16>(),
                            read_unaligned
                        ) as u32;
                        let u_var4 = if local_24 == local_1c {
                            0u32.wrapping_sub(v)
                        } else {
                            v
                        };
                        local_28 = (local_28 as u32).wrapping_add(u_var4) as i32;
                        local_24 += 1;
                    }
                    if local_c < local_28 {
                        local_c = local_28;
                        local_14 = local_1c;
                    } else if local_c == local_28 {
                        local_14 = -2;
                    }
                    local_1c += 1;
                }
                mech_trace::wr!(
                    "1004e27d",
                    (&raw mut DAT_10165922).cast::<i32>(),
                    write_unaligned,
                    local_14
                );
            }
        }
    }
}
