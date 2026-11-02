// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10160be0: *mut i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10049704(param_1: i32) -> i32 {
    mech_trace::enter!("10049704", param_1);
    // SAFETY: the global is the game's own list head; each node is a game
    // record whose word 0 is the key, word 3 the value and word 5 the next
    // pointer, read where the original does.
    unsafe {
        let mut local_8: i32 = 0;
        let mut local_c: *mut i32 = mech_trace::gr!("10049704", "10160be0", DAT_10160be0);
        while !local_c.is_null() && mech_trace::rd!("10049704", local_c, read_unaligned) != param_1
        {
            local_c = mech_trace::rd!("10049704", local_c.add(5), read_unaligned) as *mut i32;
        }
        if !local_c.is_null() {
            local_8 = mech_trace::rd!("10049704", local_c.add(3), read_unaligned);
        }
        mech_trace::rt!("10049704", local_8)
    }
}
