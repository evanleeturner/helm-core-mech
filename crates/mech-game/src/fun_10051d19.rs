// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_101f52c8: i32;
    static mut DAT_100b09dc: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100823ef(param_1: i32, param_2: i32, param_3: i32);
    fn FUN_10064f07(param_1: i32, param_2: u32, param_3: u32, param_4: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10051d19() {
    mech_trace::enter!("10051d19");
    let mut local_10: [u8; 3] = [0; 3];
    // SAFETY: the globals and callees are the game's own; DAT_101f52c8 holds
    // a pointer to an object whose count field sits at +0xc, and local_10 is
    // a live local buffer passed by address as the original does.
    unsafe {
        let local_8: i32 = mech_trace::rd!(
            "10051d19",
            ((mech_trace::gr!("10051d19", "101f52c8", DAT_101f52c8) as u32).wrapping_add(0xc)
                as *const i32),
            read_unaligned
        );
        if mech_trace::gr!("10051d19", "100b09dc", DAT_100b09dc) == 0 {
            let mut local_c: i32 = 0;
            while local_c < local_8 {
                FUN_100823ef(local_c, 1, local_10.as_mut_ptr() as usize as i32);
                FUN_10064f07(
                    local_c,
                    u32::from(local_10[0]),
                    u32::from(local_10[1]),
                    u32::from(local_10[2]),
                );
                local_c = local_c.wrapping_add(1);
            }
            mech_trace::gw!("10051d19", "100b09dc", DAT_100b09dc, 1);
        }
    }
}
