// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3c4c: i32;
    static mut DAT_100b3c50: i32;
    static mut DAT_100b3c54: i32;
    static mut DAT_100b3c48: i32;
    static mut DAT_100b3c58: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1006ab7b();
    fn FUN_10046fb4() -> i32;
    fn FUN_1006aa37();
    fn FUN_1006abdb();
    fn FUN_100470be();
    fn FUN_1003c7ea();
    fn FUN_10007c2b();
    fn FUN_100035a8();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1006a620(param_1: i32, param_2: i32) {
    mech_trace::enter!("1006a620", param_1, param_2);
    let mut local_8: Option<unsafe extern "C" fn()> = None;
    if -1 < param_1 && param_1 < 5 {
        // SAFETY: the callees and globals are the game's own; accessed as the original does.
        unsafe {
            if param_1 != 2 {
                FUN_1006ab7b();
            }
            let i_var2 = mech_trace::gr!("1006a620", "100b3c54", DAT_100b3c54);
            let i_var1 = mech_trace::gr!("1006a620", "100b3c50", DAT_100b3c50);
            let i_var3 = mech_trace::gr!("1006a620", "100b3c4c", DAT_100b3c4c);
            match param_1 {
                0 => {
                    mech_trace::gw!("1006a620", "100b3c4c", DAT_100b3c4c, param_2);
                    if param_2 != i_var3 {
                        local_8 = Some(FUN_100035a8);
                    }
                }
                1 => {
                    mech_trace::gw!("1006a620", "100b3c50", DAT_100b3c50, param_2);
                    if param_2 == 0 {
                        mech_trace::gw!(
                            "1006a620",
                            "100b3c58",
                            DAT_100b3c58,
                            ((mech_trace::gr!("1006a620", "100b3c58", DAT_100b3c58) as u32)
                                & 0xfffffffd) as i32
                        );
                    } else {
                        mech_trace::gw!(
                            "1006a620",
                            "100b3c58",
                            DAT_100b3c58,
                            ((mech_trace::gr!("1006a620", "100b3c58", DAT_100b3c58) as u32) | 2)
                                as i32
                        );
                    }
                    if param_2 != i_var1 {
                        local_8 = Some(FUN_10007c2b);
                    }
                }
                2 => {
                    mech_trace::gw!("1006a620", "100b3c54", DAT_100b3c54, param_2);
                    if param_2 != i_var2 {
                        let r = mech_trace::cr!("1006a620", "10046fb4", FUN_10046fb4());
                        if r == 0 {
                            local_8 = Some(FUN_1003c7ea);
                        } else {
                            local_8 = Some(FUN_100470be);
                        }
                        if i_var2 == 0 {
                            FUN_1006aa37();
                        }
                    }
                    FUN_1006abdb();
                }
                3 => {
                    mech_trace::gw!("1006a620", "100b3c48", DAT_100b3c48, param_2);
                    local_8 = None;
                }
                _ => {}
            }
            if let Some(f) = local_8 {
                mech_trace::cr!("1006a620", "ind", f());
            }
        }
    }
}
