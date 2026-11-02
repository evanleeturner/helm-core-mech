// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10002570(a: u32, b: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10066284(source: *mut u32, out: *mut u32) -> *mut u32 {
    mech_trace::enter!("10066284", source, out);
    // SAFETY: source and out point at the caller's four-u32 rectangles; the
    // callee is the game's own.
    unsafe {
        let v = mech_trace::cr!(
            "10066284",
            "10002570",
            FUN_10002570(mech_trace::rd!("10066284", source, read_unaligned), 0x13f)
        );
        mech_trace::wr!("10066284", out, write_unaligned, v);
        let v = mech_trace::cr!(
            "10066284",
            "10002570",
            FUN_10002570(
                mech_trace::rd!("10066284", source.add(1), read_unaligned),
                199
            )
        );
        mech_trace::wr!("10066284", out.add(1), write_unaligned, v);
        let v = mech_trace::cr!(
            "10066284",
            "10002570",
            FUN_10002570(
                mech_trace::rd!("10066284", source.add(2), read_unaligned),
                0x13f
            )
        );
        mech_trace::wr!("10066284", out.add(2), write_unaligned, v);
        let v = mech_trace::cr!(
            "10066284",
            "10002570",
            FUN_10002570(
                mech_trace::rd!("10066284", source.add(3), read_unaligned),
                199
            )
        );
        mech_trace::wr!("10066284", out.add(3), write_unaligned, v);
    }
    mech_trace::rt!("10066284", out)
}
