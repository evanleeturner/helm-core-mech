// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10160be0: *mut i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100496bb(param_1: i32) -> *mut i32 {
    mech_trace::enter!("100496bb", param_1);
    // SAFETY: the global list head and the nodes it links are the game's own
    // memory; each node is read where the original reads it.
    unsafe {
        let mut local_8 = mech_trace::gr!("100496bb", "10160be0", DAT_10160be0);
        while !local_8.is_null() && mech_trace::rd!("100496bb", local_8, read_unaligned) != param_1
        {
            local_8 = mech_trace::rd!("100496bb", local_8.add(5), read_unaligned) as *mut i32;
        }
        mech_trace::rt!("100496bb", local_8)
    }
}
