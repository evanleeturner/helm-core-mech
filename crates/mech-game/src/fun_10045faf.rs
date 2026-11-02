// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10045faf(param_1: i32, param_2: u32) {
    mech_trace::enter!("10045faf", param_1, param_2);
    // SAFETY: param_1 is an address of the caller's object; the field at
    // offset 0xa8 is written as the original does.
    unsafe {
        mech_trace::wr!(
            "10045faf",
            (param_1 as *mut u8).byte_add(0xa8).cast::<u32>(),
            write_unaligned,
            param_2
        );
    }
}
