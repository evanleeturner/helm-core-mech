// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
pub unsafe extern "C" fn FUN_10031f40(
    param_1: i32,
    param_2: *mut u32,
    param_3: *mut u32,
    param_4: i32,
) {
    mech_trace::enter!("10031f40", param_1, param_2, param_3, param_4);
    // SAFETY: param_1 points at the game's record; param_2, param_3 and
    // param_4 are caller-provided output buffers, written where the original does.
    unsafe {
        if !param_2.is_null() {
            mech_trace::wr!(
                "10031f40",
                param_2,
                write_unaligned,
                mech_trace::rd!(
                    "10031f40",
                    ((param_1.wrapping_add(8)) as *const u32),
                    read_unaligned
                )
            );
        }
        if !param_3.is_null() {
            mech_trace::wr!(
                "10031f40",
                param_3,
                write_unaligned,
                mech_trace::rd!(
                    "10031f40",
                    ((param_1.wrapping_add(0xc)) as *const u32),
                    read_unaligned
                )
            );
        }
        if param_4 != 0 {
            let i_var1 = mech_trace::rd!(
                "10031f40",
                ((param_1.wrapping_add(4)) as *const i32),
                read_unaligned
            );
            let count = (i_var1.wrapping_sub(0x10) as u32) >> 2;
            let mut local_8: u32 = 0;
            while local_8 < count {
                let v = mech_trace::rd!(
                    "10031f40",
                    ((param_1
                        .wrapping_add(0x10)
                        .wrapping_add((local_8.wrapping_mul(4)) as i32))
                        as *const u32),
                    read_unaligned
                );
                mech_trace::wr!(
                    "10031f40",
                    ((param_4.wrapping_add((local_8.wrapping_mul(4)) as i32)) as *mut u32),
                    write_unaligned,
                    v
                );
                local_8 = local_8.wrapping_add(1);
            }
        }
    }
}
