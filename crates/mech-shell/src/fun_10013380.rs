// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1008eb64: u32;
}

#[allow(non_snake_case)]
unsafe extern "fastcall" {
    fn FUN_1001e6b0(param_1: *mut i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "thiscall" fn FUN_10013380(this: *mut u8, member_selector: i32) {
    mech_trace::enter!("10013380", this, member_selector);
    // SAFETY: the global and callee are the game's own; this points at the
    // caller's object, read at the offsets the original uses.
    unsafe {
        mech_trace::gw!("10013380", "1008eb64", DAT_1008eb64, 1);
        let picked_member: *mut i32 = if member_selector == 0 {
            mech_trace::rd!(
                "10013380",
                this.byte_add(0x1a).cast::<*mut i32>(),
                read_unaligned
            )
        } else {
            mech_trace::rd!(
                "10013380",
                this.byte_add(0x16).cast::<*mut i32>(),
                read_unaligned
            )
        };
        mech_trace::cr!("10013380", "1001e6b0", FUN_1001e6b0(picked_member));
        mech_trace::gw!("10013380", "1008eb64", DAT_1008eb64, 0);
    }
}
