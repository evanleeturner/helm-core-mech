// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10053fc0(param_1: i32, param_2: *mut u32) {
    mech_trace::enter!("10053fc0", param_1, param_2);
    if param_2.is_null() {
        return;
    }
    // SAFETY: param_2 is non-null and points at the caller's record of at
    // least six u32 fields, written where the original does.
    unsafe {
        mech_trace::wr!("10053fc0", param_2.add(1), write_unaligned, 0xffff_ffff);
        mech_trace::wr!("10053fc0", param_2.add(4), write_unaligned, 0xffff_ffff);
        mech_trace::wr!("10053fc0", param_2.add(2), write_unaligned, 0);
        mech_trace::wr!("10053fc0", param_2.add(3), write_unaligned, 0);
        mech_trace::wr!("10053fc0", param_2, write_unaligned, (6 < param_1) as u32);
        mech_trace::wr!("10053fc0", param_2.add(5), write_unaligned, 0xffff_ffff);
    }
}
