// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002f34e(
    param_1: i32,
    param_2: *mut u32,
    param_3: *mut u32,
    param_4: *mut u32,
) {
    mech_trace::enter!("1002f34e", param_1, param_2, param_3, param_4);
    // SAFETY: param_1 is an address of the game's own structure and the
    // output pointers are the caller's; reads and writes happen in the
    // original's order.
    unsafe {
        mech_trace::wr!(
            "1002f34e",
            param_2,
            write_unaligned,
            mech_trace::rd!(
                "1002f34e",
                ((param_1.wrapping_add(0x30)) as *const u32),
                read_unaligned
            )
        );
        mech_trace::wr!(
            "1002f34e",
            param_3,
            write_unaligned,
            mech_trace::rd!(
                "1002f34e",
                ((param_1.wrapping_add(0x34)) as *const u32),
                read_unaligned
            )
        );
        mech_trace::wr!(
            "1002f34e",
            param_4,
            write_unaligned,
            mech_trace::rd!(
                "1002f34e",
                ((param_1.wrapping_add(0x38)) as *const u32),
                read_unaligned
            )
        );
    }
}
