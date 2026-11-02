// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a4108: u32;
    static mut DAT_100a410c: u32;
    static mut DAT_100aa1ac: u32;
    static mut DAT_100ab324: u32;
    static mut DAT_100adf34: u32;
    static mut DAT_100adf38: u32;
    static mut DAT_100b3218: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10051f43(param_1: i32) -> i32;
    fn FUN_10003742(param_1: u32, param_2: i32, param_3: u32, param_4: i32, param_5: u16) -> u8;
    fn FUN_10005207(param_1: i32, param_2: *mut i32, param_3: u32) -> u8;
    fn FUN_10005624(
        param_1: i32,
        param_2: *mut i32,
        param_3: u32,
        param_4: i32,
        param_5: i32,
    ) -> u8;
    fn FUN_100300e2(param_1: i32, param_2: i32, param_3: u32) -> u8;
}

unsafe fn rd(addr: i32) -> i32 {
    // SAFETY: the caller passes an address of readable game memory.
    unsafe { mech_trace::rd!("100065c4", (addr as usize as *const i32), read_unaligned) }
}

unsafe fn wr(addr: i32, v: i32) {
    // SAFETY: the caller passes an address of writable game memory.
    unsafe { mech_trace::wr!("100065c4", (addr as usize as *mut i32), write_unaligned, v) }
}

unsafe fn rd16(addr: i32) -> i16 {
    // SAFETY: the caller passes an address of readable game memory.
    unsafe { mech_trace::rd!("100065c4", (addr as usize as *const i16), read_unaligned) }
}

unsafe fn wr16(addr: i32, v: u16) {
    // SAFETY: the caller passes an address of writable game memory.
    unsafe { mech_trace::wr!("100065c4", (addr as usize as *mut u16), write_unaligned, v) }
}

