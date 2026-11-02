// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10002570(a: u32, b: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100662fd(source: *mut u32, out: *mut u32) -> *mut u32 {
    mech_trace::enter!("100662fd", source, out);
    // SAFETY: the callee is the game's own; source and out point at the
    // caller's two-word buffers, read and written where the original does.
    unsafe {
        let u_var1 = mech_trace::cr!(
            "100662fd",
            "10002570",
            FUN_10002570(mech_trace::rd!("100662fd", source, read_unaligned), 0x13f)
        );
        mech_trace::wr!("100662fd", out, write_unaligned, u_var1);
        let u_var1 = mech_trace::cr!(
            "100662fd",
            "10002570",
            FUN_10002570(
                mech_trace::rd!("100662fd", source.add(1), read_unaligned),
                199
            )
        );
        mech_trace::wr!("100662fd", out.add(1), write_unaligned, u_var1);
        mech_trace::rt!("100662fd", out)
    }
}
