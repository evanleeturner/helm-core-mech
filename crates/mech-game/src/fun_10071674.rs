// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf74: *mut c_void;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapAlloc(hHeap: *mut c_void, dwFlags: u32, dwBytes: u32) -> *mut c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002f34e(param_1: i32, param_2: *mut c_void, param_3: *mut c_void, param_4: *mut c_void);
    fn FUN_1002f2fc(param_1: i32, param_2: *mut c_void, param_3: *mut c_void, param_4: *mut c_void);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10071674(param_1: *mut c_void) -> *mut c_void {
    if param_1.is_null() {
        return core::ptr::null_mut();
    }
    // SAFETY: the heap handle and callees are the game's own; param_1 points at
    // a node of pointer-sized fields and the new block is 0x28 bytes, accessed
    // at the offsets the original uses.
    unsafe {
        let p1 = HeapAlloc(DAT_100adf74, 1, 0x28) as *mut u8;
        if p1.is_null() {
            return core::ptr::null_mut();
        }
        let src = param_1 as *mut u8;
        p1.byte_add(8)
            .cast::<*mut c_void>()
            .write_unaligned(param_1);
        let v0 = src.cast::<*mut c_void>().read_unaligned();
        p1.byte_add(12).cast::<*mut c_void>().write_unaligned(v0);
        let c1 = src.byte_add(4).cast::<*mut c_void>().read_unaligned();
        let r = FUN_10071674(c1);
        p1.cast::<*mut c_void>().write_unaligned(r);
        let c2 = src.byte_add(8).cast::<*mut c_void>().read_unaligned();
        let r = FUN_10071674(c2);
        p1.byte_add(4).cast::<*mut c_void>().write_unaligned(r);
        FUN_1002f34e(
            param_1 as i32,
            p1.byte_add(16).cast::<c_void>(),
            p1.byte_add(20).cast::<c_void>(),
            p1.byte_add(24).cast::<c_void>(),
        );
        FUN_1002f2fc(
            param_1 as i32,
            p1.byte_add(28).cast::<c_void>(),
            p1.byte_add(32).cast::<c_void>(),
            p1.byte_add(36).cast::<c_void>(),
        );
        p1.cast::<c_void>()
    }
}
