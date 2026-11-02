// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7584: i32;
    static mut DAT_101f64a0: [i8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1000d7f5() -> bool {
    mech_trace::enter!("1000d7f5");
    let mut b_var1: u8 = 0;
    let mut local_8: i32 = 0;
    // SAFETY: the globals are the game's own memory; the table entries are
    // read at the same 0x7c-byte stride as the original.
    unsafe {
        while local_8 < mech_trace::gr!("1000d7f5", "100a7584", DAT_100a7584) && b_var1 == 0 {
            let p = (&raw mut DAT_101f64a0)
                .cast::<u8>()
                .byte_add((local_8 as u32).wrapping_mul(0x7c) as usize);
            if mech_trace::rd!("1000d7f5", p.cast::<i32>(), read_unaligned) == -1 {
                b_var1 = 1;
            }
            local_8 = (local_8 as u32).wrapping_add(1) as i32;
        }
    }
    mech_trace::rt!("1000d7f5", b_var1 == 0)
}
