// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aafd8: i32;
    static mut DAT_100aae80: i32;
    static mut DAT_100ad6a4: i32;
    static mut DAT_100ad6a0: i32;
    static mut DAT_100aae68: i32;
    static mut DAT_100aae6c: i32;
    static mut DAT_100aae70: i32;
    static mut DAT_100aaf90: i32;
    static mut DAT_100aaf8c: i32;
    static mut DAT_100aaf94: i32;
    static mut DAT_100aaf74: i32;
    static mut DAT_100aae84: i32;
    static mut DAT_100aaf68: *mut i8;
    static mut DAT_101f52e0: [i32; 5];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1008975d(p: *mut i32);
    fn FUN_10006b9a(a: i32) -> i32;
    fn FUN_10006b20(a: i32) -> i32;
    fn FUN_10089060(a: *mut i32, b: *mut i32, c: *mut i32, d: *mut i32);
    fn FUN_1000fc90(a: i32, b: i32, c: i32) -> u8;
    fn FUN_10058e74(a: *mut i32, b: i8);
    fn FUN_1000fd2a(a: i32, b: i32) -> i32;
    fn FUN_1000fdac(a: i32, b: i32) -> i32;
    fn FUN_10019d07(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32, h: i32, i: i32);
    fn FUN_10019d88(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32);
    fn FUN_10019c6e(
        a: i32,
        b: i32,
        c: i32,
        d: i32,
        e: i32,
        f: i32,
        g: i32,
        h: i32,
        i: i32,
        j: i32,
        k: i32,
    );
    fn FUN_1003f893(a: *mut i32, b: i32, c: i32, d: *mut u32);
    fn FUN_10018265(a: i32, b: i32, c: i32);
}

