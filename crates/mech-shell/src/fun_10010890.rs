// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10017df0(sample_likely: i32, param_2: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "thiscall" fn FUN_10010890(sample_owner_likely: *mut u8, sample_likely: i32) {
    mech_trace::enter!("10010890", sample_owner_likely, sample_likely);
    // SAFETY: sample_owner_likely points at the caller's object, which holds a
    // 4-byte value at offset 0x11; the callee is the game's own.
    unsafe {
        let collection = mech_trace::rd!(
            "10010890",
            sample_owner_likely.byte_add(0x11).cast::<i32>(),
            read_unaligned
        );
        FUN_10017df0(collection, sample_likely);
    }
}
