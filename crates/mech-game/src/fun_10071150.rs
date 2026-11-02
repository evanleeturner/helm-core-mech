// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa1ac: u32;
    static mut DAT_100ab318: u32;
    static mut DAT_100ab324: u32;
    static mut DAT_100ab330: u32;
    static mut DAT_100adf38: i32;
    static mut DAT_100b4838: u32;
    static mut DAT_100b4840: u8;
    static mut DAT_100b4844: u8;
    static mut DAT_100b484d: u8;
    static mut DAT_100b4d68: u8;
    static mut DAT_100eef40: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100228e4(param_1: i32);
    fn FUN_1000a970(param_1: *mut i32, param_2: *mut i8, param_3: u32, param_4: *mut i8) -> u32;
    fn FUN_100715a5(param_1: *mut u32) -> *mut i32;
    fn FUN_1002f783(param_1: i32);
    fn FUN_1002fbc0(param_1: *mut i32);
    fn FUN_1004fd60(param_1: i32);
    fn FUN_10070b00(param_1: i32);
    fn FUN_1001ee10(param_1: i32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1005d029();
}

#[inline(always)]
unsafe fn rd(p: i32) -> i32 {
    // SAFETY: caller passes an address of a live game structure field.
    unsafe { mech_trace::rd!("10071150", (p as *const i32), read_unaligned) }
}

