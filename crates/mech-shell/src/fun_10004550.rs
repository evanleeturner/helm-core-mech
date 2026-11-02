// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10083710: i32;
    static mut DAT_10083714: i32;
    static mut DAT_10083718: i32;
    static mut DAT_10083724: i32;
    static mut DAT_10083730: i32;
    static mut DAT_10083734: i32;
    static mut DAT_1008373c: i32;
    static mut DAT_10083748: i32;
    static mut DAT_10083750: i32;
    static mut DAT_10083754: i32;
    static mut DAT_10083760: i32;
    static mut DAT_10083770: i32;
    static mut DAT_10083778: i32;
    static mut DAT_10083780: i32;
    static mut DAT_10083788: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn __ftol() -> i64;
    fn FUN_1000a120(param_1: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10004550() {
    // SAFETY: the callees and globals are the game's own memory; the divisor
    // is always 10 or 20, so the division cannot fault or overflow.
    unsafe {
        let l_var1 = __ftol();
        DAT_10083730 = FUN_1000a120(l_var1 as i32);
        DAT_10083734 = 300;
        let divisor: i32 = if DAT_10083750 == 0 { 10 } else { 20 };
        DAT_10083754 = DAT_10083710 / divisor;
        DAT_10083714 = DAT_10083748
            .wrapping_add(DAT_1008373c)
            .wrapping_add(DAT_10083780)
            .wrapping_add(DAT_10083770)
            .wrapping_add(DAT_10083730)
            .wrapping_add(DAT_10083754)
            .wrapping_add(DAT_10083788)
            .wrapping_add(DAT_10083778)
            .wrapping_add(DAT_10083724)
            .wrapping_add(DAT_10083760)
            .wrapping_add(300);
        DAT_10083718 = DAT_10083710.wrapping_sub(DAT_10083714);
    }
}
