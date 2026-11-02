// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10160be4: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1005acaa(a: *mut i32, b: i32, c: i32, d: u32, e: *mut u8, f: i32);
}

unsafe fn rd(p: *mut u8, off: usize) -> u32 {
    // SAFETY: caller guarantees p points at a readable structure.
    unsafe { p.byte_add(off).cast::<u32>().read_unaligned() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005e786(
    param_1: *mut u32,
    param_2: *mut u8,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: *mut u32,
) {
    // SAFETY: all pointers are the game's own structures, accessed exactly
    // where the original does; callbacks are the game's cdecl function pointers.
    unsafe {
        let mut local_18: u32 = 0;
        let mut b_var3 = true;
        if !param_1.is_null() && param_1.add(0xc).read_unaligned() != 0 && !param_6.is_null() {
            let u_var4 = param_6.read_unaligned();
            let pi_var1 = param_1.read_unaligned() as usize as *mut i32;
            if !pi_var1.is_null() && !param_2.is_null() && rd(param_2, 8) != 0 {
                let b_var5 = DAT_10160be4.wrapping_sub(0x31) == param_3;
                let mut local_c: i32;
                if (param_2.read() & 1) == 0 || rd(param_2, 0x14) == 0 {
                    local_c = rd(param_2, 4) as i32;
                } else {
                    let f: extern "C" fn(u32) -> i32 =
                        core::mem::transmute(rd(param_2, 0x14) as usize);
                    local_c = f(rd(param_2, 0xc));
                }
                let i_var2 = param_6.add(4).read_unaligned() as i32;
                match u_var4 {
                    1 => {
                        if rd(param_2, 0x10) != 0 {
                            let f: extern "C" fn(*mut u32, *mut u8) =
                                core::mem::transmute(rd(param_2, 0x10) as usize);
                            f(param_6, param_2);
                        }
                        if rd(param_2, 0x14) == 0 {
                            param_2.byte_add(4).cast::<u32>().write_unaligned(0);
                            local_c = rd(param_2, 4) as i32;
                        } else {
                            let f: extern "C" fn(u32) -> u32 =
                                core::mem::transmute(rd(param_2, 0x14) as usize);
                            let v = f(rd(param_2, 0xc));
                            param_2.byte_add(4).cast::<u32>().write_unaligned(v);
                            local_c = rd(param_2, 4) as i32;
                        }
                    }
                    2 => {
                        if i_var2 == param_3 {
                            let d = DAT_10160be4;
                            if d == 0x20 || d == 200 {
                                local_18 = 1;
                            } else if d == 0xc9 {
                                local_18 = 0xffff_ffff;
                            } else {
                                local_18 = b_var5 as u32;
                            }
                        }
                    }
                    4 => {
                        if rd(param_2, 0x1c) != 0 {
                            let f: extern "C" fn(u32, i32) =
                                core::mem::transmute(rd(param_2, 0x1c) as usize);
                            f(rd(param_2, 0xc), local_c);
                        }
                        b_var3 = false;
                    }
                    5 => {
                        if rd(param_2, 0x20) != 0 {
                            let f: extern "C" fn(u32) =
                                core::mem::transmute(rd(param_2, 0x20) as usize);
                            f(rd(param_2, 0xc));
                        }
                        b_var3 = false;
                    }
                    _ => {}
                }
                let list = rd(param_2, 8) as usize as *mut u8;
                let len = rd(list, 4) as i32;
                if 0 < len {
                    let sum = (len as u32)
                        .wrapping_add(local_18)
                        .wrapping_add(local_c as u32) as i32;
                    local_c = sum % len;
                    let list = rd(param_2, 8) as usize as *mut u8;
                    let ent = rd(
                        list,
                        8usize.wrapping_add((local_c as u32).wrapping_mul(4) as usize),
                    ) as usize as *mut u8;
                    FUN_1005acaa(
                        pi_var1,
                        param_4,
                        param_5,
                        param_1.add(0xc).read_unaligned(),
                        ent,
                        0x100eebb0,
                    );
                }
                if i_var2 == param_3 {
                    let idx = (param_3 as u32).wrapping_mul(5).wrapping_add(6) as usize;
                    let v = param_6.add(idx).read_unaligned();
                    if ((local_18 != 0 && v == 4) || (b_var5 && v == 5)) && rd(param_2, 0x1c) != 0 {
                        let f: extern "C" fn(u32, i32) =
                            core::mem::transmute(rd(param_2, 0x1c) as usize);
                        f(rd(param_2, 0xc), local_c);
                    }
                    if rd(param_2, 0x18) != 0 && b_var3 {
                        let f: extern "C" fn(u32, i32) =
                            core::mem::transmute(rd(param_2, 0x18) as usize);
                        f(rd(param_2, 0xc), local_c);
                    }
                }
                param_2.byte_add(4).cast::<i32>().write_unaligned(local_c);
            }
        }
    }
}
