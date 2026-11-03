// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "thiscall" {
    fn FUN_1001e620(this: *mut u8, param_2: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "thiscall" fn FUN_10013360(this: *mut u8, param_2: i32) {
    mech_trace::enter!("10013360", this, param_2);
    // SAFETY: `this` points at the game's object; the two member pointers are
    // read at their offsets and passed on to the game's own function.
    unsafe {
        let first = mech_trace::rd!(
            "10013360",
            this.byte_add(0x16).cast::<*mut u8>(),
            read_unaligned
        );
        FUN_1001e620(first, param_2);
        let second = mech_trace::rd!(
            "10013360",
            this.byte_add(0x1a).cast::<*mut u8>(),
            read_unaligned
        );
        FUN_1001e620(second, param_2);
    }
}
