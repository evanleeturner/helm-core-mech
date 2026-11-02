// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001d57c(param_1: i32, param_2: i32, param_3: u16) {
    mech_trace::enter!("1001d57c", param_1, param_2, param_3);
    // SAFETY: param_1 points at the game's object; its field at 0x20 is either
    // null or points at a table whose header and entries are read and written
    // at the same offsets as the original.
    unsafe {
        let i_var1 = mech_trace::rd!(
            "1001d57c",
            ((param_1 as u32).wrapping_add(0x20) as *const i32),
            read_unaligned
        );
        if i_var1 != 0
            && param_2
                < mech_trace::rd!(
                    "1001d57c",
                    ((i_var1 as u32).wrapping_add(6) as *const i16),
                    read_unaligned
                ) as i32
        {
            let off = mech_trace::rd!(
                "1001d57c",
                ((i_var1 as u32).wrapping_add(8) as *const i32),
                read_unaligned
            ) as u32;
            let addr = off
                .wrapping_add(i_var1 as u32)
                .wrapping_add((param_2 as u32).wrapping_mul(0x24));
            mech_trace::wr!("1001d57c", (addr as *mut u16), write_unaligned, param_3);
        }
    }
}
