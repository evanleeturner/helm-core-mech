// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a40d0: [u32; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10051f43(param_1: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1000387d() -> u32 {
    mech_trace::enter!("1000387d");
    // SAFETY: the callee and the table are the game's own; the index comes
    // from the callee and is read as the original does.
    unsafe {
        let u_var1 = mech_trace::cr!("1000387d", "10051f43", FUN_10051f43(10));
        mech_trace::rt!(
            "1000387d",
            mech_trace::rd!(
                "1000387d",
                (&raw mut DAT_100a40d0).cast::<u32>().add(u_var1 as usize),
                read_unaligned
            )
        )
    }
}
