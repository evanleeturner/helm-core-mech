// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004982f(param_1: i32) {
    mech_trace::enter!("1004982f", param_1);
    // SAFETY: param_1 is the address of the caller's structure; the u32 at
    // offset 0xc is written as the original does.
    unsafe {
        mech_trace::wr!(
            "1004982f",
            ((param_1 as usize).wrapping_add(0xc) as *mut u32),
            write_unaligned,
            0
        );
    }
}
