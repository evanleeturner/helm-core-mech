// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10049767(param_1: i32) -> u32 {
    mech_trace::enter!("10049767", param_1);
    let mut local_8: u32 = 0;
    // SAFETY: param_1 is the address of the game's stack-like structure; its
    // count at +0xc and array pointer at +8 are read and written as the original does.
    unsafe {
        if 0 < mech_trace::rd!(
            "10049767",
            (param_1.wrapping_add(0xc) as *const i32),
            read_unaligned
        ) {
            let n = mech_trace::rd!(
                "10049767",
                (param_1.wrapping_add(0xc) as *const i32),
                read_unaligned
            )
            .wrapping_sub(1);
            mech_trace::wr!(
                "10049767",
                (param_1.wrapping_add(0xc) as *mut i32),
                write_unaligned,
                n
            );
            let base = mech_trace::rd!(
                "10049767",
                (param_1.wrapping_add(8) as *const i32),
                read_unaligned
            );
            let addr = base.wrapping_add(
                mech_trace::rd!(
                    "10049767",
                    (param_1.wrapping_add(0xc) as *const i32),
                    read_unaligned
                )
                .wrapping_mul(4),
            );
            local_8 = mech_trace::rd!("10049767", (addr as *const u32), read_unaligned);
        }
    }
    mech_trace::rt!("10049767", local_8)
}
