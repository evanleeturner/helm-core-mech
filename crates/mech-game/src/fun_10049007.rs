// SPDX-License-Identifier: MIT
use core::arch::asm;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100eebbe: u8;
    static mut DAT_10160be4: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100497a9(param_1: i32) -> u32;
    fn FUN_10048ee3(param_1: i32, param_2: u32, param_3: *mut u32, param_4: *mut u32) -> u8;
    fn FUN_100037f0(param_1: u32, param_2: u32, param_3: u32, param_4: u32, param_5: i32) -> u8;
    fn FUN_1005acaa(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: *mut u8,
        param_6: i32,
    ) -> u8;
    fn FUN_100666ad(
        param_1: *mut i32,
        param_2: u32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
    ) -> u8;
    fn FUN_1006670a(
        param_1: *mut i32,
        param_2: *mut i8,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
    ) -> u8;
    fn FUN_1005aae4(param_1: i32) -> u32;
    fn FUN_1005aaf7(param_1: i32, param_2: i32) -> u32;
    fn FUN_10056fc4(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
    ) -> u32;
    fn __itoa(value: i32, dest: *mut i8, radix: i32) -> *mut i8;
    fn FUN_10049767(param_1: i32) -> u32;
    fn FUN_100497e6(param_1: i32, param_2: u32) -> bool;
}

#[inline(always)]
unsafe fn ep(b: *mut i32, i: i32) -> *mut i32 {
    b.wrapping_offset(i as isize)
}

#[inline(always)]
unsafe fn rd(b: *mut i32, i: i32) -> i32 {
    // SAFETY: caller supplies a valid game structure pointer.
    unsafe { ep(b, i).read_unaligned() }
}

