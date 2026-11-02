// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_101155e0: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10050af3(param_1: i32, param_2: u32) {
    mech_trace::enter!("10050af3", param_1, param_2);
    // SAFETY: the global is the game's own memory; the C scales the offset
    // by the element size (u32), and the write lands where the original does.
    unsafe {
        let base = &raw mut DAT_101155e0;
        let p = base.wrapping_offset(param_1.wrapping_mul(0x48) as isize);
        mech_trace::wr!("10050af3", p, write_unaligned, param_2 & 0xffff);
    }
}
