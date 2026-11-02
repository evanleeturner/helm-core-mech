// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10094370: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn sprintf(dest: *mut i8, format: *const i8, ...) -> i32;
    fn FUN_1006f780(param_1: *const core::ffi::c_void) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10037d40() {
    let mut local_20: [i8; 32] = [0; 32];
    let mut i_var1: i32 = 0;
    loop {
        let i_var2 = i_var1.wrapping_add(1);
        // SAFETY: the callees and the format string are the game's own;
        // local_20 is a live 32-byte buffer for the formatted name.
        unsafe {
            sprintf(
                local_20.as_mut_ptr(),
                (&raw mut DAT_10094370).cast::<i8>() as *const i8,
                i_var1,
            );
            FUN_1006f780(local_20.as_ptr().cast());
        }
        i_var1 = i_var2;
        if i_var2 >= 10 {
            break;
        }
    }
}
