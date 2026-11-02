// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10029027(param_1: i32) {
    mech_trace::enter!("10029027", param_1);
    // SAFETY: param_1 is the address of a game object; the field at offset
    // 0xc0 is written with zero as the original does.
    unsafe {
        mech_trace::wr!(
            "10029027",
            ((param_1.wrapping_add(0xc0)) as *mut u32),
            write_unaligned,
            0
        );
    }
}
