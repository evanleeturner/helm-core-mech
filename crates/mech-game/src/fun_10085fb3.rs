// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c9054: i32;
    static mut DAT_100c9058: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10085fb3(param_1: i32) {
    // SAFETY: param_1 is the address of the caller's node structure and the
    // linked neighbours are read and written where the original does; the
    // globals are the game's own memory.
    unsafe {
        if ((param_1.wrapping_add(2)) as *const i16).read_unaligned() != 1 {
            ((param_1.wrapping_add(2)) as *mut i16).write_unaligned(1);
            let prev = ((param_1.wrapping_add(0xc)) as *const i32).read_unaligned();
            if prev != 0 {
                (prev.wrapping_add(0x10) as *mut i32)
                    .write_unaligned(((param_1.wrapping_add(0x10)) as *const i32).read_unaligned());
            }
            let next = ((param_1.wrapping_add(0x10)) as *const i32).read_unaligned();
            if next != 0 {
                (next.wrapping_add(0xc) as *mut i32)
                    .write_unaligned(((param_1.wrapping_add(0xc)) as *const i32).read_unaligned());
            }
            if param_1 == DAT_100c9054 {
                DAT_100c9054 = ((param_1.wrapping_add(0xc)) as *const i32).read_unaligned();
            }
            if param_1 == DAT_100c9058 {
                DAT_100c9058 = ((param_1.wrapping_add(0x10)) as *const i32).read_unaligned();
            }
            ((param_1.wrapping_add(0xc)) as *mut i32).write_unaligned(0);
            ((param_1.wrapping_add(0x10)) as *mut i32).write_unaligned(0);
        }
    }
}
