// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa1ac: u32;
    static mut DAT_100adf34: u32;
    static mut DAT_100b0710: u32;
    static mut DAT_100eebbe: u8;
    static mut DAT_10123240: [u8; 0];
    static mut DAT_1012327a: [u8; 0];
    static mut DAT_1012327b: [u8; 0];
    static mut DAT_101232ee: [u8; 0];
    static mut s_BOunty__100ad748: [i8; 0];
    static mut s_Bonus__100ad728: [i8; 0];
    static mut s_Bounty__100ad730: [i8; 0];
    static mut s_Complete_100ad760: [i8; 0];
    static mut s_InComplete_100ad76c: [i8; 0];
    static mut s_In_progress_100ad778: [i8; 0];
    static mut s_Return__100ad73c: [i8; 0];
    static mut s_Secondary__100ad754: [i8; 0];
    static mut s_Target__100ad710: [i8; 0];
    static mut s_Target__100ad71c: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1005aae4(param_1: i32) -> u32;
    fn FUN_1005acaa(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: *mut u8,
        param_6: i32,
    ) -> u8;
    fn FUN_1002d49d(param_1: *mut i8, param_2: i32) -> i32;
    fn strncpy(dest: *mut i8, src: *const i8, count: usize) -> *mut i8;
    fn sprintf(dest: *mut i8, format: *const i8, ...) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002d4f1(param_1: i32, param_2: *mut i32, param_3: i32, param_4: i8) {
    // SAFETY: all globals, callees and pointers belong to the game; reads and
    // writes occur in the same order and places as the original.
    unsafe {
        let mut buf = [0u8; 0x100];
        let local_110 = buf.as_mut_ptr();
        let i_var2 = (param_1.wrapping_add(0xa4) as *const i32).read_unaligned();
        let i_var4 = (DAT_100b0710 as i32).wrapping_mul(0x3c0a);
        let i_var7 = FUN_1005aae4(param_3) as i32;
        let mut b_var5 = false;
        let base_10123240 = (&raw mut DAT_10123240).cast::<u8>();
        let base_1012327a = (&raw mut DAT_1012327a).cast::<u8>();
        let base_1012327b = (&raw mut DAT_1012327b).cast::<u8>();
        let base_101232ee = (&raw mut DAT_101232ee).cast::<u8>();

        if DAT_100aa1ac != 0 && *((DAT_100adf34.wrapping_add(10)) as *const i8) == 0 {
            let mut local_12c: i32 = 0;
            let mut b_var6 = false;
            let mut idx: i32 = 0;
            while idx < (base_10123240.byte_add(i_var4 as usize).cast::<i32>()).read_unaligned() {
                let i_var8 = idx
                    .wrapping_mul(0x13f)
                    .wrapping_add((DAT_100b0710 as i32).wrapping_mul(0x3c0a));
                let v = *base_101232ee.byte_add(i_var8 as usize);
                local_12c = local_12c.wrapping_add(v as i32);
                if v != 0 && *base_1012327b.byte_add(i_var8 as usize) == 2 {
                    b_var6 = true;
                }
                idx = idx.wrapping_add(1);
            }
            if b_var6 && local_12c == 1 {
                b_var5 = true;
            }
        }

        let mut local_118: i32 = 0;
        loop {
            if (base_10123240.byte_add(i_var4 as usize).cast::<i32>()).read_unaligned() <= local_118
            {
                return;
            }
            let i_var8 = local_118
                .wrapping_mul(0x13f)
                .wrapping_add((DAT_100b0710 as i32).wrapping_mul(0x3c0a));
            let local_c = base_1012327a.byte_add(i_var8 as usize);
            if *base_101232ee.byte_add(i_var8 as usize) != 0
                && *base_1012327b.byte_add(i_var8 as usize) as i8 == param_4
            {
                let pp = (param_1.wrapping_add(0xa8) as *const *const i32).read_unaligned();
                *param_2 = pp.read_unaligned();
                let mut local_10: i32 = 0;
                let local_8: *mut u8 = match param_4 {
                    1 => (&raw mut s_Target__100ad710).cast::<u8>(),
                    2 => {
                        if b_var5 {
                            (&raw mut s_Target__100ad71c).cast::<u8>()
                        } else {
                            (&raw mut s_Bonus__100ad728).cast::<u8>()
                        }
                    }
                    4 => (&raw mut s_Bounty__100ad730).cast::<u8>(),
                    8 => (&raw mut s_Return__100ad73c).cast::<u8>(),
                    _ => (&raw mut s_BOunty__100ad748).cast::<u8>(),
                };
                DAT_100eebbe = 6;
                let p1 = i_var2 as *mut i32;
                FUN_1005acaa(p1, *param_2, *param_2.add(1), param_3, local_8, 0x100eebb0);
                DAT_100eebbe = 0xe;
                let i_var8b = FUN_1002d49d((&raw mut s_Secondary__100ad754).cast::<i8>(), param_3);
                *param_2 = (*param_2).wrapping_add(i_var8b);

                let mut len: usize = 0;
                while *local_c.add(0xad + len) != 0 {
                    len += 1;
                }
                if len < 0x20 {
                    DAT_100eebbe = 0xe;
                    FUN_1005acaa(
                        (param_1.wrapping_add(0xa4) as *const i32).read_unaligned() as *mut i32,
                        *param_2,
                        *param_2.add(1),
                        param_3,
                        local_c.add(0xad),
                        0x100eebb0,
                    );
                    DAT_100eebbe = 0xe;
                } else {
                    let i_var8c = *param_2;
                    let i_var3 = *param_2.add(1);
                    strncpy(local_110.cast::<i8>(), local_c.add(0xad).cast::<i8>(), 0x20);
                    *local_110.add(0x20) = 0;
                    DAT_100eebbe = 0xe;
                    FUN_1005acaa(
                        (param_1.wrapping_add(0xa4) as *const i32).read_unaligned() as *mut i32,
                        *param_2,
                        *param_2.add(1),
                        param_3,
                        local_110,
                        0x100eebb0,
                    );
                    *param_2 = i_var8c;
                    *param_2.add(1) = (*param_2.add(1)).wrapping_add(i_var7);
                    let mut len2: usize = 0;
                    while *local_c.add(0xcd + len2) != 0 {
                        len2 += 1;
                    }
                    core::ptr::copy_nonoverlapping(local_c.add(0xcd), local_110, len2 + 1);
                    DAT_100eebbe = 0xe;
                    FUN_1005acaa(
                        (param_1.wrapping_add(0xa4) as *const i32).read_unaligned() as *mut i32,
                        *param_2,
                        *param_2.add(1),
                        param_3,
                        local_110,
                        0x100eebb0,
                    );
                    DAT_100eebbe = 0xe;
                    *param_2 = i_var8c;
                    *param_2.add(1) = i_var3;
                    local_10 = 1;
                }

                let tag = *local_c;
                let (fmt, color): (*const i8, u8) = if tag == 5 {
                    ((&raw const s_Complete_100ad760).cast::<i8>(), 7)
                } else if tag == 6 {
                    ((&raw const s_InComplete_100ad76c).cast::<i8>(), 0xb)
                } else {
                    ((&raw const s_In_progress_100ad778).cast::<i8>(), 0xe)
                };
                sprintf(local_110.cast::<i8>(), fmt);
                let i_a = (i_var2.wrapping_add(0xc) as *const i32).read_unaligned();
                let i_b = (i_var2.wrapping_add(4) as *const i32).read_unaligned();
                let i_c = FUN_1002d49d(local_110.cast::<i8>(), param_3);
                *param_2 = i_a.wrapping_sub(i_b).wrapping_sub(i_c);
                DAT_100eebbe = color;
                FUN_1005acaa(
                    (param_1.wrapping_add(0xa4) as *const i32).read_unaligned() as *mut i32,
                    *param_2,
                    *param_2.add(1),
                    param_3,
                    local_110,
                    0x100eebb0,
                );

                DAT_100eebbe = 0xe;
                *param_2.add(1) = (*param_2.add(1)).wrapping_add(i_var7);
                if local_10 != 0 {
                    *param_2.add(1) = (*param_2.add(1)).wrapping_add(i_var7);
                }
            }
            local_118 = local_118.wrapping_add(1);
        }
    }
}
