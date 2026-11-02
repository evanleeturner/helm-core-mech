// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100add18: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10036726(param_1: i32) -> i32;
    fn FUN_10036440(param_1: i32, param_2: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10036932() {
    // SAFETY: the globals and callees are the game's own; DAT_100add18 holds
    // a pointer to a structure read at offsets 0x1c and 0x42.
    unsafe {
        let i_var1 = FUN_10036726(
            ((DAT_100add18 as usize).wrapping_add(0x1c) as *const i32).read_unaligned(),
        );
        if -1 < i_var1 {
            let local_arg2 =
                ((DAT_100add18 as usize).wrapping_add(0x42) as *const i32).read_unaligned();
            FUN_10036440(i_var1, local_arg2);
        }
    }
}
