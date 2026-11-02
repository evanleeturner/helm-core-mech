// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004b0b0(param_1: *mut i8, param_2: *mut i8) {
    let mut dst = param_1;
    let mut src = param_2;
    let mut b_var2 = false;
    // SAFETY: both pointers refer to the caller's buffers; the source is
    // NUL-terminated and the destination is large enough, as in the original.
    unsafe {
        if *src != 0 {
            loop {
                if *src < 0x21 {
                    if !b_var2 {
                        *dst = 0x20;
                        dst = dst.add(1);
                        b_var2 = true;
                    }
                } else {
                    *dst = *src;
                    b_var2 = false;
                    dst = dst.add(1);
                }
                let next = src.add(1);
                src = next;
                if *next == 0 {
                    break;
                }
            }
            *dst = 0;
            return;
        }
        *dst = 0;
    }
}
