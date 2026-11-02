// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1003e369(param_1: i32) -> u8;
    fn FUN_1003e3e3(param_1: i32, param_2: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003dfde(param_1: *mut u16) {
    if param_1.is_null() {
        return;
    }
    // SAFETY: param_1 is non-null and points at the caller's structure; the
    // callees are the game's own functions.
    unsafe {
        if (param_1.read_unaligned() & 0x1000) != 0 {
            return;
        }
        param_1.write_unaligned(param_1.read_unaligned() | 0x1000);
        if param_1.add(2).cast::<i32>().read_unaligned() != 0 {
            FUN_1003e369(param_1 as i32);
            FUN_1003e3e3(param_1 as i32, 0x100c6d30);
        }
    }
}
