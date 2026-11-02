// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10098224: *mut c_void;
    static mut DAT_100a6744: u32;
    static mut DAT_100c49d0: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10031120();
    fn FUN_10031170();
    fn FUN_100311d0();
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapAlloc(h: *mut c_void, flags: u32, size: u32) -> *mut u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10031000(param_1: i32, param_2: i32) -> *mut u32 {
    mech_trace::enter!("10031000", param_1, param_2);
    if param_1 < 0 {
        return mech_trace::rt!("10031000", core::ptr::null_mut());
    }
    if param_2 < 0 {
        return mech_trace::rt!("10031000", core::ptr::null_mut());
    }
    // SAFETY: the heap handle and globals are the game's own; the allocation
    // is sized for the header plus param_2 entries of 0x70 bytes, and every
    // write stays inside it, as in the original.
    unsafe {
        let p3 = mech_trace::cr!(
            "10031000",
            "HeapAlloc",
            HeapAlloc(
                mech_trace::gr!("10031000", "10098224", DAT_10098224),
                1,
                (param_2 as u32).wrapping_mul(0x70).wrapping_add(0x118),
            )
        );
        let arr = (&raw mut DAT_100c49d0) as u32;
        mech_trace::wr!("10031000", p3, write_unaligned, 0x1cc);
        mech_trace::wr!("10031000", p3.add(1), write_unaligned, 0xe8);
        mech_trace::wr!("10031000", p3.add(2), write_unaligned, 0xffffffff);
        mech_trace::wr!("10031000", p3.add(3), write_unaligned, 0xffffffff);
        mech_trace::wr!("10031000", p3.add(4), write_unaligned, 0);
        mech_trace::wr!("10031000", p3.add(5), write_unaligned, arr);
        mech_trace::wr!("10031000", p3.add(6), write_unaligned, 0);
        let mut i_var4: i32 = 0;
        mech_trace::wr!("10031000", p3.add(8), write_unaligned, 0);
        mech_trace::wr!(
            "10031000",
            p3.add(7),
            write_unaligned,
            FUN_10031120 as usize as u32
        );
        mech_trace::wr!("10031000", p3.add(9), write_unaligned, param_1 as u32);
        mech_trace::wr!("10031000", p3.add(10), write_unaligned, 0xffffffff);
        mech_trace::wr!("10031000", p3.add(11), write_unaligned, 0xffffffff);
        let mut p2 = p3;
        if 0 < param_2 {
            loop {
                let i_var1 = i_var4.wrapping_add(2);
                mech_trace::wr!("10031000", p2.add(0xe), write_unaligned, 0x1cc);
                i_var4 = i_var4.wrapping_add(1);
                let base = mech_trace::gr!("10031000", "100a6744", DAT_100a6744) as usize;
                let v = mech_trace::rd!(
                    "10031000",
                    ((base.wrapping_add(0x40c)) as *const i32),
                    read_unaligned
                );
                mech_trace::wr!(
                    "10031000",
                    p2.add(0xf),
                    write_unaligned,
                    v.wrapping_mul(i_var1).wrapping_add(0xe8) as u32
                );
                mech_trace::wr!("10031000", p2.add(0x10), write_unaligned, 0xffffffff);
                mech_trace::wr!("10031000", p2.add(0x11), write_unaligned, 0xffffffff);
                mech_trace::wr!("10031000", p2.add(0x12), write_unaligned, 0);
                mech_trace::wr!("10031000", p2.add(0x13), write_unaligned, arr);
                mech_trace::wr!("10031000", p2.add(0x14), write_unaligned, 0);
                mech_trace::wr!(
                    "10031000",
                    p2.add(0x16),
                    write_unaligned,
                    FUN_100311d0 as usize as u32
                );
                mech_trace::wr!(
                    "10031000",
                    p2.add(0x15),
                    write_unaligned,
                    FUN_10031170 as usize as u32
                );
                mech_trace::wr!("10031000", p2.add(0x17), write_unaligned, param_1 as u32);
                mech_trace::wr!("10031000", p2.add(0x18), write_unaligned, 1);
                mech_trace::wr!("10031000", p2.add(0x19), write_unaligned, 0xffffffff);
                p2 = p2.add(0xe);
                if !(i_var4 < param_2) {
                    break;
                }
            }
        }
        let idx = i_var4.wrapping_mul(0xe).wrapping_add(0xe) as isize;
        mech_trace::wr!("10031000", p3.offset(idx), write_unaligned, 0xffffffff);
        mech_trace::rt!("10031000", p3)
    }
}
