// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002f272(param_1: i32, param_2: u32) {
    mech_trace::enter!("1002f272", param_1, param_2);
    // SAFETY: param_1 is the address of a caller-owned structure; the fields at
    // offsets 0x6c and 0x74 are written and read-modified as the original does.
    unsafe {
        mech_trace::wr!(
            "1002f272",
            (param_1.wrapping_add(0x6c) as *mut u32),
            write_unaligned,
            param_2
        );
        let p = param_1.wrapping_add(0x74) as *mut u32;
        mech_trace::wr!(
            "1002f272",
            p,
            write_unaligned,
            mech_trace::rd!("1002f272", p, read_unaligned) | 1
        );
    }
}
