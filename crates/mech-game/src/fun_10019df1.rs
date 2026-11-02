// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_101eb480: u32;
    static mut DAT_101eb484: u32;
    static mut DAT_101f52e0: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10056489(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
        param_7: i32,
    ) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10019df1(param_1: i32, param_2: i32, param_3: i32) {
    // SAFETY: the callee and the globals are the game's own memory; the
    // array's address is passed through and the globals are read then written
    // in the original's order.
    unsafe {
        FUN_10056489(
            (&raw mut DAT_101f52e0).cast::<i32>(),
            param_1,
            param_2,
            DAT_101eb480 as i32,
            DAT_101eb484 as i32,
            0,
            param_3,
        );
        DAT_101eb480 = param_1 as u32;
        DAT_101eb484 = param_2 as u32;
    }
}
