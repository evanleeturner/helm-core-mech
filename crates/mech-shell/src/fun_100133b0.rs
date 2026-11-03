// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "thiscall" {
    fn FUN_1001e650(this: *mut core::ffi::c_void, forwarded_char: i8) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "thiscall" fn FUN_100133b0(
    pair_holder: *mut core::ffi::c_void,
    forwarded_char: i8,
) {
    mech_trace::enter!("100133b0", pair_holder, forwarded_char);
    // SAFETY: pair_holder points at the game's object with two member pointers
    // at offsets 0x16 and 0x1a; the callee is the game's own.
    unsafe {
        let a = mech_trace::rd!(
            "100133b0",
            pair_holder.byte_add(0x16).cast::<*mut core::ffi::c_void>(),
            read_unaligned
        );
        mech_trace::cr!("100133b0", "1001e650", FUN_1001e650(a, forwarded_char));
        let b = mech_trace::rd!(
            "100133b0",
            pair_holder.byte_add(0x1a).cast::<*mut core::ffi::c_void>(),
            read_unaligned
        );
        mech_trace::cr!("100133b0", "1001e650", FUN_1001e650(b, forwarded_char));
    }
}
