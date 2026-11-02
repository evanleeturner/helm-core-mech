// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10045ee7(param_1: i32, param_2: u32) {
    mech_trace::enter!("10045ee7", param_1, param_2);
    // SAFETY: param_1 is an address the caller supplies; the original stores
    // param_2 at offset 8 from it.
    unsafe {
        mech_trace::wr!(
            "10045ee7",
            (param_1.wrapping_add(8) as *mut u32),
            write_unaligned,
            param_2
        );
    }
}
