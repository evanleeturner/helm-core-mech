// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a40f8: *mut i32;
    static mut DAT_100adf74: *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapAlloc(h: *mut core::ffi::c_void, flags: u32, size: u32) -> *mut core::ffi::c_void;
    fn HeapFree(h: *mut core::ffi::c_void, flags: u32, p: *mut core::ffi::c_void) -> i32;
    fn _AIL_allocate_sample_handle_4(param_1: i32) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1006aec6() -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10002590(param_1: u32) -> u32 {
    // SAFETY: globals and callees are the game's own; DAT_100a40f8 points at a
    // 0x508-byte block allocated here, and all indices stay within the layout
    // the original uses.
    unsafe {
        if !DAT_100a40f8.is_null() {
            return 0xffffffff;
        }
        DAT_100a40f8 = HeapAlloc(DAT_100adf74, 1, 0x508) as *mut i32;
        if DAT_100a40f8.is_null() {
            return 0xfffffffd;
        }
        let mut p = DAT_100a40f8;
        let mut c: i32 = 0x142;
        while c != 0 {
            p.write_unaligned(0);
            p = p.add(1);
            c -= 1;
        }
        let r = FUN_1006aec6();
        DAT_100a40f8.write_unaligned(r);
        if DAT_100a40f8.read_unaligned() == 0 {
            HeapFree(DAT_100adf74, 1, DAT_100a40f8 as *mut core::ffi::c_void);
            DAT_100a40f8 = core::ptr::null_mut();
            return 0xfffffffc;
        }
        if param_1 < 0x11 {
            DAT_100a40f8.add(0x11).write_unaligned(param_1 as i32);
        } else {
            DAT_100a40f8.add(0x11).write_unaligned(0x10);
        }
        let mut i: u32 = 0;
        while i < DAT_100a40f8.add(0x11).read_unaligned() as u32 {
            let h = _AIL_allocate_sample_handle_4(DAT_100a40f8.read_unaligned());
            DAT_100a40f8
                .add(i.wrapping_add(1) as usize)
                .write_unaligned(h);
            if DAT_100a40f8
                .add(i.wrapping_add(1) as usize)
                .read_unaligned()
                == 0
            {
                DAT_100a40f8.add(0x11).write_unaligned(i as i32);
                break;
            }
            let pv = HeapAlloc(DAT_100adf74, 1, 0x4000);
            let idx = i.wrapping_mul(2).wrapping_add(0xaa) as usize;
            DAT_100a40f8.add(idx).write_unaligned(pv as usize as i32);
            if DAT_100a40f8.add(idx).read_unaligned() == 0 {
                DAT_100a40f8.add(0x11).write_unaligned(i as i32);
                break;
            }
            i = i.wrapping_add(1);
        }
        i = 0;
        while i < DAT_100a40f8.add(0x11).read_unaligned() as u32 {
            DAT_100a40f8
                .add(i.wrapping_add(0x12) as usize)
                .write_unaligned(0);
            DAT_100a40f8
                .add(i.wrapping_add(0xea) as usize)
                .write_unaligned(-1);
            DAT_100a40f8
                .add(i.wrapping_add(0xfa) as usize)
                .write_unaligned(0);
            DAT_100a40f8
                .add(i.wrapping_add(0x10a) as usize)
                .write_unaligned(0x40);
            i = i.wrapping_add(1);
        }
        1
    }
}
