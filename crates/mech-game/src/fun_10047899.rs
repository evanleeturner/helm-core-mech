// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf74: *mut c_void;
    static mut DAT_100af0e4: u32;
    static mut DAT_100af150: *mut i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1007bbed(
        param_1: *mut i16,
        param_2: *mut i32,
        param_3: *mut i8,
        param_4: u32,
        param_5: *mut u32,
        param_6: *mut i32,
    ) -> *mut u16;
    fn FUN_1008664f(param_1: u32, param_2: *mut i32) -> u8;
}

unsafe extern "stdcall" {
    fn HeapFree(h: *mut c_void, flags: u32, mem: *mut c_void) -> i32;
}

unsafe fn fill(base: i32, count: i32, mut p: *const u16) -> *const u16 {
    let mut c = base;
    let mut i = 0;
    while i < count {
        // SAFETY: p points at the returned rect table, c at the caller's records.
        unsafe {
            let a = p.read_unaligned() as i16 as i32;
            let b = p.add(1).read_unaligned() as i16 as i32;
            let w = p.add(2).read_unaligned() as i16 as i32;
            let h = p.add(3).read_unaligned() as i16 as i32;
            ((c.wrapping_add(4)) as *mut i32).write_unaligned(a);
            ((c.wrapping_add(8)) as *mut i32).write_unaligned(b);
            ((c.wrapping_add(0xc)) as *mut i32).write_unaligned(w.wrapping_add(a).wrapping_sub(1));
            ((c.wrapping_add(0x10)) as *mut i32).write_unaligned(b.wrapping_add(h).wrapping_sub(1));
            p = p.add(4);
        }
        c = c.wrapping_add(0x14);
        i += 1;
    }
    p
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10047899(
    param_1: *mut i16,
    param_2: i32,
    param_3: i32,
    param_4: *mut i32,
) -> u32 {
    if param_2 == 0 || param_3 == 0 || param_4.is_null() {
        return 0;
    }
    let mut local_8: u32 = 0;
    // SAFETY: the callees and globals are the game's own; param pointers are the caller's.
    unsafe {
        let lp_mem = FUN_1007bbed(
            param_1,
            DAT_100af0e4 as *mut i32,
            DAT_100af150,
            5,
            &raw mut local_8,
            core::ptr::null_mut(),
        );
        if lp_mem.is_null() {
            return 0;
        }
        let mut p = fill(param_2, 5, lp_mem as *const u16);
        p = fill(param_3, 0x13, p);
        param_4.write_unaligned(p.read_unaligned() as i16 as i32);
        param_4
            .add(1)
            .write_unaligned(p.add(1).read_unaligned() as i16 as i32);
        if param_1.read_unaligned() == -1 {
            HeapFree(DAT_100adf74, 1, lp_mem as *mut c_void);
        } else {
            FUN_1008664f(
                param_1.read_unaligned() as i32 as u32,
                DAT_100af0e4 as *mut i32,
            );
        }
    }
    1
}
