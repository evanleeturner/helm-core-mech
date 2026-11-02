// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100add18: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10036142(param_1: i32) -> i32;
    fn FUN_10007f49(param_1: i32, param_2: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10036649(param_1: i32) {
    mech_trace::enter!("10036649", param_1);
    let mut param_1 = param_1;
    // SAFETY: the callees and the global are the game's own memory and code.
    unsafe {
        let i_var1 = mech_trace::cr!("10036649", "10036142", FUN_10036142(1));
        let i_var2 = mech_trace::cr!("10036649", "10036142", FUN_10036142(2));
        let mut store = false;
        if param_1 == 0 {
            if i_var1 != 0 && i_var2 == 0 {
                param_1 = 1;
                store = true;
            } else if i_var1 == 0 && i_var2 != 0 {
                param_1 = 2;
                store = true;
            } else if i_var1 != 0 {
                store = true;
            }
        } else {
            if param_1 == 1 && i_var1 == 0 {
                return;
            }
            if param_1 != 2 {
                store = true;
            }
        }
        if !store && i_var2 == 0 {
            return;
        }
        mech_trace::gw!("10036649", "100add18", DAT_100add18, param_1 as u32);
        mech_trace::cr!("10036649", "10007f49", FUN_10007f49(param_1, 0));
    }
}
