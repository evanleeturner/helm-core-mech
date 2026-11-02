// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_101155c4: u8;
    static mut DAT_101155e4: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1001d671(param_1: *mut u8) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100509fd(param_1: i32, param_2: i32) {
    mech_trace::enter!("100509fd", param_1, param_2);
    let i_var1 = param_1.wrapping_mul(0x48);
    // SAFETY: the globals are the game's own tables, indexed by the caller's
    // entry number at 0x48 bytes per entry; the callee is the game's own.
    unsafe {
        let a = (&raw mut DAT_101155c4)
            .byte_offset(i_var1 as isize)
            .cast::<i32>();
        let b = (&raw mut DAT_101155e4)
            .byte_offset(i_var1 as isize)
            .cast::<i32>();
        if -1 < mech_trace::rd!("100509fd", a, read_unaligned)
            && mech_trace::rd!("100509fd", a, read_unaligned) == param_2
            && mech_trace::rd!("100509fd", b, read_unaligned) != 0
        {
            mech_trace::cr!(
                "100509fd",
                "1001d671",
                FUN_1001d671(
                    mech_trace::rd!("100509fd", b, read_unaligned) as u32 as usize as *mut u8
                )
            );
            mech_trace::wr!("100509fd", b, write_unaligned, 0);
            mech_trace::wr!("100509fd", a, write_unaligned, -1);
        }
    }
}
