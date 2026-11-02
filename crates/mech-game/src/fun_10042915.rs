// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38fc: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10051f43(param_1: i32) -> i32;
    fn FUN_1004403d(param_1: i32, param_2: u8) -> u8;
    fn FUN_10044a2b(param_1: i32) -> i32;
    fn FUN_10042f22(param_1: i32, param_2: u32, param_3: i16, param_4: i16) -> u8;
    fn FUN_1002a6ac(param_1: i32) -> u8;
}

unsafe fn r32(base: i32, off: i32) -> i32 {
    // SAFETY: the caller passes a game object address; the read is where the original reads.
    unsafe {
        mech_trace::rd!(
            "10042915",
            (base.wrapping_add(off) as *const i32),
            read_unaligned
        )
    }
}

unsafe fn w32(base: i32, off: i32, v: i32) {
    // SAFETY: the caller passes a game object address; the write is where the original writes.
    unsafe {
        mech_trace::wr!(
            "10042915",
            (base.wrapping_add(off) as *mut i32),
            write_unaligned,
            v
        )
    }
}

unsafe fn r16(base: i32, off: i32) -> u16 {
    // SAFETY: the caller passes a game object address; the read is where the original reads.
    unsafe {
        mech_trace::rd!(
            "10042915",
            (base.wrapping_add(off) as *const u16),
            read_unaligned
        )
    }
}

unsafe fn w16(base: i32, off: i32, v: u16) {
    // SAFETY: the caller passes a game object address; the write is where the original writes.
    unsafe {
        mech_trace::wr!(
            "10042915",
            (base.wrapping_add(off) as *mut u16),
            write_unaligned,
            v
        )
    }
}

