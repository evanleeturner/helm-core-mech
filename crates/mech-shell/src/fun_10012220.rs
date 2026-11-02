// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a6724: *mut i8;
    static mut DAT_1008eb6c: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1005fb90();
    fn FUN_1000f360();
}

#[allow(non_snake_case)]
unsafe extern "fastcall" {
    fn FUN_10010790(param_1: *mut i8);
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1006ec30(param_1: *mut u8);
    fn FUN_100640a0(param_1: *const i8);
    fn FUN_1006fde0(param_1: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10012220(param_1: i32) {
    mech_trace::enter!("10012220", param_1);
    // SAFETY: the callees and globals are the game's own memory; the
    // calls and reads happen in the original's order.
    unsafe {
        FUN_1005fb90();
        let pc_var1 = mech_trace::gr!("10012220", "100a6724", DAT_100a6724);
        if !mech_trace::gr!("10012220", "100a6724", DAT_100a6724).is_null() {
            FUN_10010790(mech_trace::gr!("10012220", "100a6724", DAT_100a6724));
            FUN_1006ec30(pc_var1.cast::<u8>());
        }
        FUN_1000f360();
        FUN_100640a0((&raw mut DAT_1008eb6c).cast::<i8>());
        FUN_1006fde0(param_1.wrapping_add(1) as u32);
    }
}
