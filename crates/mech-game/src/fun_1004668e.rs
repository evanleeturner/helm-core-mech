// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b04c0: *mut u32;
    static mut DAT_100b04c4: u32;
    static mut DAT_100c7cf0: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100463ee(param_1: u32, param_2: i32);
    fn FUN_10046824(param_1: *mut u32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10046563() -> u32;
    fn FUN_10046370();
    fn FUN_1004639a();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1004668e() {
    mech_trace::enter!("1004668e");
    // SAFETY: the globals and callees are the game's own; the pointer in
    // DAT_100b04c0 is read only in the case the original reads it.
    unsafe {
        if mech_trace::gr!("1004668e", "100b04c4", DAT_100b04c4) != 0 {
            let u_var1 = mech_trace::cr!("1004668e", "10046563", FUN_10046563());
            match u_var1 {
                1 => {}
                2 => {
                    FUN_100463ee(
                        mech_trace::rd!(
                            "1004668e",
                            mech_trace::gr!("1004668e", "100b04c0", DAT_100b04c0),
                            read_unaligned
                        ),
                        0,
                    );
                }
                3 => {
                    FUN_10046824((&raw mut DAT_100c7cf0).cast::<u32>());
                    FUN_10046370();
                }
                4 => {
                    FUN_1004639a();
                }
                _ => {}
            }
        }
    }
}
