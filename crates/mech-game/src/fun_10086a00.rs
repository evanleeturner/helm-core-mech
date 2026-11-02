// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100bf7e8: [u32; 0];
}

unsafe extern "C" {
    fn __strcmpi(_Str1: *mut i8, _Str2: *mut i8) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10086a00(str2: *mut i8) -> *mut u32 {
    let mut local_8: u32 = 0;
    // SAFETY: the table is the game's own memory; each entry's first word is a
    // pointer to a C string, and str2 is the caller's string.
    unsafe {
        let base = (&raw mut DAT_100bf7e8).cast::<u32>();
        loop {
            if 0x58 < local_8 {
                return core::ptr::null_mut();
            }
            let p = base.add(local_8.wrapping_mul(8) as usize).read_unaligned();
            let i_var1 = __strcmpi(p as *mut i8, str2);
            if i_var1 == 0 {
                break;
            }
            local_8 = local_8.wrapping_add(1);
        }
        base.add(local_8.wrapping_mul(8) as usize)
    }
}
