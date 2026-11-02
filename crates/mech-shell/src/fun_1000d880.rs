// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _ftol() -> i64;
    fn FUN_1000a160(param_1: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000d880(param_1: i32) {
    mech_trace::enter!("1000d880", param_1);
    let p = param_1.wrapping_add(0x250) as *mut i32;
    // SAFETY: the callees are the game's own; param_1 + 0x250 points at the
    // caller's i32 field, written and read where the original does.
    unsafe {
        let l_var3 = mech_trace::cr!("1000d880", "_ftol", _ftol());
        mech_trace::wr!("1000d880", p, write_unaligned, l_var3 as i32);
        let i_var1 = mech_trace::cr!(
            "1000d880",
            "1000a160",
            FUN_1000a160(mech_trace::rd!("1000d880", p, read_unaligned).wrapping_mul(2))
        );
        mech_trace::wr!("1000d880", p, write_unaligned, i_var1 / 2);
    }
}
