// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a5030: [i32; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10008371(param_1: i32, param_2: i32) {
    mech_trace::enter!("10008371", param_1, param_2);
    let i_var1 = (param_1 as u32).wrapping_sub(1) as i32;
    if i_var1 > -1 && i_var1 < 5 && param_2 > -1 && param_2 < 0x21 && i_var1 != 3 {
        // SAFETY: the global array is the game's own; the index is within 0..5.
        unsafe {
            mech_trace::wr!(
                "10008371",
                (&raw mut DAT_100a5030)
                    .cast::<i32>()
                    .add((i_var1 * 2) as usize),
                write_unaligned,
                param_2
            );
        }
    }
}