#[inline(always)]
unsafe fn wb(local_c: *mut i32, off: i32, v: u8) {
    // SAFETY: local_c points at the game's object whose first field points at a
    // structure holding another pointer at +0x4c.
    unsafe {
        let q = rd(mech_trace::rd!("10071150", local_c, read_unaligned).wrapping_add(0x4c));
        mech_trace::wr!("10071150", (q.wrapping_add(off) as *mut u8), write, v);
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10071150(
    param_1: i32,
    param_2: i32,
    _param_3: u32,
    _param_4: u32,
) -> u32 {
    mech_trace::enter!("10071150", param_1, param_2, _param_3, _param_4);
    // SAFETY: all globals, callees and pointers are the game's own memory,
    // accessed where the original accesses them.
    unsafe {
        let tab = (&raw mut DAT_100eef40).cast::<u32>();
        let mut local_c: *mut i32 = core::ptr::null_mut();
        if -1 < param_1 && param_1 < 0x3c {
            let e = mech_trace::rd!("10071150", tab.offset(param_1 as isize), read_unaligned);
            local_c = rd(e.wrapping_add(0x20) as i32) as *mut i32;
        }
        if local_c.is_null() {
            return mech_trace::rt!("10071150", 0);
        }
        let ent = mech_trace::rd!("10071150", tab.offset(param_1 as isize), read_unaligned);
        let u_var1 = mech_trace::rd!(
            "10071150",
            ((ent.wrapping_add(0x14)) as *const u16),
            read_unaligned
        );
        if param_2 == 0 && (u_var1 & 2) == 0 {
            return mech_trace::rt!("10071150", 0);
        }
        let l = |i: usize| local_c.add(i);
        mech_trace::wr!("10071150", l(0xb), write_unaligned, 0);
        mech_trace::wr!(
            "10071150",
            l(10),
            write_unaligned,
            mech_trace::rd!("10071150", l(0xb), read_unaligned)
        );
        mech_trace::wr!("10071150", l(3), write_unaligned, 0);
        mech_trace::wr!(
            "10071150",
            l(2),
            write_unaligned,
            mech_trace::rd!("10071150", l(3), read_unaligned)
        );
        mech_trace::wr!("10071150", l(0xf), write_unaligned, 0);
        mech_trace::wr!(
            "10071150",
            l(0xe),
            write_unaligned,
            mech_trace::rd!("10071150", l(0xf), read_unaligned)
        );
        mech_trace::wr!("10071150", l(7), write_unaligned, 0);
        mech_trace::wr!(
            "10071150",
            l(6),
            write_unaligned,
            mech_trace::rd!("10071150", l(7), read_unaligned)
        );
        {
            let q = rd(mech_trace::rd!("10071150", local_c, read_unaligned).wrapping_add(0x4c));
            mech_trace::wr!(
                "10071150",
                (q.wrapping_add(8) as *mut i32),
                write_unaligned,
                0
            );
        }
        wb(local_c, 0x24, 1);
        wb(local_c, 0x1c, 1);
        wb(local_c, 0x18, 1);
        wb(local_c, 0x1d, 0);
        wb(local_c, 0x1e, 0);
        wb(local_c, 0x1f, 0);
        wb(local_c, 0x20, 0);
        wb(local_c, 0x21, 0);
        wb(local_c, 0x25, 0);
        wb(local_c, 0x26, 0);
        wb(local_c, 0x2e, 0);
        wb(local_c, 0x2f, 0);
        wb(local_c, 0x30, 0);
        wb(local_c, 0x31, 0);
        wb(local_c, 0x43, 0);
        mech_trace::wr!(
            "10071150",
            l(0x100),
            write_unaligned,
            mech_trace::rd!("10071150", l(0x11a), read_unaligned)
        );
        mech_trace::gw!("10071150", "100b4838", DAT_100b4838, param_1 as u32);
        FUN_100228e4(local_c as i32);
        let off = param_1.wrapping_mul(0x16) as isize;
        let a = (&raw mut DAT_100b4844).offset(off).cast::<i8>();
        let b = mech_trace::rd!(
            "10071150",
            (&raw mut DAT_100b4840).offset(off).cast::<u32>(),
            read_unaligned
        );
        let c = (&raw mut DAT_100b484d).offset(off).cast::<i8>();
        mech_trace::cr!("10071150", "1000a970", FUN_1000a970(local_c, a, b, c));
        let src = mech_trace::rd!(
            "10071150",
            (&raw mut DAT_100b4d68)
                .offset(param_1.wrapping_mul(4) as isize)
                .cast::<*mut u32>(),
            read_unaligned
        );
        let pi_var3 = mech_trace::cr!("10071150", "100715a5", FUN_100715a5(src));
        mech_trace::wr!(
            "10071150",
            ((mech_trace::rd!("10071150", local_c, read_unaligned).wrapping_add(0x40))
                as *mut *mut i32),
            write_unaligned,
            pi_var3
        );
        FUN_1002f783(rd(
            mech_trace::rd!("10071150", local_c, read_unaligned).wrapping_add(0x40)
        ));
        FUN_1002fbc0(
            rd(mech_trace::rd!("10071150", local_c, read_unaligned).wrapping_add(0x40)) as *mut i32,
        );
        mech_trace::wr!(
            "10071150",
            (mech_trace::rd!("10071150", local_c, read_unaligned).wrapping_add(0x1c) as *mut u32),
            write_unaligned,
            0xffff_ffff
        );
        mech_trace::wr!(
            "10071150",
            (mech_trace::rd!("10071150", local_c, read_unaligned).wrapping_add(0xdc) as *mut u32),
            write_unaligned,
            0x1000
        );
        let ent2 = mech_trace::rd!("10071150", tab.offset(param_1 as isize), read_unaligned);
        mech_trace::wr!(
            "10071150",
            ((ent2.wrapping_add(0x14)) as *mut u16),
            write_unaligned,
            (u_var1 & 0xbff8) | 0x10
        );
        if mech_trace::gr!("10071150", "100aa1ac", DAT_100aa1ac) != 0 {
            FUN_1004fd60(param_1);
        }
        FUN_10070b00(mech_trace::rd!("10071150", local_c, read_unaligned));
        FUN_1001ee10(mech_trace::rd!("10071150", local_c, read_unaligned));
        mech_trace::wr!(
            "10071150",
            local_c.cast::<u8>().add(0x442).cast::<u16>(),
            write_unaligned,
            0
        );
        if param_1 == mech_trace::gr!("10071150", "100adf38", DAT_100adf38) {
            FUN_1005d029();
            mech_trace::gw!("10071150", "100ab318", DAT_100ab318, 0);
            mech_trace::gw!("10071150", "100ab330", DAT_100ab330, 0);
            mech_trace::gw!("10071150", "100ab324", DAT_100ab324, 0);
        }
        mech_trace::gw!("10071150", "100b4838", DAT_100b4838, 0xffff_ffff);
        mech_trace::rt!("10071150", 1)
    }
}
