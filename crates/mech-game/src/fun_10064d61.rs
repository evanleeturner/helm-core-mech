// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10064ea3(param_1: u32) -> u32;
    fn FUN_10064e6a(param_1: u8) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10064d61(param_1: *mut u32) -> u32 {
    mech_trace::enter!("10064d61", param_1);
    // SAFETY: param_1 points at the caller's record; the reads happen in the
    // original's order and the callees are the game's own.
    unsafe {
        if mech_trace::cr!(
            "10064d61",
            "10064ea3",
            FUN_10064ea3(mech_trace::rd!("10064d61", param_1, read_unaligned))
        ) != 0
        {
            return mech_trace::rt!("10064d61", 2);
        }
        if mech_trace::cr!(
            "10064d61",
            "10064ea3",
            FUN_10064ea3(mech_trace::rd!("10064d61", param_1.add(1), read_unaligned))
        ) != 0
        {
            return mech_trace::rt!("10064d61", 2);
        }
        let b = mech_trace::rd!("10064d61", param_1.add(2), read_unaligned) as u8 as u32;
        let v = (((b >> 3) & 1) << 3) | (b & 0x80) | (((b >> 4) & 7) << 4) | (b & 7);
        if mech_trace::cr!("10064d61", "10064e6a", FUN_10064e6a(v as u8)) != 0 {
            return mech_trace::rt!("10064d61", 2);
        }
        if mech_trace::cr!(
            "10064d61",
            "10064e6a",
            FUN_10064e6a(mech_trace::rd!(
                "10064d61",
                param_1.byte_add(9).cast::<u8>(),
                read
            ))
        ) != 0
        {
            return mech_trace::rt!("10064d61", 2);
        }
        if mech_trace::cr!(
            "10064d61",
            "10064e6a",
            FUN_10064e6a(mech_trace::rd!(
                "10064d61",
                param_1.byte_add(10).cast::<u8>(),
                read
            ))
        ) != 0
        {
            return mech_trace::rt!("10064d61", 2);
        }
        mech_trace::rt!("10064d61", 0)
    }
}
