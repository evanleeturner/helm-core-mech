// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10160ee8: i32;
    static mut DAT_10160eb8: i32;
    static mut DAT_10160ec4: i32;
    static mut DAT_10160eb4: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10055b60(a: i32, b: i32, c: i32, d: u8) -> u32;
    fn FUN_10055b90(a: i32, b: i32, c: i32, d: i32, e: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005558c(
    mut param_1: i32,
    param_2: i32,
    mut param_3: u32,
    param_4: i32,
    mut param_5: i32,
    param_6: i32,
) -> i32 {
    let mut local_14: i32 = (param_2 as u32).wrapping_sub(param_1 as u32) as i32;
    let mut local_8: i32 = (param_6 as u32).wrapping_sub(param_5 as u32) as i32;
    if local_8 < 0 {
        local_14 = 0u32.wrapping_sub(local_14 as u32) as i32;
        local_8 = 0u32.wrapping_sub(local_8 as u32) as i32;
        param_1 = param_2;
        param_5 = param_6;
    }
    // SAFETY: the callees and globals are the game's own; arguments are plain values.
    unsafe {
        if param_4 == 0 {
            param_3 = FUN_10055b60(DAT_10160ee8, param_3 as i32, local_8, DAT_10160eb8 as u8);
        } else {
            param_3 = FUN_10055b60(param_3 as i32, DAT_10160ec4, local_8, DAT_10160eb4 as u8);
        }
        let diff = param_3.wrapping_sub(local_14 as u32);
        if diff == 0 {
            param_5
        } else {
            FUN_10055b90(param_1, param_5, local_14, local_8, diff as i32)
        }
    }
}
