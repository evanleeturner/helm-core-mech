// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10003d60(a: i32, b: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1006604b(frame: i32, source: *mut i32, out: *mut i32) -> *mut i32 {
    mech_trace::enter!("1006604b", frame, source, out);
    // SAFETY: frame is an address of the caller's structure; source and out
    // point at the caller's four-int arrays; the callee is the game's own.
    unsafe {
        let i_var1 = (mech_trace::rd!(
            "1006604b",
            (frame.wrapping_add(0xc) as *const i32),
            read_unaligned
        ) as u32)
            .wrapping_sub(mech_trace::rd!(
                "1006604b",
                (frame.wrapping_add(4) as *const i32),
                read_unaligned
            ) as u32) as i32;
        let i_var2 = (mech_trace::rd!(
            "1006604b",
            (frame.wrapping_add(0x10) as *const i32),
            read_unaligned
        ) as u32)
            .wrapping_sub(mech_trace::rd!(
                "1006604b",
                (frame.wrapping_add(8) as *const i32),
                read_unaligned
            ) as u32) as i32;
        let i_var3 = mech_trace::cr!(
            "1006604b",
            "10003d60",
            FUN_10003d60(i_var1, mech_trace::rd!("1006604b", source, read_unaligned))
        );
        mech_trace::wr!("1006604b", out, write_unaligned, i_var3);
        let i_var3 = mech_trace::cr!(
            "1006604b",
            "10003d60",
            FUN_10003d60(
                i_var2,
                mech_trace::rd!("1006604b", source.add(1), read_unaligned)
            )
        );
        mech_trace::wr!("1006604b", out.add(1), write_unaligned, i_var3);
        let i_var1 = mech_trace::cr!(
            "1006604b",
            "10003d60",
            FUN_10003d60(
                i_var1,
                mech_trace::rd!("1006604b", source.add(2), read_unaligned)
            )
        );
        mech_trace::wr!("1006604b", out.add(2), write_unaligned, i_var1);
        let i_var1 = mech_trace::cr!(
            "1006604b",
            "10003d60",
            FUN_10003d60(
                i_var2,
                mech_trace::rd!("1006604b", source.add(3), read_unaligned)
            )
        );
        mech_trace::wr!("1006604b", out.add(3), write_unaligned, i_var1);
    }
    mech_trace::rt!("1006604b", out)
}
