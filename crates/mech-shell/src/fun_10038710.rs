// SPDX-License-Identifier: MIT

use core::ptr::null_mut;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10094484: u8;
    static mut DAT_10094488: i32;
    static mut s_The_campaign_chunk___s_was_not_o_1009448c: u8;
    static mut DAT_100a6738: *mut u8;
    static mut DAT_100a679c: *mut i32;
    static mut DAT_100a67a0: *mut i32;
    static mut DAT_100a67cc: *mut i32;
    static mut DAT_100a67d0: *mut i32;
    static mut DAT_100a67e4: *mut i32;
    static mut DAT_100a67e8: *mut i32;
    static mut DAT_100a67ac: *mut i32;
    static mut DAT_100a67b0: *mut i32;
    static mut DAT_10083508: *mut u8;
    static mut DAT_10098224: *mut u8;
    static mut DAT_100944ec: u32;
    static mut DAT_100944f0: u8;
    static mut DAT_100944e4: u32;
    static mut DAT_100944e8: u8;
    static mut DAT_100944d0: i32;
    static mut DAT_100944cc: i32;
    static mut s_ran_dscn1_100944d4: u8;
    static mut DAT_100944e0: u8;
    static mut DAT_100944f4: u8;
    static mut DAT_100944f8: u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10037d40();
    fn HeapAlloc(h: *mut u8, flags: u32, size: u32) -> *mut u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1005bbb0(param_1: *mut i8) -> *mut i8;
    fn _strstr(s: *mut i8, sub: *mut i8) -> *mut i8;
    fn _atoi(s: *mut i8) -> i32;
    fn _sprintf(dst: *mut i8, fmt: *const i8, ...) -> i32;
    fn FUN_1005b1b0(param_1: *mut i8, param_2: u32);
    fn FUN_10017c90(
        param_1: *mut u32,
        param_2: i32,
        param_3: u32,
        param_4: u8,
        param_5: u32,
    ) -> u32;
    fn FUN_1000ae80(param_1: *mut u8) -> *mut u8;
    fn FUN_10017df0(param_1: *mut i32, param_2: u32) -> u32;
    fn FUN_1000d230(param_1: i32) -> i32;
    fn FUN_1000d260(param_1: i32, param_2: i32) -> i32;
    fn FUN_10050390(param_1: *mut i32);
    fn FUN_100503d0(param_1: *mut i32);
    fn FUN_1000be30(param_1: *mut u8) -> u32;
    fn FUN_10038160(param_1: i32) -> *mut i8;
    fn FUN_10037fc0(param_1: i32) -> *mut i8;
    fn _strncmp(a: *mut i8, b: *mut i8, n: u32) -> i32;
    fn FUN_100385c0(param_1: *mut i8) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "thiscall" {
    fn FUN_100364c0(this: *mut u8, param_1: *mut i8, param_2: u32, param_3: *mut i32) -> i32;
}

unsafe fn rd(p: *const u8, off: usize) -> i32 {
    // SAFETY: the caller supplies a pointer valid for a 4-byte read at off.
    unsafe { mech_trace::rd!("10038710", p.byte_add(off).cast::<i32>(), read_unaligned) }
}

unsafe fn wr(p: *mut u8, off: usize, v: i32) {
    // SAFETY: the caller supplies a pointer valid for a 4-byte write at off.
    unsafe {
        mech_trace::wr!(
            "10038710",
            p.byte_add(off).cast::<i32>(),
            write_unaligned,
            v
        )
    }
}

unsafe fn wr8(p: *mut u8, off: usize, v: u8) {
    // SAFETY: the caller supplies a pointer valid for a byte write at off.
    unsafe { mech_trace::wr!("10038710", p.byte_add(off), write, v) }
}

unsafe fn strlen(p: *const u8) -> usize {
    let mut n = 0usize;
    // SAFETY: p points at a NUL-terminated string.
    unsafe {
        while mech_trace::rd!("10038710", p.byte_add(n), read) != 0 {
            n += 1;
        }
    }
    n
}

