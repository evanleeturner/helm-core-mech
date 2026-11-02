// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a678c: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000ad30(param_1: i32, param_2: i32) -> i32;
    fn FUN_1006b230(param_1: i32, param_2: i32, param_3: i8) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000d620(param_1: i32) -> bool {
    // SAFETY: the callees and the global are the game's own; param_1 is an
    // address of a game structure whose field at 0x810 is read.
    unsafe {
        let key = ((param_1 as u32).wrapping_add(0x810) as *const i32).read_unaligned();
        let i_var1 = FUN_1000ad30(DAT_100a678c, key);
        let i_var1 = FUN_1006b230(param_1, i_var1, 1);
        i_var1 != 0
    }
}
