// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10025a50(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
    ) -> i32;
    fn FUN_1005ffd8(param_1: i32, param_2: i32, param_3: i32, param_4: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100600c7(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
) -> u32 {
    // SAFETY: param_1 points at the game's object; its fields at 0x4c and 0x1c
    // are read where the original does, and the callees are the game's own.
    unsafe {
        let p = (param_1.wrapping_add(0x4c) as *const i32).read_unaligned();
        if p == 0 {
            FUN_1005ffd8(param_1, param_2, param_3, param_4)
        } else {
            let f = (param_1.wrapping_add(0x1c) as *const i32).read_unaligned();
            let r = FUN_10025a50(p as *mut i32, f, param_2, param_3, param_4);
            (r as u32) & 1
        }
    }
}
