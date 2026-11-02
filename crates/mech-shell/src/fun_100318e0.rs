// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10092000: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100318e0(param_1: i32) {
    mech_trace::enter!("100318e0", param_1);
    let mut i_var1: i32 = 0;
    loop {
        // SAFETY: DAT_10092000 is the game's own pointer to a structure; the
        // slots at offset 0x28c are read and written in place as the original does.
        unsafe {
            let p = mech_trace::gr!("100318e0", "10092000", DAT_10092000)
                .wrapping_add(0x28c)
                .wrapping_add(i_var1 as u32) as *mut i32;
            if mech_trace::rd!("100318e0", p, read_unaligned) == param_1 {
                mech_trace::wr!("100318e0", p, write_unaligned, -1);
            }
        }
        i_var1 = i_var1.wrapping_add(4);
        if i_var1 >= 100 {
            break;
        }
    }
}
