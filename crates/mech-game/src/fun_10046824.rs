// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b04c4: u32;
    static mut DAT_100c7cf0: u32;
    static mut DAT_100c7cf4: u32;
    static mut DAT_100c7cf8: u32;
    static mut DAT_100c7cfc: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10046563() -> u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10046f3b(param_1: *mut u32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10046824(param_1: *mut u32) {
    mech_trace::enter!("10046824", param_1);
    // SAFETY: the globals and callees are the game's own; param_1 points at
    // the caller's four-dword buffer, written where the original does.
    unsafe {
        if mech_trace::gr!("10046824", "100b04c4", DAT_100b04c4) != 0 {
            let u_var1 = mech_trace::cr!("10046824", "10046563", FUN_10046563());
            match u_var1 {
                1 | 2 => {
                    mech_trace::wr!("10046824", param_1, write_unaligned, 0);
                    mech_trace::wr!("10046824", param_1.add(1), write_unaligned, 0);
                    mech_trace::wr!("10046824", param_1.add(2), write_unaligned, 0);
                    mech_trace::wr!("10046824", param_1.add(3), write_unaligned, 0);
                }
                3 => {
                    mech_trace::cr!("10046824", "10046f3b", FUN_10046f3b(param_1));
                }
                4 => {
                    mech_trace::wr!(
                        "10046824",
                        param_1,
                        write_unaligned,
                        mech_trace::gr!("10046824", "100c7cf0", DAT_100c7cf0)
                    );
                    mech_trace::wr!(
                        "10046824",
                        param_1.add(1),
                        write_unaligned,
                        mech_trace::gr!("10046824", "100c7cf4", DAT_100c7cf4)
                    );
                    mech_trace::wr!(
                        "10046824",
                        param_1.add(2),
                        write_unaligned,
                        mech_trace::gr!("10046824", "100c7cf8", DAT_100c7cf8)
                    );
                    mech_trace::wr!(
                        "10046824",
                        param_1.add(3),
                        write_unaligned,
                        mech_trace::gr!("10046824", "100c7cfc", DAT_100c7cfc)
                    );
                }
                _ => {}
            }
        }
    }
}