unsafe fn reset30(dst: &mut [i32; 5]) {
    // SAFETY: DAT_101f52e0 is the game's own five-int array.
    unsafe {
        let p = &raw mut DAT_101f52e0 as *const i32;
        let mut i = 0;
        while i < 5 {
            dst[i] = p.add(i).read();
            i += 1;
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
#[allow(unused_assignments)]
pub unsafe extern "C" fn FUN_100186c5(param_1: i32) {
    let mut local_dc = [0i32; 12];
    let mut local_90 = [0u32; 21];
    let mut local_30 = [0i32; 5];
    let mut local_1c: i32;
    let local_14: i32;
    let mut local_10: i32 = 0;
    let local_18: i32;
    let local_8: i32;
    let local_c: i32;
    let local_ac: i32;
    let mut local_a8: i32;
    let mut local_a4: i32 = 0;
    let mut local_a0: i32 = 0;
    let mut local_9c: i32 = 0;
    let mut local_98: i32;
    let mut local_94: i32;

    // SAFETY: param_1 points at the caller's structure; globals and callees are
    // the game's own; all pointers passed are to live locals.
    unsafe {
        local_ac = ((param_1.wrapping_add(0x2c)) as *const i32).read_unaligned();
        local_c = ((param_1.wrapping_add(0x30)) as *const i32).read_unaligned();
        local_18 = ((param_1.wrapping_add(0x34)) as *const i32).read_unaligned();
        local_8 = ((param_1.wrapping_add(0x38)) as *const i32).read_unaligned();
        local_a8 = 0;
        reset30(&mut local_30);
        local_98 = 0;
        local_1c = 0;
        local_94 = DAT_100aafd8;
        FUN_1008975d(local_dc.as_mut_ptr());
        local_dc[0] = FUN_10006b9a(((param_1.wrapping_add(0x14)) as *const i32).read_unaligned());
        local_dc[4] = local_dc[0];
        local_dc[3] = FUN_10006b20(((param_1.wrapping_add(0x14)) as *const i32).read_unaligned());
        local_dc[1] = 0i32.wrapping_sub(local_dc[3]);
        local_dc[11] = 0;
        local_dc[10] = 0;
        local_dc[9] = 0;
        FUN_10089060(
            local_dc.as_mut_ptr(),
            &raw mut local_1c,
            &raw mut local_94,
            &raw mut local_98,
        );
        let b1 = FUN_1000fc90(local_ac, local_8, param_1);
        let b2 = FUN_1000fc90(local_c, local_18, param_1);
        let b3 = FUN_1000fc90(local_c, local_8, param_1);
        let b4 = FUN_1000fc90(local_ac, local_18, param_1);
        local_14 = (b1 as i32)
            .wrapping_mul(4)
            .wrapping_add((b2 as i32).wrapping_mul(2))
            .wrapping_add((b3 as i32).wrapping_mul(8))
            .wrapping_add(b4 as i32);
        if DAT_100aae80 == 0 {
            DAT_100ad6a4 = 0xd0;
        } else {
            DAT_100ad6a4 = 0xdf;
        }
        local_90[2] = (DAT_100ad6a0 as u32).wrapping_shl(0x10);
        local_90[20] = 0x7e0000;
        local_90[14] = 0x7e0000;
        if DAT_100aae68 < 1 || DAT_100aae6c < 1 || DAT_100aae70 < 1 {
            DAT_100aaf90 = 0;
        }
        local_90[8] = local_90[2];
        match local_14 {
            0 => {
                local_30[1] = local_30[1].wrapping_add(local_ac);
                local_30[2] = local_30[2].wrapping_add(local_18);
                local_30[3] = local_30[1].wrapping_add(local_c).wrapping_sub(local_ac);
                local_30[4] = local_30[2].wrapping_add(local_8).wrapping_sub(local_18);
                if DAT_100aaf90 == 0 {
                    FUN_10058e74(local_30.as_mut_ptr(), 0);
                } else {
                    FUN_10058e74(local_30.as_mut_ptr(), DAT_100ad6a4 as i8);
                }
            }
            1 => {
                local_a4 = FUN_1000fdac(local_18, param_1);
                local_10 = FUN_1000fd2a(local_ac, param_1);
                if DAT_100aaf8c != 0 {
                    FUN_10019d88(
                        local_ac,
                        local_18,
                        local_ac,
                        local_10,
                        local_a4,
                        local_18,
                        DAT_100ad6a0,
                    );
                }
                if DAT_100aaf90 == 0 {
                    FUN_10019c6e(
                        local_ac, local_8, local_c, local_8, local_c, local_18, local_a4, local_18,
                        local_ac, local_10, 0,
                    );
                } else {
                    FUN_10019c6e(
                        local_ac,
                        local_8,
                        local_c,
                        local_8,
                        local_c,
                        local_18,
                        local_a4,
                        local_18,
                        local_ac,
                        local_10,
                        DAT_100ad6a4,
                    );
                }
            }
            2 => {
                local_a4 = FUN_1000fdac(local_18, param_1);
                local_a0 = FUN_1000fd2a(local_c, param_1);
                if DAT_100aaf8c != 0 {
                    FUN_10019d88(
                        local_c,
                        local_18,
                        local_a4,
                        local_18,
                        local_c,
                        local_a0,
                        DAT_100ad6a0,
                    );
                }
                if DAT_100aaf90 == 0 {
                    FUN_10019c6e(
                        local_ac, local_18, local_ac, local_8, local_c, local_8, local_c, local_a0,
                        local_a4, local_18, 0,
                    );
                } else {
                    FUN_10019c6e(
                        local_ac,
                        local_18,
                        local_ac,
                        local_8,
                        local_c,
                        local_8,
                        local_c,
                        local_a0,
                        local_a4,
                        local_18,
                        DAT_100ad6a4,
                    );
                }
            }
            3 => {
                local_10 = FUN_1000fd2a(local_ac, param_1);
                local_a0 = FUN_1000fd2a(local_c, param_1);
                if DAT_100aaf94 != 0 && DAT_100aaf8c != 0 {
                    local_90[18] = local_c as u32;
                    local_90[0] = local_c as u32;
                    local_90[12] = local_ac as u32;
                    local_90[6] = local_ac as u32;
                    local_90[1] = local_a0.wrapping_sub(local_94) as u32;
                    local_90[7] = local_10.wrapping_sub(local_94) as u32;
                    local_90[13] = local_10 as u32;
                    local_a8 = 1;
                    local_90[19] = local_a0 as u32;
                }
                if local_a0 == local_10 {
                    if DAT_100aaf8c != 0 {
                        local_30[1] = local_30[1].wrapping_add(local_ac);
                        local_30[2] = local_30[2].wrapping_add(local_18);
                        local_30[3] = local_30[1].wrapping_add(local_c).wrapping_sub(local_ac);
                        local_30[4] = local_30[2].wrapping_add(local_10).wrapping_sub(local_18);
                        FUN_10058e74(local_30.as_mut_ptr(), DAT_100ad6a0 as i8);
                    }
                    reset30(&mut local_30);
                    local_30[1] = local_30[1].wrapping_add(local_ac);
                    local_30[2] = local_30[2].wrapping_add(local_10);
                    local_30[3] = local_30[1].wrapping_add(local_c).wrapping_sub(local_ac);
                    local_30[4] = local_30[2].wrapping_add(local_8).wrapping_sub(local_10);
                    if DAT_100aaf90 == 0 {
                        FUN_10058e74(local_30.as_mut_ptr(), 0);
                    } else {
                        FUN_10058e74(local_30.as_mut_ptr(), DAT_100ad6a4 as i8);
                    }
                } else {
                    if DAT_100aaf8c != 0 {
                        FUN_10019d07(
                            local_c,
                            local_18,
                            local_ac,
                            local_18,
                            local_ac,
                            local_10,
                            local_c,
                            local_a0,
                            DAT_100ad6a0,
                        );
                    }
                    if DAT_100aaf90 == 0 {
                        FUN_10019d07(
                            local_ac, local_8, local_c, local_8, local_c, local_a0, local_ac,
                            local_10, 0,
                        );
                    } else {
                        FUN_10019d07(
                            local_ac,
                            local_8,
                            local_c,
                            local_8,
                            local_c,
                            local_a0,
                            local_ac,
                            local_10,
                            DAT_100ad6a4,
                        );
                    }
                }
            }
            4 => {
                local_9c = FUN_1000fdac(local_8, param_1);
                local_10 = FUN_1000fd2a(local_ac, param_1);
                if DAT_100aaf8c != 0 {
                    FUN_10019d88(
                        local_ac,
                        local_10,
                        local_ac,
                        local_8,
                        local_9c,
                        local_8,
                        DAT_100ad6a0,
                    );
                }
                if DAT_100aaf90 == 0 {
                    FUN_10019c6e(
                        local_c, local_8, local_c, local_18, local_ac, local_18, local_ac,
                        local_10, local_9c, local_8, 0,
                    );
                } else {
                    FUN_10019c6e(
                        local_c,
                        local_8,
                        local_c,
                        local_18,
                        local_ac,
                        local_18,
                        local_ac,
                        local_10,
                        local_9c,
                        local_8,
                        DAT_100ad6a4,
                    );
                }
            }
            5 => {
                local_a4 = FUN_1000fdac(local_18, param_1);
                local_9c = FUN_1000fdac(local_8, param_1);
                if local_a4 == local_9c {
                    if DAT_100aaf8c != 0 {
                        local_30[1] = local_30[1].wrapping_add(local_ac);
                        local_30[2] = local_30[2].wrapping_add(local_18);
                        local_30[3] = local_30[1]
                            .wrapping_add(local_a4)
                            .wrapping_sub(local_ac)
                            .wrapping_add(-1);
                        local_30[4] = local_30[2]
                            .wrapping_add(local_8)
                            .wrapping_sub(local_18)
                            .wrapping_add(-1);
                        FUN_10058e74(local_30.as_mut_ptr(), DAT_100ad6a0 as i8);
                    }
                    reset30(&mut local_30);
                    local_30[1] =
                        local_30[1].wrapping_add(local_a4.wrapping_sub(local_ac).wrapping_add(1));
                    local_30[2] = local_30[2].wrapping_add(local_18);
                    local_30[3] = local_30[1]
                        .wrapping_add(local_c)
                        .wrapping_sub(local_a4)
                        .wrapping_add(-1);
                    local_30[4] = local_30[2]
                        .wrapping_add(local_8)
                        .wrapping_sub(local_18)
                        .wrapping_add(-1);
                    if DAT_100aaf90 == 0 {
                        FUN_10058e74(local_30.as_mut_ptr(), 0);
                    } else {
                        FUN_10058e74(local_30.as_mut_ptr(), DAT_100ad6a4 as i8);
                    }
                } else {
                    if DAT_100aaf8c != 0 {
                        FUN_10019d07(
                            local_ac,
                            local_18,
                            local_ac,
                            local_8,
                            local_9c,
                            local_8,
                            local_a4,
                            local_18,
                            DAT_100ad6a0,
                        );
                    }
                    if DAT_100aaf90 == 0 {
                        FUN_10019d07(
                            local_c, local_8, local_c, local_18, local_a4, local_18, local_9c,
                            local_8, 0,
                        );
                    } else {
                        FUN_10019d07(
                            local_c,
                            local_8,
                            local_c,
                            local_18,
                            local_a4,
                            local_18,
                            local_9c,
                            local_8,
                            DAT_100ad6a4,
                        );
                    }
                }
                if DAT_100aaf94 != 0 && DAT_100aaf8c != 0 {
                    local_90[0] = local_a4.wrapping_sub(local_1c) as u32;
                    local_90[6] = local_9c.wrapping_sub(local_1c) as u32;
                    local_90[12] = local_9c as u32;
                    local_90[18] = local_a4 as u32;
                    local_90[1] = local_18 as u32;
                    local_90[19] = local_18 as u32;
                    local_90[7] = local_8 as u32;
                    local_90[13] = local_8 as u32;
                    local_a8 = 1;
                }
            }
            7 => {
                local_9c = FUN_1000fdac(local_8, param_1);
                local_a0 = FUN_1000fd2a(local_c, param_1);
                local_10 = FUN_1000fd2a(local_ac, param_1);
                if DAT_100aaf90 == 0 {
                    FUN_10019d88(local_9c, local_8, local_c, local_8, local_c, local_a0, 0);
                } else {
                    FUN_10019d88(
                        local_9c,
                        local_8,
                        local_c,
                        local_8,
                        local_c,
                        local_a0,
                        DAT_100ad6a4,
                    );
                }
                if DAT_100aaf8c != 0 {
                    FUN_10019c6e(
                        local_c,
                        local_18,
                        local_ac,
                        local_18,
                        local_ac,
                        local_8,
                        local_9c,
                        local_8,
                        local_c,
                        local_a0,
                        DAT_100ad6a0,
                    );
                    if DAT_100aaf94 != 0 {
                        local_90[12] = local_ac as u32;
                        local_90[6] = local_ac as u32;
                        local_90[0] = local_c as u32;
                        local_90[18] = local_c as u32;
                        local_90[1] = local_a0.wrapping_sub(local_94) as u32;
                        local_90[7] = local_10.wrapping_sub(local_94) as u32;
                        local_90[13] = local_10 as u32;
                        local_90[19] = local_a0 as u32;
                        local_a8 = 1;
                    }
                }
            }
            8 => {
                local_9c = FUN_1000fdac(local_8, param_1);
                local_a0 = FUN_1000fd2a(local_c, param_1);
                if DAT_100aaf8c != 0 {
                    FUN_10019d88(
                        local_9c,
                        local_8,
                        local_c,
                        local_8,
                        local_c,
                        local_a0,
                        DAT_100ad6a0,
                    );
                }
                if DAT_100aaf90 == 0 {
                    FUN_10019c6e(
                        local_c, local_18, local_ac, local_18, local_ac, local_8, local_9c,
                        local_8, local_c, local_a0, 0,
                    );
                } else {
                    FUN_10019c6e(
                        local_c,
                        local_18,
                        local_ac,
                        local_18,
                        local_ac,
                        local_8,
                        local_9c,
                        local_8,
                        local_c,
                        local_a0,
                        DAT_100ad6a4,
                    );
                }
            }
            10 => {
                local_a4 = FUN_1000fdac(local_18, param_1);
                local_9c = FUN_1000fdac(local_8, param_1);
                if local_a4 == local_9c {
                    local_30[1] = local_30[1].wrapping_add(local_ac);
                    local_30[2] = local_30[2].wrapping_add(local_18);
                    local_30[3] = local_30[1]
                        .wrapping_add(local_a4)
                        .wrapping_sub(local_ac)
                        .wrapping_add(-1);
                    local_30[4] = local_30[2]
                        .wrapping_add(local_8)
                        .wrapping_sub(local_18)
                        .wrapping_add(-1);
                    if DAT_100aaf90 == 0 {
                        FUN_10058e74(local_30.as_mut_ptr(), 0);
                    } else {
                        FUN_10058e74(local_30.as_mut_ptr(), DAT_100ad6a4 as i8);
                    }
                    if DAT_100aaf8c != 0 {
                        reset30(&mut local_30);
                        local_30[1] = local_30[1]
                            .wrapping_add(local_a4.wrapping_sub(local_ac).wrapping_add(1));
                        local_30[2] = local_30[2].wrapping_add(local_18);
                        local_30[3] = local_30[1]
                            .wrapping_add(local_c)
                            .wrapping_sub(local_a4)
                            .wrapping_add(-1);
                        local_30[4] = local_30[2]
                            .wrapping_add(local_8)
                            .wrapping_sub(local_18)
                            .wrapping_add(-1);
                        FUN_10058e74(local_30.as_mut_ptr(), DAT_100ad6a0 as i8);
                    }
                } else {
                    if DAT_100aaf90 == 0 {
                        FUN_10019d07(
                            local_ac, local_18, local_ac, local_8, local_9c, local_8, local_a4,
                            local_18, 0,
                        );
                    } else {
                        FUN_10019d07(
                            local_ac,
                            local_18,
                            local_ac,
                            local_8,
                            local_9c,
                            local_8,
                            local_a4,
                            local_18,
                            DAT_100ad6a4,
                        );
                    }
                    if DAT_100aaf8c != 0 {
                        FUN_10019d07(
                            local_c,
                            local_8,
                            local_c,
                            local_18,
                            local_a4,
                            local_18,
                            local_9c,
                            local_8,
                            DAT_100ad6a0,
                        );
                    }
                }
                if DAT_100aaf94 != 0 && DAT_100aaf8c != 0 {
                    local_90[0] = local_a4.wrapping_sub(local_1c) as u32;
                    local_90[6] = local_9c.wrapping_sub(local_1c) as u32;
                    local_90[12] = local_9c as u32;
                    local_90[18] = local_a4 as u32;
                    local_90[1] = local_18 as u32;
                    local_90[19] = local_18 as u32;
                    local_90[7] = local_8 as u32;
                    local_90[13] = local_8 as u32;
                    local_a8 = 1;
                }
            }
            0xb => {
                local_9c = FUN_1000fdac(local_8, param_1);
                local_10 = FUN_1000fd2a(local_ac, param_1);
                local_a0 = FUN_1000fd2a(local_c, param_1);
                if DAT_100aaf90 == 0 {
                    FUN_10019d88(local_ac, local_10, local_ac, local_8, local_9c, local_8, 0);
                } else {
                    FUN_10019d88(
                        local_ac,
                        local_10,
                        local_ac,
                        local_8,
                        local_9c,
                        local_8,
                        DAT_100ad6a4,
                    );
                }
                if DAT_100aaf8c != 0 {
                    FUN_10019c6e(
                        local_c,
                        local_8,
                        local_c,
                        local_18,
                        local_ac,
                        local_18,
                        local_ac,
                        local_10,
                        local_9c,
                        local_8,
                        DAT_100ad6a0,
                    );
                    if DAT_100aaf94 != 0 {
                        local_90[12] = local_ac as u32;
                        local_90[6] = local_ac as u32;
                        local_90[0] = local_c as u32;
                        local_90[18] = local_c as u32;
                        local_90[1] = local_a0.wrapping_sub(local_94) as u32;
                        local_90[7] = local_10.wrapping_sub(local_94) as u32;
                        local_90[13] = local_10 as u32;
                        local_90[19] = local_a0 as u32;
                        local_a8 = 1;
                    }
                }
            }
            0xc => {
                local_10 = FUN_1000fd2a(local_ac, param_1);
                local_a0 = FUN_1000fd2a(local_c, param_1);
                if DAT_100aaf94 != 0 && DAT_100aaf8c != 0 {
                    local_90[6] = local_c.wrapping_sub(local_1c) as u32;
                    local_90[0] = local_ac.wrapping_add(local_1c) as u32;
                    local_90[19] = local_10 as u32;
                    local_90[1] = local_10.wrapping_sub(local_94) as u32;
                    local_90[7] = local_a0.wrapping_sub(local_94) as u32;
                    local_a8 = 1;
                    local_90[12] = local_90[6];
                    local_90[13] = local_a0 as u32;
                    local_90[18] = local_90[0];
                }
                if local_a0 == local_10 {
                    local_30[1] = local_30[1].wrapping_add(local_ac);
                    local_30[2] = local_30[2].wrapping_add(local_18);
                    local_30[3] = local_30[1].wrapping_add(local_c).wrapping_sub(local_ac);
                    local_30[4] = local_30[2].wrapping_add(local_10).wrapping_sub(local_18);
                    if DAT_100aaf90 == 0 {
                        FUN_10058e74(local_30.as_mut_ptr(), 0);
                    } else {
                        FUN_10058e74(local_30.as_mut_ptr(), DAT_100ad6a4 as i8);
                    }
                    if DAT_100aaf8c != 0 {
                        reset30(&mut local_30);
                        local_30[1] = local_30[1].wrapping_add(local_ac);
                        local_30[2] = local_30[2].wrapping_add(local_10);
                        local_30[3] = local_30[1].wrapping_add(local_c).wrapping_sub(local_ac);
                        local_30[4] = local_30[2].wrapping_add(local_8).wrapping_sub(local_10);
                        FUN_10058e74(local_30.as_mut_ptr(), DAT_100ad6a0 as i8);
                    }
                } else {
                    if DAT_100aaf90 == 0 {
                        FUN_10019d07(
                            local_c, local_18, local_ac, local_18, local_ac, local_10, local_c,
                            local_a0, 0,
                        );
                    } else {
                        FUN_10019d07(
                            local_c,
                            local_18,
                            local_ac,
                            local_18,
                            local_ac,
                            local_10,
                            local_c,
                            local_a0,
                            DAT_100ad6a4,
                        );
                    }
                    if DAT_100aaf8c != 0 {
                        FUN_10019d07(
                            local_ac,
                            local_8,
                            local_c,
                            local_8,
                            local_c,
                            local_a0,
                            local_ac,
                            local_10,
                            DAT_100ad6a0,
                        );
                    }
                }
            }
            0xd => {
                local_a4 = FUN_1000fdac(local_18, param_1);
                local_a0 = FUN_1000fd2a(local_c, param_1);
                if DAT_100aaf90 == 0 {
                    FUN_10019d88(local_c, local_18, local_a4, local_18, local_c, local_a0, 0);
                } else {
                    FUN_10019d88(
                        local_c,
                        local_18,
                        local_a4,
                        local_18,
                        local_c,
                        local_a0,
                        DAT_100ad6a4,
                    );
                }
                if DAT_100aaf8c != 0 {
                    FUN_10019c6e(
                        local_ac,
                        local_18,
                        local_ac,
                        local_8,
                        local_c,
                        local_8,
                        local_c,
                        local_a0,
                        local_a4,
                        local_18,
                        DAT_100ad6a0,
                    );
                }
            }
            0xe => {
                local_a4 = FUN_1000fdac(local_18, param_1);
                local_10 = FUN_1000fd2a(local_ac, param_1);
                if DAT_100aaf90 == 0 {
                    FUN_10019d88(
                        local_ac, local_18, local_ac, local_10, local_a4, local_18, 0,
                    );
                } else {
                    FUN_10019d88(
                        local_ac,
                        local_18,
                        local_ac,
                        local_10,
                        local_a4,
                        local_18,
                        DAT_100ad6a4,
                    );
                }
                if DAT_100aaf8c != 0 {
                    FUN_10019c6e(
                        local_ac,
                        local_8,
                        local_c,
                        local_8,
                        local_c,
                        local_18,
                        local_a4,
                        local_18,
                        local_ac,
                        local_10,
                        DAT_100ad6a0,
                    );
                }
            }
            0xf => {
                if DAT_100aaf8c != 0 {
                    if DAT_100aaf94 != 0 {
                        let b = FUN_1000fc90(local_ac, local_8.wrapping_add(local_94), param_1);
                        if b as i32 == 0 {
                            local_10 = FUN_1000fd2a(local_ac, param_1);
                            local_a0 = FUN_1000fd2a(local_c, param_1);
                            if local_a0.wrapping_sub(local_94) < local_8
                                || local_10.wrapping_sub(local_94) < local_8
                            {
                                local_90[18] = local_c as u32;
                                local_90[0] = local_c as u32;
                                local_90[12] = local_ac as u32;
                                local_90[6] = local_ac as u32;
                                local_90[1] = local_a0.wrapping_sub(local_94) as u32;
                                local_90[7] = local_10.wrapping_sub(local_94) as u32;
                                local_90[13] = local_10 as u32;
                                local_a8 = 1;
                                local_90[19] = local_a0 as u32;
                            }
                        }
                    }
                    local_30[1] = local_30[1].wrapping_add(local_ac);
                    local_30[2] = local_30[2].wrapping_add(local_18);
                    local_30[3] = local_30[1].wrapping_add(local_c).wrapping_sub(local_ac);
                    local_30[4] = local_30[2].wrapping_add(local_8).wrapping_sub(local_18);
                    FUN_10058e74(local_30.as_mut_ptr(), DAT_100ad6a0 as i8);
                }
            }
            _ => {}
        }
        let _ = (local_a4, local_9c, local_98);
        if local_a8 != 0 && DAT_100aaf74 != 0 {
            FUN_1003f893(
                &raw mut DAT_101f52e0 as *mut i32,
                0x8000,
                4,
                local_90.as_mut_ptr(),
            );
        }
        if DAT_100aae84 != 0 && DAT_100aaf90 != 0 {
            FUN_10018265(
                DAT_100aaf68.byte_add(0x3c).cast::<i32>().read_unaligned(),
                DAT_100aae68,
                0x4a,
            );
            if 0 < DAT_100aae68 {
                FUN_10018265(DAT_100aae68, DAT_100aae6c, 0x4a);
            }
        }
    }
}
