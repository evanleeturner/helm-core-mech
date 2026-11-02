// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aaf70: u8;
    static mut DAT_100adf34: u32;
    static mut DAT_100adf38: u32;
    static mut DAT_100b395c: u32;
    static mut DAT_100eeedc: u32;
    static mut DAT_100eef40: u32;
    static mut DAT_10114ec8: u32;
    static mut DAT_100aaf68: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10051f43(param_1: i32) -> i32;
    fn FUN_100689bf(param_1: i32);
    fn FUN_1002ea87(param_1: i32);
    fn FUN_1002ebd9(param_1: i32);
    fn FUN_1002ed27(param_1: i32);
    fn FUN_1002eded(param_1: i32);
    fn FUN_1002e822(param_1: i32);
    fn FUN_1006147f(param_1: i32);
    fn FUN_10008c56(param_1: *mut i32);
    fn FUN_10066609(param_1: *mut i32, param_2: i32);
    fn FUN_10058e74(param_1: *mut i32, param_2: u8) -> u32;
    fn FUN_10068905(param_1: *mut u32);
    fn FUN_1006afdd(param_1: i32, param_2: u32, param_3: *mut u32, param_4: i32);
    fn FUN_10068a11(param_1: i32, param_2: i32, param_3: i32);
    fn FUN_1001a6ca(param_1: *mut u32, param_2: *mut u32) -> u32;
    fn FUN_1002f727(param_1: i32);
    fn FUN_1002f783(param_1: i32);
    fn FUN_10068962(param_1: *mut u32);
    fn FUN_1001a8a3(
        param_1: *mut i32,
        param_2: *mut i32,
        param_3: *mut i32,
        param_4: *mut i32,
        param_5: *mut i32,
        param_6: *mut i32,
    );
    fn FUN_1006b002(param_1: i32, param_2: u32, param_3: *mut u32, param_4: i32, param_5: *mut i32);
    fn FUN_10011ae1(param_1: i32, param_2: *mut i32);
    fn FUN_100117f2(param_1: i32, param_2: u32, param_3: i32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1004baf4() -> *mut u32;
    fn FUN_1004bb42() -> u32;
}

unsafe fn rd(addr: i32) -> i32 {
    // SAFETY: the caller passes an address of readable game memory.
    unsafe { (addr as usize as *const i32).read_unaligned() }
}

