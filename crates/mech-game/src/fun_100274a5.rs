// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100274a5(param_1: i16, param_2: i32, param_3: i16) -> u32 {
    mech_trace::enter!("100274a5", param_1, param_2, param_3);
    let mut local_8: i16 = 0;
    loop {
        if param_3 <= local_8 {
            return mech_trace::rt!("100274a5", 0);
        }
        let off = (local_8 as i32).wrapping_mul(8);
        // SAFETY: param_2 is the address of the caller's table of 8-byte entries.
        let v = unsafe {
            mech_trace::rd!(
                "100274a5",
                ((param_2.wrapping_add(4).wrapping_add(off)) as *const i32),
                read_unaligned
            )
        };
        if v == param_1 as i32 {
            break;
        }
        local_8 = local_8.wrapping_add(1);
    }
    let off = (local_8 as i32).wrapping_mul(8);
    // SAFETY: same table; the matching entry's first dword is read.
    unsafe {
        mech_trace::rt!(
            "100274a5",
            mech_trace::rd!(
                "100274a5",
                (param_2.wrapping_add(off) as *const u32),
                read_unaligned
            )
        )
    }
}
