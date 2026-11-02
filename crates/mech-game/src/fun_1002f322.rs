// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002f322(
    param_1: i32,
    param_2: *mut u32,
    param_3: *mut u32,
    param_4: *mut u32,
) {
    mech_trace::enter!("1002f322", param_1, param_2, param_3, param_4);
    // SAFETY: param_1 is an address of the game's own structure; the three
    // fields are read and written to the caller's output pointers in order.
    unsafe {
        mech_trace::wr!(
            "1002f322",
            param_2,
            write_unaligned,
            mech_trace::rd!(
                "1002f322",
                ((param_1.wrapping_add(0x60)) as *const u32),
                read_unaligned
            )
        );
        mech_trace::wr!(
            "1002f322",
            param_3,
            write_unaligned,
            mech_trace::rd!(
                "1002f322",
                ((param_1.wrapping_add(0x64)) as *const u32),
                read_unaligned
            )
        );
        mech_trace::wr!(
            "1002f322",
            param_4,
            write_unaligned,
            mech_trace::rd!(
                "1002f322",
                ((param_1.wrapping_add(0x68)) as *const u32),
                read_unaligned
            )
        );
    }
}
