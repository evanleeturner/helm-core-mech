// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002b120(param_1: i32);
    fn FUN_1002b855(param_1: i32, param_2: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002b7de(param_1: i32, param_2: *mut i16) {
    // SAFETY: param_2 is the caller's pointer, read only where the original reads it;
    // the callees are the game's own.
    unsafe {
        if param_2.is_null() || param_2.read_unaligned() == 0 {
            FUN_1002b120(param_1);
        } else if param_2.read_unaligned() == 7 {
            let addr = param_2 as usize as u32;
            let hi = u32::from((addr >> 16) as u16) << 16;
            let lo = u32::from(param_2.add(1).read_unaligned() as u16);
            FUN_1002b855(param_1, (hi | lo) as i32);
        }
    }
}