#[inline(always)]
unsafe fn rem(a: i32, b: i32) -> i32 {
    let r: i32;
    // SAFETY: the machine's own idiv, faulting on zero as the original does.
    unsafe {
        asm!("cdq", "idiv {d:e}", d = in(reg) b, inout("eax") a => _, out("edx") r, options(nostack));
    }
    r
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10049007(param_1: *mut i32) {
    // SAFETY: all pointers and globals belong to the game; accessed as the original does.
    unsafe {
        let mut local_14: i32 = 0x10;
        let mut local_20: i32 = 0;
        let mut local_28: i32 = 0;
        let mut local_24: i32 = 0;
        let mut local_40: i32 = 0;
        let mut local_3c: i32 = 0;
        if param_1.is_null() {
            return;
        }
        let local_2c = rd(param_1, 0xc);
        if local_2c == 0 {
            return;
        }
        let local_48 = FUN_100497a9(param_1 as i32) as *mut i32;
        if local_48.is_null() {
            return;
        }
        let local_30 = param_1.read_unaligned() as *mut i32;
        if local_30.is_null() {
            return;
        }
        let mut local_44 = rd(local_48, 4);
        let mut local_80: i32 = -1;
        let mut local_34: i32 = 0;
        while local_34 < rd(local_48, 3) {
            let k = rd(local_48, local_34.wrapping_mul(5).wrapping_add(6));
            if k == 2 || k == 6 {
                local_80 = local_34;
            }
            if k == 3 {
                local_14 = local_34;
                local_20 = 1;
            }
            local_34 = local_34.wrapping_add(1);
        }
        if local_80 == -1 {
            local_20 = 0;
        }
        if rd(local_48, 0) == 0 {
            local_48.write_unaligned(1);
        } else {
            if rd(local_48, 0) == 1 {
                local_48.write_unaligned(2);
            }
            if DAT_10160be4 != 0 {
                local_24 = DAT_10160be4 as i32;
                if 0x2f < DAT_10160be4 && DAT_10160be4 < 0x3a {
                    let local_10 = (DAT_10160be4 as i32).wrapping_sub(0x30);
                    if local_10 == 0 {
                        if local_80 != -1 {
                            local_44 = local_80;
                            local_24 = 0xd;
                        }
                    } else if 0 < local_10 && local_10 < rd(local_48, 3).wrapping_sub(local_20) {
                        if local_14 < local_10 {
                            DAT_10160be4 = DAT_10160be4.wrapping_add(1);
                            local_44 = local_10;
                            if 0x39 < DAT_10160be4 {
                                DAT_10160be4 = 0x30;
                            }
                        } else {
                            local_44 = (DAT_10160be4 as i32).wrapping_sub(0x31);
                        }
                        local_24 = 0xd;
                    }
                }
                FUN_10048ee3(
                    local_24,
                    rd(local_48, local_44.wrapping_mul(5).wrapping_add(6)) as u32,
                    local_48 as *mut u32,
                    &raw mut local_28 as *mut u32,
                );
                if local_28 != 0 {
                    local_44 = rem(
                        rd(local_48, 3)
                            .wrapping_add(local_44)
                            .wrapping_add(local_28),
                        rd(local_48, 3),
                    );
                }
                if rd(local_48, 4) != local_44 && rd(param_1, 10) != -1 {
                    FUN_100037f0(0, 0, 0, rd(param_1, 10) as u32, 0);
                }
            }
        }
        if local_28 == 0 {
            local_28 = 1;
        }
        if rd(local_48, local_44.wrapping_mul(5).wrapping_add(6)) == 3 {
            local_44 = rem(
                rd(local_48, 3)
                    .wrapping_add(local_44)
                    .wrapping_add(local_28),
                rd(local_48, 3),
            );
            if 0x2f < DAT_10160be4 && DAT_10160be4 < 0x3a {
                DAT_10160be4 = DAT_10160be4.wrapping_add(1);
                if 0x39 < DAT_10160be4 {
                    DAT_10160be4 = 0x30;
                }
                FUN_10048ee3(
                    local_24,
                    rd(local_48, local_44.wrapping_mul(5).wrapping_add(6)) as u32,
                    local_48 as *mut u32,
                    &raw mut local_28 as *mut u32,
                );
                if local_28 == 0 {
                    local_28 = 1;
                }
            }
        }
        ep(local_48, 4).write_unaligned(local_44);
        DAT_100eebbe = rd(param_1, 0xd) as u8;
        if rd(local_48, 1) != 0 {
            local_40 = rd(param_1, 0x12);
            local_3c = rd(param_1, 0x13);
            FUN_1005acaa(
                local_30,
                local_40,
                local_3c,
                local_2c,
                rd(local_48, 1) as *mut u8,
                0x100eebb0,
            );
        }
        if (ep(param_1, 1).cast::<u8>().read() & 4) == 0 {
            FUN_1006670a(
                local_30,
                rd(local_48, 1) as *mut i8,
                local_40,
                local_3c,
                local_2c,
                1,
            );
        } else {
            FUN_100666ad(local_30, local_40 as u32, local_3c, local_2c, 1);
        }
        let local_1c = FUN_1005aae4(local_2c) as i32;
        let local_54 = rd(param_1, 0x14);
        let mut local_50 = rd(param_1, 0x15).wrapping_add(local_1c / 2);
        local_40 = rd(param_1, 0x16);
        local_3c = rd(param_1, 0x17);
        let local_c = rd(param_1, 0x18);
        let mut local_8 = rd(param_1, 0x19);
        let mut local_4c = FUN_1005aaf7(local_2c, 0x30) as i32;
        local_4c = local_4c.wrapping_mul(2);
        let iv = FUN_1005aaf7(local_2c, 0x2e) as i32;
        local_4c = local_4c.wrapping_add(iv);
        let mut local_10: i32 = 0;
        let mut local_38: i32 = 0;
        let mut local_7c = [0u8; 40];
        local_34 = 0;
        while local_34 < rd(local_48, 3) {
            if local_34 == local_44 {
                DAT_100eebbe = rd(param_1, 0xe) as u8;
            } else {
                DAT_100eebbe = rd(param_1, 0xd) as u8;
            }
            let local_18 = ep(local_48, local_34.wrapping_mul(5).wrapping_add(6));
            if (ep(param_1, 1).cast::<u8>().read() & 8) == 0 && local_34 == local_80 {
                local_38 = local_38.wrapping_add(
                    rd(param_1, 0xf)
                        .wrapping_sub(local_34)
                        .wrapping_sub(1)
                        .wrapping_mul(rd(param_1, 0x11)),
                );
            }
            local_3c = local_3c.wrapping_add(local_38);
            local_8 = local_8.wrapping_add(local_38);
            local_50 = local_50.wrapping_add(local_38);
            local_38 = rd(param_1, 0x11);
            if local_34 == local_44 && rd(param_1, 8) != 0 {
                FUN_10056fc4(local_30, rd(param_1, 8), 0, local_54, local_50);
            }
            if local_18.read_unaligned() != 3 {
                if local_34 == local_80 {
                    local_10 = 0;
                } else {
                    local_10 = local_10.wrapping_add(1);
                }
                __itoa(local_10, local_7c.as_mut_ptr() as *mut i8, 10);
                FUN_1005acaa(
                    local_30,
                    local_40,
                    local_3c,
                    local_2c,
                    local_7c.as_mut_ptr(),
                    0x100eebb0,
                );
            }
            if rd(local_18, 1) != 0 {
                FUN_1005acaa(
                    local_30,
                    local_40.wrapping_add(local_4c),
                    local_3c,
                    local_2c,
                    rd(local_18, 1) as *mut u8,
                    0x100eebb0,
                );
            }
            if rd(local_18, 2) != 0 {
                let f: extern "C" fn(*mut i32, i32, i32, i32, i32, *mut i32) =
                    core::mem::transmute(rd(local_18, 2) as usize);
                f(
                    param_1,
                    rd(local_18, 3),
                    local_34,
                    local_c,
                    local_8,
                    local_48,
                );
            }
            local_34 = local_34.wrapping_add(1);
        }
        DAT_100eebbe = 0xe;
        let iv1 = local_48.read_unaligned();
        if iv1 == 3 {
            local_48.write_unaligned(2);
            let idx = local_44.wrapping_mul(5).wrapping_add(10);
            if rd(local_48, idx) != 0 && {
                FUN_100497e6(param_1 as i32, rd(local_48, idx) as u32);
                rd(param_1, 9) != -1
            } {
                FUN_100037f0(0, 0, 0, rd(param_1, 9) as u32, 0);
            }
        } else if 3 < iv1 && iv1 < 6 {
            local_48.write_unaligned(0);
            ep(local_48, 4).write_unaligned(0);
            FUN_10049767(param_1 as i32);
        }
    }
}
