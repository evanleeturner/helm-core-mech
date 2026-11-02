// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10002570(a: u32, b: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10066209(source: i32, out: i32) -> i32 {
    mech_trace::enter!("10066209", source, out);
    // SAFETY: source and out are addresses of caller-owned structures; the
    // fields are read and written at the original's offsets, and the callee
    // is the game's own.
    unsafe {
        let s = source as *const u8;
        let o = out as *mut u8;
        let mut i_var1 = mech_trace::cr!(
            "10066209",
            "10002570",
            FUN_10002570(
                mech_trace::rd!("10066209", s.byte_add(4).cast::<u32>(), read_unaligned),
                0x13f
            )
        );
        mech_trace::wr!(
            "10066209",
            o.byte_add(4).cast::<i32>(),
            write_unaligned,
            i_var1
        );
        i_var1 = mech_trace::cr!(
            "10066209",
            "10002570",
            FUN_10002570(
                mech_trace::rd!("10066209", s.byte_add(8).cast::<u32>(), read_unaligned),
                199
            )
        );
        mech_trace::wr!(
            "10066209",
            o.byte_add(8).cast::<i32>(),
            write_unaligned,
            i_var1
        );
        i_var1 = mech_trace::cr!(
            "10066209",
            "10002570",
            FUN_10002570(
                mech_trace::rd!("10066209", s.byte_add(0xc).cast::<u32>(), read_unaligned),
                0x13f
            )
        );
        mech_trace::wr!(
            "10066209",
            o.byte_add(0xc).cast::<i32>(),
            write_unaligned,
            i_var1
        );
        i_var1 = mech_trace::cr!(
            "10066209",
            "10002570",
            FUN_10002570(
                mech_trace::rd!("10066209", s.byte_add(0x10).cast::<u32>(), read_unaligned),
                199
            )
        );
        mech_trace::wr!(
            "10066209",
            o.byte_add(0x10).cast::<i32>(),
            write_unaligned,
            i_var1
        );
    }
    mech_trace::rt!("10066209", out)
}
