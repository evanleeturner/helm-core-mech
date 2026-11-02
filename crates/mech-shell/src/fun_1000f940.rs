// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1008e3f0: u32;
    static mut DAT_1008e410: i8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000f940(param_1: u32) {
    mech_trace::enter!("1000f940", param_1);
    // SAFETY: the globals are the game's own memory; the table starts at
    // DAT_1008e3f0 and the scan stops at the address of DAT_1008e410.
    unsafe {
        let base = &raw mut DAT_1008e3f0;
        let end = (&raw mut DAT_1008e410) as *mut i32;
        let mut i_var2: usize = 0;
        let mut pi_var1 = base as *mut i32;
        loop {
            if mech_trace::rd!("1000f940", pi_var1, read_unaligned) == 0 {
                mech_trace::wr!("1000f940", base.add(i_var2), write_unaligned, param_1);
                return;
            }
            pi_var1 = pi_var1.wrapping_add(1);
            i_var2 = i_var2.wrapping_add(1);
            if pi_var1 >= end {
                break;
            }
        }
    }
}
