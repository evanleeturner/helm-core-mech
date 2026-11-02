// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7b70: u32;
    static mut DAT_100a9670: u8;
    static mut DAT_100a9688: u8;
    static mut DAT_100a9690: u8;
    static mut DAT_100a9694: u8;
    static mut DAT_100a9698: u8;
    static mut DAT_100aafac: u32;
    static mut DAT_100adf34: u32;
    static mut DAT_100b06e4: u32;
    static mut DAT_100b06e8: i32;
    static mut DAT_100b3904: u32;
    static mut DAT_10160bc0: u32;
    static mut DAT_10160bc4: u32;
    static mut DAT_10160bc8: u32;
    static mut DAT_10160bcc: u32;
    static mut DAT_10160bd0: u32;
    static mut DAT_10160bd4: u32;
    static mut DAT_101f26f0: u8;
    static mut DAT_101f26f8: u8;
    static mut DAT_101f26fc: u8;
    static mut DAT_101f2700: u8;
    static mut DAT_101f2704: u8;
    static mut DAT_101f2708: u8;
    static mut DAT_101f270c: u8;
    static mut DAT_101f2710: u8;
    static mut DAT_101f2714: u8;
    static mut DAT_101f2718: u8;
    static mut PTR_DAT_100aaf68: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1004c273() -> u8;
    fn FUN_1004bfb6() -> u8;
    fn FUN_10060c34() -> u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002f322(param_1: i32, param_2: *mut u32, param_3: *mut u32, param_4: *mut u32) -> u8;
    fn FUN_1004b839(
        param_1: i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
        param_7: i32,
    ) -> u8;
    fn FUN_1004b9f3(
        param_1: u32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
    ) -> u8;
    fn FUN_1007ca5c(param_1: i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32) -> u8;
    fn FUN_1002fc2b(param_1: *mut u32) -> u32;
    fn FUN_1002fcce(param_1: *mut i32) -> u8;
    fn FUN_1002f727(param_1: i32) -> u8;
    fn FUN_1005f7ad(param_1: i16, param_2: u16) -> u8;
    fn FUN_1005f81d(param_1: i16, param_2: u16) -> u8;
    fn FUN_10049a38(param_1: i32) -> u8;
}

fn at(base: *mut u8, off: i32) -> *mut u8 {
    base.wrapping_offset(off as isize)
}

unsafe fn rd(p: *mut u8) -> i32 {
    // SAFETY: caller passes an address inside the game's own tables.
    unsafe { p.cast::<i32>().read_unaligned() }
}

