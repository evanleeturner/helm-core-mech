// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10082bf0: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10050400(param_1: *mut i32) {
    if param_1.is_null() {
        return;
    }
    // SAFETY: param_1 is non-null and points at the caller's struct; the
    // table at DAT_10082bf0 is the game's own memory, read at the original's
    // offset.
    unsafe {
        let i_var1 = param_1.read_unaligned();
        param_1.byte_add(4).write_unaligned(10000);
        match i_var1 {
            1 | 2 | 5 | 6 => {
                let v = (&raw mut DAT_10082bf0)
                    .byte_offset(i_var1.wrapping_mul(0xc) as isize)
                    .cast::<i32>()
                    .read_unaligned();
                param_1.byte_add(4).write_unaligned(v);
            }
            3 | 4 => {
                param_1.byte_add(4).write_unaligned(50000);
            }
            _ => {}
        }
    }
}
