// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10017ec0(sample_likely: i32, param_2: i32, param_3: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "thiscall" fn FUN_100108b0(this: *mut u8, sample_likely: i32) {
    mech_trace::enter!("100108b0", this, sample_likely);
    // SAFETY: `this` points at the caller's object, which holds a 4-byte
    // field at offset 0x11; the callee is the game's own function.
    unsafe {
        let field_0x11 = mech_trace::rd!(
            "100108b0",
            this.byte_add(0x11).cast::<i32>(),
            read_unaligned
        );
        FUN_10017ec0(field_0x11, sample_likely, 0);
    }
}
