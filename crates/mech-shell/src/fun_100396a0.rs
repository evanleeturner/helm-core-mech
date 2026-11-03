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
    fn FUN_1006ece0(buf: *mut u8, size: u32, count: u32, file: *mut u8) -> u32;
    fn _fclose(file: *mut u8) -> i32;
    fn FUN_10018060(param_1: i32, param_2: i32) -> i32;
}

macro_rules! fill {
    ($off:expr, $dat:expr) => {{
        let dst = (mech_trace::gr!("100396a0", "10083508", DAT_10083508) as usize)
            .wrapping_add($off) as *mut i32;
        let d = $dat;
        if d == 0 {
            mech_trace::wr!("100396a0", dst, write_unaligned, 0);
        } else {
            mech_trace::wr!(
                "100396a0",
                dst,
                write_unaligned,
                mech_trace::rd!(
                    "100396a0",
                    (d.wrapping_add(4) as usize as *const i32),
                    read_unaligned
                )
            );
        }
    }};
}

macro_rules! rd_loop {
    ($file:expr, $off:expr, $dat:expr, $size:expr) => {{
        let cp = (mech_trace::gr!("100396a0", "10083508", DAT_10083508) as usize).wrapping_add($off)
            as *const i32;
        let mut i: i32 = 0;
        let mut count: i32 = mech_trace::rd!("100396a0", cp, read_unaligned);
        if 0 < count {
            loop {
                let i5 = i.wrapping_add(1);
                let p = mech_trace::cr!("100396a0", "10018060", FUN_10018060($dat, i)) as usize
                    as *mut u8;
                mech_trace::cr!("100396a0", "1006ece0", FUN_1006ece0(p, $size, 1, $file));
                i = i5;
                count = mech_trace::rd!("100396a0", cp, read_unaligned);
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
    mech_trace::enter!("100396a0", param_1);
    // SAFETY: all globals, the state struct and the callees belong to the
    // game; offsets and reads/writes follow the original exactly.
    unsafe {
        let file = mech_trace::cr!(
            "100396a0",
            "FID_conflict___wfopen",
            FID_conflict___wfopen(param_1, &raw mut DAT_10094530)
        );
        if file.is_null() {
            return mech_trace::rt!("100396a0", 0);
        }

        let base = mech_trace::gr!("100396a0", "10083508", DAT_10083508) as usize;
        let u1: i32 = mech_trace::rd!(
            "100396a0",
            ((base.wrapping_add(0x23c3)) as *const i32),
            read_unaligned
        );
        let u2: i32 = mech_trace::rd!(
            "100396a0",
            ((base.wrapping_add(0x23c7)) as *const i32),
            read_unaligned
        );
        mech_trace::wr!(
            "100396a0",
            ((base.wrapping_add(0x23c7)) as *mut i32),
            write_unaligned,
            u1
        );
        mech_trace::wr!(
            "100396a0",
            (mech_trace::gr!("100396a0", "10083508", DAT_10083508) as usize as *mut u8),
            write,
            0
        );

        fill!(0x12e, mech_trace::gr!("100396a0", "100a6798", DAT_100a6798));
        fill!(0x132, mech_trace::gr!("100396a0", "100a67a4", DAT_100a67a4));
        fill!(0x136, mech_trace::gr!("100396a0", "100a67a8", DAT_100a67a8));
        fill!(0x13a, mech_trace::gr!("100396a0", "100a67b4", DAT_100a67b4));
        fill!(0x13e, mech_trace::gr!("100396a0", "100a67b8", DAT_100a67b8));
        fill!(0x142, mech_trace::gr!("100396a0", "100a679c", DAT_100a679c));
        fill!(0x146, mech_trace::gr!("100396a0", "100a67e4", DAT_100a67e4));
        fill!(0x14a, mech_trace::gr!("100396a0", "100a67e8", DAT_100a67e8));
        fill!(0x14e, mech_trace::gr!("100396a0", "100a67cc", DAT_100a67cc));
        fill!(0x152, mech_trace::gr!("100396a0", "100a67d0", DAT_100a67d0));
        fill!(0x156, mech_trace::gr!("100396a0", "100a67b0", DAT_100a67b0));
        fill!(0x15a, mech_trace::gr!("100396a0", "100a67ac", DAT_100a67ac));

        mech_trace::cr!(
            "100396a0",
            "1006ece0",
            FUN_1006ece0(
                mech_trace::gr!("100396a0", "10083508", DAT_10083508) as usize as *mut u8,
                0x2dfb,
                1,
                file
            )
        );

        let base = mech_trace::gr!("100396a0", "10083508", DAT_10083508) as usize;
        mech_trace::wr!(
            "100396a0",
            ((base.wrapping_add(0x23c7)) as *mut i32),
            write_unaligned,
            u2
        );
        mech_trace::wr!(
            "100396a0",
            ((base.wrapping_add(0x23c3)) as *mut i32),
            write_unaligned,
            u1
        );

        rd_loop!(
            file,
            0x156,
            mech_trace::gr!("100396a0", "100a67b0", DAT_100a67b0),
            0x853
        );
        rd_loop!(
            file,
            0x15a,
            mech_trace::gr!("100396a0", "100a67ac", DAT_100a67ac),
            0x1a
        );
        rd_loop!(
            file,
            0x142,
            mech_trace::gr!("100396a0", "100a679c", DAT_100a679c),
            0x853
        );
        rd_loop!(
            file,
            0x142,
            mech_trace::gr!("100396a0", "100a67a0", DAT_100a67a0),
            0x853
        );
        rd_loop!(
            file,
            0x146,
            mech_trace::gr!("100396a0", "100a67e4", DAT_100a67e4),
            0x2066
        );
        rd_loop!(
            file,
            0x14a,
            mech_trace::gr!("100396a0", "100a67e8", DAT_100a67e8),
            0x205e
        );
        rd_loop!(
            file,
            0x14e,
            mech_trace::gr!("100396a0", "100a67cc", DAT_100a67cc),
            0x1a
        );
        rd_loop!(
            file,
            0x152,
            mech_trace::gr!("100396a0", "100a67d0", DAT_100a67d0),
            0x1a
        );
        rd_loop!(
            file,
            0x12e,
            mech_trace::gr!("100396a0", "100a6798", DAT_100a6798),
            0x853
        );
        rd_loop!(
            file,
            0x12e,
            mech_trace::gr!("100396a0", "100a678c", DAT_100a678c),
            0x853
        );
        rd_loop!(
            file,
            0x132,
            mech_trace::gr!("100396a0", "100a67a4", DAT_100a67a4),
            0x2066
        );
        rd_loop!(
            file,
            0x136,
            mech_trace::gr!("100396a0", "100a67a8", DAT_100a67a8),
            0x205e
        );
        rd_loop!(
            file,
            0x13a,
            mech_trace::gr!("100396a0", "100a67b4", DAT_100a67b4),
            0x1a
        );
        rd_loop!(
            file,
            0x13e,
            mech_trace::gr!("100396a0", "100a67b8", DAT_100a67b8),
            0x1a
        );

        let r = mech_trace::cr!("100396a0", "_fclose", _fclose(file));
        mech_trace::rt!("100396a0", ((r as u32) & 0xffffff00) | 1)
    }
}
