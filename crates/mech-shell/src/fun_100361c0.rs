// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10093914: [i32; 0];
    static mut DAT_10093a2c: [i32; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100361c0() {
    let mut i_var1: i32 = 0;
    loop {
        // SAFETY: the globals are the game's own arrays; offsets 0, 0x40 and
        // 0x80 are written within them as the original does.
        unsafe {
            (&raw mut DAT_10093914)
                .cast::<u8>()
                .byte_add(i_var1 as usize)
                .cast::<u32>()
                .write_unaligned(0xffff_ffff);
            (&raw mut DAT_10093a2c)
                .cast::<u8>()
                .byte_add(i_var1 as usize)
                .cast::<u32>()
                .write_unaligned(0xffff_ffff);
        }
        i_var1 = (i_var1 as u32).wrapping_add(0x40) as i32;
        if i_var1 >= 0xc0 {
            break;
        }
    }
}
