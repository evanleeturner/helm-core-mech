// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100640ee(param_1: i32, param_2: *mut u32, param_3: *mut u32, param_4: *mut u32);
    fn FUN_1002fc9d(param_1: i32) -> i32;
    fn FUN_1002f462(param_1: i32, param_2: *mut u32);
    fn FUN_1002f37a(param_1: i32, param_2: u32, param_3: u32, param_4: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10064114(param_1: i32, param_2: i32) {
    let mut local_10: u32 = 0;
    let mut local_c: u32 = 0;
    let mut local_8: u32 = 0;
    // SAFETY: the callees are the game's own; the locals outlive the calls and
    // param_1 points at a game object whose field at 0x48 is read.
    unsafe {
        FUN_100640ee(
            param_1,
            &raw mut local_8,
            &raw mut local_c,
            &raw mut local_10,
        );
        let p_var1 =
            FUN_1002fc9d(((param_1 as usize).wrapping_add(0x48) as *const i32).read_unaligned())
                as usize as *mut u32;
        FUN_1002f462(param_2, p_var1);
        FUN_1002f37a(param_2, local_8, local_c, local_10);
    }
}
