// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001c299(param_1: i32, param_2: u32) {
    mech_trace::enter!("1001c299", param_1, param_2);
    // SAFETY: param_1 is the address of the caller's node list structure;
    // the field at offset 0x20 is written as the original does.
    unsafe {
        mech_trace::wr!(
            "1001c299",
            ((param_1 as usize).wrapping_add(0x20) as *mut u32),
            write_unaligned,
            param_2
        );
    }
}
