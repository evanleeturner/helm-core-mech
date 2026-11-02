// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a73d0: [u8; 1];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000bf23(
    param_1: *mut u32,
    param_2: *mut i32,
    param_3: *mut i32,
) -> u32 {
    mech_trace::enter!("1000bf23", param_1, param_2, param_3);
    let mut param_2 = param_2;
    let mut param_3 = param_3;
    let mut l: [i16; 70] = [
        0x1b58, 1, 0, 0x1b8a, 1, 0, 0x1bbc, -60, 0, 0x1bee, -60, 0, 0x1c20, -50, 0, 0x1c52, 1, 0,
        0x1c84, 1, 0, 0x1cb6, 1, 0, 0x1ce8, 1, 0, 0x1d1a, 1, 0, 0x1d4c, 1, 0, 0x1d7e, 1, 0, 0x1db0,
        1, 0, 0x1de2, 1, 0, 0x1e14, 2, 0, 0x1e46, 0, 0, 0x1e78, 2, 0, 0x1eaa, 3, 0, 0x1edc, 2, 0,
        0x1f40, 1, 0, 0x2134, 1, 0, 0x2328, 0, 0, 0x251c, 0, 0, 0,
    ];
    // SAFETY: all pointers are the caller's game structures; the global is the game's table.
    unsafe {
        let mut local_a0: u32 = mech_trace::rd!("1000bf23", param_1, read_unaligned) & 0xffff;
        let mut s_var2: i16 = 0;
        let mut local_8: i32 = 0;
        let mut local_9c: i32 = 0;
        loop {
            if 7 < local_9c {
                local_9c = 0;
                while local_9c
                    < (mech_trace::rd!("1000bf23", param_1.byte_add(16), read_unaligned) as i32)
                    && local_9c < 0xe
                {
                    let idx = mech_trace::rd!("1000bf23", param_3, read_unaligned) / 100;
                    let p = (&raw mut DAT_100a73d0)
                        .cast::<u8>()
                        .offset((idx as isize) * 2);
                    let v = mech_trace::rd!("1000bf23", p.cast::<i16>(), read_unaligned);
                    local_a0 = v.wrapping_add(local_a0 as i16) as u16 as u32;
                    param_3 = param_3.add(2);
                    local_9c += 1;
                }
                return mech_trace::rt!(
                    "1000bf23",
                    (((param_1 as usize as u32) >> 16) << 16) | (local_a0 as u16 as u32)
                );
            }
            let cnt =
                mech_trace::rd!("1000bf23", param_2.byte_add(36), read_unaligned) as i16 as i32;
            let mut local_a4: i32 = 0;
            while local_a4 < cnt {
                local_8 = local_8
                    .wrapping_add(mech_trace::rd!("1000bf23", param_2.add(1), read_unaligned))
                    .wrapping_add(mech_trace::rd!("1000bf23", param_2, read_unaligned));
                let v = mech_trace::rd!(
                    "1000bf23",
                    param_2
                        .cast::<u8>()
                        .offset((local_a4 as isize) * 2 + 0xc)
                        .cast::<u16>(),
                    read_unaligned
                ) as i32;
                let mut a8: i32 = 0x16;
                while -1 < a8 {
                    if (l[(a8 * 3) as usize] as i32) < v {
                        let i = (a8 * 3 + 2) as usize;
                        l[i] = l[i].wrapping_add(1);
                        s_var2 = s_var2.wrapping_add(1);
                        break;
                    }
                    a8 -= 1;
                }
                local_a4 += 1;
            }
            let mut a8: i32 = 0x14;
            let mut s_var1: i16;
            loop {
                s_var1 = local_a0 as i16;
                if !(-1 < a8) {
                    break;
                }
                let b = (a8 * 3) as usize;
                if l[b + 2] != 0 {
                    let prod = l[b + 1].wrapping_mul(l[b + 2]);
                    if l[b + 1] < 0 {
                        local_a0 = s_var1.wrapping_sub(prod) as u16 as u32;
                    } else {
                        let p1 = mech_trace::rd!("1000bf23", param_1, read_unaligned) as i16;
                        local_a0 = prod.wrapping_mul(p1).wrapping_add(s_var1) as u16 as u32;
                    }
                }
                a8 -= 1;
            }
            let mut s_var3 = s_var2;
            if l[0x41] != 0 {
                s_var3 = s_var2.wrapping_mul(2);
            }
            let p2 = mech_trace::rd!("1000bf23", param_2.add(2), read_unaligned) as i16;
            local_a0 = p2
                .wrapping_add(s_var1)
                .wrapping_add(s_var3)
                .wrapping_add(local_8 as i16) as u16 as u32;
            local_9c += 1;
            param_2 = param_2.add(10);
        }
    }
}
