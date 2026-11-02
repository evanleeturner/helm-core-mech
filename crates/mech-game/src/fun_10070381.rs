// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b4438: u8;
    static mut DAT_100c7fd8: u32;
    static mut DAT_100c7fdc: u32;
    static mut DAT_100eedb0: u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn MapVirtualKeyA(param_1: u32, param_2: u32) -> u32;
    fn GetKeyState(param_1: i32) -> i16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _tolower(_c: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10070381(code: u32) {
    mech_trace::enter!("10070381", code);
    // SAFETY: the globals and callees are the game's own; the table
    // addresses are read and written where the original does.
    unsafe {
        let mut local_8: u16 = mech_trace::rd!(
            "10070381",
            (&raw mut DAT_100b4438)
                .byte_add(code.wrapping_mul(2) as usize)
                .cast::<u16>(),
            read_unaligned
        );
        if local_8 == 0 {
            let u_var2 = mech_trace::cr!("10070381", "MapVirtualKeyA", MapVirtualKeyA(code, 2));
            let i_var3 = mech_trace::cr!(
                "10070381",
                "_tolower",
                _tolower((u_var2 as i8) as i16 as i32)
            );
            local_8 = i_var3 as u16;
        }
        if local_8 != 0 {
            let mut u_var1 = mech_trace::cr!("10070381", "GetKeyState", GetKeyState(0x11)) as u16;
            if (u_var1 & 0x1000) != 0 {
                local_8 |= 0x100;
            }
            u_var1 = mech_trace::cr!("10070381", "GetKeyState", GetKeyState(0x12)) as u16;
            if (u_var1 & 0x1000) != 0 {
                local_8 |= 0x400;
            }
            u_var1 = mech_trace::cr!("10070381", "GetKeyState", GetKeyState(0x10)) as u16;
            if (u_var1 & 0x1000) != 0 {
                local_8 |= 0x200;
            }
            mech_trace::wr!(
                "10070381",
                (&raw mut DAT_100eedb0)
                    .byte_add(
                        mech_trace::gr!("10070381", "100c7fdc", DAT_100c7fdc).wrapping_mul(2)
                            as usize
                    )
                    .cast::<u16>(),
                write_unaligned,
                local_8
            );
            mech_trace::gw!(
                "10070381",
                "100c7fdc",
                DAT_100c7fdc,
                mech_trace::gr!("10070381", "100c7fdc", DAT_100c7fdc).wrapping_add(1)
            );
            if mech_trace::gr!("10070381", "100c7fdc", DAT_100c7fdc) == 0x40 {
                mech_trace::gw!("10070381", "100c7fdc", DAT_100c7fdc, 0);
            } else if mech_trace::gr!("10070381", "100c7fd8", DAT_100c7fd8)
                == mech_trace::gr!("10070381", "100c7fdc", DAT_100c7fdc)
            {
                mech_trace::gw!(
                    "10070381",
                    "100c7fd8",
                    DAT_100c7fd8,
                    mech_trace::gr!("10070381", "100c7fd8", DAT_100c7fd8).wrapping_add(1)
                );
                if mech_trace::gr!("10070381", "100c7fd8", DAT_100c7fd8) == 0x40 {
                    mech_trace::gw!("10070381", "100c7fd8", DAT_100c7fd8, 0);
                }
            }
        }
    }
}
