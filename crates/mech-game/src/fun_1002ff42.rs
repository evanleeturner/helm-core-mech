// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002ff42(param_1: i32) -> i32 {
    mech_trace::enter!("1002ff42", param_1);
    let mut local_c: i32 = 0;
    // SAFETY: param_1 and the chain of nodes it links are the game's own
    // structures; each is read at the same offsets as the original.
    unsafe {
        let mut i_var2: i32 = mech_trace::rd!(
            "1002ff42",
            ((param_1.wrapping_add(4)) as *const i32),
            read_unaligned
        );
        while i_var2 != 0 {
            let i_var1 = i_var2;
            local_c = i_var1;
            i_var2 = mech_trace::rd!(
                "1002ff42",
                ((i_var1.wrapping_add(8)) as *const i32),
                read_unaligned
            );
        }
    }
    mech_trace::rt!("1002ff42", local_c)
}
