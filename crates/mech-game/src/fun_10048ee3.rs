// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10048ee3(
    param_1: i32,
    param_2: u32,
    param_3: *mut u32,
    param_4: *mut u32,
) {
    if param_1 < 0xe {
        if param_1 == 0xd {
            // SAFETY: param_3 is the caller's valid out pointer, written where the original does.
            unsafe {
                match param_2 {
                    0 => param_3.write_unaligned(3),
                    2 => param_3.write_unaligned(4),
                    5 | 6 => param_3.write_unaligned(5),
                    _ => {}
                }
            }
            return;
        }
        if param_1 != 9 {
            return;
        }
        // SAFETY: param_4 is the caller's valid out pointer.
        unsafe {
            param_4.write_unaligned(1);
        }
    } else {
        if param_1 < 199 {
            if param_1 != 0xc6 {
                if param_1 != 0x1b {
                    return;
                }
                // SAFETY: param_3 is the caller's valid out pointer.
                unsafe {
                    param_3.write_unaligned(5);
                }
                return;
            }
        } else {
            if param_1 == 199 {
                // SAFETY: param_4 is the caller's valid out pointer.
                unsafe {
                    param_4.write_unaligned(1);
                }
                return;
            }
            if param_1 != 0x209 {
                return;
            }
        }
        // SAFETY: param_4 is the caller's valid out pointer.
        unsafe {
            param_4.write_unaligned(0xffff_ffff);
        }
    }
}
