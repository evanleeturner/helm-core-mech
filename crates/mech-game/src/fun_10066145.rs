// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10002570(a: u32, b: i32) -> i32;
    fn FUN_10003d60(a: i32, b: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10066145(param_1: i32, param_2: u32) -> i32 {
    mech_trace::enter!("10066145", param_1, param_2);
    // SAFETY: the callees are the game's own; param_1 points at the caller's
    // struct, with fields read and written at the original's offsets.
    unsafe {
        let p = param_1 as *mut u8;
        let i_var1 = mech_trace::cr!("10066145", "10002570", FUN_10002570(param_2, 0xd555));
        let mut v = mech_trace::cr!(
            "10066145",
            "10003d60",
            FUN_10003d60(
                mech_trace::rd!("10066145", p.byte_add(4).cast::<i32>(), read_unaligned),
                0x13f
            )
        );
        mech_trace::wr!("10066145", p.byte_add(4).cast::<i32>(), write_unaligned, v);
        v = mech_trace::cr!(
            "10066145",
            "10003d60",
            FUN_10003d60(
                mech_trace::rd!("10066145", p.byte_add(8).cast::<i32>(), read_unaligned),
                199
            )
        );
        mech_trace::wr!("10066145", p.byte_add(8).cast::<i32>(), write_unaligned, v);
        v = mech_trace::cr!(
            "10066145",
            "10003d60",
            FUN_10003d60(
                mech_trace::rd!("10066145", p.byte_add(8).cast::<i32>(), read_unaligned),
                i_var1
            )
        );
        mech_trace::wr!("10066145", p.byte_add(8).cast::<i32>(), write_unaligned, v);
        v = mech_trace::cr!(
            "10066145",
            "10003d60",
            FUN_10003d60(
                mech_trace::rd!("10066145", p.byte_add(0xc).cast::<i32>(), read_unaligned),
                0x13f
            )
        );
        mech_trace::wr!(
            "10066145",
            p.byte_add(0xc).cast::<i32>(),
            write_unaligned,
            v
        );
        v = mech_trace::cr!(
            "10066145",
            "10003d60",
            FUN_10003d60(
                mech_trace::rd!("10066145", p.byte_add(0x10).cast::<i32>(), read_unaligned),
                199
            )
        );
        mech_trace::wr!(
            "10066145",
            p.byte_add(0x10).cast::<i32>(),
            write_unaligned,
            v
        );
        v = mech_trace::cr!(
            "10066145",
            "10003d60",
            FUN_10003d60(
                mech_trace::rd!("10066145", p.byte_add(0x10).cast::<i32>(), read_unaligned),
                i_var1
            )
        );
        mech_trace::wr!(
            "10066145",
            p.byte_add(0x10).cast::<i32>(),
            write_unaligned,
            v
        );
    }
    mech_trace::rt!("10066145", param_1)
}
