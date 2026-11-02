// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a678c: u32;
    static mut DAT_100a6798: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000ce60(param_1: i32) -> i32;
    fn FUN_1000ad30(param_1: i32, param_2: i32) -> i32;
    fn FUN_10017ec0(param_1: i32, param_2: i32, param_3: i8) -> u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1000ce20() -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1004d0a0() {
    // SAFETY: the callees and globals are the game's own; the pointers
    // derived from the returned integers are read and written at the
    // offsets the original uses.
    unsafe {
        let i_var1 = FUN_1000ce60(DAT_100a6798 as i32);
        let i_var2 = FUN_1000ad30(
            DAT_100a678c as i32,
            ((i_var1 as u32).wrapping_add(0x810) as *const i32).read_unaligned(),
        );
        FUN_10017ec0(DAT_100a6798 as i32, i_var1, 1);
        FUN_10017ec0(DAT_100a678c as i32, i_var2, 1);
        let i_var1 = FUN_1000ce20();
        ((i_var1 as u32).wrapping_add(0x20) as *mut u32).write_unaligned(0xffffffff);
    }
}
