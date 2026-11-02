// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aaf68: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1003ba89(param_1: *mut i32, param_2: *mut i32, param_3: *mut i32) -> u32;
    fn FUN_10066f36(param_1: i32, param_2: *mut i32, param_3: *mut i32) -> *mut i32;
    fn FUN_10034495(param_1: i32, param_2: i32, param_3: u32, param_4: i32, param_5: i32) -> u32;
    fn FUN_1003510c(param_1: i32, param_2: i32, param_3: i32) -> u8;
    fn FUN_1000f1c0(param_1: i32, param_2: i32, param_3: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10034c0a(param_1: i32, param_2: i32) {
    let mut local_34: [i32; 2] = [0; 2];
    let mut local_20: i32;
    let mut local_24: i32;
    let mut local_28: i32;
    let mut local_8: i32 = 0;
    let mut local_c: i32 = 0;
    let mut local_10: i32 = 0;
    let mut local_14: i32 = 0;

    // SAFETY: param_1 points at the game's object; the callees and globals are
    // the game's own; locals are passed by pointer as in the original.
    unsafe {
        local_20 = ((param_1.wrapping_add(0x50)) as *const i32).read_unaligned();
        local_24 = ((param_1.wrapping_add(0x54)) as *const i32).read_unaligned();
        local_28 = ((param_1.wrapping_add(0x58)) as *const i32).read_unaligned();
        let i_var1 = FUN_1003ba89(&raw mut local_20, &raw mut local_24, &raw mut local_28);
        if i_var1 == 0 {
            local_34[0] = local_20;
            local_34[1] = local_24;
            let p = local_34.as_mut_ptr();
            FUN_10066f36(0x101f52e0, p, p);
            if param_2 == 0 {
                local_8 = 0xca;
            } else if param_2 == 2 {
                local_8 = 0xcd;
            } else {
                local_8 = 199;
            }
            FUN_10034495(
                1,
                ((param_1.wrapping_add(4)) as *const i32).read_unaligned(),
                local_20 as u32,
                local_24,
                param_2,
            );
            FUN_1003510c(local_34[0], local_34[1], local_8);
        } else {
            let obj = ((param_1.wrapping_add(0x20)) as *const i32).read_unaligned();
            let mut local_2c = ((obj.wrapping_add(0x41c)) as *const i32).read_unaligned();
            let g = DAT_100aaf68;
            let arg = ((g.wrapping_add(0x94)) as *const i32).read_unaligned();
            local_2c = FUN_1000f1c0(arg, local_2c, local_28);
            if param_2 == 0 {
                local_8 = 0x97;
                local_c = 0xa0;
                local_10 = 0xa9;
                local_14 = 0xb2;
            } else if param_2 == 1 {
                local_8 = 0x9a;
                local_c = 0xa3;
                local_10 = 0xac;
                local_14 = 0xb5;
            } else if param_2 == 2 {
                local_8 = 0x9d;
                local_c = 0xa6;
                local_10 = 0xaf;
                local_14 = 0xb8;
            }
            let r = FUN_10034495(
                0,
                ((param_1.wrapping_add(4)) as *const i32).read_unaligned(),
                local_20 as u32,
                local_24,
                param_2,
            );
            if r != 0 {
                FUN_1003510c(
                    local_20.wrapping_sub(local_2c),
                    local_24.wrapping_sub(local_2c),
                    local_8,
                );
                FUN_1003510c(
                    local_20.wrapping_add(local_2c),
                    local_24.wrapping_sub(local_2c),
                    local_c,
                );
                FUN_1003510c(
                    local_20.wrapping_sub(local_2c),
                    local_24.wrapping_add(local_2c),
                    local_10,
                );
                FUN_1003510c(
                    local_20.wrapping_add(local_2c),
                    local_24.wrapping_add(local_2c),
                    local_14,
                );
            }
        }
    }
}
