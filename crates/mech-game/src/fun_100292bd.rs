// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100292bd(param_1: i32) -> u32 {
    mech_trace::enter!("100292bd", param_1);
    // SAFETY: param_1 is the address of the caller's object; the u16 at
    // offset 0x16c is written as the original does.
    unsafe {
        mech_trace::wr!(
            "100292bd",
            ((param_1 as usize).wrapping_add(0x16c) as *mut u16),
            write_unaligned,
            0
        );
    }
    mech_trace::rt!("100292bd", 1)
}
