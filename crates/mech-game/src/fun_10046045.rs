// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10046045(param_1: i32) {
    mech_trace::enter!("10046045", param_1);
    // SAFETY: param_1 is the address of a caller-owned structure; the u16 at
    // offset 4 is written as the original does.
    unsafe {
        mech_trace::wr!(
            "10046045",
            ((param_1 as usize).wrapping_add(4) as *mut u16),
            write_unaligned,
            0
        );
    }
}
