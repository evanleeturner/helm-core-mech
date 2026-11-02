// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut PTR_DAT_100aaf68: *mut u8;
    static mut DAT_10160eb0: i32;
    static mut DAT_10160eb4: i32;
    static mut DAT_10160eb8: i32;
    static mut DAT_10160ebc: i32;
    static mut DAT_10160ec0: i32;
    static mut DAT_10160ec4: i32;
    static mut _DAT_10160ec8: i32;
    static mut _DAT_10160ecc: i32;
    static mut DAT_10160ed0: i32;
    static mut _DAT_10160ed4: i32;
    static mut _DAT_10160ed8: i32;
    static mut DAT_10160edc: i32;
    static mut DAT_10160ee0: i32;
    static mut _DAT_10160ee4: i32;
    static mut DAT_10160ee8: i32;
    static mut _DAT_10160eec: i32;
    static mut DAT_10160ef0: i32;
    static mut DAT_10160ef4: i32;
    static mut DAT_10160ef8: i32;
    static mut DAT_10160efc: i32;
    static mut DAT_10160f00: i32;
    static mut DAT_10160f04: i32;
    static mut DAT_10160f08: i32;
    static mut DAT_10160f0c: i32;
    static mut DAT_10160f10: i32;
    static mut DAT_10160f14: i32;
    static mut DAT_10160f18: i32;
    static mut DAT_10160f1c: i32;
    static mut DAT_10160f20: i32;
    static mut DAT_10160f24: i32;
    static mut DAT_10160f28: i32;
    static mut DAT_10160f2c: i32;
    static mut DAT_10160f30: i32;
    static mut DAT_10160f34: i32;
    static mut DAT_10160f38: i32;
    static mut DAT_10160f3c: i32;
    static mut DAT_10160f40: i32;
    static mut _DAT_10160f44: i32;
    static mut _DAT_10160f48: i32;
    static mut _DAT_10160f4c: i32;
    static mut _DAT_10160f50: i32;
    static mut DAT_10160f54: i32;
    static mut _DAT_10160f58: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1007d245(param_1: i32);
    fn FUN_10003d60(param_1: i32, param_2: i32) -> i32;
    fn FUN_1007d21a(param_1: i32, param_2: i32, param_3: i32, param_4: i32);
}

#[inline(always)]
unsafe fn rd32(p: *mut u8, off: usize) -> i32 {
    // SAFETY: caller guarantees p points at a structure at least off+4 bytes long.
    unsafe { mech_trace::rd!("1003b2f0", p.byte_add(off).cast::<i32>(), read_unaligned) }
}