unsafe fn copy_str(dst: *mut u8, src: *const u8) {
    let n = unsafe {
        // SAFETY: src is NUL-terminated.
        strlen(src)
    } + 1;
    let mut i = 0usize;
    // SAFETY: dst has room for the string and its terminator.
    unsafe {
        while i < n {
            mech_trace::wr!(
                "10038710",
                dst.byte_add(i),
                write,
                mech_trace::rd!("10038710", src.byte_add(i), read)
            );
            i += 1;
        }
    }
}

unsafe fn copy_bytes(dst: *mut u8, src: *const u8, n: u32) {
    let mut i = 0usize;
    // SAFETY: both pointers are valid for n bytes, copied forward.
    unsafe {
        while i < n as usize {
            mech_trace::wr!(
                "10038710",
                dst.byte_add(i),
                write,
                mech_trace::rd!("10038710", src.byte_add(i), read)
            );
            i += 1;
        }
    }
}

unsafe fn fill_mer(rec: *mut u8, tail: *const u8) -> usize {
    // SAFETY: rec is a record of at least 0x84c bytes from the game.
    unsafe {
        wr8(rec, 0x828, b'M');
        wr8(rec, 0x829, b'E');
        wr8(rec, 0x82a, b'R');
        let mut q = 0x823usize;
        while mech_trace::rd!("10038710", rec.byte_add(q), read) != 0 {
            q += 1;
        }
        mech_trace::wr!(
            "10038710",
            rec.byte_add(q).cast::<u32>(),
            write_unaligned,
            mech_trace::rd!("10038710", tail.cast::<u32>(), read_unaligned)
        );
        mech_trace::wr!(
            "10038710",
            rec.byte_add(q + 4),
            write,
            mech_trace::rd!("10038710", tail.byte_add(4), read)
        );
        q
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10038710(param_1: *mut i8) -> u32 {
    mech_trace::enter!("10038710", param_1);
    // SAFETY: the callees and globals are the game's own; param_1 and the
    // chunk data are read and written where the original does.
    unsafe {
        let mut local_338: *mut u8 = mech_trace::cr!("10038710", "ind", null_mut());
        let mut local_32c: i32 = 0;
        let mut local_334: i32 = 0;
        let mut local_324: i32 = 0;
        let mut local_328: i32 = 0;
        let mut flag = false;
        let mut local_320 = [0u8; 32];
        let mut local_300 = [0u8; 256];
        let mut local_200 = [0u8; 256];
        let mut local_100 = [0u8; 256];

        FUN_10037d40();
        let s = mech_trace::cr!("10038710", "1005bbb0", FUN_1005bbb0(param_1));
        let pcvar4 = mech_trace::cr!(
            "10038710",
            "_strstr",
            _strstr(s, &raw mut DAT_10094484 as *mut i8)
        );
        if !pcvar4.is_null() {
            local_320[1] = 0;
            flag = true;
            local_320[0] = mech_trace::rd!("10038710", param_1.byte_add(3), read) as u8;
            local_328 = mech_trace::cr!(
                "10038710",
                "_atoi",
                _atoi(local_320.as_mut_ptr() as *mut i8)
            );
        }
        let i5 = mech_trace::cr!(
            "10038710",
            "100364c0",
            FUN_100364c0(
                mech_trace::gr!("10038710", "100a6738", DAT_100a6738),
                param_1,
                1,
                &raw mut DAT_10094488
            )
        );
        if i5 == 0 {
            mech_trace::cr!(
                "10038710",
                "_sprintf",
                _sprintf(
                    local_100.as_mut_ptr() as *mut i8,
                    &raw const s_The_campaign_chunk___s_was_not_o_1009448c as *const i8,
                    param_1,
                )
            );
            FUN_1005b1b0(local_100.as_mut_ptr() as *mut i8, 0);
        } else {
            local_324 = rd(i5 as usize as *const u8, 4);
            local_338 = (i5 as usize as *mut u8).byte_add(0xc);
            local_334 = 0xc;
            local_32c = i5;
        }
        if local_338.is_null() {
            return mech_trace::rt!("10038710", 0);
        }
        mech_trace::cr!(
            "10038710",
            "10017c90",
            FUN_10017c90(&raw mut DAT_100a679c as *mut u32, 10, 0x1000ae30, 4, 0)
        );
        mech_trace::cr!(
            "10038710",
            "10017c90",
            FUN_10017c90(&raw mut DAT_100a67a0 as *mut u32, 10, 0x1000ae30, 4, 0)
        );
        mech_trace::cr!(
            "10038710",
            "10017c90",
            FUN_10017c90(&raw mut DAT_100a67cc as *mut u32, 10, 0, 1, 0x1000bde0)
        );
        mech_trace::cr!(
            "10038710",
            "10017c90",
            FUN_10017c90(&raw mut DAT_100a67d0 as *mut u32, 10, 0, 1, 0x1000bde0)
        );
        mech_trace::cr!(
            "10038710",
            "10017c90",
            FUN_10017c90(&raw mut DAT_100a67e4 as *mut u32, 10, 0, 1, 0)
        );
        mech_trace::cr!(
            "10038710",
            "10017c90",
            FUN_10017c90(&raw mut DAT_100a67e8 as *mut u32, 10, 0, 1, 0)
        );
        wr8(
            mech_trace::gr!("10038710", "10083508", DAT_10083508),
            0x239d,
            0,
        );

        let mut r_last: i32 = 0;
        loop {
            let p = local_338;
            let tag = rd(p, 0);
            let mut ngpc = false;
            match tag {
                0x4c4e5753 => {
                    let mut cnt = (rd(p, 4) as u32).wrapping_sub(8) >> 4;
                    if cnt != 0 {
                        let mut rec = p.byte_add(8);
                        loop {
                            let c =
                                mech_trace::cr!("10038710", "1000d230", FUN_1000d230(rd(rec, 0)));
                            let i9 = mech_trace::cr!(
                                "10038710",
                                "1000d260",
                                FUN_1000d260(c, rd(rec, 0))
                            );
                            if c == 0 || c == 2 {
                                let p7 = mech_trace::cr!(
                                    "10038710",
                                    "HeapAlloc",
                                    HeapAlloc(
                                        mech_trace::gr!("10038710", "10098224", DAT_10098224),
                                        1,
                                        0x1a
                                    )
                                );
                                if c == 2 {
                                    wr(p7, 0, i9);
                                } else {
                                    wr(p7, 0, rd(rec, 0));
                                }
                                wr8(p7, 8, c as u8);
                                wr(p7, 4, rd(rec, 8));
                                wr(p7, 0xe, rd(rec, 0xc));
                                wr(p7, 0x12, rd(rec, 4));
                                wr(p7, 0xa, -1);
                                FUN_10050390(p7 as *mut i32);
                                mech_trace::cr!(
                                    "10038710",
                                    "10017df0",
                                    FUN_10017df0(
                                        mech_trace::gr!("10038710", "100a67ac", DAT_100a67ac),
                                        p7 as u32
                                    )
                                );
                            } else if c == 1 {
                                let p7 = mech_trace::cr!(
                                    "10038710",
                                    "HeapAlloc",
                                    HeapAlloc(
                                        mech_trace::gr!("10038710", "10098224", DAT_10098224),
                                        1,
                                        0x1a
                                    )
                                );
                                wr(p7, 0, i9);
                                wr8(p7, 8, 1);
                                wr(p7, 4, rd(rec, 8));
                                wr(p7, 0x16, rd(rec, 4));
                                wr(p7, 0x12, rd(rec, 0xc));
                                wr(p7, 0xe, 1);
                                wr(p7, 0xa, -1);
                                FUN_100503d0(p7 as *mut i32);
                                mech_trace::cr!(
                                    "10038710",
                                    "10017df0",
                                    FUN_10017df0(
                                        mech_trace::gr!("10038710", "100a67ac", DAT_100a67ac),
                                        p7 as u32
                                    )
                                );
                            }
                            rec = rec.byte_add(0x10);
                            cnt = cnt.wrapping_sub(1);
                            if cnt == 0 {
                                break;
                            }
                        }
                    }
                }
                0x4c484d53 => {
                    let mut cnt = (rd(p, 4) as u32).wrapping_sub(8) / 0x1c;
                    if cnt != 0 {
                        let mut off = 0usize;
                        loop {
                            let rec = p.byte_add(8 + off);
                            let pc = mech_trace::cr!("10038710", "1000ae80", FUN_1000ae80(rec));
                            if !pc.is_null() {
                                fill_mer(pc, &raw const DAT_100944ec as *const u8);
                                wr(pc, 0x818, rd(rec, 0x14));
                                wr(pc, 0x843, rd(rec, 0x18));
                                wr8(pc, 0x84b, 0);
                                wr(pc, 0x847, rd(rec, 0x10));
                                mech_trace::cr!(
                                    "10038710",
                                    "10017df0",
                                    FUN_10017df0(
                                        mech_trace::gr!("10038710", "100a67b0", DAT_100a67b0),
                                        pc as u32
                                    )
                                );
                            }
                            off += 0x1c;
                            cnt = cnt.wrapping_sub(1);
                            if cnt == 0 {
                                break;
                            }
                        }
                    }
                }
                0x4e475043 => {
                    ngpc = true;
                    wr(
                        mech_trace::gr!("10038710", "10083508", DAT_10083508),
                        0x122,
                        rd(p, 0xc),
                    );
                    wr(
                        mech_trace::gr!("10038710", "10083508", DAT_10083508),
                        0x126,
                        rd(p, 0x10),
                    );
                    wr(
                        mech_trace::gr!("10038710", "10083508", DAT_10083508),
                        0x1e6,
                        rd(p, 0x64),
                    );
                    wr(
                        mech_trace::gr!("10038710", "10083508", DAT_10083508),
                        0x1ea,
                        rd(p, 0x68),
                    );
                    wr(
                        mech_trace::gr!("10038710", "10083508", DAT_10083508),
                        0x1ee,
                        rd(p, 0x84),
                    );
                    wr(
                        mech_trace::gr!("10038710", "10083508", DAT_10083508),
                        0x1f2,
                        rd(p, 0x88),
                    );
                    wr(
                        mech_trace::gr!("10038710", "10083508", DAT_10083508),
                        0x1f6,
                        rd(p, 0x8c),
                    );
                    wr(
                        mech_trace::gr!("10038710", "10083508", DAT_10083508),
                        0x1fa,
                        rd(p, 0x70),
                    );
                    wr(
                        mech_trace::gr!("10038710", "10083508", DAT_10083508),
                        0x1fe,
                        rd(p, 8),
                    );
                    if flag {
                        mech_trace::cr!(
                            "10038710",
                            "_sprintf",
                            _sprintf(
                                mech_trace::gr!("10038710", "10083508", DAT_10083508)
                                    .byte_add(0x166) as *mut i8,
                                &raw const s_ran_dscn1_100944d4 as *const i8,
                                local_328,
                            )
                        );
                    } else {
                        copy_str(
                            mech_trace::gr!("10038710", "10083508", DAT_10083508).byte_add(0x166),
                            p.byte_add(0x14),
                        );
                    }
                    copy_str(
                        mech_trace::gr!("10038710", "10083508", DAT_10083508).byte_add(0x176),
                        p.byte_add(0x24),
                    );
                    copy_str(
                        mech_trace::gr!("10038710", "10083508", DAT_10083508).byte_add(0x186),
                        p.byte_add(0x34),
                    );
                    copy_str(
                        mech_trace::gr!("10038710", "10083508", DAT_10083508).byte_add(0x196),
                        p.byte_add(0x44),
                    );
                    copy_str(
                        mech_trace::gr!("10038710", "10083508", DAT_10083508).byte_add(0x1b6),
                        p.byte_add(0x54),
                    );
                    copy_str(
                        mech_trace::gr!("10038710", "10083508", DAT_10083508).byte_add(0x1c6),
                        p.byte_add(0x90),
                    );
                    copy_str(
                        mech_trace::gr!("10038710", "10083508", DAT_10083508).byte_add(0x1d6),
                        p.byte_add(0x74),
                    );
                    copy_str(
                        mech_trace::gr!("10038710", "10083508", DAT_10083508).byte_add(0x1b6),
                        &raw const DAT_100944e0,
                    );
                    local_334 = local_334.wrapping_add(0xa0);
                }
                0x4f4d4d41 => {
                    let mut cnt = (rd(p, 4) as u32).wrapping_sub(8) >> 4;
                    if cnt != 0 {
                        let mut rec = p.byte_add(8);
                        loop {
                            let mut i = 0i32;
                            if 0 < rd(rec, 4) {
                                loop {
                                    i = i.wrapping_add(1);
                                    let p7 = mech_trace::cr!(
                                        "10038710",
                                        "HeapAlloc",
                                        HeapAlloc(
                                            mech_trace::gr!("10038710", "10098224", DAT_10098224),
                                            1,
                                            0x1a
                                        )
                                    );
                                    wr(p7, 0, rd(rec, 0));
                                    wr(p7, 0xe, 1);
                                    wr(p7, 4, rd(rec, 8));
                                    FUN_100503d0(p7 as *mut i32);
                                    wr(p7, 0x12, rd(rec, 0xc));
                                    wr(p7, 0x16, 100);
                                    wr8(p7, 8, 1);
                                    mech_trace::cr!(
                                        "10038710",
                                        "10017df0",
                                        FUN_10017df0(
                                            mech_trace::gr!("10038710", "100a67d0", DAT_100a67d0),
                                            p7 as u32
                                        )
                                    );
                                    if !(i < rd(rec, 4)) {
                                        break;
                                    }
                                }
                            }
                            rec = rec.byte_add(0x10);
                            cnt = cnt.wrapping_sub(1);
                            if cnt == 0 {
                                break;
                            }
                        }
                    }
                }
                0x5357454e => {
                    let n = rd(p, 4) as u32;
                    copy_bytes(
                        mech_trace::gr!("10038710", "10083508", DAT_10083508).byte_add(0x342),
                        p.byte_add(8),
                        n.wrapping_sub(1),
                    );
                    wr8(
                        mech_trace::gr!("10038710", "10083508", DAT_10083508)
                            .byte_add(rd(p, 4) as usize),
                        0x342,
                        0,
                    );
                }
                0x50555145 => {
                    let mut cnt = (rd(p, 4) as u32).wrapping_sub(8) / 0x14;
                    if cnt != 0 {
                        let mut rec = p.byte_add(8);
                        loop {
                            let p7 = mech_trace::cr!(
                                "10038710",
                                "HeapAlloc",
                                HeapAlloc(
                                    mech_trace::gr!("10038710", "10098224", DAT_10098224),
                                    1,
                                    0x16
                                )
                            );
                            wr8(p7, 8, 2);
                            wr(p7, 4, rd(rec, 4));
                            wr(p7, 0, rd(rec, 0));
                            wr(p7, 0xa, rd(rec, 8));
                            wr(p7, 0xe, rd(rec, 0xc));
                            wr(p7, 0x12, rd(rec, 0x10));
                            mech_trace::cr!(
                                "10038710",
                                "10017df0",
                                FUN_10017df0(
                                    mech_trace::gr!("10038710", "100a67cc", DAT_100a67cc),
                                    p7 as u32
                                )
                            );
                            cnt = cnt.wrapping_sub(1);
                            rec = rec.byte_add(0x14);
                            if cnt == 0 {
                                break;
                            }
                        }
                    }
                }
                0x544c4854 => {
                    let mut cnt = (rd(p, 4) as u32).wrapping_sub(8) >> 4;
                    if cnt != 0 {
                        let mut rec = p.byte_add(8);
                        loop {
                            copy_str(local_300.as_mut_ptr(), rec);
                            let r = mech_trace::cr!(
                                "10038710",
                                "100364c0",
                                FUN_100364c0(
                                    mech_trace::gr!("10038710", "100a6738", DAT_100a6738),
                                    local_300.as_mut_ptr() as *mut i8,
                                    1,
                                    &raw mut DAT_100944d0,
                                )
                            );
                            if r != 0 {
                                let pc = mech_trace::cr!("10038710", "10038160", FUN_10038160(r));
                                if !pc.is_null() {
                                    mech_trace::cr!(
                                        "10038710",
                                        "10017df0",
                                        FUN_10017df0(
                                            mech_trace::gr!("10038710", "100a67e8", DAT_100a67e8),
                                            pc as u32
                                        )
                                    );
                                }
                            }
                            rec = rec.byte_add(0x10);
                            cnt = cnt.wrapping_sub(1);
                            if cnt == 0 {
                                break;
                            }
                        }
                    }
                }
                0x544c484d => {
                    let mut cnt = (rd(p, 4) as u32).wrapping_sub(8) / 0x18;
                    if cnt != 0 {
                        let mut rec = p.byte_add(8);
                        loop {
                            let pc = mech_trace::cr!("10038710", "1000ae80", FUN_1000ae80(rec));
                            if !pc.is_null() {
                                fill_mer(pc, &raw const DAT_100944e4 as *const u8);
                                wr(pc, 0x818, rd(rec, 0x10));
                                wr(pc, 0x843, rd(rec, 0x14));
                                wr8(pc, 0x84b, 0);
                                mech_trace::cr!(
                                    "10038710",
                                    "10017df0",
                                    FUN_10017df0(
                                        mech_trace::gr!("10038710", "100a679c", DAT_100a679c),
                                        pc as u32
                                    )
                                );
                                let u = mech_trace::cr!("10038710", "1000be30", FUN_1000be30(pc));
                                mech_trace::cr!(
                                    "10038710",
                                    "10017df0",
                                    FUN_10017df0(
                                        mech_trace::gr!("10038710", "100a67a0", DAT_100a67a0),
                                        u
                                    )
                                );
                            }
                            rec = rec.byte_add(0x18);
                            cnt = cnt.wrapping_sub(1);
                            if cnt == 0 {
                                break;
                            }
                        }
                    }
                }
                0x544c5450 => {
                    let mut cnt = (rd(p, 4) as u32).wrapping_sub(8) >> 4;
                    if cnt != 0 {
                        let mut rec = p.byte_add(8);
                        loop {
                            copy_str(local_300.as_mut_ptr(), rec);
                            let r = mech_trace::cr!(
                                "10038710",
                                "100364c0",
                                FUN_100364c0(
                                    mech_trace::gr!("10038710", "100a6738", DAT_100a6738),
                                    local_300.as_mut_ptr() as *mut i8,
                                    1,
                                    &raw mut DAT_100944cc,
                                )
                            );
                            if r != 0 {
                                let pc = mech_trace::cr!("10038710", "10037fc0", FUN_10037fc0(r));
                                if !pc.is_null() {
                                    mech_trace::cr!(
                                        "10038710",
                                        "10017df0",
                                        FUN_10017df0(
                                            mech_trace::gr!("10038710", "100a67e4", DAT_100a67e4),
                                            pc as u32
                                        )
                                    );
                                }
                            }
                            rec = rec.byte_add(0x10);
                            cnt = cnt.wrapping_sub(1);
                            if cnt == 0 {
                                break;
                            }
                        }
                    }
                }
                0x544c4e57 => {
                    let mut cnt = (rd(p, 4) as u32).wrapping_sub(8) / 0xc;
                    if cnt != 0 {
                        let mut rec = p.byte_add(8);
                        loop {
                            let p7 = mech_trace::cr!(
                                "10038710",
                                "HeapAlloc",
                                HeapAlloc(
                                    mech_trace::gr!("10038710", "10098224", DAT_10098224),
                                    1,
                                    0x1a
                                )
                            );
                            wr8(p7, 8, 0);
                            wr(p7, 0, rd(rec, 0));
                            wr(p7, 4, rd(rec, 4));
                            FUN_10050390(p7 as *mut i32);
                            wr(p7, 0xe, rd(rec, 8));
                            wr(p7, 0xa, -1);
                            mech_trace::cr!(
                                "10038710",
                                "10017df0",
                                FUN_10017df0(
                                    mech_trace::gr!("10038710", "100a67cc", DAT_100a67cc),
                                    p7 as u32
                                )
                            );
                            cnt = cnt.wrapping_sub(1);
                            rec = rec.byte_add(0xc);
                            if cnt == 0 {
                                break;
                            }
                        }
                    }
                }
                0x544e5443 => {
                    let n = rd(p, 4) as u32;
                    copy_bytes(
                        mech_trace::gr!("10038710", "10083508", DAT_10083508).byte_add(0x1342),
                        p.byte_add(8),
                        n.wrapping_sub(1),
                    );
                    wr8(
                        mech_trace::gr!("10038710", "10083508", DAT_10083508)
                            .byte_add(rd(p, 4) as usize),
                        0x1342,
                        0,
                    );
                }
                0x54534c46 => {
                    let mut cnt = (rd(p, 4) as u32).wrapping_sub(8) >> 4;
                    wr(
                        mech_trace::gr!("10038710", "10083508", DAT_10083508),
                        0x2396,
                        cnt as i32,
                    );
                    if cnt != 0 {
                        let mut off = 0usize;
                        let mut rec = p.byte_add(8);
                        loop {
                            copy_str(local_200.as_mut_ptr(), rec);
                            off += 0x10;
                            rec = rec.byte_add(0x10);
                            copy_str(
                                mech_trace::gr!("10038710", "10083508", DAT_10083508)
                                    .byte_add(0x292 + off),
                                local_200.as_ptr(),
                            );
                            cnt = cnt.wrapping_sub(1);
                            if cnt == 0 {
                                break;
                            }
                        }
                    }
                }
                0x54534c53 => {
                    let mut cnt = (rd(p, 4) as u32).wrapping_sub(8) >> 4;
                    wr(
                        mech_trace::gr!("10038710", "10083508", DAT_10083508),
                        0x2392,
                        cnt as i32,
                    );
                    if cnt != 0 {
                        let mut off = 0usize;
                        let mut rec = p.byte_add(8);
                        loop {
                            copy_str(local_200.as_mut_ptr(), rec);
                            off += 0x10;
                            rec = rec.byte_add(0x10);
                            copy_str(
                                mech_trace::gr!("10038710", "10083508", DAT_10083508)
                                    .byte_add(0x1f2 + off),
                                local_200.as_ptr(),
                            );
                            cnt = cnt.wrapping_sub(1);
                            if cnt == 0 {
                                break;
                            }
                        }
                    }
                }
                _ => {}
            }
            if !ngpc {
                local_334 = local_334.wrapping_add(rd(p, 4));
            }
            if local_324 > local_334 {
                local_338 = (local_334.wrapping_add(local_32c)) as u32 as usize as *mut u8;
            } else {
                break;
            }
        }

        let len = strlen(&raw const DAT_100944f4);
        let r = mech_trace::cr!(
            "10038710",
            "_strncmp",
            _strncmp(param_1, &raw mut DAT_100944f8 as *mut i8, len as u32)
        );
        r_last = r;
        if r == 0 {
            r_last = mech_trace::cr!("10038710", "100385c0", FUN_100385c0(param_1));
        }
        mech_trace::rt!("10038710", ((r_last as u32) & 0xffffff00) | 1)
    }
}
