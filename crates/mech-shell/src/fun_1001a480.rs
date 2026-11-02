// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10083d8c: *mut i8;
    static mut DAT_1008fb70: [i8; 4];
    static mut DAT_10093c10: i32;
    static mut DAT_1008f908: [i16; 45];
    static mut DAT_1008f970: [i16; 8];
    static mut DAT_100a67a4: i32;
    static mut DAT_100a67a8: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn __strnicmp(s1: *const i8, s2: *const i8, n: u32) -> i32;
    fn _strncpy(dst: *mut i8, src: *const i8, n: u32) -> *mut i8;
    fn FUN_1005e020(param_1: i32, param_2: *mut i8) -> i32;
    fn FUN_10018060(param_1: i32, param_2: i32) -> i32;
    fn FUN_1000adb0(param_1: i32, param_2: i32) -> i32;
    fn FUN_10036660(param_1: *mut u8, param_2: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001a480(
    param_1: *mut i8,
    param_2: *mut i8,
    param_3: *mut i8,
    param_4: i32,
    param_5: i32,
    param_6: i32,
    param_7: i32,
    param_8: i32,
) {
    mech_trace::enter!(
        "1001a480", param_1, param_2, param_3, param_4, param_5, param_6, param_7, param_8
    );
    let mut param_6 = param_6;
    let mut local_64 = [0u32; 0x19];
    let rec: *mut u8 = local_64.as_mut_ptr().cast::<u8>();
    // SAFETY: the globals and callees are the game's own; the pointer
    // arguments point at the caller's strings, and rec points at a local
    // buffer of 0x64 bytes that all offsets stay within.
    unsafe {
        let table = (&raw mut DAT_10083d8c) as *mut *mut i8;
        let mut idx: i32 = 0;
        if !(mech_trace::dr!("1001a480", table)).is_null() {
            let mut pp = table;
            loop {
                if mech_trace::cr!(
                    "1001a480",
                    "__strnicmp",
                    __strnicmp(param_1, mech_trace::dr!("1001a480", pp), 3)
                ) == 0
                {
                    break;
                }
                pp = pp.add(10);
                idx = idx.wrapping_add(1);
                if (mech_trace::dr!("1001a480", pp)).is_null() {
                    break;
                }
            }
        }
        let entry = table.offset((idx as isize) * 10);
        if (mech_trace::dr!("1001a480", entry)).is_null() {
            return;
        }
        if param_6 < 1 {
            param_6 = 1;
        }
        if 8 < param_6 {
            param_6 = 8;
        }
        if param_5 == 0 {
            param_6 = 0;
        }
        for i in 0..0x19 {
            local_64[i] = 0;
        }
        local_64[0] = mech_trace::gr!("1001a480", "10093c10", DAT_10093c10) as u32;
        local_64[1] = 100;
        mech_trace::cr!(
            "1001a480",
            "_strncpy",
            _strncpy(rec.add(0x2d).cast::<i8>(), param_1, 8)
        );
        mech_trace::dw!("1001a480", rec.add(0x35), 0);
        mech_trace::cr!(
            "1001a480",
            "_strncpy",
            _strncpy(
                rec.add(0x24).cast::<i8>(),
                mech_trace::dr!("1001a480", entry.add(1)),
                8
            )
        );
        mech_trace::dw!("1001a480", rec.add(0x2c), 0);

        let mut s = param_2 as *const i8;
        let mut d = rec.add(0x36).cast::<i8>();
        loop {
            let c = mech_trace::dr!("1001a480", s);
            mech_trace::dw!("1001a480", d, c);
            s = s.add(1);
            d = d.add(1);
            if c == 0 {
                break;
            }
        }
        mech_trace::dw!("1001a480", rec.add(0x4b), 0);
        let mut s = param_3 as *const i8;
        let mut d = rec.add(0x4c).cast::<i8>();
        loop {
            let c = mech_trace::dr!("1001a480", s);
            mech_trace::dw!("1001a480", d, c);
            s = s.add(1);
            d = d.add(1);
            if c == 0 {
                break;
            }
        }
        mech_trace::dw!("1001a480", rec.add(0x61), 0);
        mech_trace::wr!(
            "1001a480",
            rec.add(0x22).cast::<i16>(),
            write_unaligned,
            0x400
        );
        let r = mech_trace::cr!(
            "1001a480",
            "__strnicmp",
            __strnicmp(param_1.add(5), (&raw mut DAT_1008fb70) as *const i8, 3)
        );
        mech_trace::wr!("1001a480", rec.add(0x08).cast::<i16>(), write_unaligned, -2);
        if r == 0 {
            let v = mech_trace::cr!(
                "1001a480",
                "1005e020",
                FUN_1005e020(8, rec.add(0x2d).cast::<i8>())
            );
            mech_trace::wr!(
                "1001a480",
                rec.add(0x08).cast::<i16>(),
                write_unaligned,
                v as i16
            );
        }
        let v = mech_trace::cr!(
            "1001a480",
            "1005e020",
            FUN_1005e020(1, rec.add(0x24).cast::<i8>())
        );
        mech_trace::wr!(
            "1001a480",
            rec.add(0x0a).cast::<i16>(),
            write_unaligned,
            v as i16
        );
        mech_trace::dw!("1001a480", rec.add(0x0d), (param_4 == 0) as u8);
        mech_trace::dw!("1001a480", rec.add(0x0c), param_5 as u8);
        if mech_trace::dr!("1001a480", rec.add(0x0d)) != 0 && param_5 == 0 {
            mech_trace::dw!("1001a480", rec.add(0x0e), 0);
        } else {
            mech_trace::dw!("1001a480", rec.add(0x0e), 2);
        }
        let tbl = (&raw mut DAT_1008f908) as *const i16;
        let base = (param_6 as isize) * 5;
        for k in 0..4usize {
            mech_trace::wr!(
                "1001a480",
                rec.add(0x10 + k * 2).cast::<i16>(),
                write_unaligned,
                mech_trace::dr!("1001a480", tbl.offset(base + k as isize))
            );
        }
        mech_trace::wr!("1001a480", rec.add(0x20).cast::<i16>(), write_unaligned, 6);
        mech_trace::wr!(
            "1001a480",
            rec.add(0x18).cast::<i16>(),
            write_unaligned,
            mech_trace::dr!("1001a480", tbl.offset(base + 4))
        );
        mech_trace::wr!("1001a480", rec.add(0x1a).cast::<i16>(), write_unaligned, 0);
        mech_trace::wr!("1001a480", rec.add(0x1c).cast::<i16>(), write_unaligned, 0);
        mech_trace::wr!(
            "1001a480",
            rec.add(0x1e).cast::<i16>(),
            write_unaligned,
            param_4 as i16
        );

        if param_8 < 0 {
            if -1 < param_7 {
                let p = mech_trace::cr!(
                    "1001a480",
                    "1000adb0",
                    FUN_1000adb0(
                        mech_trace::gr!("1001a480", "100a67a4", DAT_100a67a4),
                        param_7
                    )
                );
                if p != 0 {
                    let src = (p.wrapping_add(0x28)) as *const i16;
                    for k in 0..8usize {
                        mech_trace::wr!(
                            "1001a480",
                            rec.add(0x10 + k * 2).cast::<i16>(),
                            write_unaligned,
                            mech_trace::rd!("1001a480", src.add(k), read_unaligned)
                        );
                    }
                }
            }
        } else {
            let p = mech_trace::cr!(
                "1001a480",
                "10018060",
                FUN_10018060(mech_trace::gr!("1001a480", "100a67a8", DAT_100a67a8), 0)
            );
            if p != 0 {
                let src = (p.wrapping_add(0x2d)) as *const u8;
                for k in 0..8usize {
                    mech_trace::wr!(
                        "1001a480",
                        rec.add(0x10 + k * 2).cast::<i16>(),
                        write_unaligned,
                        mech_trace::rd!("1001a480", src.add(k * 2).cast::<i16>(), read_unaligned)
                    );
                }
            }
        }
        let def = (&raw mut DAT_1008f970) as *const i16;
        let mut off: usize = 0;
        while off < 0x10 {
            let q = rec.add(0x10 + off).cast::<i16>();
            if mech_trace::rd!("1001a480", q, read_unaligned) < 0 {
                mech_trace::wr!(
                    "1001a480",
                    q,
                    write_unaligned,
                    mech_trace::rd!("1001a480", def.byte_add(off), read_unaligned)
                );
            }
            off += 2;
        }
        FUN_10036660(rec, 0x62);
    }
}
