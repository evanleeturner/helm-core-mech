// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a365c: i32;
    static mut DAT_100a3664: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100576a0(index: i32) -> i32 {
    // SAFETY: the globals are the game's own memory, read by value.
    unsafe {
        if -1 < index && index < DAT_100a365c {
            return (DAT_100a3664 as u32).wrapping_add((index as u32).wrapping_mul(0x78)) as i32;
        }
    }
    0
}
