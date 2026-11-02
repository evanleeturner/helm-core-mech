// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa6b8: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1006b420() -> i32 {
    let mut i_var1: i32 = 0;
    let mut i_var2: i32 = -1;
    loop {
        // SAFETY: the global holds the address of the game's own structure;
        // the slot read lies within it.
        unsafe {
            let addr = DAT_100aa6b8
                .wrapping_add(0x28c)
                .wrapping_add((i_var1 as u32).wrapping_mul(4));
            if (addr as *const i32).read_unaligned() == -1 {
                i_var2 = i_var1;
            }
        }
        i_var1 = i_var1.wrapping_add(1);
        if i_var1 >= 0x19 {
            break;
        }
    }
    i_var1 = 0x18;
    if -1 < i_var2 {
        i_var1 = i_var2;
    }
    i_var1
}
