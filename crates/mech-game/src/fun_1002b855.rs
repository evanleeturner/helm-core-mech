// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100eef40: [i32; 1];
    static mut DAT_10123130: [i32; 1];
    static mut DAT_101232f3: [i8; 1];
    static mut DAT_101600e0: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1004d47f(param_1: i32) -> i32;
    fn FUN_1002b6fe(param_1: i32, param_2: *mut i32, param_3: i32) -> i32;
    fn FUN_10044933(param_1: i32) -> i32;
    fn FUN_1002a4e0(param_1: i32, param_2: i32, param_3: u16, param_4: i32);
}

unsafe fn r32(p: i32, off: i32) -> i32 {
    // SAFETY: caller supplies a valid game object address.
    unsafe { ((p as usize).wrapping_add(off as usize) as *const i32).read_unaligned() }
}

unsafe fn r16u(p: i32, off: i32) -> u16 {
    // SAFETY: caller supplies a valid game object address.
    unsafe { ((p as usize).wrapping_add(off as usize) as *const u16).read_unaligned() }
}

unsafe fn r16s(p: i32, off: i32) -> i16 {
    // SAFETY: caller supplies a valid game object address.
    unsafe { ((p as usize).wrapping_add(off as usize) as *const i16).read_unaligned() }
}

unsafe fn r8(p: i32, off: i32) -> u8 {
    // SAFETY: caller supplies a valid game object address.
    unsafe { ((p as usize).wrapping_add(off as usize) as *const u8).read() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002b855(param_1: i32, param_2: u32) -> u32 {
    let mut local_38: [i32; 5] = [0; 5];
    let mut local_14: u32 = 0;
    let local_4c: u32;

    if (param_2 & 0xf000) != 0 {
        return 0;
    }
    // SAFETY: the callees and globals are the game's own; all pointers read
    // are game object addresses supplied by the game's tables.
    unsafe {
        let mut i_var1 = FUN_1004d47f(param_1);
        if i_var1 == -1 {
            return 0;
        }
        let i_var2 = FUN_1002b6fe(param_1, local_38.as_mut_ptr(), 1);
        if i_var2 == 0 {
            return 0;
        }
        if (param_2 & 0xf00) == 0x200 {
            let tbl = (&raw mut DAT_100eef40).cast::<i32>();
            let obj = tbl.offset((param_2 & 0xff) as isize).read();
            local_14 = r16u(obj, 0x17c) as u32;
        }
        let u_var6 = param_2 as u16;
        let d3130 = (&raw mut DAT_10123130).cast::<i32>();
        let dat = d3130.offset(param_1 as isize).read();
        let local_8: i32 =
            ((&raw mut DAT_101600e0).cast::<u8>() as i32).wrapping_add(param_1.wrapping_mul(0x3c));
        if dat == 4 {
            local_4c_set(&mut 0);
            let mut res: u32 = 0;
            let p = ((&raw mut DAT_101232f3).cast::<u8>() as i32)
                .wrapping_add(dat.wrapping_mul(0x13f))
                .wrapping_add(param_1.wrapping_mul(0x3c0a));
            let mut local_58 = r32(p, 0);
            let tmp_i = r32(p, 0);
            if tmp_i == 0 {
                local_58 = i_var2;
            }
            let mut local_18: i32 = 0;
            while local_18 < i_var2 && local_58 != 0 {
                let local_10 = local_38[local_18 as usize];
                let t = r32(local_10, 0x182);
                if r16s(local_10, 0x176) != 2 && r16s(local_10, 0x176) != 3 {
                    i_var1 = FUN_10044933(r32(local_10, 0x20));
                    if i_var1 == 0 && t != 0 {
                        FUN_1002a4e0(local_10, 2, u_var6, 1);
                        res = 1;
                    }
                }
                local_18 = local_18.wrapping_add(1);
            }
            if r32(local_8, 8) != 0 {
                let local_10 = local_38[4];
                if r16s(local_10, 0x176) != 2 && r16s(local_10, 0x176) != 3 {
                    i_var1 = FUN_10044933(r32(local_10, 0x20));
                    if i_var1 == 0 {
                        let t = r32(local_10, 0x182);
                        if t != 0 {
                            FUN_1002a4e0(local_10, 2, u_var6, 1);
                            res = 1;
                        }
                    }
                }
            }
            return res;
        } else {
            let mut local_c: i32 = -1;
            let mut local_48: i32 = 0x7fff;
            let mut local_18: i32 = 0;
            while local_18 < i_var2 {
                let local_10 = local_38[local_18 as usize];
                if r32(local_10, 4) != i_var1
                    && (r8(local_10, 0x17e) & 3) == 0
                    && r16s(local_10, 0x176) != 2
                    && r16s(local_10, 0x176) != 3
                {
                    let i_var3 = FUN_10044933(r32(local_10, 0x20));
                    if i_var3 == 0 {
                        let t = r32(local_10, 0x182);
                        if t != 0 {
                            let u_var4 = (r16u(local_10, 0x17c) as u32).wrapping_sub(local_14);
                            let u_var5 = ((u_var4 as i32) >> 0x1f) as u32;
                            let a = (u_var4 ^ u_var5).wrapping_sub(u_var5) as i32;
                            if a < local_48 {
                                local_48 = a;
                                local_c = r32(local_10, 4);
                            }
                        }
                    }
                }
                local_18 = local_18.wrapping_add(1);
            }
            if r32(local_8, 8) != 0 {
                let local_10 = local_38[4];
                if r32(local_10, 4) != i_var1
                    && (r8(local_10, 0x17e) & 3) == 0
                    && r16s(local_10, 0x176) != 2
                    && r16s(local_10, 0x176) != 3
                {
                    let i_var2b = FUN_10044933(r32(local_10, 0x20));
                    if i_var2b == 0 {
                        let t = r32(local_10, 0x182);
                        if t != 0 {
                            let u_var4 = (r16u(local_10, 0x17c) as u32).wrapping_sub(local_14);
                            let u_var5 = ((u_var4 as i32) >> 0x1f) as u32;
                            let a = (u_var4 ^ u_var5).wrapping_sub(u_var5) as i32;
                            if a < local_48 {
                                local_c = r32(local_10, 4);
                            }
                        }
                    }
                }
            }
            let tbl = (&raw mut DAT_100eef40).cast::<i32>();
            let mut local_10: i32;
            if local_c == -1 {
                local_10 = tbl.offset(i_var1 as isize).read();
                let t = r32(local_10, 0x182);
                if (r8(local_10, 0x17e) & 3) != 0
                    || r16s(local_10, 0x176) == 2
                    || r16s(local_10, 0x176) == 3
                {
                    local_10 = 0;
                } else {
                    i_var1 = FUN_10044933(r32(local_10, 0x20));
                    if i_var1 != 0 || t == 0 {
                        local_10 = 0;
                    }
                }
            } else {
                local_10 = tbl.offset(local_c as isize).read();
            }
            if local_10 == 0 {
                local_4c = 0;
            } else {
                FUN_1002a4e0(local_10, 2, u_var6, 1);
                let pp = (local_10 as usize).wrapping_add(0x17e) as *mut u32;
                let mut tmp_u = pp.read_unaligned();
                tmp_u |= 0x20;
                pp.write_unaligned(tmp_u);
                tmp_u = pp.read_unaligned();
                tmp_u &= 0xffffffbf;
                pp.write_unaligned(tmp_u);
                local_4c = 1;
            }
        }
    }
    local_4c
}

#[inline(always)]
fn local_4c_set(_x: &mut u32) {}
