// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1009c6ac: u32;
    static mut DAT_1009f378: u32;
    static mut DAT_1009f37c: u32;
    static mut DAT_1009fdc0: u8;
    static mut DAT_100bdf90: u8;
    static mut DAT_100bdf98: u8;
    static mut DAT_100bdf9c: u8;
    static mut DAT_100bdfa0: u8;
    static mut DAT_100bed70: u8;
    static mut DAT_100bed78: u8;
    static mut DAT_100bed7c: u8;
    static mut DAT_1009c5f8: u32;
    static mut DAT_1009fe70: u8;
    static mut DAT_1009fe40: u8;
    static mut DAT_1009fe58: u8;
    static mut DAT_1009fdd0: u8;
    static mut DAT_1009fe10: u8;
    static mut DAT_1009fe30: u8;
    static mut DAT_1009fe00: u8;
    static mut DAT_1009fe20: u8;
    static mut DAT_1009fdc4: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FID_conflict___wfopen(filename: *const u8, mode: *const u8) -> *mut u8;
    fn FID_conflict__fwprintf(file: *mut u8, format: *const u8, ...) -> i32;
    fn FUN_10053780(param_1: *mut u8, param_2: *const u8, param_3: *mut i32);
    fn _fclose(file: *mut u8) -> i32;
    fn FUN_1005b1b0(param_1: *const u8);
}

unsafe fn rd(p: *mut u32, i: isize) -> u32 {
    // SAFETY: the caller passes a pointer into the game's tables.
    unsafe { p.wrapping_offset(i).read_unaligned() }
}

unsafe fn wr(p: *mut u32, i: isize, v: u32) {
    // SAFETY: the caller passes a pointer into the game's tables.
    unsafe { p.wrapping_offset(i).write_unaligned(v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10053a40() -> u32 {
    // SAFETY: all globals, tables and callees are the game's own memory;
    // table walks read and write where the original does.
    unsafe {
        let end7c = (&raw mut DAT_100bed7c) as usize;
        let end78 = (&raw mut DAT_100bed78) as usize;
        let end70 = (&raw mut DAT_100bed70) as usize;

        let mut p4 = (&raw mut DAT_100bdf9c) as *mut u32;
        loop {
            let mut u5 = rd(p4, -3);
            if (u5 as i32) > -1 {
                if (u5 as i32) < 2 {
                    wr(p4, 0, 0);
                } else if u5 == 2 {
                    u5 = rd(p4, -1);
                    if rd(p4, 1) == rd(p4, 2) {
                        u5 = (u5 & 7) << 4;
                    }
                    wr(p4, 0, u5);
                }
            }
            p4 = p4.wrapping_add(6);
            if !((p4 as usize) < end7c) {
                break;
            }
        }

        let mut i7: i32 = 0;
        p4 = (&raw mut DAT_100bdfa0) as *mut u32;
        loop {
            if (rd(p4, -3) as i32) > -1 && (rd(p4, -2) & 7) != 0 {
                let u5 = (rd(p4, -2) & 7) << 4;
                let u1 = rd(p4, -4);
                if u1 == 0 {
                    let mut i6: i32 = 0;
                    let mut p2 = (&raw mut DAT_100bdf98) as *mut u32;
                    loop {
                        if i6 != i7
                            && (rd(p2, -1) as i32) > -1
                            && rd(p2, -2) == 0
                            && rd(p2, 2) == rd(p4, 0)
                        {
                            let v = rd(p2, 0) | u5;
                            wr(p2, 0, v);
                        }
                        p2 = p2.wrapping_add(6);
                        i6 = i6.wrapping_add(1);
                        if !((p2 as usize) < end78) {
                            break;
                        }
                    }
                } else if (u1 as i32) > 0 && (u1 as i32) < 3 {
                    let mut i6: i32 = 0;
                    let mut p3 = (&raw mut DAT_100bdf90) as *mut u32;
                    loop {
                        if i6 != i7 && (rd(p3, 1) as i32) > -1 && rd(p3, 0) != 0 {
                            if rd(p3, 4) == rd(p4, 0) {
                                let v = rd(p3, 2) | u5;
                                wr(p3, 2, v);
                            }
                            if rd(p3, 0) == 2 {
                                if rd(p3, 5) == rd(p4, 0) {
                                    let v = rd(p3, 3) | u5;
                                    wr(p3, 3, v);
                                }
                                if rd(p3, 0) != 2 {
                                    // goto LAB_10053b5e: skip the rest
                                    p3 = p3.wrapping_add(6);
                                    i6 = i6.wrapping_add(1);
                                    if !((p3 as usize) < end70) {
                                        break;
                                    }
                                    continue;
                                }
                                if rd(p4, 1) == rd(p3, 4) {
                                    let v = rd(p3, 2) | ((rd(p4, -1) & 7) << 4);
                                    wr(p3, 2, v);
                                }
                            }
                            if rd(p3, 0) == 2 && rd(p3, 5) == rd(p4, 1) {
                                let v = rd(p3, 3) | ((rd(p4, -1) & 7) << 4);
                                wr(p3, 3, v);
                            }
                        }
                        p3 = p3.wrapping_add(6);
                        i6 = i6.wrapping_add(1);
                        if !((p3 as usize) < end70) {
                            break;
                        }
                    }
                }
            }
            p4 = p4.wrapping_add(6);
            i7 = i7.wrapping_add(1);
            if 0x100bed7fusize < (p4 as usize) {
                let file = FID_conflict___wfopen(&raw const DAT_1009fdc4, &raw const DAT_1009fdc0);
                DAT_1009f378 = 0;
                DAT_1009f37c = 0;
                if file.is_null() {
                    FUN_1005b1b0(&raw const DAT_1009fe70);
                } else {
                    let mut u5: u32 = 0;
                    let mut p = (&raw mut DAT_100bdf98) as *mut u32;
                    FID_conflict__fwprintf(file, &raw const DAT_1009fdd0);
                    loop {
                        if (rd(p, -1) as i32) > -1 {
                            let tbl = &raw const DAT_1009c5f8;
                            let s = tbl.wrapping_add((u5 % 0x25) as usize).read_unaligned();
                            FUN_10053780(
                                file,
                                s as usize as *const u8,
                                p.wrapping_offset(-2) as *mut i32,
                            );
                            let v = rd(p, 0) & 0x8000_0007;
                            wr(p, 0, v);
                            wr(p, 1, 0);
                        }
                        p = p.wrapping_add(6);
                        u5 = u5.wrapping_add(1);
                        if !((p as usize) < end78) {
                            break;
                        }
                    }
                    let mut loc: [u32; 6] = [1, DAT_1009c6ac, 0, 0xffff_ffff, 0x66, 0xffff_ffff];
                    let lp = loc.as_mut_ptr() as *mut i32;
                    FUN_10053780(file, &raw const DAT_1009fe00, lp);
                    FUN_10053780(file, &raw const DAT_1009fe10, lp);
                    loc[4] = 100;
                    FUN_10053780(file, &raw const DAT_1009fe20, lp);
                    FUN_10053780(file, &raw const DAT_1009fe30, lp);
                    FID_conflict__fwprintf(file, &raw const DAT_1009fe40, DAT_1009f378);
                    FID_conflict__fwprintf(file, &raw const DAT_1009fe58, DAT_1009f37c);
                    _fclose(file);
                }
                if DAT_1009f378 < 0x29 && DAT_1009f37c < 0x5b {
                    return 1;
                }
                return 0;
            }
        }
    }
}
