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
    mech_trace::enter!("10070ed9", filename);
    // SAFETY: the callees and the global are the game's own; the malloc'd block
    // is 0x4bc bytes, so indices 0 and 0x12d are in range.
    unsafe {
        let p = mech_trace::cr!("10070ed9", "_malloc", _malloc(0x4bc)) as *mut i32;
        if p.is_null() {
            return mech_trace::rt!("10070ed9", core::ptr::null_mut());
        }
        let f = mech_trace::cr!(
            "10070ed9",
            "_fopen",
            _fopen(filename, (&raw mut DAT_100b4834).cast::<i8>())
        );
        mech_trace::wr!("10070ed9", p, write_unaligned, f as i32);
        if mech_trace::rd!("10070ed9", p, read_unaligned) == 0 {
            return mech_trace::rt!("10070ed9", core::ptr::null_mut());
        }
        mech_trace::wr!("10070ed9", p.add(0x12d), write_unaligned, 0);
        mech_trace::cr!("10070ed9", "10070d99", FUN_10070d99(p.cast::<u32>()));
        mech_trace::rt!("10070ed9", p)
    }
}
