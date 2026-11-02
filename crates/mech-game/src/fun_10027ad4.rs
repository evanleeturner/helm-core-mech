// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100293c6(param_1: i32) -> u32;
    fn FUN_10028bde(
        param_1: i32,
        param_2: u16,
        param_3: i16,
        param_4: *mut u16,
        param_5: *mut u16,
    ) -> i32;
    fn FUN_1002a4e0(param_1: i32, param_2: i16, param_3: u16, param_4: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10027ad4(param_1: i32) {
    mech_trace::enter!("10027ad4", param_1);
    let mut local_14: [u16; 2] = [0; 2];
    let mut local_c: [u16; 2] = [0; 2];
    let base = param_1 as usize as *const u8;
    // SAFETY: param_1 points at the game's object; offsets are read as the
    // original does, and the callees are the game's own.
    unsafe {
        if (mech_trace::rd!("10027ad4", base.byte_add(0x17e), read) & 1) != 0 {
            return;
        }
        let u_var1 = mech_trace::cr!("10027ad4", "100293c6", FUN_100293c6(param_1));
        let mut local_8 = u_var1 as u16;
        if local_8 == 0xffff {
            return;
        }
        let w17a = mech_trace::rd!(
            "10027ad4",
            base.byte_add(0x17a).cast::<u16>(),
            read_unaligned
        ) as u32;
        let s176 = mech_trace::rd!(
            "10027ad4",
            base.byte_add(0x176).cast::<i16>(),
            read_unaligned
        );
        let d1a2 = mech_trace::rd!(
            "10027ad4",
            base.byte_add(0x1a2).cast::<u32>(),
            read_unaligned
        );
        if !(w17a != d1a2 || s176 != 8) {
            return;
        }
        let s176 = mech_trace::rd!(
            "10027ad4",
            base.byte_add(0x176).cast::<i16>(),
            read_unaligned
        );
        let w17a = mech_trace::rd!(
            "10027ad4",
            base.byte_add(0x17a).cast::<u16>(),
            read_unaligned
        ) as u32;
        if !((s176 != 2 && s176 != 3) || w17a != (local_8 as i16 as i32 as u32)) {
            return;
        }
        if mech_trace::rd!(
            "10027ad4",
            base.byte_add(0x1a8).cast::<i32>(),
            read_unaligned
        ) != 0
        {
            let local_10 = mech_trace::cr!(
                "10027ad4",
                "10028bde",
                FUN_10028bde(
                    param_1,
                    local_8,
                    -3,
                    local_14.as_mut_ptr(),
                    local_c.as_mut_ptr(),
                )
            );
            if local_10 != 0 {
                let p10 = local_10 as usize as *const u8;
                if (mech_trace::rd!("10027ad4", p10.byte_add(0x17e), read) & 0x40) == 0 {
                    mech_trace::cr!("10027ad4", "1002a4e0", FUN_1002a4e0(local_10, 0, 0, 0));
                } else {
                    local_8 = 0xffff;
                }
            }
        }
        if local_8 != 0xffff {
            mech_trace::cr!("10027ad4", "1002a4e0", FUN_1002a4e0(param_1, 2, local_8, 1));
        }
    }
}
