// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa2c8: i8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1006604e(param_1: i32, param_2: i8) {
    mech_trace::enter!("1006604e", param_1, param_2);
    // SAFETY: the global is the game's own memory; the write lands at the
    // byte offset param_1 from its address, as the original does.
    unsafe {
        mech_trace::wr!(
            "1006604e",
            (&raw mut DAT_100aa2c8).byte_offset(param_1 as isize),
            write,
            param_2
        );
    }
}
