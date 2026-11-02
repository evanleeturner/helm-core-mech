// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10083250: [u8; 0];
    static mut DAT_10083254: [u8; 0];
    static mut DAT_10083258: [u8; 0];
    static mut DAT_10083610: [u8; 0];
    static mut DAT_10083710: u32;
    static mut DAT_10083738: u32;
    static mut DAT_1008373c: u32;
    static mut DAT_10083790: u32;
    static mut DAT_10083798: u32;
    static mut DAT_1008379c: [u8; 0];
    static mut DAT_10083800: [u8; 0];
    static mut DAT_10083878: [u8; 0];
    static mut DAT_10083c70: u16;
    static mut DAT_10083c78: u16;
    static mut DAT_10083c82: u16;
    static mut DAT_10083c8a: u16;
    static mut DAT_10083c8e: u16;
    static mut DAT_10083c94: u16;
    static mut DAT_10083c9c: u16;
    static mut DAT_10083ca0: u16;
    static mut DAT_10083ca6: u16;
    static mut DAT_10083cae: u16;
    static mut DAT_10083cb2: u16;
    static mut DAT_10083cb8: u16;
    static mut DAT_10083cc0: u16;
    static mut DAT_10083cca: u16;
    static mut DAT_10083cd2: u16;
    static mut DAT_10083cdc: u16;
    static mut DAT_10083ce4: u16;
    static mut DAT_10083cee: u16;
    static mut DAT_10083cf6: u16;
    static mut DAT_100ae160: [u8; 0];
    static mut DAT_100b0260: [u32; 0];
    static mut DAT_100b0328: [u8; 0];
    static mut DAT_100b0330: u32;
    static mut DAT_100b0334: u32;
    static mut DAT_100b0338: u32;
    static mut DAT_100b033c: u32;
    static mut DAT_100b0340: u32;
    static mut DAT_100b0344: u32;
    static mut DAT_100b0348: [u8; 0];
    static mut DAT_100b0388: [u8; 0];
    static mut DAT_100b038c: [u8; 0];
    static mut DAT_100b0418: u32;
    static mut DAT_100b0420: u32;
    static mut DAT_100b043c: [u8; 0];
    static mut DAT_100b0440: u32;
    static mut DAT_100b0444: u32;
    static mut DAT_100b0448: u32;
    static mut DAT_100b0468: u32;
    static mut DAT_100b046c: u32;
    static mut DAT_100b0470: u32;
    static mut DAT_100b0490: u32;
    static mut DAT_100b0494: u32;
    static mut DAT_100b0498: u32;
    static mut DAT_100b04b8: u32;
    static mut DAT_100b04c0: u32;
    static mut DAT_100b04e0: u32;
    static mut DAT_100b04e8: u32;
    static mut DAT_100b0508: u32;
    static mut DAT_100b0510: u32;
    static mut DAT_100b0530: u32;
    static mut DAT_100b0538: u32;
    static mut DAT_100b057c: [u8; 0];
    static mut s_mek__s_10083248: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn sprintf(dest: *mut u8, format: *const u8, ...) -> i32;
    fn fopen(filename: *const u8, mode: *const u8) -> *mut u8;
    fn FUN_1007e180(param_1: *const u8) -> u32;
    fn fread(dst: *mut u8, size: u32, count: u32, file: *mut u8) -> u32;
    fn fclose(file: *mut u8) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100069e0(param_1: u32) -> u32 {
    mech_trace::enter!("100069e0", param_1);
    // SAFETY: all globals and callees are the game's own memory and
    // functions; pointers are walked exactly as the original does.
    unsafe {
        let mut p = (&raw mut DAT_100b0330).cast::<u32>();
        for _ in 0..6 {
            mech_trace::wr!("100069e0", p, write_unaligned, 0);
            p = p.add(1);
        }
        let mut p = (&raw mut DAT_100b0418).cast::<u32>();
        for _ in 0..0x50 {
            mech_trace::wr!("100069e0", p, write_unaligned, 0);
            p = p.add(1);
        }
        let mut p = (&raw mut DAT_100b0388).cast::<u32>();
        for _ in 0..0x1c {
            mech_trace::wr!("100069e0", p, write_unaligned, 0);
            p = p.add(1);
        }
        let mut p = (&raw mut DAT_100b0260).cast::<u32>();
        for _ in 0..0x32 {
            mech_trace::wr!("100069e0", p, write_unaligned, 0);
            p = p.add(1);
        }
        let mut p = (&raw mut DAT_100b0348).cast::<u32>();
        for _ in 0..0xe {
            mech_trace::wr!("100069e0", p, write_unaligned, 0);
            p = p.add(1);
        }
        mech_trace::wr!("100069e0", p.cast::<u16>(), write_unaligned, 0);
        mech_trace::gw!("100069e0", "100b0340", DAT_100b0340, 0);

        let mut i4: i32 = 0;
        loop {
            let v = mech_trace::rd!(
                "100069e0",
                (&raw mut DAT_10083800)
                    .cast::<u8>()
                    .byte_add(i4 as usize)
                    .cast::<i32>(),
                read_unaligned
            );
            if v == -1 {
                break;
            }
            mech_trace::wr!(
                "100069e0",
                (&raw mut DAT_100b0388)
                    .cast::<u8>()
                    .byte_add(i4 as usize)
                    .cast::<i32>(),
                write_unaligned,
                v
            );
            let i5 = i4.wrapping_add(8);
            mech_trace::gw!(
                "100069e0",
                "100b0340",
                DAT_100b0340,
                mech_trace::gr!("100069e0", "100b0340", DAT_100b0340).wrapping_add(1)
            );
            mech_trace::wr!(
                "100069e0",
                (&raw mut DAT_100b038c)
                    .cast::<u8>()
                    .byte_add(i4 as usize)
                    .cast::<u32>(),
                write_unaligned,
                0xffffffff
            );
            i4 = i5;
            if i5 >= 0x70 {
                break;
            }
        }

        mech_trace::gw!("100069e0", "100b0344", DAT_100b0344, 0);
        let mut p9 = (&raw mut DAT_1008379c).cast::<i32>();
        let mut p13 = (&raw mut DAT_100b0260).cast::<i32>();
        loop {
            if mech_trace::rd!("100069e0", p9, read_unaligned) == -1 {
                break;
            }
            let i5 = mech_trace::rd!("100069e0", p9, read_unaligned).wrapping_sub(10000);
            let mut p2 = (&raw mut DAT_100b0260).cast::<i32>();
            let mut i4 = (i5 / 10000).wrapping_mul(100).wrapping_add(0x2711);
            loop {
                let t = mech_trace::rd!("100069e0", p2, read_unaligned);
                if t == 0 {
                    break;
                }
                if t == i4 {
                    i4 = i4.wrapping_add(1);
                }
                p2 = p2.add(2);
                if (p2 as usize) >= ((&raw mut DAT_100b0328) as usize) {
                    break;
                }
            }
            mech_trace::wr!("100069e0", p13, write_unaligned, i4);
            p9 = p9.add(1);
            mech_trace::gw!(
                "100069e0",
                "100b0344",
                DAT_100b0344,
                mech_trace::gr!("100069e0", "100b0344", DAT_100b0344).wrapping_add(1)
            );
            mech_trace::wr!("100069e0", p13.add(1), write_unaligned, i5 / 100);
            p13 = p13.add(2);
            if (p9 as usize) >= ((&raw mut DAT_10083800) as usize) {
                break;
            }
        }

        let mut p13 = (&raw mut DAT_10083878).cast::<i32>();
        let mut p8 = (&raw mut DAT_100b043c).cast::<u16>();
        loop {
            let mut p10 = p8.wrapping_sub(0xc);
            let mut i4: i32 = 0xc;
            mech_trace::wr!("100069e0", p8.add(1), write_unaligned, 0x100);
            mech_trace::wr!("100069e0", p8, write_unaligned, 0xc);
            loop {
                let i5 = mech_trace::rd!("100069e0", p13, read_unaligned);
                if i5 == -1 {
                    mech_trace::wr!("100069e0", p8, write_unaligned, 6);
                    mech_trace::wr!("100069e0", p10, write_unaligned, 0);
                } else if i5 < 10000 {
                    mech_trace::wr!("100069e0", p10, write_unaligned, i5 as i16 as u16);
                } else {
                    let mut i3: usize = 0;
                    let mut p9 = (&raw mut DAT_1008379c).cast::<i32>();
                    loop {
                        if mech_trace::rd!("100069e0", p9, read_unaligned) == i5 {
                            let v = mech_trace::rd!(
                                "100069e0",
                                (&raw mut DAT_100b0260).cast::<u32>().add(i3 * 2),
                                read_unaligned
                            );
                            mech_trace::wr!("100069e0", p10, write_unaligned, v as u16);
                            break;
                        }
                        p9 = p9.add(1);
                        i3 += 1;
                        if (p9 as usize) >= ((&raw mut DAT_10083800) as usize) {
                            break;
                        }
                    }
                }
                p10 = p10.wrapping_add(1);
                p13 = p13.add(1);
                i4 = i4.wrapping_sub(1);
                if i4 == 0 {
                    break;
                }
            }
            p8 = p8.wrapping_add(0x14);
            if (p8 as usize) >= ((&raw mut DAT_100b057c) as usize) {
                break;
            }
        }

        mech_trace::gw!(
            "100069e0",
            "100b0420",
            DAT_100b0420,
            mech_trace::gr!("100069e0", "10083c70", DAT_10083c70) as u32
        );
        mech_trace::gw!(
            "100069e0",
            "100b0448",
            DAT_100b0448,
            mech_trace::gr!("100069e0", "10083c82", DAT_10083c82) as i16 as i32 as u32
        );
        mech_trace::gw!(
            "100069e0",
            "100b0470",
            DAT_100b0470,
            mech_trace::gr!("100069e0", "10083c94", DAT_10083c94) as u32
        );
        mech_trace::gw!(
            "100069e0",
            "100b0498",
            DAT_100b0498,
            mech_trace::gr!("100069e0", "10083ca6", DAT_10083ca6) as u32
        );
        mech_trace::gw!(
            "100069e0",
            "100b04c0",
            DAT_100b04c0,
            mech_trace::gr!("100069e0", "10083cb8", DAT_10083cb8) as i16 as i32 as u32
        );
        mech_trace::gw!(
            "100069e0",
            "100b04e8",
            DAT_100b04e8,
            mech_trace::gr!("100069e0", "10083cca", DAT_10083cca) as u32
        );
        mech_trace::gw!(
            "100069e0",
            "100b0510",
            DAT_100b0510,
            mech_trace::gr!("100069e0", "10083cdc", DAT_10083cdc) as i16 as i32 as u32
        );
        mech_trace::gw!(
            "100069e0",
            "100b0538",
            DAT_100b0538,
            mech_trace::gr!("100069e0", "10083cee", DAT_10083cee) as u32
        );
        mech_trace::gw!(
            "100069e0",
            "100b0418",
            DAT_100b0418,
            mech_trace::gr!("100069e0", "10083c78", DAT_10083c78) as i16 as i32 as u32
        );
        mech_trace::gw!(
            "100069e0",
            "100b0440",
            DAT_100b0440,
            mech_trace::gr!("100069e0", "10083c8a", DAT_10083c8a) as i16 as i32 as u32
        );
        mech_trace::gw!(
            "100069e0",
            "100b0468",
            DAT_100b0468,
            mech_trace::gr!("100069e0", "10083c9c", DAT_10083c9c) as u32
        );
        mech_trace::gw!(
            "100069e0",
            "100b0490",
            DAT_100b0490,
            mech_trace::gr!("100069e0", "10083cae", DAT_10083cae) as u32
        );
        mech_trace::gw!(
            "100069e0",
            "100b04b8",
            DAT_100b04b8,
            mech_trace::gr!("100069e0", "10083cc0", DAT_10083cc0) as u32
        );
        mech_trace::gw!(
            "100069e0",
            "100b04e0",
            DAT_100b04e0,
            mech_trace::gr!("100069e0", "10083cd2", DAT_10083cd2) as u32
        );
        mech_trace::gw!(
            "100069e0",
            "100b0508",
            DAT_100b0508,
            mech_trace::gr!("100069e0", "10083ce4", DAT_10083ce4) as u32
        );
        mech_trace::gw!(
            "100069e0",
            "100b0530",
            DAT_100b0530,
            mech_trace::gr!("100069e0", "10083cf6", DAT_10083cf6) as u32
        );
        mech_trace::gw!(
            "100069e0",
            "100b0444",
            DAT_100b0444,
            mech_trace::gr!("100069e0", "10083c8e", DAT_10083c8e) as i16 as i32 as u32
        );
        mech_trace::gw!(
            "100069e0",
            "100b046c",
            DAT_100b046c,
            mech_trace::gr!("100069e0", "10083ca0", DAT_10083ca0) as u32
        );
        mech_trace::gw!(
            "100069e0",
            "100b0494",
            DAT_100b0494,
            mech_trace::gr!("100069e0", "10083cb2", DAT_10083cb2) as u32
        );
        mech_trace::gw!(
            "100069e0",
            "100b0330",
            DAT_100b0330,
            ((mech_trace::gr!("100069e0", "10083710", DAT_10083710) as i32).wrapping_add(0x32)
                / 100) as u32
        );
        mech_trace::gw!(
            "100069e0",
            "100b0334",
            DAT_100b0334,
            mech_trace::gr!("100069e0", "10083790", DAT_10083790)
        );
        mech_trace::gw!(
            "100069e0",
            "100b0338",
            DAT_100b0338,
            mech_trace::gr!("100069e0", "10083798", DAT_10083798)
        );
        mech_trace::gw!(
            "100069e0",
            "100b033c",
            DAT_100b033c,
            (((mech_trace::gr!("100069e0", "1008373c", DAT_1008373c) as i32).wrapping_add(0x32)
                / 100)
                .wrapping_add(10) as u32)
                .wrapping_mul(mech_trace::gr!("100069e0", "10083738", DAT_10083738))
        );

        let mut src = (&raw mut DAT_10083610).cast::<u8>();
        let mut len: u32 = 0;
        loop {
            len = len.wrapping_add(1);
            let c = mech_trace::rd!("100069e0", src, read);
            src = src.add(1);
            if c == 0 {
                break;
            }
        }
        let mut src = src.wrapping_sub(len as usize);
        let mut dst = (&raw mut DAT_100b0348).cast::<u8>();
        let mut n = len >> 2;
        while n != 0 {
            mech_trace::wr!(
                "100069e0",
                dst.cast::<u32>(),
                write_unaligned,
                mech_trace::rd!("100069e0", src.cast::<u32>(), read_unaligned)
            );
            src = src.add(4);
            dst = dst.add(4);
            n -= 1;
        }
        let mut n = len & 3;
        while n != 0 {
            mech_trace::wr!(
                "100069e0",
                dst,
                write,
                mech_trace::rd!("100069e0", src, read)
            );
            src = src.add(1);
            dst = dst.add(1);
            n -= 1;
        }

        mech_trace::cr!(
            "100069e0",
            "sprintf",
            sprintf(
                (&raw mut DAT_100ae160).cast::<u8>(),
                (&raw const s_mek__s_10083248).cast::<u8>(),
                param_1,
            )
        );
        let mut file = mech_trace::cr!(
            "100069e0",
            "fopen",
            fopen(
                (&raw const DAT_100ae160).cast::<u8>(),
                (&raw const DAT_10083250).cast::<u8>(),
            )
        );
        if file.is_null() {
            mech_trace::cr!(
                "100069e0",
                "1007e180",
                FUN_1007e180((&raw const DAT_10083254).cast::<u8>())
            );
            file = mech_trace::cr!(
                "100069e0",
                "fopen",
                fopen(
                    (&raw const DAT_100ae160).cast::<u8>(),
                    (&raw const DAT_10083258).cast::<u8>(),
                )
            );
            if file.is_null() {
                return mech_trace::rt!("100069e0", 0);
            }
        }
        mech_trace::cr!(
            "100069e0",
            "fread",
            fread((&raw mut DAT_100b0330).cast::<u8>(), 0x18, 1, file)
        );
        mech_trace::cr!(
            "100069e0",
            "fread",
            fread((&raw mut DAT_100b0418).cast::<u8>(), 0x28, 8, file)
        );
        mech_trace::cr!(
            "100069e0",
            "fread",
            fread(
                (&raw mut DAT_100b0388).cast::<u8>(),
                8,
                mech_trace::gr!("100069e0", "100b0340", DAT_100b0340),
                file
            )
        );
        mech_trace::cr!(
            "100069e0",
            "fread",
            fread(
                (&raw mut DAT_100b0260).cast::<u8>(),
                8,
                mech_trace::gr!("100069e0", "100b0344", DAT_100b0344),
                file
            )
        );
        mech_trace::cr!(
            "100069e0",
            "fread",
            fread((&raw mut DAT_100b0348).cast::<u8>(), 0x3a, 1, file)
        );
        mech_trace::cr!("100069e0", "fclose", fclose(file));
        mech_trace::rt!("100069e0", 1)
    }
}
