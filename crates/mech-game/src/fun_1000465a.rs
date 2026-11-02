// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa1ac: i32;
    static mut DAT_100adf38: i32;
    static mut DAT_100adf34: i32;
    static mut DAT_100b3904: i32;
    static mut DAT_100b38fc: i32;
    static mut _DAT_100a1030: f64;
    static mut _DAT_100a1038: f64;
    static mut _DAT_100a1040: f64;
    static mut DAT_100a4118: i32;
    static mut _DAT_100c4c8c: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100082c4(param_1: i32, param_2: i32);
    fn FUN_10051f43(param_1: i32) -> i32;
    fn FUN_100044c6(param_1: i32, param_2: *mut i32);
    fn FUN_10004b19(param_1: i32, param_2: *mut i32);
    fn FUN_10003742(param_1: i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000465a(param_1: *mut i32) {
    // SAFETY: param_1 points at the game's object; globals and callees are the
    // game's own; all reads and writes follow the original's offsets and order.
    unsafe {
        let p = param_1;
        let obj = || p.read_unaligned() as *const u8;
        let w14 = || obj().byte_add(0x14).cast::<u16>().read_unaligned();
        let id = || obj().byte_add(4).cast::<i32>().read_unaligned();
        let g = |i: usize| p.add(i).read_unaligned();
        let s = |i: usize, v: i32| p.add(i).write_unaligned(v);
        let fp = p.add(0x110).cast::<u16>();
        let fl = || fp.read_unaligned();

        if (w14() & 2) != 0 || (w14() & 4) != 0 {
            return;
        }
        if !(DAT_100aa1ac == 0 || id() == DAT_100adf38) {
            return;
        }
        if (*(DAT_100adf34.wrapping_add(4) as *const u8) == 0) && id() == DAT_100adf38 {
            s(0xf3, 0);
            s(0xf1, 0);
            return;
        }
        let prod = (g(0xf4) as u32).wrapping_mul(DAT_100b3904 as u32);
        let shifted = prod.wrapping_shl((g(0xf5) == 3) as u32);
        let mut i_var1 = (g(0xf1) as u32).wrapping_sub(shifted) as i32;
        s(0xf3, (g(0xf3) as u32).wrapping_add(i_var1 as u32) as i32);
        if g(0xf3) < 0 {
            s(0xf3, 0);
        }
        let i_var2 = g(0xf3) >> 0x10;
        if (fl() & 4) != 0
            && (fl() & 8) == 0
            && g(0xf5) != 3
            && 0x43e < (DAT_100b38fc as u32).wrapping_sub(g(0xef) as u32) as i32
        {
            s(0xf5, 3);
            s(0xef, DAT_100b38fc);
            if id() == DAT_100adf38 {
                FUN_100082c4(0xd, -1);
            }
        }
        if i_var2 < 0x65 {
            if (i_var2 as f64) <= _DAT_100a1030 {
                if (i_var2 as f64) <= _DAT_100a1038 {
                    if (fl() & 4) == 0 {
                        if DAT_100a4118 != 0
                            && 0x2d4
                                < (DAT_100b38fc as u32).wrapping_sub(_DAT_100c4c8c as u32) as i32
                            && id() == DAT_100adf38
                            && (i_var2 as f64) < _DAT_100a1040
                        {
                            DAT_100a4118 = 0;
                        }
                        fp.write_unaligned(fl() & 0xefff);
                    } else {
                        if g(0xf5) == 3
                            && 0x2d4 < (DAT_100b38fc as u32).wrapping_sub(g(0xef) as u32) as i32
                        {
                            fp.write_unaligned(fl() & 0xfffb);
                            s(0xf5, 0);
                        }
                        if (fl() & 8) != 0 {
                            fp.write_unaligned(fl() & 0xfff7);
                            fp.write_unaligned(fl() & 0xfffb);
                        }
                    }
                } else if id() == DAT_100adf38 && DAT_100a4118 == 0 && 0 < i_var1 {
                    _DAT_100c4c8c = DAT_100b38fc;
                    DAT_100a4118 = 1;
                    FUN_100082c4(0, -1);
                }
            } else if (fl() & 4) == 0 {
                s(0xef, DAT_100b38fc);
                fp.write_unaligned(fl() | 4);
                if (fl() & 0x1000) == 0 && id() == DAT_100adf38 {
                    fp.write_unaligned(fl() | 0x1000);
                    FUN_100082c4(3, -1);
                    FUN_10003742(0xfe, 100, 0x40, 5, 0x50);
                }
            }
        } else if id() != DAT_100adf38 || *(DAT_100adf34.wrapping_add(1) as *const u8) == 0 {
            if (fl() & 8) == 0 {
                if 0xe24 < (DAT_100b38fc as u32).wrapping_sub(g(0xef) as u32) as i32 {
                    FUN_10004b19(id(), p);
                }
            } else if DAT_100b3904 != 0 {
                i_var1 = FUN_10051f43((4000i64 / (DAT_100b3904 as i64)) as i32);
                if i_var1 < 3 {
                    FUN_100044c6(id(), p);
                }
            }
        }
    }
}
