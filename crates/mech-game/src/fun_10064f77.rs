// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38ec: *mut u8;
    static mut DAT_100c7e70: u32;
    static mut DAT_100c7e74: u32;
    static mut DAT_100c7e78: u32;
    static mut DAT_100c7e7c: u32;
    static mut DAT_100c7e80: u32;
    static mut DAT_100c7f88: u32;
    static mut DAT_100c7f8c: u32;
    static mut DAT_100c7f90: u32;
    static mut DAT_100c7f94: u32;
    static mut DAT_100c7fa4: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10064cd8(param_1: *mut u8, param_2: u32) -> u32;
    fn FUN_10065891(param_1: *mut u8) -> u32;
    fn FUN_10064d15(param_1: u32) -> i32;
    fn FUN_10064e6a(param_1: u8) -> u32;
    fn FUN_10065126(param_1: i32, param_2: *mut u8) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10065815() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10064f77(
    mut param_1: i32,
    mut param_2: i32,
    mut param_3: i32,
    mut param_4: i32,
    param_5: u32,
) -> i32 {
    mech_trace::enter!("10064f77", param_1, param_2, param_3, param_4, param_5);
    // SAFETY: globals and callees are the game's own; the local buffer is
    // 18 bytes and every write stays inside it.
    unsafe {
        if param_3 < 0 {
            param_3 = mech_trace::gr!("10064f77", "100c7e70", DAT_100c7e70) as i32;
            param_1 = 0;
        }
        if param_4 < 0 {
            param_4 = mech_trace::gr!("10064f77", "100c7e74", DAT_100c7e74) as i32;
            param_2 = 0;
        }
        if param_1 < 0 {
            param_1 = 0;
        }
        if param_2 < 0 {
            param_2 = 0;
        }
        let local_c: i32;
        if mech_trace::gr!("10064f77", "100c7fa4", DAT_100c7fa4) == 0
            || mech_trace::cr!(
                "10064f77",
                "10064cd8",
                FUN_10064cd8(
                    mech_trace::gr!("10064f77", "100b38ec", DAT_100b38ec),
                    mech_trace::gr!("10064f77", "100c7fa4", DAT_100c7fa4).wrapping_mul(3)
                )
            ) == 0
        {
            let mut buf = [0u8; 18];
            let p = buf.as_mut_ptr();
            buf[0] = 0x2c;
            mech_trace::gw!("10064f77", "100c7f90", DAT_100c7f90, param_1 as u32);
            mech_trace::wr!(
                "10064f77",
                p.byte_add(1).cast::<i32>(),
                write_unaligned,
                param_1
            );
            mech_trace::gw!("10064f77", "100c7e78", DAT_100c7e78, param_2 as u32);
            mech_trace::wr!(
                "10064f77",
                p.byte_add(5).cast::<i32>(),
                write_unaligned,
                param_2
            );
            mech_trace::gw!("10064f77", "100c7f94", DAT_100c7f94, param_3 as u32);
            mech_trace::wr!(
                "10064f77",
                p.byte_add(9).cast::<i32>(),
                write_unaligned,
                param_3
            );
            mech_trace::gw!("10064f77", "100c7f88", DAT_100c7f88, param_4 as u32);
            mech_trace::wr!(
                "10064f77",
                p.byte_add(13).cast::<i32>(),
                write_unaligned,
                param_4
            );
            buf[17] = 0;
            if mech_trace::cr!("10064f77", "10065891", FUN_10065891(p)) == 0 {
                let mut local_8 = mech_trace::cr!(
                    "10064f77",
                    "10064d15",
                    FUN_10064d15(mech_trace::gr!("10064f77", "100c7fa4", DAT_100c7fa4))
                );
                if local_8 == 1 {
                    local_8 = 2;
                }
                if mech_trace::cr!("10064f77", "10064e6a", FUN_10064e6a(local_8 as u8)) == 0 {
                    mech_trace::gw!("10064f77", "100c7e7c", DAT_100c7e7c, 0);
                    mech_trace::gw!("10064f77", "100c7e80", DAT_100c7e80, 0);
                    mech_trace::gw!("10064f77", "100c7f8c", DAT_100c7f8c, param_5);
                    let r = mech_trace::cr!(
                        "10064f77",
                        "10065126",
                        FUN_10065126(local_8, FUN_10065815 as usize as *mut u8)
                    );
                    if r == 0 {
                        if mech_trace::cr!("10064f77", "10064e6a", FUN_10064e6a(0)) == 0 {
                            local_c = 0;
                        } else {
                            local_c = 2;
                        }
                    } else {
                        local_c = r;
                    }
                } else {
                    local_c = 2;
                }
            } else {
                local_c = 2;
            }
        } else {
            local_c = 2;
        }
        mech_trace::rt!("10064f77", local_c)
    }
}
