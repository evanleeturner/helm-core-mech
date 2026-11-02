// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001d32b(param_1: i32, param_2: *mut i32, param_3: *mut i32) {
    mech_trace::enter!("1001d32b", param_1, param_2, param_3);
    // SAFETY: param_1 is the address of the game's object; its field at 0x20
    // points at a record whose shorts are read; the output pointers are
    // written only when non-null.
    unsafe {
        let i_var1 = mech_trace::rd!(
            "1001d32b",
            ((param_1 as usize).wrapping_add(0x20) as *const i32),
            read_unaligned
        );
        if i_var1 != 0 {
            if !param_2.is_null() {
                mech_trace::wr!(
                    "1001d32b",
                    param_2,
                    write_unaligned,
                    i32::from(mech_trace::rd!(
                        "1001d32b",
                        ((i_var1 as usize).wrapping_add(4) as *const i16),
                        read_unaligned
                    ),)
                );
            }
            if !param_3.is_null() {
                mech_trace::wr!(
                    "1001d32b",
                    param_3,
                    write_unaligned,
                    i32::from(mech_trace::rd!(
                        "1001d32b",
                        ((i_var1 as usize).wrapping_add(6) as *const i16),
                        read_unaligned
                    ),)
                );
            }
        }
    }
}
