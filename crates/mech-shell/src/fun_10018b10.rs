// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut LAB_10018d40: u8;
    static mut LAB_10018d90: u8;
    static mut LAB_10018df0: u8;
    static mut LAB_10018eb0: u8;
    static mut LAB_10018fe0: u8;
    static mut DAT_10098224: *mut c_void;
    static mut DAT_100a6744: u32;
    static mut DAT_100a674c: u32;
    static mut DAT_100a6798: u32;
    static mut DAT_100c49d0: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10018060(param_1: i32, param_2: i32) -> u32;
    fn FUN_100507a0(param_1: i32) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapAlloc(h: *mut c_void, flags: u32, size: u32) -> *mut i32;
}

#[inline(always)]
unsafe fn w(p: *mut i32, i: i32, v: i32) {
    // SAFETY: caller guarantees the index lies within the allocated block.
    unsafe { p.offset(i as isize).write_unaligned(v) }
}

#[inline(always)]
unsafe fn rd(base: u32) -> i32 {
    // SAFETY: base is a game object pointer; field at +0x40c is read.
    unsafe { (base.wrapping_add(0x40c) as *const i32).read_unaligned() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10018b10(param_1: i32, param_2: i32, param_3: i32) -> *mut i32 {
    if param_3 == 0 {
        return core::ptr::null_mut();
    }
    // SAFETY: globals and callees are the game's own; param_3 points at a
    // game structure; the heap block is sized for every index written.
    unsafe {
        let count = ((param_3 as u32).wrapping_add(4) as *const i32).read_unaligned();
        let p2 = HeapAlloc(
            DAT_10098224,
            1,
            count.wrapping_mul(0x70).wrapping_add(0xa8) as u32,
        );
        w(p2, 0, param_1);
        w(p2, 1, param_2);
        w(p2, 2, -1);
        w(p2, 3, rd(DAT_100a674c));
        w(p2, 4, 0);
        w(p2, 5, (&raw mut DAT_100c49d0) as i32);
        w(p2, 6, 0);
        w(p2, 7, (&raw mut LAB_10018d40) as i32);
        w(p2, 8, 0);
        w(p2, 9, -1);
        w(p2, 0xb, 0);
        w(p2, 0xe, param_1.wrapping_add(200));
        w(p2, 0xf, param_2);
        w(p2, 0x10, -1);
        w(p2, 0x11, rd(DAT_100a674c));
        w(p2, 0x12, 0);
        w(p2, 0x13, (&raw mut DAT_100c49d0) as i32);
        w(p2, 0x14, 0);
        let mut i_var5: i32 = 2;
        let mut i_var4: i32 = 0;
        let mut local_8: i32 = 2;
        w(p2, 0x15, (&raw mut LAB_10018d90) as i32);
        w(p2, 0x16, 0);
        w(p2, 0x17, -1);
        w(p2, 0x19, 0);
        let count = ((param_3 as u32).wrapping_add(4) as *const i32).read_unaligned();
        if 0 < count {
            loop {
                let b = p2.offset(i_var5.wrapping_mul(0xe) as isize);
                w(b, 0, param_1);
                w(
                    b,
                    1,
                    rd(DAT_100a6744).wrapping_mul(local_8).wrapping_add(param_2),
                );
                w(b, 2, 200);
                w(b, 3, rd(DAT_100a674c));
                w(b, 4, 0);
                w(b, 5, (&raw mut DAT_100c49d0) as i32);
                w(b, 6, 0);
                w(b, 7, (&raw mut LAB_10018df0) as i32);
                w(b, 8, (&raw mut LAB_10018eb0) as i32);
                w(b, 9, i_var4);
                w(b, 0xb, 0);
                let p1 = p2.offset(i_var5.wrapping_add(1).wrapping_mul(0xe) as isize);
                let mut i_var3 = FUN_10018060(DAT_100a6798 as i32, i_var4) as i32;
                i_var5 = i_var5.wrapping_add(2);
                i_var4 = i_var4.wrapping_add(1);
                i_var3 = FUN_100507a0(i_var3);
                w(p1, 0xc, i_var3);
                w(p1, 0, param_1.wrapping_add(200));
                w(
                    p1,
                    1,
                    rd(DAT_100a6744).wrapping_mul(local_8).wrapping_add(param_2),
                );
                w(p1, 2, 200);
                local_8 = local_8.wrapping_add(1);
                w(p1, 3, rd(DAT_100a6744));
                w(p1, 4, 0);
                w(p1, 5, (&raw mut DAT_100c49d0) as i32);
                w(p1, 6, 0);
                w(p1, 7, (&raw mut LAB_10018fe0) as i32);
                w(p1, 8, 0);
                w(p1, 0xb, 0);
                let c = ((param_3 as u32).wrapping_add(4) as *const i32).read_unaligned();
                if !(i_var4 < c) {
                    break;
                }
            }
        }
        w(p2, i_var5.wrapping_mul(0xe), -1);
        p2
    }
}
