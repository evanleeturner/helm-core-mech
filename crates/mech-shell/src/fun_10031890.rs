// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10092000: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10031890(param_1: i32) -> i32 {
    mech_trace::enter!("10031890", param_1);
    let mut i_var1: i32 = 0;
    if 0 < param_1 {
        let mut i_var3: i32 = 0;
        let mut i_var2: i32 = 8;
        loop {
            let mut i_var4: i32 = 0xc;
            loop {
                // SAFETY: DAT_10092000 is the game's pointer to a structure;
                // the slot at offset 0x368 + i_var3 is read and cleared in place.
                unsafe {
                    let p = (mech_trace::gr!("10031890", "10092000", DAT_10092000)
                        .wrapping_add(0x368)
                        .wrapping_add(i_var3 as u32)) as *mut i32;
                    if mech_trace::rd!("10031890", p, read_unaligned) == param_1 {
                        i_var1 = i_var1.wrapping_add(1);
                        mech_trace::wr!("10031890", p, write_unaligned, 0);
                    }
                }
                i_var3 = i_var3.wrapping_add(4);
                i_var4 = i_var4.wrapping_sub(1);
                if i_var4 == 0 {
                    break;
                }
            }
            i_var2 = i_var2.wrapping_sub(1);
            if i_var2 == 0 {
                break;
            }
        }
    }
    mech_trace::rt!("10031890", i_var1)
}
