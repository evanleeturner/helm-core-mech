// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adcf8: [u32; 8];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003629e(param_1: i32) -> u32 {
    mech_trace::enter!("1003629e", param_1);
    let mut local_8: u32 = 7;
    if param_1 < 8 {
        // SAFETY: the table is the game's own memory; the index is read
        // exactly as the original does, with no bounds check.
        unsafe {
            local_8 = mech_trace::rd!(
                "1003629e",
                (&raw mut DAT_100adcf8)
                    .cast::<u32>()
                    .offset(param_1 as isize),
                read_unaligned
            );
        }
    }
    mech_trace::rt!("1003629e", local_8)
}
