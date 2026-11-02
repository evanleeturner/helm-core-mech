// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100980f8: [u8; 256];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10040270() {
    // SAFETY: DAT_100980f8 is the game's own 256-byte table; indices stay
    // below 0x100.
    unsafe {
        let p = (&raw mut DAT_100980f8).cast::<u8>();
        p.write(0xff);
        let mut i_var1: i32 = 1;
        loop {
            p.add(i_var1 as usize).write(i_var1 as u8);
            i_var1 = i_var1.wrapping_add(1);
            if i_var1 >= 0x100 {
                break;
            }
        }
        p.add(1).write(0x11);
    }
}
