// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a40a0: u32;
    static mut DAT_100c4c88: u32;
    static mut DAT_10114db0: i32;
    static mut DAT_10218070: u32;
    static mut DAT_10218074: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100609fa(param_1: i32, param_2: u32) -> u8;
    fn FUN_10003742(param_1: u32, param_2: i32, param_3: u32, param_4: i32, param_5: u16) -> u8;
    fn FUN_100082c4(param_1: i32, param_2: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000242a(_param_1: u32, param_2: i32) {
    mech_trace::enter!("1000242a", _param_1, param_2);
    // SAFETY: the globals and callees are the game's own; DAT_10114db0 holds a
    // pointer to a structure whose short at offset 6 is read as the original does.
    unsafe {
        if mech_trace::gr!("1000242a", "10218074", DAT_10218074) != param_2 {
            if param_2 == 1 {
                let p = (mech_trace::gr!("1000242a", "10114db0", DAT_10114db0) as usize
                    as *const u8)
                    .byte_add(6)
                    .cast::<i16>();
                if mech_trace::rd!("1000242a", p, read_unaligned) < 1 {
                    mech_trace::gw!("1000242a", "10218074", DAT_10218074, 1);
                    mech_trace::gw!("1000242a", "100c4c88", DAT_100c4c88, 0);
                    mech_trace::cr!("1000242a", "100609fa", FUN_100609fa(0xc, 0xb5));
                    mech_trace::cr!(
                        "1000242a",
                        "10003742",
                        FUN_10003742(0x95, 100, 0x40, 5, 0x50)
                    );
                    mech_trace::cr!("1000242a", "100082c4", FUN_100082c4(0x1c, 1));
                }
            } else if param_2 == 0 {
                mech_trace::gw!("1000242a", "100c4c88", DAT_100c4c88, 1);
                mech_trace::gw!("1000242a", "100a40a0", DAT_100a40a0, 0xffffffff);
                mech_trace::gw!("1000242a", "10218070", DAT_10218070, 0);
            }
        }
    }
}
