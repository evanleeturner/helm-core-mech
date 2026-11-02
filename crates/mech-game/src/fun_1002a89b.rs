// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10083b74(param_1: i32, param_2: i32, param_3: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002a89b(param_1: i32, param_2: i16) {
    // SAFETY: param_1 is the address of the game's own object; fields are read
    // and written at the original's offsets, and the callee is the game's own.
    unsafe {
        ((param_1.wrapping_add(0xdc)) as *mut i32).write_unaligned(param_2 as i32);
        FUN_10083b74(param_1, 1, 1);
        let a = ((param_1.wrapping_add(0x1a2)) as *const i32).read_unaligned();
        let b = ((param_1.wrapping_add(0xdc)) as *const i32).read_unaligned();
        if a == b {
            FUN_10083b74(param_1, 1, 1);
        }
        let v = ((param_1.wrapping_add(0xdc)) as *const u32).read_unaligned();
        ((param_1.wrapping_add(0x178)) as *mut i16).write_unaligned(v as i16);
    }
}
