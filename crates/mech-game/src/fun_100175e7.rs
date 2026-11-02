// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _fread(
        _DstBuf: *mut core::ffi::c_void,
        _ElementSize: usize,
        _Count: usize,
        _File: *mut core::ffi::c_void,
    ) -> usize;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100175e7(
    file: *mut core::ffi::c_void,
    param_2: i32,
    param_3: i32,
) -> u32 {
    let mut local_c: u32 = 0;
    let mut local_8: usize = 0;
    // SAFETY: file is the caller's FILE handle and param_2 is the address of
    // the caller's buffer; reads and writes happen where the original does.
    unsafe {
        while local_c < (param_3 as u32).wrapping_sub(1) {
            local_8 = _fread(
                local_c.wrapping_add(param_2 as u32) as *mut core::ffi::c_void,
                1,
                1,
                file,
            );
            if local_8 == 0 {
                break;
            }
            if *(local_c.wrapping_add(param_2 as u32) as *const i8) == b'\n' as i8 {
                break;
            }
            local_c = local_c.wrapping_add(1);
        }
        if local_c != 0 || local_8 != 0 {
            local_c = local_c.wrapping_add(1);
            *(local_c.wrapping_add(param_2 as u32) as *mut u8) = 0;
        }
    }
    local_c
}
