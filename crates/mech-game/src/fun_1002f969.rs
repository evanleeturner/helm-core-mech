// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002f969(param_1: i32, param_2: u32) {
    // SAFETY: param_1 points at the game's node structure; its fields at
    // 0x4, 0x6c and each child's 0x8 link are read and the short at +0x14
    // of the 0x6c record is written, as in the original.
    unsafe {
        let p = param_1 as usize as *const u8;
        let rec = p.byte_add(0x6c).cast::<i32>().read_unaligned();
        if rec != 0 {
            (rec as usize as *mut u8)
                .byte_add(0x14)
                .cast::<i16>()
                .write_unaligned(param_2 as i16);
        }
        let mut local_8 = p.byte_add(4).cast::<i32>().read_unaligned();
        while local_8 != 0 {
            FUN_1002f969(local_8, param_2);
            local_8 = (local_8 as usize as *const u8)
                .byte_add(8)
                .cast::<i32>()
                .read_unaligned();
        }
    }
}
