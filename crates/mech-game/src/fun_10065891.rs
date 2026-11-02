// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10064e6a(param_1: u8) -> u32;
    fn FUN_10064ea3(param_1: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10065891(param_1: *mut u8) -> u32 {
    mech_trace::enter!("10065891", param_1);
    // SAFETY: param_1 points at the caller's record, read at the original's
    // offsets; the callees are the game's own.
    unsafe {
        if mech_trace::cr!(
            "10065891",
            "10064e6a",
            FUN_10064e6a(mech_trace::rd!("10065891", param_1, read))
        ) != 0
        {
            return mech_trace::rt!("10065891", 2);
        }
        if mech_trace::cr!(
            "10065891",
            "10064ea3",
            FUN_10064ea3(mech_trace::rd!(
                "10065891",
                param_1.byte_add(1).cast::<u32>(),
                read_unaligned
            ))
        ) != 0
        {
            return mech_trace::rt!("10065891", 2);
        }
        if mech_trace::cr!(
            "10065891",
            "10064ea3",
            FUN_10064ea3(mech_trace::rd!(
                "10065891",
                param_1.byte_add(5).cast::<u32>(),
                read_unaligned
            ))
        ) != 0
        {
            return mech_trace::rt!("10065891", 2);
        }
        if mech_trace::cr!(
            "10065891",
            "10064ea3",
            FUN_10064ea3(mech_trace::rd!(
                "10065891",
                param_1.byte_add(9).cast::<u32>(),
                read_unaligned
            ))
        ) != 0
        {
            return mech_trace::rt!("10065891", 2);
        }
        if mech_trace::cr!(
            "10065891",
            "10064ea3",
            FUN_10064ea3(mech_trace::rd!(
                "10065891",
                param_1.byte_add(0xd).cast::<u32>(),
                read_unaligned
            ))
        ) != 0
        {
            return mech_trace::rt!("10065891", 2);
        }
        let b = u32::from(mech_trace::rd!("10065891", param_1.byte_add(0x11), read));
        let v = (((b >> 3) & 3) << 3)
            | (((b >> 5) & 1) << 5)
            | (((b >> 6) & 1) << 6)
            | (b & 0x80)
            | (b & 7);
        mech_trace::rt!(
            "10065891",
            if mech_trace::cr!("10065891", "10064e6a", FUN_10064e6a(v as u8)) != 0 {
                2
            } else {
                0
            }
        )
    }
}
