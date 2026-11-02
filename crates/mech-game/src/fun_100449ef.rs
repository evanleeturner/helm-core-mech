// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100449ef(param_1: i32, param_2: i16) {
    // SAFETY: param_1 points at the caller's object, whose field at 0x4c
    // points at a record; bytes 0x20 and 0x21 of it are written as the original does.
    unsafe {
        let p = (param_1 as *const u8)
            .byte_add(0x4c)
            .cast::<i32>()
            .read_unaligned() as *mut u8;
        p.byte_add(0x20).write(param_2 as u8);
        if param_2 == 0 {
            let p = (param_1 as *const u8)
                .byte_add(0x4c)
                .cast::<i32>()
                .read_unaligned() as *mut u8;
            p.byte_add(0x21).write(1);
        } else {
            let p = (param_1 as *const u8)
                .byte_add(0x4c)
                .cast::<i32>()
                .read_unaligned() as *mut u8;
            p.byte_add(0x21).write(0);
        }
    }
}
