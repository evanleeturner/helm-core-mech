// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100af17c: u32;
    static mut DAT_101686f0: u32;
    static mut DAT_100af0b4: u32;
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
pub unsafe extern "C" fn FUN_1003517a(param_1: i32, param_2: i32, param_3: i32, param_4: *mut i32) {
    mech_trace::enter!("1003517a", param_1, param_2, param_3, param_4);
    // SAFETY: the globals and callees are the game's own; the pointer
    // returned by FUN_1008668b is only passed on, never dereferenced here.
    unsafe {
        let pu_var1 = mech_trace::cr!(
            "1003517a",
            "1008668b",
            FUN_1008668b(
                mech_trace::gr!("1003517a", "100af17c", DAT_100af17c) as i32,
                (param_3 as u32).wrapping_add(mech_trace::gr!(
                    "1003517a",
                    "101686f0",
                    DAT_101686f0
                )),
                mech_trace::gr!("1003517a", "100af0b4", DAT_100af0b4) as *mut i32,
            )
        );
        if !pu_var1.is_null() {
            mech_trace::cr!(
                "1003517a",
                "10056fc4",
                FUN_10056fc4(param_4, pu_var1 as i32, 0, param_1, param_2)
            );
            mech_trace::cr!(
                "1003517a",
                "1008664f",
                FUN_1008664f(
                    (param_3 as u32).wrapping_add(mech_trace::gr!(
                        "1003517a",
                        "101686f0",
                        DAT_101686f0
                    )),
                    mech_trace::gr!("1003517a", "100af0b4", DAT_100af0b4) as *mut i32,
                )
            );
        }
    }
}
