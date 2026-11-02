// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_101ec750: u8;
    static mut DAT_101ec780: u8;
    static mut DAT_101ec78c: u8;
    static mut DAT_101ec794: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1004d5e3(param_1: i32) -> u32;
    fn FUN_1002f322(param_1: i32, param_2: *mut u32, param_3: *mut u32, param_4: *mut u32) -> u8;
    fn FUN_1000fc30(param_1: i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_10051f43(param_1: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000fa08(param_1: i32) -> u32 {
    // SAFETY: the globals and callees are the game's own; param_1 points at
    // a game object and the table entries are 0x60 bytes each, accessed
    // where the original does.
    unsafe {
        let p = param_1 as *const u8;
        let i_var1 = FUN_1004d5e3(p.byte_add(4).cast::<i32>().read_unaligned());
        let mut local_20: i32 = 0;
        while local_20 < 0xff {
            let off = (local_20 as usize).wrapping_mul(0x60);
            let local_18 = (&raw mut DAT_101ec750).byte_add(off).cast::<i32>();
            let f_8c = (&raw mut DAT_101ec78c)
                .byte_add(off)
                .cast::<i32>()
                .read_unaligned();
            if f_8c != 0 {
                let f_80 = (&raw mut DAT_101ec780)
                    .byte_add(off)
                    .cast::<i32>()
                    .read_unaligned();
                if f_80 != 0 {
                    let t = local_18.read_unaligned();
                    if t == 3 || t == 4 || t == 10 || t == 0xc || t == 9 {
                        let f_94 = (&raw mut DAT_101ec794)
                            .byte_add(off)
                            .cast::<i32>()
                            .read_unaligned();
                        let i_var2 = FUN_1004d5e3(f_94);
                        if i_var2 != i_var1 {
                            let mut local_10: u32 = 0;
                            let mut local_14: u32 = 0;
                            let mut local_1c: u32 = 0;
                            FUN_1002f322(
                                local_18.add(0xc).read_unaligned(),
                                &raw mut local_10,
                                &raw mut local_14,
                                &raw mut local_1c,
                            );
                            let i_var2 = p
                                .byte_add(0x50)
                                .cast::<i32>()
                                .read_unaligned()
                                .wrapping_sub(local_10 as i32);
                            let i_var3 = p
                                .byte_add(0x54)
                                .cast::<i32>()
                                .read_unaligned()
                                .wrapping_sub(local_14 as i32);
                            let local_8 = p
                                .byte_add(0x58)
                                .cast::<i32>()
                                .read_unaligned()
                                .wrapping_sub(local_1c as i32);
                            let local_c = FUN_1000fc30(i_var2, i_var3, local_8);
                            if local_c < 0xc351 {
                                if local_18.add(0x15).read_unaligned() < 0 {
                                    let i_var4 = local_c.wrapping_sub(0x9c4);
                                    if i_var4 < 1 {
                                        local_18.add(0x15).write_unaligned(0);
                                    } else {
                                        let d = local_18
                                            .add(3)
                                            .read_unaligned()
                                            .wrapping_mul(i_var2)
                                            .wrapping_add(
                                                local_18
                                                    .add(5)
                                                    .read_unaligned()
                                                    .wrapping_mul(local_8),
                                            )
                                            .wrapping_add(
                                                local_18
                                                    .add(4)
                                                    .read_unaligned()
                                                    .wrapping_mul(i_var3),
                                            );
                                        if 0 < d {
                                            let r = FUN_10051f43(100);
                                            if r < 0x4c {
                                                if i_var4 < 100 {
                                                    local_18.add(0x15).write_unaligned(0x9c4);
                                                } else {
                                                    let r2 = FUN_10051f43(i_var4 / 100);
                                                    local_18.add(0x15).write_unaligned(
                                                        r2.wrapping_mul(100).wrapping_add(0x9c4),
                                                    );
                                                }
                                            } else {
                                                local_18.add(0x15).write_unaligned(0);
                                            }
                                        }
                                    }
                                }
                                let v = local_18.add(0x15).read_unaligned();
                                if 0 < v && local_c <= v {
                                    let f = local_18.add(0xf).read_unaligned();
                                    local_18.add(0xf).write_unaligned(f | 0x8000);
                                }
                            }
                        }
                    }
                }
            }
            local_20 += 1;
        }
        1
    }
}
