// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100af17c: u32;
    static mut DAT_101686f0: u32;
    static mut DAT_101f52e0: [u8; 0];
    static mut PTR_DAT_100af0b4: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1008668b(param_1: i32, param_2: u32, param_3: *mut i32) -> *mut u16;
    fn FUN_10056fc4(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
    ) -> u32;
    fn FUN_1008664f(param_1: u32, param_2: *mut i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003510c(param_1: i32, param_2: i32, param_3: i32) {
    // SAFETY: the globals and callees are the game's own; the pointers passed
    // are the game's values, used exactly as the original does.
    unsafe {
        let p_var1 = FUN_1008668b(
            DAT_100af17c as i32,
            (param_3 as u32).wrapping_add(DAT_101686f0),
            PTR_DAT_100af0b4 as *mut i32,
        );
        if !p_var1.is_null() {
            FUN_10056fc4(
                (&raw mut DAT_101f52e0).cast::<i32>(),
                p_var1 as i32,
                0,
                param_1,
                param_2,
            );
            FUN_1008664f(
                (param_3 as u32).wrapping_add(DAT_101686f0),
                PTR_DAT_100af0b4 as *mut i32,
            );
        }
    }
}
