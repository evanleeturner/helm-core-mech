// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf38: i32;
    static mut DAT_100b38fc: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10051f43(param_1: i32) -> i32;
    fn FUN_1002950f(param_1: i32, param_2: i16);
    fn FUN_10023258(param_1: i32, param_2: *mut i32) -> i32;
    fn FUN_1002306a(param_1: i32, param_2: *mut i32) -> i32;
    fn FUN_1004c350(param_1: i32, param_2: i32) -> u8;
    fn FUN_100234a9(param_1: i32);
    fn FUN_1002974a(param_1: i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_1002977f(param_1: i32, param_2: i32, param_3: i32) -> i32;
}

unsafe fn rd32(addr: i32) -> i32 {
    // SAFETY: the caller guarantees addr points at readable game memory.
    unsafe { (addr as *const i32).read_unaligned() }
}

unsafe fn wr32(addr: i32, v: i32) {
    // SAFETY: the caller guarantees addr points at writable game memory.
    unsafe { (addr as *mut i32).write_unaligned(v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10022e3c(param_1: i32, param_2: i32) -> i32 {
    let mut local_c: i32 = 0;
    let mut local_10: i32 = 0;
    let mut local_14: i32 = 0;
    // SAFETY: param_1 points at the game's object; all offsets, globals and
    // callees are the game's own memory and code.
    unsafe {
        let mut i_var2 = param_2.wrapping_sub(rd32(param_1.wrapping_add(0x6c)));

        let v18e = rd32(param_1.wrapping_add(0x18e));
        if v18e <= DAT_100b38fc {
            local_14 = FUN_10051f43((param_1.wrapping_add(0x186) as *const i8).read() as i32);
            let tmp = local_14.wrapping_mul(0x16).wrapping_add(DAT_100b38fc);
            wr32(param_1.wrapping_add(0x18e), tmp);

            let sum = rd32(rd32(param_1.wrapping_add(0x20)).wrapping_add(0x1c))
                .wrapping_add(rd32(param_1.wrapping_add(0x5c)))
                .wrapping_add(rd32(param_1.wrapping_add(0xd8)));
            let local_8 = sum % 0x1680000;

            if i_var2 < 0xa0000 && -0xa0000 < i_var2 && local_8 < 0xa0000 && -0xa0000 < local_8 {
                let skip = if ((param_1.wrapping_add(0x17a) as *const u16).read_unaligned() & 0xff)
                    as i32
                    != DAT_100adf38
                {
                    i_var2 = FUN_10051f43(3);
                    i_var2 != 0
                } else {
                    false
                };
                if !skip {
                    FUN_1002950f(
                        param_1,
                        (param_1.wrapping_add(0x17a) as *const i16).read_unaligned(),
                    );
                    if local_14 == 0 {
                        let v192 = rd32(param_1.wrapping_add(0x192));
                        let local_1c = if v192 == 0 {
                            FUN_1002306a(param_1, &raw mut local_c)
                        } else {
                            FUN_10023258(param_1, &raw mut local_c)
                        };
                        if local_1c != 0 && FUN_1004c350(param_1, 1) != 0 {
                            let v192 = rd32(param_1.wrapping_add(0x192));
                            let base = rd32(param_1.wrapping_add(0x4c));
                            if v192 == 0 {
                                (base.wrapping_add(0x25) as *mut u8).write(1);
                            } else {
                                (base.wrapping_add(0x27) as *mut u8).write(1);
                            }
                            local_10 = 1;
                        }
                    }
                }
            }
        }

        if local_c != 0 && local_10 != 0 {
            FUN_100234a9(param_1);
        }
        if local_10 == 0 {
            local_14 = 0;
        }
        let i_var2 = FUN_1002974a(param_1, param_2, local_14);
        wr32(
            rd32(param_1.wrapping_add(0x4c)).wrapping_add(4),
            i_var2.wrapping_mul(0x2d00),
        );
        let i_var2 = FUN_1002977f(param_1, rd32(param_1.wrapping_add(0xd8)), local_14);
        wr32(
            rd32(param_1.wrapping_add(0x4c)),
            i_var2.wrapping_mul(-0xf00),
        );
    }
    local_10
}
