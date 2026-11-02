// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa2d8: [i8; 0x26];
    static mut DAT_100aa214: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10013b09(param_1: u32, param_2: *mut i8) -> u32 {
    mech_trace::enter!("10013b09", param_1, param_2);
    let mut i_var1: i32 = 0x26;
    let mut in_zf = false;
    // SAFETY: the global string is the game's own; param_2 points at the
    // caller's string, read byte by byte only until the first mismatch or 0x26 bytes.
    unsafe {
        let mut p1 = param_2 as *const i8;
        let mut p2 = &raw const DAT_100aa2d8 as *const i8;
        loop {
            if i_var1 == 0 {
                break;
            }
            i_var1 = i_var1.wrapping_sub(1);
            in_zf = mech_trace::dr!("10013b09", p1) == mech_trace::dr!("10013b09", p2);
            p1 = p1.add(1);
            p2 = p2.add(1);
            if !in_zf {
                break;
            }
        }
        if in_zf {
            mech_trace::gw!("10013b09", "100aa214", DAT_100aa214, param_1);
        }
    }
    mech_trace::rt!("10013b09", 1)
}
