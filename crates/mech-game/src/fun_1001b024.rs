// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ab318: i32;
    static mut DAT_101eb454: i32;
    static mut DAT_100ba9ac: i32;
    static mut DAT_101eb3a4: i32;
    static mut DAT_100ba9a8: i32;
    static mut DAT_100ab090: i32;
    static mut DAT_101eb450: i32;
    static mut DAT_101eb3a0: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1001a8a3(
        param_1: *mut i32,
        param_2: *mut i32,
        param_3: *mut i32,
        param_4: *mut i32,
        param_5: *mut i32,
        param_6: *mut i32,
    );
    fn FUN_10067af1(param_1: *mut i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001b024(
    param_1: *mut i32,
    param_2: *mut i32,
    param_3: *mut i32,
    param_4: *mut i32,
    param_5: *mut i32,
    param_6: *mut i32,
) {
    // SAFETY: the callees and globals are the game's own; the pointer
    // parameters are the caller's and are read and written where the
    // original does.
    unsafe {
        FUN_1001a8a3(param_1, param_2, param_3, param_4, param_5, param_6);
        if DAT_100ab318 == 0 {
            DAT_101eb454 = DAT_100ba9ac;
            let i_var1 = FUN_10067af1(&raw mut DAT_101eb450);
            DAT_101eb3a4 = DAT_100ba9a8;
            let i_var2 = FUN_10067af1(&raw mut DAT_101eb3a0);
            param_2.write_unaligned(param_2.read_unaligned().wrapping_add(i_var1));
            param_1.write_unaligned(param_1.read_unaligned().wrapping_add(i_var2));
        }
        DAT_100ab090 = 1;
    }
}
