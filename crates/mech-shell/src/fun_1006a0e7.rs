// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1006a0e7(param_1: i32, param_2: i32) -> i32 {
    mech_trace::enter!("1006a0e7", param_1, param_2);
    // SAFETY: param_1 is the address of the caller's shp data; the entry
    // table and the offset it holds are read as the original does.
    unsafe {
        let off = mech_trace::rd!(
            "1006a0e7",
            (param_1
                .wrapping_add(8)
                .wrapping_add(param_2.wrapping_mul(8)) as *const i32),
            read_unaligned
        );
        mech_trace::rt!(
            "1006a0e7",
            mech_trace::rd!(
                "1006a0e7",
                (off.wrapping_add(param_1) as *const i32),
                read_unaligned
            )
        )
    }
}
