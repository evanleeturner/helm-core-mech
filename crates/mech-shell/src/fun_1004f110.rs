// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_000b0f00: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1004f110() {
    mech_trace::enter!("1004f110");
    // SAFETY: the global is the game's own memory; the loop masks each dword
    // in the range starting at it, as the original does.
    unsafe {
        let mut p_var1: *mut u32 = &raw mut DAT_000b0f00;
        loop {
            let p_var2 = p_var1.wrapping_add(1);
            let v = mech_trace::rd!("1004f110", p_var1, read_unaligned);
            mech_trace::wr!("1004f110", p_var1, write_unaligned, v & 0xff00ff00);
            p_var1 = p_var2;
            if !((p_var2 as usize as i32) < 0xb0fa0) {
                break;
            }
        }
    }
}
