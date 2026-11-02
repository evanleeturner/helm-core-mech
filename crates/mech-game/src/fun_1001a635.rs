// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7b60: u32;
    static mut DAT_100ab074: u32;
    static mut DAT_100ab078: u32;
    static mut PTR_DAT_100aaf68: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1001a620() -> u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10003742(param_1: u32, param_2: i32, param_3: u32, param_4: i32, param_5: u16) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001a635(param_1: i32) {
    mech_trace::enter!("1001a635", param_1);
    // SAFETY: the globals and callees are the game's own; PTR_DAT_100aaf68
    // holds the address of a structure whose field at offset 0x18 is read
    // and written where the original does.
    unsafe {
        let p = mech_trace::gr!("1001a635", "PTR_DAT_100aaf68", PTR_DAT_100aaf68).wrapping_add(0x18)
            as *mut i32;
        let i_var1 = mech_trace::rd!("1001a635", p, read_unaligned);
        if param_1 != 0 {
            mech_trace::gw!("1001a635", "100ab074", DAT_100ab074, 0x10000);
            mech_trace::gw!("1001a635", "100ab078", DAT_100ab078, 0x10000);
        }
        let i_var2 = mech_trace::cr!("1001a635", "1001a620", FUN_1001a620()) as i32;
        if i_var2 == 0 {
            let p = mech_trace::gr!("1001a635", "PTR_DAT_100aaf68", PTR_DAT_100aaf68)
                .wrapping_add(0x18) as *mut u32;
            mech_trace::wr!(
                "1001a635",
                p,
                write_unaligned,
                mech_trace::gr!("1001a635", "100ab074", DAT_100ab074)
            );
        } else {
            let p = mech_trace::gr!("1001a635", "PTR_DAT_100aaf68", PTR_DAT_100aaf68)
                .wrapping_add(0x18) as *mut u32;
            mech_trace::wr!(
                "1001a635",
                p,
                write_unaligned,
                mech_trace::gr!("1001a635", "100ab078", DAT_100ab078)
            );
        }
        let p = mech_trace::gr!("1001a635", "PTR_DAT_100aaf68", PTR_DAT_100aaf68).wrapping_add(0x18)
            as *const i32;
        if mech_trace::rd!("1001a635", p, read_unaligned) != i_var1 {
            mech_trace::gw!("1001a635", "100a7b60", DAT_100a7b60, 1);
            mech_trace::cr!(
                "1001a635",
                "10003742",
                FUN_10003742(0x15b, 100, 0x40, 5, 0x50)
            );
        }
    }
}
