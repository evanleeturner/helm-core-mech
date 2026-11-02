// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3c48: u32;
    static mut DAT_100b3c4c: u32;
    static mut DAT_100b3c50: u32;
    static mut DAT_100b3c54: u32;
    static mut DAT_100b3c58: u32;
    static mut DAT_100aa1ac: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1006abdb();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1006a540(param_1: i32) -> u32 {
    mech_trace::enter!("1006a540", param_1);
    let mut local_8: u32 = 0;
    if param_1 > -1 {
        // SAFETY: the globals and the callee are the game's own memory and code.
        unsafe {
            match param_1 {
                0 => {
                    local_8 = mech_trace::gr!("1006a540", "100b3c4c", DAT_100b3c4c);
                }
                1 => {
                    local_8 = mech_trace::gr!("1006a540", "100b3c50", DAT_100b3c50);
                }
                2 => {
                    local_8 = mech_trace::gr!("1006a540", "100b3c54", DAT_100b3c54);
                    mech_trace::gu!("1006a540", "100b3c58", DAT_100b3c58, |=, 8);
                    if mech_trace::gr!("1006a540", "100aa1ac", DAT_100aa1ac) == 0 {
                        FUN_1006abdb();
                    }
                }
                3 => {
                    local_8 = mech_trace::gr!("1006a540", "100b3c48", DAT_100b3c48);
                }
                4 => {
                    local_8 =
                        ((mech_trace::gr!("1006a540", "100b3c58", DAT_100b3c58) & 2) != 0) as u32;
                }
                _ => {}
            }
        }
    }
    mech_trace::rt!("1006a540", local_8)
}
