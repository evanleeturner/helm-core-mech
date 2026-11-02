// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aaf68: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1001cac9(param_1: i32, param_2: *mut u32, param_3: *mut u32, param_4: *mut u32) -> u32;
    fn FUN_1003ba89(param_1: *mut i32, param_2: *mut i32, param_3: *mut i32) -> u32;
    fn FUN_10066f36(param_1: i32, param_2: *mut i32, param_3: *mut i32) -> *mut i32;
    fn FUN_10034495(param_1: i32, param_2: i32, param_3: u32, param_4: i32, param_5: i32) -> u32;
    fn FUN_1003510c(param_1: i32, param_2: i32, param_3: i32) -> u8;
    fn FUN_1000f1c0(param_1: i32, param_2: i32, param_3: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10034e74(param_1: i32, param_2: i32, param_3: i32) {
    let mut local_20: i32 = 0;
    let mut local_24: i32 = 0;
    let mut local_28: i32 = 0;
    let mut local_38: [i32; 2] = [0; 2];
    let mut local_8: i32 = 0;
    let mut local_c: i32 = 0;
    let mut local_10: i32 = 0;
    let mut local_14: i32 = 0;

    if param_2 == 0 {
        return;
    }
    // SAFETY: param_2 points at the game's object; the callees and globals are
    // the game's own, and locals are passed by pointer as the original does.
    unsafe {
        let obj = param_2 as usize;
        let f6c = ((obj + 0x6c) as *const i32).read_unaligned();
        if f6c == 0 {
            return;
        }
        let mut local_2c: i32 = FUN_1001cac9(
            f6c,
            &raw mut local_20 as *mut u32,
            &raw mut local_24 as *mut u32,
            &raw mut local_28 as *mut u32,
        ) as i32;
        let r = FUN_1003ba89(&raw mut local_20, &raw mut local_24, &raw mut local_28);
        if r == 0 {
            local_38[0] = local_20;
            local_38[1] = local_24;
            let p = local_38.as_mut_ptr();
            FUN_10066f36(0x101f52e0, p, p);
            if param_3 == 0 {
                local_8 = 0xca;
            } else if param_3 == 2 {
                local_8 = 0xcd;
            } else {
                local_8 = 199;
            }
            FUN_10034495(1, param_1, local_20 as u32, local_24, param_3);
            FUN_1003510c(local_38[0], local_38[1], local_8);
        } else {
            let base = DAT_100aaf68 as usize;
            let v94 = ((base + 0x94) as *const i32).read_unaligned();
            local_2c = FUN_1000f1c0(v94, local_2c >> 1, local_28);
            let v84 = ((DAT_100aaf68 as usize + 0x84) as *const i32).read_unaligned();
            let local_30 = v84 >> 1;
            if local_30 < local_2c {
                local_2c = local_30;
            }
            if param_3 == 0 {
                local_8 = 0x97;
                local_c = 0xa0;
                local_10 = 0xa9;
                local_14 = 0xb2;
            } else if param_3 == 1 {
                local_8 = 0x9a;
                local_c = 0xa3;
                local_10 = 0xac;
                local_14 = 0xb5;
            } else if param_3 == 2 {
                local_8 = 0x9d;
                local_c = 0xa6;
                local_10 = 0xaf;
                local_14 = 0xb8;
            }
            let r2 = FUN_10034495(0, param_1, local_20 as u32, local_24, param_3);
            if r2 != 0 {
                let mut l18 = local_20.wrapping_sub(local_2c);
                let mut l1c = local_24.wrapping_sub(local_2c);
                FUN_1003510c(l18, l1c, local_8);
                l18 = local_20.wrapping_add(local_2c);
                l1c = local_24.wrapping_sub(local_2c);
                FUN_1003510c(l18, l1c, local_c);
                l18 = local_20.wrapping_sub(local_2c);
                l1c = local_24.wrapping_add(local_2c);
                FUN_1003510c(l18, l1c, local_10);
                l18 = local_20.wrapping_add(local_2c);
                l1c = local_24.wrapping_add(local_2c);
                FUN_1003510c(l18, l1c, local_14);
            }
        }
    }
}
