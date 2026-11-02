// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a4104: u32;
    static mut DAT_100aafa4: u32;
    static mut DAT_100adcd0: u32;
    static mut DAT_100b09f8: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000242a(param_1: u32, param_2: i32) -> u8;
    fn FUN_100082c4(param_1: i32, param_2: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10055da2(param_1: u32, param_2: i32) {
    mech_trace::enter!("10055da2", param_1, param_2);
    // SAFETY: the globals and callees are the game's own; values are
    // stored and passed exactly as the original does.
    unsafe {
        match param_1 {
            0x2a => {
                mech_trace::gw!("10055da2", "100adcd0", DAT_100adcd0, param_2 as u32);
            }
            0x53 => {
                mech_trace::gw!("10055da2", "100a4104", DAT_100a4104, param_2 as u32);
            }
            0x57 => {
                mech_trace::gw!("10055da2", "100b09f8", DAT_100b09f8, param_2 as u32);
            }
            0xbf => {
                mech_trace::cr!("10055da2", "1000242a", FUN_1000242a(0, param_2));
            }
            0xc0 => {
                if param_2 == 0 {
                    mech_trace::gw!("10055da2", "100aafa4", DAT_100aafa4, 0);
                } else {
                    mech_trace::gw!("10055da2", "100aafa4", DAT_100aafa4, 1);
                    mech_trace::cr!("10055da2", "100082c4", FUN_100082c4(0x1b, 1));
                }
            }
            _ => {}
        }
    }
}
