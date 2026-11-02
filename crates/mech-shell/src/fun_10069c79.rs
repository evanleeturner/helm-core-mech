// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10069c79(param_1: i32, param_2: i32, param_3: *mut u8) {
    let mut bytes_left: i32 = 0x300;
    let mut src_cursor: *const u8 = param_1.wrapping_add(param_2).wrapping_sub(0x300) as *const u8;
    let mut dest: *mut u8 = param_3;
    loop {
        // SAFETY: the caller supplies a source buffer with at least 0x300
        // bytes ending at param_1 + param_2, and a destination of 0x300 bytes.
        unsafe {
            *dest = *src_cursor >> 2;
            src_cursor = src_cursor.add(1);
            dest = dest.add(1);
        }
        bytes_left = bytes_left.wrapping_sub(1);
        if bytes_left == 0 {
            break;
        }
    }
}