unsafe fn rda(a: i32) -> i32 {
    // SAFETY: caller passes an address the original dereferences.
    unsafe { (a as usize as *const i32).read_unaligned() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1004b4ef() {
    // SAFETY: all globals, tables and callees are the game's own memory;
    // reads, writes and calls happen in the original's order.
    unsafe {
        FUN_1004c273();
        if (DAT_100adf34.wrapping_add(4) as usize as *const u8).read() != 0 {
            FUN_1004bfb6();
        }
        let mut local_10: i32 = 0;
        while local_10 < 0xff {
            let iv4 = local_10.wrapping_mul(0x2c);
            let p7 = at(&raw mut DAT_101f26f0, iv4) as *mut i32;
            if rd(at(&raw mut DAT_101f2708, iv4)) != 0 {
                let iv8 = rd(at(&raw mut DAT_101f2714, iv4)).wrapping_mul(0x2c);
                if (at(&raw mut DAT_100a9698, iv8).read() & 1) != 0 {
                    FUN_1002f322(
                        p7.read_unaligned(),
                        at(&raw mut DAT_101f26f8, iv4) as *mut u32,
                        at(&raw mut DAT_101f26fc, iv4) as *mut u32,
                        at(&raw mut DAT_101f2700, iv4) as *mut u32,
                    );
                }
                if rd(at(&raw mut DAT_100a9688, iv8)) != 0 {
                    let iv1 = rd(at(&raw mut DAT_101f26f8, iv4));
                    let iv2 = rd(at(&raw mut DAT_101f26fc, iv4));
                    let iv3 = rd(at(&raw mut DAT_101f2700, iv4));
                    let local_28: i32;
                    if rd(at(&raw mut DAT_101f2714, iv4)) == 0x23 {
                        local_28 = rd(at(&raw mut DAT_100a9670, iv8))
                            .wrapping_sub(rd(at(&raw mut DAT_101f2704, iv4)))
                            .wrapping_mul(0x18);
                    } else {
                        let a = rda(p7.read_unaligned().wrapping_add(0x6c));
                        local_28 = rda(a.wrapping_add(0x40));
                    }
                    if (((DAT_100adf34.wrapping_add(2) as usize as *const u8).read() != 0)
                        || (rd(at(&raw mut DAT_101f2714, iv4)) == 0x16))
                        || (rd(at(&raw mut DAT_101f2714, iv4)) == 0x23)
                    {
                        FUN_1004b839(
                            rd(at(&raw mut DAT_101f2718, iv4)),
                            iv1,
                            iv2,
                            iv3,
                            local_28,
                            rd(at(&raw mut DAT_100a9690, iv8)),
                            rd(at(&raw mut DAT_100a9694, iv8)),
                        );
                    }
                    FUN_1004b9f3(
                        rd(at(&raw mut DAT_101f2718, iv4)) as u32,
                        iv1,
                        iv2,
                        iv3,
                        local_28,
                        rd(at(&raw mut DAT_100a9690, iv8)),
                    );
                    FUN_1007ca5c(iv1, iv2, iv3, local_28, rd(at(&raw mut DAT_100a9690, iv8)));
                }
                let p2704 = at(&raw mut DAT_101f2704, iv4);
                let nv = rd(p2704).wrapping_sub(DAT_100b3904 as i32);
                p2704.cast::<i32>().write_unaligned(nv);
                if rd(p2704) < 1 {
                    let mut b5 = true;
                    let mut b6 = false;
                    if (rd(at(&raw mut DAT_101f2710, iv4)) != 0 && DAT_100b06e4 != 0)
                        && local_10 == DAT_100b06e8
                    {
                        let r = FUN_10060c34();
                        if r == 0 {
                            b6 = true;
                        } else {
                            b5 = false;
                        }
                    }
                    if b5 {
                        if b6 {
                            DAT_100a7b70 = 1;
                            let base = PTR_DAT_100aaf68;
                            (base.wrapping_add(0x1c) as usize as *mut u32)
                                .write_unaligned(DAT_10160bc0);
                            (base.wrapping_add(0x20) as usize as *mut u32)
                                .write_unaligned(DAT_10160bc4);
                            (base.wrapping_add(0x24) as usize as *mut u32)
                                .write_unaligned(DAT_10160bc8);
                            (PTR_DAT_100aaf68.wrapping_add(0x28) as usize as *mut i16)
                                .write_unaligned(DAT_10160bcc as i16);
                            (PTR_DAT_100aaf68.wrapping_add(0x2a) as usize as *mut i16)
                                .write_unaligned(DAT_10160bd0 as i16);
                            DAT_100aafac = DAT_10160bd4;
                            at(&raw mut DAT_101f2710, iv4)
                                .cast::<u32>()
                                .write_unaligned(0);
                            DAT_100b06e4 = 0;
                            DAT_100b06e8 = -1;
                        }
                        if p7.read_unaligned() != 0 {
                            let r = FUN_1002fc2b(p7.read_unaligned() as usize as *mut u32);
                            if r != 0 {
                                FUN_1002fcce(p7.read_unaligned() as usize as *mut i32);
                            }
                            FUN_1002f727(p7.read_unaligned());
                        }
                        if 0 < rd(at(&raw mut DAT_101f270c, iv4)) {
                            FUN_1005f7ad(rd(at(&raw mut DAT_101f270c, iv4)) as i16, 0);
                            FUN_1005f81d(rd(at(&raw mut DAT_101f270c, iv4)) as i16, 0);
                        }
                        FUN_10049a38(local_10);
                    }
                }
            }
            local_10 = local_10.wrapping_add(1);
        }
    }
}