#[inline(always)]
unsafe fn rd16(p: *mut u8, off: usize) -> i32 {
    // SAFETY: caller guarantees p points at a structure at least off+2 bytes long.
    unsafe { mech_trace::rd!("1003b2f0", p.byte_add(off).cast::<i16>(), read_unaligned) as i32 }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003b2f0(param_1: *mut u8) {
    mech_trace::enter!("1003b2f0", param_1);
    // SAFETY: globals and callees are the game's own; param_1 points at the
    // caller's structure, read at the same offsets as the original.
    unsafe {
        mech_trace::gw!("1003b2f0", "PTR_DAT_100aaf68", PTR_DAT_100aaf68, param_1);
        FUN_1007d245(rd16(param_1, 0x2a));
        mech_trace::gw!("1003b2f0", "10160f18", DAT_10160f18, rd32(param_1, 0x9c));
        mech_trace::gw!("1003b2f0", "10160f1c", DAT_10160f1c, rd32(param_1, 0xa0));
        mech_trace::gw!(
            "1003b2f0",
            "_DAT_10160ed4",
            _DAT_10160ed4,
            rd32(param_1, 0x94)
        );
        mech_trace::gw!(
            "1003b2f0",
            "_DAT_10160ed8",
            _DAT_10160ed8,
            rd32(param_1, 0x98)
        );
        mech_trace::gw!("1003b2f0", "10160f20", DAT_10160f20, rd32(param_1, 0x54));
        mech_trace::gw!("1003b2f0", "10160f24", DAT_10160f24, rd32(param_1, 0x58));
        mech_trace::gw!("1003b2f0", "10160f28", DAT_10160f28, rd32(param_1, 0x5c));
        mech_trace::gw!("1003b2f0", "10160f2c", DAT_10160f2c, rd32(param_1, 0x60));
        mech_trace::gw!("1003b2f0", "10160f30", DAT_10160f30, rd32(param_1, 100));
        mech_trace::gw!("1003b2f0", "10160f34", DAT_10160f34, rd32(param_1, 0x68));
        mech_trace::gw!("1003b2f0", "10160f0c", DAT_10160f0c, rd32(param_1, 0x6c));
        mech_trace::gw!("1003b2f0", "10160f10", DAT_10160f10, rd32(param_1, 0x70));
        mech_trace::gw!("1003b2f0", "10160f14", DAT_10160f14, rd32(param_1, 0x74));
        mech_trace::gw!(
            "1003b2f0",
            "10160f38",
            DAT_10160f38,
            mech_trace::gr!("1003b2f0", "10160f0c", DAT_10160f0c)
        );
        mech_trace::gw!(
            "1003b2f0",
            "10160f3c",
            DAT_10160f3c,
            mech_trace::gr!("1003b2f0", "10160f10", DAT_10160f10)
        );
        mech_trace::gw!(
            "1003b2f0",
            "10160f40",
            DAT_10160f40,
            mech_trace::gr!("1003b2f0", "10160f14", DAT_10160f14)
        );
        mech_trace::gw!(
            "1003b2f0",
            "10160ef4",
            DAT_10160ef4,
            mech_trace::cr!(
                "1003b2f0",
                "10003d60",
                FUN_10003d60(
                    mech_trace::gr!("1003b2f0", "10160f18", DAT_10160f18),
                    mech_trace::gr!("1003b2f0", "10160f20", DAT_10160f20)
                )
            )
        );
        mech_trace::gw!(
            "1003b2f0",
            "10160ef8",
            DAT_10160ef8,
            mech_trace::cr!(
                "1003b2f0",
                "10003d60",
                FUN_10003d60(
                    mech_trace::gr!("1003b2f0", "10160f18", DAT_10160f18),
                    mech_trace::gr!("1003b2f0", "10160f24", DAT_10160f24)
                )
            )
        );
        mech_trace::gw!(
            "1003b2f0",
            "10160efc",
            DAT_10160efc,
            mech_trace::cr!(
                "1003b2f0",
                "10003d60",
                FUN_10003d60(
                    mech_trace::gr!("1003b2f0", "10160f18", DAT_10160f18),
                    mech_trace::gr!("1003b2f0", "10160f28", DAT_10160f28)
                )
            )
        );
        mech_trace::gw!(
            "1003b2f0",
            "10160f00",
            DAT_10160f00,
            mech_trace::cr!(
                "1003b2f0",
                "10003d60",
                FUN_10003d60(
                    mech_trace::gr!("1003b2f0", "10160f1c", DAT_10160f1c),
                    mech_trace::gr!("1003b2f0", "10160f2c", DAT_10160f2c)
                )
            )
        );
        mech_trace::gw!(
            "1003b2f0",
            "10160f04",
            DAT_10160f04,
            mech_trace::cr!(
                "1003b2f0",
                "10003d60",
                FUN_10003d60(
                    mech_trace::gr!("1003b2f0", "10160f1c", DAT_10160f1c),
                    mech_trace::gr!("1003b2f0", "10160f30", DAT_10160f30)
                )
            )
        );
        mech_trace::gw!(
            "1003b2f0",
            "10160f08",
            DAT_10160f08,
            mech_trace::cr!(
                "1003b2f0",
                "10003d60",
                FUN_10003d60(
                    mech_trace::gr!("1003b2f0", "10160f1c", DAT_10160f1c),
                    mech_trace::gr!("1003b2f0", "10160f34", DAT_10160f34)
                )
            )
        );
        mech_trace::gw!(
            "1003b2f0",
            "_DAT_10160f48",
            _DAT_10160f48,
            rd32(param_1, 0x78)
        );
        mech_trace::gw!(
            "1003b2f0",
            "_DAT_10160f44",
            _DAT_10160f44,
            rd32(param_1, 0x7c)
        );
        mech_trace::gw!(
            "1003b2f0",
            "_DAT_10160f4c",
            _DAT_10160f4c,
            rd32(param_1, 0x80)
        );
        FUN_1007d21a(
            rd32(param_1, 0x1c),
            rd32(param_1, 0x20),
            rd32(param_1, 0x24),
            rd16(param_1, 0x28),
        );
        mech_trace::gw!("1003b2f0", "10160f54", DAT_10160f54, rd32(param_1, 0x3c));
        mech_trace::gw!("1003b2f0", "10160ec0", DAT_10160ec0, rd32(param_1, 0x2c));
        mech_trace::gw!("1003b2f0", "10160ef0", DAT_10160ef0, rd32(param_1, 0x40));
        mech_trace::gw!("1003b2f0", "10160ee0", DAT_10160ee0, rd32(param_1, 0x34));
        mech_trace::gw!("1003b2f0", "10160edc", DAT_10160edc, rd32(param_1, 0x30));
        mech_trace::gw!("1003b2f0", "10160ed0", DAT_10160ed0, rd32(param_1, 0x38));
        mech_trace::gw!(
            "1003b2f0",
            "10160ebc",
            DAT_10160ebc,
            (rd32(param_1, 0x40) as u32).wrapping_shl(2) as i32
        );
        mech_trace::gw!(
            "1003b2f0",
            "10160eb0",
            DAT_10160eb0,
            (rd32(param_1, 0x3c) as u32).wrapping_shl(2) as i32
        );
        mech_trace::gw!(
            "1003b2f0",
            "_DAT_10160f50",
            _DAT_10160f50,
            (rd32(param_1, 0x34) as u32).wrapping_shl(2) as i32
        );
        mech_trace::gw!(
            "1003b2f0",
            "_DAT_10160eec",
            _DAT_10160eec,
            (rd32(param_1, 0x38) as u32).wrapping_shl(2) as i32
        );
        mech_trace::gw!(
            "1003b2f0",
            "_DAT_10160ee4",
            _DAT_10160ee4,
            (rd32(param_1, 0x2c) as u32).wrapping_shl(2) as i32
        );
        mech_trace::gw!(
            "1003b2f0",
            "_DAT_10160f58",
            _DAT_10160f58,
            (rd32(param_1, 0x30) as u32).wrapping_shl(2) as i32
        );
        mech_trace::gw!(
            "1003b2f0",
            "_DAT_10160ecc",
            _DAT_10160ecc,
            rd32(param_1, 0x84)
        );
        mech_trace::gw!(
            "1003b2f0",
            "_DAT_10160ec8",
            _DAT_10160ec8,
            rd32(param_1, 0x88)
        );
        mech_trace::gw!("1003b2f0", "10160ec4", DAT_10160ec4, rd32(param_1, 0x8c));
        mech_trace::gw!("1003b2f0", "10160ee8", DAT_10160ee8, rd32(param_1, 0x90));
        mech_trace::gw!("1003b2f0", "10160eb4", DAT_10160eb4, rd16(param_1, 0xa4));
        mech_trace::gw!("1003b2f0", "10160eb8", DAT_10160eb8, rd16(param_1, 0xa6));
    }
}
