// SPDX-License-Identifier: MIT

unsafe extern "C" {
    fn FUN_1002a4e0(param_1: i32, param_2: i16, param_3: u16, param_4: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002a3e0(param_1: i32, param_2: i16, param_3: u16) {
    // SAFETY: param_1 is the address of the game's own object; the fields are
    // read and written at the original's offsets, and the callee is the game's.
    unsafe {
        let idx = ((param_1.wrapping_add(0x16c)) as *const u16).read_unaligned();
        if (idx as i16) == 0 {
            FUN_1002a4e0(param_1, param_2, param_3, 0);
        } else {
            let local_8: u32 = ((param_3 as u32).wrapping_shl(16)) | (param_2 as u16 as u32);
            let addr = (param_1 as u32)
                .wrapping_add(0x154)
                .wrapping_add((idx as u32).wrapping_mul(4));
            (addr as *mut u32).write_unaligned(local_8);
        }
    }
}
