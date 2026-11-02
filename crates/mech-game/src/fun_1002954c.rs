// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10215680: [u8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002954c(param_1: u16) -> i32 {
    let u_var1: u16 = param_1 & 0xf00;
    let mut local_8: i32;
    if u_var1 == 0x100 {
        let idx: u32 = ((param_1 as i16 as i32) as u32) & 0xff;
        // SAFETY: the table is the game's own memory; the entry at idx * 0x54
        // is read as an i32.
        unsafe {
            local_8 = (&raw mut DAT_10215680)
                .cast::<u8>()
                .byte_add(idx.wrapping_mul(0x54) as usize)
                .cast::<i32>()
                .read_unaligned();
        }
    } else if u_var1 == 0x200 || u_var1 == 0x400 {
        local_8 = 10000;
    } else {
        local_8 = 100;
    }
    if local_8 < 100 {
        local_8 = 100;
    }
    local_8
}
