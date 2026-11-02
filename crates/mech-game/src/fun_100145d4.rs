// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100af098: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10014642(param_1: i32, param_2: i32, param_3: i32, param_4: i32) -> i32;
    fn FUN_10014718(param_1: i32, param_2: u32, param_3: u32, param_4: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100145d4(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: *mut i32,
) -> u32 {
    // SAFETY: the callees and the global are the game's own; param_4 points
    // at a caller-provided i32 that is written and read back as the original does.
    unsafe {
        let i_var1 = FUN_10014642(DAT_100af098 as i32, param_1, param_2, param_3);
        param_4.write_unaligned(i_var1);
        if param_4.read_unaligned() != 0 {
            let r = FUN_10014718(
                param_4.read_unaligned(),
                param_1 as u32,
                param_2 as u32,
                param_3 as u32,
            );
            if r != 0 {
                return 1;
            }
        }
        0
    }
}
