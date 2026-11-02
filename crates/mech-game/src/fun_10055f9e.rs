// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b1260: u8;
    static mut DAT_100b1264: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1005f040(
        param_1: i32,
        param_2: i32,
        param_3: *mut u32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
        param_7: i32,
    ) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10055f9e(
    param_1: i32,
    param_2: i32,
    param_3: *mut i32,
    param_4: i32,
    param_5: i32,
) -> u32 {
    let mut param_3 = param_3;
    let mut local_24: i32 = 0;
    let mut local_30: i32 = 0;
    let mut local_18: i32 = 0;
    let mut local_8: i32 = 0;
    let mut local_c: i32 = 0;
    let mut local_10: i32 = 0;
    let mut local_20: i32 = 0;

    if param_2 < 2 {
        return 0;
    }
    // SAFETY: param_3 points at the caller's array of three 6-int records;
    // globals and callee are the game's own.
    unsafe {
        for _ in 0..3 {
            let f3 = param_3.byte_add(12).read_unaligned();
            let f4 = param_3.byte_add(16).read_unaligned();
            if f3 == 0 {
                if f4 == 0 {
                    if param_5 == 0 {
                        local_8 = param_3.read_unaligned();
                        local_c = param_3.byte_add(4).read_unaligned();
                        local_24 = 1;
                    }
                } else {
                    local_10 = param_3.read_unaligned();
                    local_20 = param_3.byte_add(4).read_unaligned();
                    local_30 = 1;
                    if f4 < 0 {
                        local_18 = 1;
                    }
                }
            } else {
                if param_5 == 1 {
                    local_8 = param_3.read_unaligned();
                    local_c = param_3.byte_add(4).read_unaligned();
                    local_24 = 1;
                }
                if f3 < 0 {
                    local_18 = 1;
                }
            }
            param_3 = param_3.byte_add(24);
        }
        if local_24 == 0 || local_30 == 0 {
            return 0;
        }
        let local_2c: i32 = local_20.wrapping_sub(local_c) >> 1;
        let mut local_28: i32 = local_10.wrapping_sub(local_8) >> 1;
        let mut buf = [0i32; 24];
        let uvar1: i32;
        let uvar2: i32;
        if param_5 == 0 {
            buf[0] = local_8.wrapping_sub(local_2c);
            buf[1] = local_28.wrapping_add(local_c);
            buf[6] = local_2c.wrapping_add(local_8);
            buf[7] = local_c.wrapping_sub(local_28);
            uvar1 = local_2c;
            uvar2 = local_28.wrapping_neg();
        } else {
            let s = local_10.wrapping_sub(local_8) >> 31;
            local_28 = (local_28 ^ s).wrapping_sub(s);
            buf[0] = local_10.wrapping_sub(local_28);
            buf[1] = local_20.wrapping_sub(local_28);
            buf[6] = local_28.wrapping_add(local_10);
            buf[7] = local_20.wrapping_sub(local_28);
            uvar1 = local_28;
            uvar2 = local_28;
        }
        buf[19] = local_28.wrapping_add(local_20);
        buf[18] = local_10.wrapping_sub(uvar1);
        buf[13] = local_20.wrapping_add(uvar2);
        buf[12] = uvar1.wrapping_add(local_10);

        let base0 = &raw mut DAT_100b1260 as *const u8;
        let base1 = &raw mut DAT_100b1264 as *const u8;
        for i in 0..4i32 {
            let iu = i as usize;
            buf[iu * 6 + 4] = base1.byte_add(iu * 8).cast::<i32>().read_unaligned();
            if local_18 == 0 {
                buf[iu * 6 + 3] = base0.byte_add(iu * 8).cast::<i32>().read_unaligned();
            } else {
                let local_14: i32 = if (i & 1) == 0 { 1 } else { -1 };
                let off = local_14.wrapping_add(i).wrapping_mul(8);
                buf[iu * 6 + 3] = base0
                    .byte_offset(off as isize)
                    .cast::<i32>()
                    .read_unaligned();
            }
        }
        FUN_1005f040(param_1, 4, buf.as_mut_ptr().cast::<u32>(), param_4, 1, 0, 0);
        local_2c as u32
    }
}
