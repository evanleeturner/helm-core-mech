// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1008379c: [i32; 25];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000a3a0(param_1: i32) -> i32 {
    let mut i_var1: i32 = 0;
    let i_target: i32 =
        ((((param_1 as u32).wrapping_mul(4)).wrapping_add(400)).wrapping_mul(0x19) as i32) / 100;
    // SAFETY: the global array is the game's own memory; the pointer stays
    // within its 25 elements and is only read.
    unsafe {
        let base = (&raw mut DAT_1008379c).cast::<i32>();
        let end = base.add(25);
        let mut pi_var2 = base;
        loop {
            let v = pi_var2.read_unaligned();
            if v == -1 {
                return i_var1;
            }
            if v / 100 == i_target {
                i_var1 = i_var1.wrapping_add(1);
            }
            pi_var2 = pi_var2.add(1);
            if pi_var2 >= end {
                break;
            }
        }
    }
    i_var1
}
