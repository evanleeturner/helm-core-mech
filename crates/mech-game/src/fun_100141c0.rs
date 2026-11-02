// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1003df3a(param_1: *mut u16);
    fn FUN_1003ddbc(param_1: *mut u16);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100141c0(param_1: *mut u16, param_2: i32) {
    // SAFETY: param_1 points at the caller's structure; the field at byte
    // offset 0x24 is written, and the callees are the game's own.
    unsafe {
        param_1.add(0x12).cast::<i32>().write_unaligned(param_2);
        if param_2 == 4 {
            FUN_1003df3a(param_1);
        } else {
            FUN_1003ddbc(param_1);
        }
    }
}
