// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa558: u32;
    static mut DAT_100aa55c: u32;
    static mut DAT_100aa560: u32;
    static mut DAT_100ab094: u32;
    static mut DAT_10160c00: [u8; 0];
    static mut DAT_100aaf68: *const u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1003d2d3(param_1: i32, param_2: i32) -> u8;
    fn FUN_1000fc30(param_1: i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_1007be21(param_1: *mut i32, param_2: u32) -> i32;
    fn FUN_1007c7bc(param_1: i32, param_2: i32, param_3: i32, param_4: i32) -> u8;
    fn FUN_1002f322(param_1: i32, param_2: *mut u32, param_3: *mut u32, param_4: *mut u32) -> u8;
    fn FUN_100037f0(param_1: u32, param_2: u32, param_3: u32, param_4: u32, param_5: i32) -> u8;
    fn FUN_10002570(param_1: u32, param_2: i32) -> u32;
    fn FUN_10003d60(param_1: i32, param_2: i32) -> i32;
}

#[inline(always)]
unsafe fn rd(addr: i32) -> i32 {
    // SAFETY: caller supplies an address of a valid game structure field.
    unsafe { (addr as usize as *const i32).read_unaligned() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1007f876(
    param_1: i32,
    param_2: *mut i32,
    param_3: *mut i32,
    param_4: *mut i32,
    param_5: *mut i32,
) -> u32 {
    // SAFETY: all pointers and globals belong to the game; accessed exactly
    // where the original does.
    unsafe {
        let mut local_18: i32 = 0;
        FUN_1003d2d3(*param_2, *param_4);
        let tbl = (&raw mut DAT_10160c00) as *const u8;
        let mut local_c: i32 = tbl
            .byte_add((local_18 as isize as usize).wrapping_mul(4))
            .cast::<i32>()
            .read_unaligned();
        loop {
            if local_c == 0 {
                return 0;
            }
            local_18 = local_18.wrapping_add(1);
            *param_5 = 0;
            let mut local_1c: i32 = rd(local_c.wrapping_add(0x10));
            while local_1c != 0 {
                if rd(local_1c.wrapping_add(0x24)) == 6
                    && ((local_1c.wrapping_add(2) as usize as *const u16).read_unaligned() & 0x100)
                        == 0
                {
                    let local_24: u32 =
                        (*param_2).wrapping_sub(rd(local_1c.wrapping_add(0x34))) as u32;
                    let local_28: u32 =
                        (*param_3).wrapping_sub(rd(local_1c.wrapping_add(0x38))) as u32;
                    let local_8: u32 =
                        (*param_4).wrapping_sub(rd(local_1c.wrapping_add(0x3c))) as u32;
                    let local_10: i32 = rd(local_1c.wrapping_add(0x40))
                        .wrapping_add(rd(param_1.wrapping_add(0x41c)));
                    let mut local_14: i32 =
                        FUN_1000fc30(local_24 as i32, local_28 as i32, local_8 as i32);
                    if local_14 < local_10 {
                        let flags =
                            (local_1c.wrapping_add(2) as usize as *const u16).read_unaligned();
                        if ((flags as u8) & 0xf0) == 0x50 {
                            let local_2c = rd(local_1c.wrapping_add(0x18)) as usize as *mut i32;
                            local_14 = FUN_1000fc30(
                                rd(param_1.wrapping_add(0x428)),
                                rd(param_1.wrapping_add(0x42c)),
                                rd(param_1.wrapping_add(0x430)),
                            );
                            if 0 < local_14 {
                                let local_30 = FUN_1007be21(local_2c, 1);
                                if -1 < local_30 {
                                    FUN_1007c7bc(
                                        local_30,
                                        rd(param_1.wrapping_add(0x428)).wrapping_mul(2),
                                        rd(param_1.wrapping_add(0x42c)).wrapping_mul(2),
                                        rd(param_1.wrapping_add(0x430)).wrapping_mul(2),
                                    );
                                    let mut local_34: u32 = 0;
                                    let mut local_38: i32 = 0;
                                    let mut local_3c: u32 = 0;
                                    FUN_1002f322(
                                        local_2c as usize as i32,
                                        &raw mut local_34,
                                        (&raw mut local_38).cast::<u32>(),
                                        &raw mut local_3c,
                                    );
                                    let p = DAT_100aaf68;
                                    local_34 = local_34
                                        .wrapping_sub(p.cast::<i32>().read_unaligned() as u32);
                                    local_38 = local_38
                                        .wrapping_sub(p.byte_add(4).cast::<i32>().read_unaligned());
                                    local_3c = local_3c.wrapping_sub(
                                        p.byte_add(8).cast::<i32>().read_unaligned() as u32,
                                    );
                                    FUN_100037f0(
                                        local_34,
                                        local_38 as u32,
                                        local_3c,
                                        0xa9,
                                        DAT_100ab094 as i32,
                                    );
                                }
                            }
                            return 0;
                        }
                        if local_14 == 0 {
                            DAT_100aa55c = 0;
                            DAT_100aa558 = 0;
                            DAT_100aa560 = 0x10000;
                        } else {
                            DAT_100aa558 = FUN_10002570(local_24, local_14);
                            DAT_100aa55c = FUN_10002570(local_28, local_14);
                            DAT_100aa560 = FUN_10002570(local_8, local_14);
                        }
                        let mut iv = FUN_10003d60(DAT_100aa558 as i32, local_10);
                        *param_2 = rd(local_1c.wrapping_add(0x34)).wrapping_add(iv);
                        iv = FUN_10003d60(DAT_100aa55c as i32, local_10);
                        *param_3 = rd(local_1c.wrapping_add(0x38)).wrapping_add(iv);
                        iv = FUN_10003d60(DAT_100aa560 as i32, local_10);
                        *param_4 = rd(local_1c.wrapping_add(0x3c)).wrapping_add(iv);
                        iv = FUN_1000fc30(
                            rd(param_1.wrapping_add(0x434)),
                            rd(param_1.wrapping_add(0x438)),
                            rd(param_1.wrapping_add(0x43c)),
                        );
                        let local_20: i32 = if iv.wrapping_neg() >> 2 < 9 {
                            -4
                        } else {
                            (iv.wrapping_neg() >> 2).wrapping_neg() >> 1
                        };
                        iv = FUN_10003d60(DAT_100aa558 as i32, local_20);
                        (param_1.wrapping_add(0x428) as usize as *mut i32).write_unaligned(iv);
                        iv = FUN_10003d60(DAT_100aa55c as i32, local_20);
                        (param_1.wrapping_add(0x42c) as usize as *mut i32).write_unaligned(iv);
                        iv = FUN_10003d60(DAT_100aa560 as i32, local_20);
                        (param_1.wrapping_add(0x430) as usize as *mut i32).write_unaligned(iv);
                        *param_5 = local_1c;
                        return 1;
                    }
                }
                local_1c = rd(local_1c.wrapping_add(0x10));
            }
            local_c = tbl
                .byte_add((local_18 as isize as usize).wrapping_mul(4))
                .cast::<i32>()
                .read_unaligned();
        }
    }
}
