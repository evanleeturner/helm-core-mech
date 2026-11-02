// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa6b8: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1006b3f0() -> i32 {
    mech_trace::enter!("1006b3f0");
    let mut i_var1: i32 = 0;
    let mut i_var2: i32 = -1;
    loop {
        // SAFETY: DAT_100aa6b8 is the game's pointer to a structure; the slot
        // table at offset 0x2f0 is read where the original reads it.
        unsafe {
            let base = mech_trace::gr!("1006b3f0", "100aa6b8", DAT_100aa6b8);
            let addr = base
                .wrapping_add(0x2f0)
                .wrapping_add((i_var1 as u32).wrapping_mul(8));
            if mech_trace::rd!("1006b3f0", (addr as *const i32), read_unaligned) == -1 {
                i_var2 = i_var1;
            }
        }
        i_var1 = i_var1.wrapping_add(1);
        if i_var1 >= 0xe {
            break;
        }
    }
    i_var1 = 0xd;
    if -1 < i_var2 {
        i_var1 = i_var2;
    }
    mech_trace::rt!("1006b3f0", i_var1)
}
