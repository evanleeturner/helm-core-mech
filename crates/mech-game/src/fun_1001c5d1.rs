// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ab0c0: u32;
    static mut DAT_100adf74: *mut c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1001c303(
        param_1: i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: *mut u32,
    ) -> *mut i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapAlloc(h: *mut c_void, flags: u32, size: u32) -> *mut c_void;
    fn HeapFree(h: *mut c_void, flags: u32, mem: *mut c_void) -> i32;
}

#[inline(always)]
unsafe fn w16(p: *mut u16, idx: usize, v: u16) {
    // SAFETY: p points at a live 0x54-byte block; idx is within it.
    unsafe { mech_trace::wr!("1001c5d1", p.add(idx), write_unaligned, v) }
}

#[inline(always)]
unsafe fn r32(p: *mut u16, idx: usize) -> u32 {
    // SAFETY: p points at a live 0x54-byte block; idx is within it.
    unsafe { mech_trace::rd!("1001c5d1", p.add(idx).cast::<u32>(), read_unaligned) }
}

#[inline(always)]
unsafe fn w32(p: *mut u16, idx: usize, v: u32) {
    // SAFETY: p points at a live 0x54-byte block; idx is within it.
    unsafe { mech_trace::wr!("1001c5d1", p.add(idx).cast::<u32>(), write_unaligned, v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001c5d1(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: *mut u32,
) -> *mut u16 {
    mech_trace::enter!("1001c5d1", param_1, param_2, param_3, param_4);
    // SAFETY: the heap handle, globals and callee are the game's own; the
    // block returned by HeapAlloc is 0x54 bytes and only written within that.
    unsafe {
        let mut lp_mem = mech_trace::cr!(
            "1001c5d1",
            "HeapAlloc",
            HeapAlloc(
                mech_trace::gr!("1001c5d1", "100adf74", DAT_100adf74),
                1,
                0x54
            )
        )
        .cast::<u16>();
        if lp_mem.is_null() {
            lp_mem = core::ptr::null_mut();
        } else {
            w16(
                lp_mem,
                0,
                (mech_trace::gr!("1001c5d1", "100ab0c0", DAT_100ab0c0) as u16) | 0x8000,
            );
            w16(lp_mem, 0xc, 0);
            w16(lp_mem, 0xd, 0);
            w16(lp_mem, 0xe, 0);
            w16(lp_mem, 0xf, 0);
            let v = r32(lp_mem, 0xe);
            w32(lp_mem, 0x10, v);
            let pi_var1 = mech_trace::cr!(
                "1001c5d1",
                "1001c303",
                FUN_1001c303(
                    lp_mem as usize as i32,
                    0,
                    param_1,
                    param_2,
                    param_3,
                    param_4,
                )
            );
            if pi_var1.is_null() {
                mech_trace::cr!(
                    "1001c5d1",
                    "HeapFree",
                    HeapFree(
                        mech_trace::gr!("1001c5d1", "100adf74", DAT_100adf74),
                        1,
                        lp_mem.cast::<c_void>()
                    )
                );
                lp_mem = core::ptr::null_mut();
            } else {
                w16(lp_mem, 1, 0);
                w16(lp_mem, 0xb, 0);
                w16(lp_mem, 10, 0);
                w16(lp_mem, 0x1e, 0);
                w16(lp_mem, 0x1f, 0);
                let v = r32(lp_mem, 0x1e);
                w32(lp_mem, 0x1c, v);
                let v = r32(lp_mem, 0x1c);
                w32(lp_mem, 0x1a, v);
                w16(lp_mem, 0x18, 0);
                w16(lp_mem, 0x19, 0);
                let v = r32(lp_mem, 0x18);
                w32(lp_mem, 0x16, v);
                let v = r32(lp_mem, 0x16);
                w32(lp_mem, 0x14, v);
                w16(lp_mem, 0x20, 0);
                w16(lp_mem, 0x21, 0);
                w16(lp_mem, 0x26, 0);
                w16(lp_mem, 0x27, 0);
                w16(lp_mem, 4, 0);
                w16(lp_mem, 5, 0);
                let v = r32(lp_mem, 4);
                w32(lp_mem, 2, v);
                w16(lp_mem, 8, 0);
                w16(lp_mem, 9, 0);
                let v = r32(lp_mem, 8);
                w32(lp_mem, 6, v);
                w16(lp_mem, 0x12, 4);
                w16(lp_mem, 0x13, 0);
                w16(lp_mem, 0x28, 0);
                w16(lp_mem, 0x29, 0);
            }
        }
        mech_trace::rt!("1001c5d1", lp_mem)
    }
}
