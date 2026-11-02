// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf74: *mut core::ffi::c_void;
    static mut DAT_100adf78: i32;
    static mut DAT_100adf7c: i32;
    static mut DAT_101f52c8: *mut i32;
    static mut DAT_101f52cc: i32;
    static mut DAT_101f52d4: i32;
    static mut DAT_101f52d8: i32;
    static mut DAT_101f52dc: i32;
    static mut DAT_101f52f4: i32;
    static mut DAT_101f52f8: i32;
    static mut DAT_101f52fc: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapAlloc(
        hHeap: *mut core::ffi::c_void,
        dwFlags: u32,
        dwBytes: u32,
    ) -> *mut core::ffi::c_void;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1000e530() -> bool {
    // SAFETY: the globals and HeapAlloc are the game's own; the allocated
    // block is 0x18 bytes, so the six i32 writes stay inside it.
    unsafe {
        DAT_101f52c8 = HeapAlloc(DAT_100adf74, 9, 0x18) as *mut i32;
        let b_var1 = !DAT_101f52c8.is_null();
        if b_var1 {
            DAT_101f52c8.write_unaligned(DAT_100adf78);
            DAT_101f52c8.add(1).write_unaligned(DAT_100adf7c);
            DAT_101f52c8.add(2).write_unaligned(1);
            DAT_101f52c8.add(3).write_unaligned(0x100);
            DAT_101f52c8.add(4).write_unaligned(1);
            DAT_101f52c8.add(5).write_unaligned(0);
            DAT_101f52d8 = (DAT_100adf7c as u32).wrapping_mul(DAT_100adf78 as u32) as i32;
            DAT_101f52dc = DAT_100adf78;
            DAT_101f52cc = DAT_100adf7c;
            DAT_101f52f4 = (DAT_100adf78 as u32).wrapping_sub(1) as i32;
            DAT_101f52d4 = (DAT_100adf7c as u32).wrapping_sub(1) as i32;
            DAT_101f52f8 = DAT_100adf78 / 2;
            DAT_101f52fc = DAT_100adf7c / 2;
        }
        b_var1
    }
}