unsafe fn copy_out(src: *const u32) {
    // SAFETY: src holds 26 dwords; the destination is the game's own buffer.
    unsafe {
        let mut s = src;
        let mut d = (&raw mut DAT_100aaf70).cast::<u32>();
        let mut n = 0x1a;
        while n != 0 {
            d.write_unaligned(s.read_unaligned());
            s = s.add(1);
            d = d.add(1);
            n -= 1;
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100682e7(param_1: i32) {
    // SAFETY: param_1 points at the game's object; globals and callees are
    // the game's own; the local buffer mirrors the original stack frame.
    unsafe {
        let mut buf = [0u32; 33];
        let b = buf.as_mut_ptr();
        let bi = b.cast::<i32>();

        if rd(param_1.wrapping_add(4)) as i16 == 0 {
            return;
        }
        let v = rd(param_1.wrapping_add(0x30));
        if v == 0 {
            return;
        }
        (param_1.wrapping_add(0xb0) as usize as *mut u32).write_unaligned(DAT_10114ec8);
        let s6 = (rd(param_1.wrapping_add(6)) & 0xffff) as u16 as i16;
        let notset = v != 1 && v != 9 && v != 10 && v != 0xc && v != 0xb && v != 0xd && v != 2;
        if s6 == 1 && notset {
            if DAT_100b395c != 0 {
                let r = FUN_10051f43(10);
                if r < 7 {
                    DAT_100b395c = 0;
                }
                FUN_100689bf(param_1);
                return;
            }
            let r = FUN_10051f43(10);
            if r < 3 {
                DAT_100b395c = 1;
            }
        } else if 2 < s6 && notset {
            FUN_100689bf(param_1);
            return;
        }

        let tbl = (&raw mut DAT_100eef40) as *const u32;
        let ent = tbl.add(DAT_100adf38 as usize).read_unaligned();
        let piVar1 = rd((ent as i32).wrapping_add(0x20)) as usize as *mut i32;
        let p1 = piVar1 as usize as i32;
        let base = rd(p1);
        let a4 = rd(param_1.wrapping_add(0xa4));
        let c = rd(param_1.wrapping_add(0xc));

        match v as u32 {
            0 => {}
            2 => {
                FUN_10011ae1(p1, a4 as usize as *mut i32);
            }
            3 => {
                FUN_1001a6ca(DAT_100aaf68 as usize as *mut u32, b);
                FUN_10068905(b.add(7));
                if DAT_100eeedc == 0 {
                    bi.add(3)
                        .write(rd(rd(p1).wrapping_add(0x60)).wrapping_add(0xb40000));
                } else {
                    let q = rd(p1);
                    bi.add(3)
                        .write(rd(q.wrapping_add(0x60)).wrapping_add(rd(q.wrapping_add(0x6c))));
                }
                bi.add(4).write(0);
                FUN_1002f727(rd(rd(p1).wrapping_add(0x40)));
                FUN_1006afdd(c, 0x20000, b, 0);
                FUN_1002f783(rd(rd(p1).wrapping_add(0x40)));
                if DAT_100eeedc == 0 {
                    FUN_10068a11(param_1, 6, 0xd6);
                } else {
                    FUN_10066609(a4 as usize as *mut i32, 6);
                }
                copy_out(b.add(7));
            }
            4 => {
                FUN_1001a6ca(DAT_100aaf68 as usize as *mut u32, b);
                FUN_10068962(b.add(7));
                FUN_1001a8a3(bi.add(4), bi.add(3), bi.add(5), bi, bi.add(1), bi.add(2));
                FUN_1002f727(rd(rd(p1).wrapping_add(0x40)));
                FUN_1006b002(c, 0x20000, b, 0, piVar1);
                FUN_1002f783(rd(rd(p1).wrapping_add(0x40)));
                FUN_10066609(a4 as usize as *mut i32, 6);
                copy_out(b.add(7));
            }
            5 => {
                FUN_1001a6ca(DAT_100aaf68 as usize as *mut u32, b);
                FUN_10068905(b.add(7));
                let q = rd(p1);
                bi.write(rd(q.wrapping_add(0x50)));
                bi.add(1).write(rd(rd(p1).wrapping_add(0x54)));
                bi.add(2).write(rd(rd(p1).wrapping_add(0x58)));
                bi.add(4).write(0x5a0000);
                bi.add(5).write(0);
                FUN_1002f727(rd(rd(p1).wrapping_add(0x40)));
                FUN_1006afdd(c, 0x20000, b, 0);
                FUN_1002f783(rd(rd(p1).wrapping_add(0x40)));
                FUN_10068a11(param_1, 6, 0xd3);
                copy_out(b.add(7));
            }
            6 => {
                let g = DAT_100adf34.wrapping_add(9) as usize as *const u8;
                if g.read() != 0 {
                    FUN_10008c56(a4 as usize as *mut i32);
                    FUN_10066609(a4 as usize as *mut i32, 6);
                }
            }
            7 => {
                let mut local_90 = FUN_1004baf4();
                if local_90.is_null() {
                    FUN_1004bb42();
                    local_90 = FUN_1004baf4();
                }
                if local_90.is_null() {
                    FUN_10058e74(a4 as usize as *mut i32, 0);
                } else {
                    FUN_10068905(b.add(7));
                    local_90.add(4).write_unaligned(0);
                    FUN_1006afdd(c, 0x20000, local_90, 0);
                    copy_out(b.add(7));
                }
                FUN_10068a11(param_1, 6, 0xd9);
            }
            8 => {
                FUN_1006147f(param_1);
            }
            9 => {
                FUN_1002e822(param_1);
            }
            10 => {
                FUN_1002eded(param_1);
            }
            0xb => {
                FUN_1002ea87(param_1);
            }
            0xc => {
                FUN_1002ebd9(param_1);
            }
            0xd => {
                FUN_1002ed27(param_1);
            }
            _ => {
                FUN_100117f2(p1, a4 as u32, c);
            }
        }
        let _ = base;
    }
}
