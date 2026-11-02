// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a67a4: *mut i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1000ce20() -> i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10017ec0(param_1: i32, param_2: i32, param_3: i8) -> u8;
    fn FUN_10017d70(param_1: *mut i32) -> u32;
    fn FUN_10017c90(
        param_1: *mut u32,
        param_2: i32,
        param_3: u32,
        param_4: u8,
        param_5: u32,
    ) -> u32;
    fn FUN_10017df0(param_1: *mut i32, param_2: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1004c220() {
    // SAFETY: the callees and the global are the game's own; i_var1 is a
    // pointer-sized handle the game returned, written at offset 0x20 as the
    // original does.
    unsafe {
        let i_var1 = FUN_1000ce20();
        if i_var1 != 0 {
            FUN_10017ec0(DAT_100a67a4 as i32, i_var1, 0);
            ((i_var1 as u32).wrapping_add(0x20) as *mut u32).write_unaligned(0xffffffff);
            FUN_10017d70(DAT_100a67a4);
            FUN_10017c90((&raw mut DAT_100a67a4).cast::<u32>(), 10, 0, 4, 0);
            FUN_10017df0(DAT_100a67a4, i_var1 as u32);
        }
    }
}