unsafe fn udiv(n: u32, d: u32) -> u32 {
    let q: u32;
    // SAFETY: the machine's own div; a zero divisor faults as in the original.
    unsafe {
        core::arch::asm!(
            "div {d}",
            d = in(reg) d,
            inout("eax") n => q,
            inout("edx") 0u32 => _,
            options(nomem, nostack)
        );
    }
    q
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100065c4(param_1: i32, param_2: *mut i32, param_3: i32, param_4: u32) {
    mech_trace::enter!("100065c4", param_1, param_2, param_3, param_4);
    let mut param_4 = param_4;
    // SAFETY: globals and callees are the game's own; param_2 points at the
    // game's object whose fields are read and written where the original does.
    unsafe {
        let mut local_8: i32 = 0;
        let mut local_18: i32 = 0;
        let local_c: u32;
        let id = rd(mech_trace::rd!("100065c4", param_2, read_unaligned).wrapping_add(4)) as u32;
        if !((id != mech_trace::gr!("100065c4", "100adf38", DAT_100adf38)
            || mech_trace::dr!(
                "100065c4",
                ((mech_trace::gr!("100065c4", "100adf34", DAT_100adf34) as usize).wrapping_add(1)
                    as *const i8)
            ) == 0)
            && mech_trace::gr!("100065c4", "100ab324", DAT_100ab324) != 0
            && (mech_trace::gr!("100065c4", "100aa1ac", DAT_100aa1ac) == 0
                || id == mech_trace::gr!("100065c4", "100adf38", DAT_100adf38))
            && 0 < param_3)
        {
            return;
        }
        let b5 = (param_4 & 0x8000) != 0;
        if b5 {
            param_4 &= 0xffff7fff;
        }
        if !(0 < (param_4 as i32) && (param_4 as i32) < 9) {
            return;
        }
        if param_4 == 3 {
            param_4 = mech_trace::cr!("100065c4", "10051f43", FUN_10051f43(3)) as u32;
            if param_4 == 0 {
                param_4 = 2;
            } else if param_4 == 1 {
                param_4 = 3;
            } else if param_4 == 2 {
                param_4 = 4;
            }
        }
        let i1: i32 = mech_trace::rd!("100065c4", param_2.byte_add(0x16 * 4), read_unaligned)
            .wrapping_add((param_4 as i32).wrapping_mul(0x28));
        let i3: i32 = i1.wrapping_sub(0x28);
        if b5 && (param_4 == 2 || param_4 == 4 || param_4 == 3) {
            local_8 = 1;
            local_c = (((rd16(i1.wrapping_sub(2)) as i32) as u32) & 0xf0) >> 4;
        } else {
            local_c = ((rd16(i1.wrapping_sub(2)) as i32) as u32) & 0xf;
        }
        let p2 = i3.wrapping_add(local_8.wrapping_mul(4));
        wr(p2, rd(p2).wrapping_sub(param_3));
        wr16(
            i1.wrapping_sub(2),
            (rd16(i1.wrapping_sub(2)) as u16) | 0x8000,
        );
        if rd(i3.wrapping_add(local_8.wrapping_mul(4))) < 1 {
            let id =
                rd(mech_trace::rd!("100065c4", param_2, read_unaligned).wrapping_add(4)) as u32;
            if (rd16(i1.wrapping_sub(2)) as u16 & 0x4000) == 0
                && id == mech_trace::gr!("100065c4", "100adf38", DAT_100adf38)
                && mech_trace::rd!("100065c4", param_2.byte_add(0xf5 * 4), read_unaligned) == 2
            {
                mech_trace::cr!(
                    "100065c4",
                    "10003742",
                    FUN_10003742(0x101, 100, 0x40, 5, 0x50)
                );
            }
            wr16(
                i1.wrapping_sub(2),
                (rd16(i1.wrapping_sub(2)) as u16) | 0x4000,
            );
            let a = i1.wrapping_sub(0x20);
            wr(
                a,
                rd(a).wrapping_add(rd(i3.wrapping_add(local_8.wrapping_mul(4)))),
            );
            wr(i3.wrapping_add(local_8.wrapping_mul(4)), 0);
            let id =
                rd(mech_trace::rd!("100065c4", param_2, read_unaligned).wrapping_add(4)) as u32;
            if id == mech_trace::gr!("100065c4", "100adf38", DAT_100adf38)
                && (param_4 == 1 || param_4 == 3)
                && mech_trace::rd!("100065c4", param_2.byte_add(0xf5 * 4), read_unaligned) != 4
                && 0x20000 < param_3
            {
                mech_trace::gw!("100065c4", "100b3218", DAT_100b3218, 1);
            }
            if rd(i1.wrapping_sub(0x20)) < 1 {
                mech_trace::cr!(
                    "100065c4",
                    "10005207",
                    FUN_10005207(param_1, param_2, param_4)
                );
                return;
            }
            let mut local_14: i32 = 0;
            loop {
                let i4 = mech_trace::cr!("100065c4", "10051f43", FUN_10051f43(5));
                if !(local_14 < i4) {
                    break;
                }
                let i4 = mech_trace::cr!("100065c4", "10051f43", FUN_10051f43(100));
                if i4 < 0x14 && 0 < rd16(i1.wrapping_sub(4)) {
                    if i4 == 0xc {
                        mech_trace::cr!(
                            "100065c4",
                            "10005207",
                            FUN_10005207(param_1, param_2, param_4)
                        );
                        return;
                    }
                    let i6 = 0;
                    let i4 = mech_trace::cr!(
                        "100065c4",
                        "10051f43",
                        FUN_10051f43(rd16(i1.wrapping_sub(4)) as i32)
                    );
                    mech_trace::cr!(
                        "100065c4",
                        "10005624",
                        FUN_10005624(param_1, param_2, param_4, i4, i6)
                    );
                }
                local_14 = local_14.wrapping_add(1);
            }
        }
        if local_c != 0 {
            let id =
                rd(mech_trace::rd!("100065c4", param_2, read_unaligned).wrapping_add(4)) as u32;
            let cur = rd(i3.wrapping_add(local_8.wrapping_mul(4)));
            let d = if id == mech_trace::gr!("100065c4", "100adf38", DAT_100adf38) {
                mech_trace::gr!("100065c4", "100a410c", DAT_100a410c)
            } else {
                mech_trace::gr!("100065c4", "100a4108", DAT_100a4108)
            };
            let q = udiv(cur as u32, d) as i32;
            let sum = rd(i1.wrapping_sub(0x20)).wrapping_add(q);
            let num = sum.wrapping_mul(3);
            let den = (local_c << 0x10) as i32;
            local_18 = 0xf_i32.wrapping_sub(num / den);
        }
        mech_trace::cr!(
            "100065c4",
            "100300e2",
            FUN_100300e2(
                rd(mech_trace::rd!("100065c4", param_2, read_unaligned).wrapping_add(0x40)),
                local_18,
                param_4,
            )
        );
    }
}
