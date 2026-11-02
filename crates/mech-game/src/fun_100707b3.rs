// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aaf68: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100707b3(param_1: i32, param_2: *mut i32) -> *mut i32 {
    mech_trace::enter!("100707b3", param_1, param_2);
    // SAFETY: the global is the game's own pointer value; param_1 is the address
    // of a game object and param_2 points at the caller's output buffer, read and
    // written where the original does.
    unsafe {
        let base = mech_trace::gr!("100707b3", "100aaf68", DAT_100aaf68) as *const i32;
        let obj = param_1 as u32 as *const u8;
        mech_trace::wr!(
            "100707b3",
            param_2,
            write_unaligned,
            mech_trace::rd!("100707b3", base, read_unaligned).wrapping_sub(mech_trace::rd!(
                "100707b3",
                obj.byte_add(0x50).cast::<i32>(),
                read_unaligned
            ))
        );
        mech_trace::wr!(
            "100707b3",
            param_2.byte_add(4),
            write_unaligned,
            mech_trace::rd!("100707b3", base.byte_add(4), read_unaligned).wrapping_sub(
                mech_trace::rd!("100707b3", obj.byte_add(0x54).cast::<i32>(), read_unaligned)
            )
        );
        mech_trace::wr!(
            "100707b3",
            param_2.byte_add(8),
            write_unaligned,
            mech_trace::rd!("100707b3", base.byte_add(8), read_unaligned).wrapping_sub(
                mech_trace::rd!("100707b3", obj.byte_add(0x58).cast::<i32>(), read_unaligned)
            )
        );
        mech_trace::rt!("100707b3", param_2)
    }
}
