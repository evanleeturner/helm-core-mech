// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001d32b(param_1: i32, param_2: *mut i32, param_3: *mut i32) {
    // SAFETY: param_1 is the address of the game's object; its field at 0x20
    // points at a record whose shorts are read; the output pointers are
    // written only when non-null.
    unsafe {
        let i_var1 = ((param_1 as usize).wrapping_add(0x20) as *const i32).read_unaligned();
        if i_var1 != 0 {
            if !param_2.is_null() {
                param_2.write_unaligned(i32::from(
                    ((i_var1 as usize).wrapping_add(4) as *const i16).read_unaligned(),
                ));
            }
            if !param_3.is_null() {
                param_3.write_unaligned(i32::from(
                    ((i_var1 as usize).wrapping_add(6) as *const i16).read_unaligned(),
                ));
            }
        }
    }
}
