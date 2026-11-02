// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10044bba(param_1: i32, param_2: i32) -> u32 {
    // SAFETY: param_1 points at a game object whose field at 0x20 points at
    // another game object; the fields are read at the original's offsets.
    unsafe {
        let i_var1 = (param_1 as *const u8)
            .byte_add(0x20)
            .cast::<i32>()
            .read_unaligned() as usize as *const u8;
        if i_var1.byte_add(0x3f4).cast::<i32>().read_unaligned() < 6
            || i_var1.byte_add(0x420).cast::<i32>().read_unaligned() == 0
            || param_2 <= (i_var1.byte_add(0x3cc).cast::<i32>().read_unaligned() >> 0x10)
        {
            0
        } else {
            1
        }
    }
}