unsafe fn w8(base: i32, off: i32, v: u8) {
    // SAFETY: the caller passes a game object address; the write is where the original writes.
    unsafe { mech_trace::wr!("10042915", (base.wrapping_add(off) as *mut u8), write, v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10042915(param_1: i32) {
    mech_trace::enter!("10042915", param_1);
    // SAFETY: param_1 is the game's object; globals and callees are the game's own.
    unsafe {
        w16(param_1, 0x17a, r16(param_1, 0x178));
        w32(param_1, 0x1d0, 0);
        w32(param_1, 0x1d8, 0);
        w32(param_1, 0x1d4, r32(param_1, 0x1d8));
        w32(param_1, 0x1ee, r32(param_1, 0x1d4));
        w32(param_1, 0x1ea, 0);
        w32(
            param_1,
            0x1cc,
            (mech_trace::gr!("10042915", "100b38fc", DAT_100b38fc) as i32).wrapping_add(0x235a),
        );
        match r16(param_1, 0x1c4) {
            0 => {
                let i_var1 = mech_trace::cr!("10042915", "10051f43", FUN_10051f43(5));
                w32(
                    param_1,
                    0x1cc,
                    i_var1
                        .wrapping_add(8)
                        .wrapping_mul(0xb5)
                        .wrapping_add(mech_trace::gr!("10042915", "100b38fc", DAT_100b38fc) as i32),
                );
            }
            1 => {
                w32(
                    param_1,
                    0x1cc,
                    (mech_trace::gr!("10042915", "100b38fc", DAT_100b38fc) as i32)
                        .wrapping_add(0xe24),
                );
                w32(param_1, 0x1d8, -1);
            }
            2 | 6 => {}
            3 => {
                let i_var1 = mech_trace::cr!("10042915", "10051f43", FUN_10051f43(2));
                if i_var1 == 0 {
                    w32(param_1, 0x1d4, -1);
                } else {
                    w32(param_1, 0x1d4, 1);
                }
            }
            4 => {
                w32(r32(param_1, 0x4c), 8, 0);
                w32(r32(param_1, 0x4c), 0xc, 0);
                w32(param_1, 0x1ea, 1);
                mech_trace::cr!("10042915", "1004403d", FUN_1004403d(param_1, 1));
                w32(
                    param_1,
                    0x1cc,
                    (mech_trace::gr!("10042915", "100b38fc", DAT_100b38fc) as i32)
                        .wrapping_add(0x389),
                );
            }
            5 => {
                mech_trace::cr!("10042915", "10044a2b", FUN_10044a2b(param_1));
                w32(param_1, 0x1ea, 1);
            }
            7 => {
                if r32(param_1, 0xc0) < 4000 {
                    let i_var1 = mech_trace::cr!("10042915", "10051f43", FUN_10051f43(2));
                    let m = ((i_var1 == 0) as u16).wrapping_neg() & 8;
                    mech_trace::cr!(
                        "10042915",
                        "10042f22",
                        FUN_10042f22(
                            param_1,
                            (r32(param_1, 4) as u32) | 0x200,
                            m.wrapping_add(4) as i16,
                            10000,
                        )
                    );
                } else {
                    let i_var1 = mech_trace::cr!("10042915", "10051f43", FUN_10051f43(2));
                    let m = ((i_var1 == 0) as u16).wrapping_neg() & 0xc;
                    mech_trace::cr!(
                        "10042915",
                        "10042f22",
                        FUN_10042f22(
                            param_1,
                            (r32(param_1, 4) as u32) | 0x200,
                            m.wrapping_add(3) as i16,
                            10000,
                        )
                    );
                }
            }
            8 => {
                let i_var1 = mech_trace::cr!("10042915", "10051f43", FUN_10051f43(0xb));
                w32(
                    param_1,
                    0x1cc,
                    i_var1
                        .wrapping_add(10)
                        .wrapping_mul(0xb5)
                        .wrapping_add(mech_trace::gr!("10042915", "100b38fc", DAT_100b38fc) as i32),
                );
                w8(r32(param_1, 0x4c), 0x30, 1);
            }
            9 => {
                w32(param_1, 0x1d4, 0);
                w32(r32(param_1, 0x4c), 0xc, 0);
                w32(param_1, 0x1ea, 1);
            }
            10 => {
                if r32(r32(param_1, 0x20), 0x3d8) == 0
                    || (r16(param_1, 0x1c6) as i16) != 10
                    || r32(param_1, 0x7c) != -1
                {
                    w8(r32(param_1, 0x4c), 0x30, 1);
                    w32(
                        param_1,
                        0x1cc,
                        (mech_trace::gr!("10042915", "100b38fc", DAT_100b38fc) as i32)
                            .wrapping_add(0x5a8),
                    );
                } else {
                    w32(
                        param_1,
                        0x1cc,
                        (mech_trace::gr!("10042915", "100b38fc", DAT_100b38fc) as i32)
                            .wrapping_add(0x2d4),
                    );
                }
            }
            0xb => {
                w32(r32(param_1, 0x4c), 0xc, 0);
                w32(param_1, 0x1ea, 1);
                if r32(param_1, 0x1d4) == 0 || r32(param_1, 0x1d4) == 2 {
                    w8(r32(param_1, 0x4c), 0x1e, 1);
                } else {
                    w8(r32(param_1, 0x4c), 0x20, 1);
                }
                mech_trace::cr!("10042915", "1004403d", FUN_1004403d(param_1, 1));
                w32(
                    param_1,
                    0x1cc,
                    (mech_trace::gr!("10042915", "100b38fc", DAT_100b38fc) as i32)
                        .wrapping_add(0x16a),
                );
            }
            0xc => {
                w32(
                    param_1,
                    0x1cc,
                    (mech_trace::gr!("10042915", "100b38fc", DAT_100b38fc) as i32)
                        .wrapping_add(0x46b4),
                );
                mech_trace::cr!("10042915", "1002a6ac", FUN_1002a6ac(param_1));
            }
            0xd => {
                w32(
                    param_1,
                    0x1cc,
                    (mech_trace::gr!("10042915", "100b38fc", DAT_100b38fc) as i32)
                        .wrapping_add(0x46b4),
                );
                let i_var1 = mech_trace::cr!("10042915", "10051f43", FUN_10051f43(0x10));
                w32(param_1, 0x1d4, i_var1);
            }
            _ => {}
        }
    }
}
