// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b4834: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _malloc(size: usize) -> *mut core::ffi::c_void;
    fn _fopen(filename: *mut i8, mode: *mut i8) -> *mut core::ffi::c_void;
    fn FUN_10070d99(filename: *mut u32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10070ed9(filename: *mut i8) -> *mut i32 {
    // SAFETY: the callees and the global are the game's own; the malloc'd block
    // is 0x4bc bytes, so indices 0 and 0x12d are in range.
    unsafe {
        let p = _malloc(0x4bc) as *mut i32;
        if p.is_null() {
            return core::ptr::null_mut();
        }
        let f = _fopen(filename, (&raw mut DAT_100b4834).cast::<i8>());
        p.write_unaligned(f as i32);
        if p.read_unaligned() == 0 {
            return core::ptr::null_mut();
        }
        p.add(0x12d).write_unaligned(0);
        FUN_10070d99(p.cast::<u32>());
        p
    }
}
