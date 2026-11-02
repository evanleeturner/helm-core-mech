// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10083508: i32;
    static mut DAT_10094530: u8;
    static mut DAT_100a678c: i32;
    static mut DAT_100a6798: i32;
    static mut DAT_100a679c: i32;
    static mut DAT_100a67a0: i32;
    static mut DAT_100a67a4: i32;
    static mut DAT_100a67a8: i32;
    static mut DAT_100a67ac: i32;
    static mut DAT_100a67b0: i32;
    static mut DAT_100a67b4: i32;
    static mut DAT_100a67b8: i32;
    static mut DAT_100a67cc: i32;
    static mut DAT_100a67d0: i32;
    static mut DAT_100a67e4: i32;
    static mut DAT_100a67e8: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FID_conflict___wfopen(param_1: *mut i8, param_2: *mut u8) -> *mut u8;
    fn _fread(buf: *mut u8, size: u32, count: u32, file: *mut u8) -> u32;
    fn _fclose(file: *mut u8) -> i32;
    fn FUN_10018060(param_1: i32, param_2: i32) -> i32;
}

macro_rules! fill {
    ($off:expr, $dat:expr) => {{
        let dst = (DAT_10083508 as usize).wrapping_add($off) as *mut i32;
        let d = $dat;
        if d == 0 {
            dst.write_unaligned(0);
        } else {
            dst.write_unaligned((d.wrapping_add(4) as usize as *const i32).read_unaligned());
        }
    }};
}

macro_rules! rd_loop {
    ($file:expr, $off:expr, $dat:expr, $size:expr) => {{
        let cp = (DAT_10083508 as usize).wrapping_add($off) as *const i32;
        let mut i: i32 = 0;
        let mut count: i32 = cp.read_unaligned();
        if 0 < count {
            loop {
                let i5 = i.wrapping_add(1);
                let p = FUN_10018060($dat, i) as usize as *mut u8;
                _fread(p, $size, 1, $file);
                i = i5;
                count = cp.read_unaligned();
                if !(i5 < count) {
                    break;
                }
            }
        }
    }};
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100396a0(param_1: *mut i8) -> u32 {
    // SAFETY: all globals, the state struct and the callees belong to the
    // game; offsets and reads/writes follow the original exactly.
    unsafe {
        let file = FID_conflict___wfopen(param_1, &raw mut DAT_10094530);
        if file.is_null() {
            return 0;
        }

        let base = DAT_10083508 as usize;
        let u1: i32 = ((base.wrapping_add(0x23c3)) as *const i32).read_unaligned();
        let u2: i32 = ((base.wrapping_add(0x23c7)) as *const i32).read_unaligned();
        ((base.wrapping_add(0x23c7)) as *mut i32).write_unaligned(u1);
        (DAT_10083508 as usize as *mut u8).write(0);

        fill!(0x12e, DAT_100a6798);
        fill!(0x132, DAT_100a67a4);
        fill!(0x136, DAT_100a67a8);
        fill!(0x13a, DAT_100a67b4);
        fill!(0x13e, DAT_100a67b8);
        fill!(0x142, DAT_100a679c);
        fill!(0x146, DAT_100a67e4);
        fill!(0x14a, DAT_100a67e8);
        fill!(0x14e, DAT_100a67cc);
        fill!(0x152, DAT_100a67d0);
        fill!(0x156, DAT_100a67b0);
        fill!(0x15a, DAT_100a67ac);

        _fread(DAT_10083508 as usize as *mut u8, 0x2dfb, 1, file);

        let base = DAT_10083508 as usize;
        ((base.wrapping_add(0x23c7)) as *mut i32).write_unaligned(u2);
        ((base.wrapping_add(0x23c3)) as *mut i32).write_unaligned(u1);

        rd_loop!(file, 0x156, DAT_100a67b0, 0x853);
        rd_loop!(file, 0x15a, DAT_100a67ac, 0x1a);
        rd_loop!(file, 0x142, DAT_100a679c, 0x853);
        rd_loop!(file, 0x142, DAT_100a67a0, 0x853);
        rd_loop!(file, 0x146, DAT_100a67e4, 0x2066);
        rd_loop!(file, 0x14a, DAT_100a67e8, 0x205e);
        rd_loop!(file, 0x14e, DAT_100a67cc, 0x1a);
        rd_loop!(file, 0x152, DAT_100a67d0, 0x1a);
        rd_loop!(file, 0x12e, DAT_100a6798, 0x853);
        rd_loop!(file, 0x12e, DAT_100a678c, 0x853);
        rd_loop!(file, 0x132, DAT_100a67a4, 0x2066);
        rd_loop!(file, 0x136, DAT_100a67a8, 0x205e);
        rd_loop!(file, 0x13a, DAT_100a67b4, 0x1a);
        rd_loop!(file, 0x13e, DAT_100a67b8, 0x1a);

        let r = _fclose(file);
        ((r as u32) & 0xffffff00) | 1
    }
}
