// SPDX-License-Identifier: MIT

#[allow(non_camel_case_types)]
pub struct FILE {
    _private: [u8; 0],
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _fread(
        _DstBuf: *mut core::ffi::c_void,
        _ElementSize: u32,
        _Count: u32,
        _File: *mut FILE,
    ) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10071107(
    param_1: *mut u32,
    dstBuf: *mut core::ffi::c_void,
    count: u32,
) -> u32 {
    // SAFETY: param_1 points at the caller's struct whose first field is a FILE pointer;
    // _fread is the game's own C runtime routine.
    unsafe {
        let file = param_1.read_unaligned() as usize as *mut FILE;
        let s_var1 = _fread(dstBuf, 1, count, file);
        if s_var1 != count {
            return 0xffffffff;
        }
        s_var1
    }
}
