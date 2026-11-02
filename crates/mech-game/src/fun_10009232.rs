// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7310: *mut i32;
    static mut DAT_100adf38: i32;
    static mut DAT_100eef40: [i32; 1];
    static mut DAT_100b0710: i32;
    static mut DAT_10215670: [u8; 1];
    static mut DAT_100ef030: [u8; 1];
    static mut DAT_100af17c: i32;
    static mut DAT_101686f0: i32;
    static mut PTR_DAT_100af0b4: *mut i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100357e3(param_1: *mut i32) -> i32;
    fn FUN_100091ee(param_1: i32, param_2: u8) -> i32;
    fn FUN_1004d5e3(param_1: u32) -> u32;
    fn FUN_1004d60d(param_1: u32) -> u32;
    fn FUN_1008668b(param_1: i32, param_2: i32, param_3: *mut i32) -> *mut u16;
    fn FUN_10056fc4(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
    ) -> i32;
    fn FUN_1008664f(param_1: i32, param_2: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10009232(param_1: *mut i32) {
    // SAFETY: all globals, callees and pointers belong to the game; reads and
    // writes occur in the same order as the original.
    unsafe {
        let mut local_20: *mut i32 = param_1.read_unaligned() as usize as *mut i32;
        let mut local_14: i32 = -1;
        let mut local_30: i32 = 2;
        if !DAT_100a7310.is_null() {
            local_20 = DAT_100a7310;
        }
        let idx = DAT_100adf38;
        let local_c: i32 = (&raw mut DAT_100eef40)
            .cast::<i32>()
            .offset(idx as isize)
            .read_unaligned();
        let mut local_2c: [i32; 3] = [0; 3];
        local_2c[0] = (local_c.wrapping_add(200) as usize as *const i32).read_unaligned();
        local_2c[1] = (local_c.wrapping_add(0xcc) as usize as *const i32).read_unaligned();
        local_2c[2] = (local_c.wrapping_add(0xd0) as usize as *const i32).read_unaligned();
        let mut local_10: i32 = FUN_100357e3(local_2c.as_mut_ptr());
        if local_10 != 0 && param_1.add(0x20).read_unaligned() != 0 {
            let f = core::mem::transmute::<i32, unsafe extern "C" fn(*mut i32, i32, i32) -> i32>(
                param_1.add(0x20).read_unaligned(),
            );
            local_10 = f(local_20, local_2c[0], local_2c[1]);
        }
        let mut local_38: u32 =
            (local_c.wrapping_add(0xdc) as usize as *const u32).read_unaligned();
        loop {
            if local_30 == 0 {
                return;
            }
            if local_38 != 0 && (local_38 & 0x1000) == 0 {
                let local_1c: u32 = local_38 & 0xf00;
                let local_8: u32 = local_38 & 0xff;
                let local_18: u32;
                let mut local_34: u32;
                if local_1c == 0x100 {
                    if local_10 == 0 {
                        local_18 = 0;
                    } else {
                        let addr = (&raw mut DAT_10215670 as *mut u8 as i32)
                            .wrapping_add(local_8.wrapping_mul(0x54) as i32);
                        let r = FUN_100091ee(addr, DAT_100b0710 as u8);
                        local_18 = (r != 0) as u32;
                    }
                    local_34 = 5;
                } else if local_1c == 0x200 {
                    local_18 = FUN_1004d5e3(local_8);
                    let e = (&raw mut DAT_100eef40)
                        .cast::<i32>()
                        .add(local_8 as usize)
                        .read_unaligned();
                    let w = (e.wrapping_add(0x14) as usize as *const u16).read_unaligned();
                    if (w & 0x1400) == 0 || (w & 0x800) != 0 {
                        local_34 = 3;
                    } else {
                        local_34 = 2;
                    }
                } else {
                    if local_1c != 0x400 {
                        return;
                    }
                    local_18 = FUN_1004d60d(local_8);
                    let w = (&raw mut DAT_100ef030)
                        .cast::<u8>()
                        .add(local_8.wrapping_mul(0x40) as usize)
                        .cast::<u16>()
                        .read_unaligned();
                    if (w & 0x1400) == 0 {
                        local_34 = 3;
                    } else {
                        local_34 = 2;
                    }
                }
                if local_10 == 0 {
                    local_34 = 6;
                }
                let base = param_1.add(0x1c).read_unaligned();
                let a = (base as u32)
                    .wrapping_add(local_34.wrapping_mul(0xc))
                    .wrapping_add(local_18.wrapping_mul(4));
                local_14 = (a as usize as *const i32).read_unaligned();
                if local_14 == 0 {
                    return;
                }
                let puvar2 = FUN_1008668b(
                    DAT_100af17c,
                    DAT_101686f0.wrapping_add(local_14),
                    PTR_DAT_100af0b4,
                );
                if !puvar2.is_null() {
                    if local_10 == 0 {
                        if param_1.add(0x21).read_unaligned() != 0 {
                            let g = core::mem::transmute::<
                                i32,
                                unsafe extern "C" fn(*mut i32, *mut i32, *mut i32),
                            >(
                                param_1.add(0x21).read_unaligned()
                            );
                            g(local_20, local_2c.as_mut_ptr(), local_2c.as_mut_ptr());
                            FUN_10056fc4(
                                local_20,
                                puvar2 as usize as i32,
                                0,
                                local_2c[0],
                                local_2c[1],
                            );
                        }
                    } else {
                        FUN_10056fc4(
                            local_20,
                            puvar2 as usize as i32,
                            0,
                            local_2c[0],
                            local_2c[1],
                        );
                    }
                    FUN_1008664f(DAT_101686f0.wrapping_add(local_14), PTR_DAT_100af0b4);
                }
            }
            local_2c[0] = (local_c.wrapping_add(0xf0) as usize as *const i32).read_unaligned();
            local_2c[1] = (local_c.wrapping_add(0xf4) as usize as *const i32).read_unaligned();
            local_2c[2] = (local_c.wrapping_add(0xf8) as usize as *const i32).read_unaligned();
            local_10 = FUN_100357e3(local_2c.as_mut_ptr());
            if local_10 != 0 && param_1.add(0x20).read_unaligned() != 0 {
                let f = core::mem::transmute::<i32, unsafe extern "C" fn(*mut i32, i32, i32) -> i32>(
                    param_1.add(0x20).read_unaligned(),
                );
                local_10 = f(local_20, local_2c[0], local_2c[1]);
            }
            local_38 = (local_c.wrapping_add(0x104) as usize as *const u32).read_unaligned();
            local_30 = local_30.wrapping_sub(1);
        }
    }
}
