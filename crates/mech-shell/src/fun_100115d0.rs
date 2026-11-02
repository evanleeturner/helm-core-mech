// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a50f0: u32;
    static mut DAT_100c61fc: *mut u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _fread(dst: *mut u8, size: u32, count: u32, file: *mut u8) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100115d0(file: *mut u8) {
    // SAFETY: the globals and _fread are the game's own; DAT_100c61fc points
    // at the game's table buffer, read and written where the original does.
    unsafe {
        let mut i_var3: i32 = 0;
        let mut pi_var2 = DAT_100c61fc.byte_add(8) as *const i32;
        let mut i_var1: i32 = 8;
        loop {
            if pi_var2.read_unaligned() != -1 {
                i_var3 = i_var3.wrapping_add(1);
            }
            pi_var2 = pi_var2.wrapping_add(8);
            i_var1 = i_var1.wrapping_sub(1);
            if i_var1 == 0 {
                break;
            }
        }
        DAT_100c61fc
            .byte_add(4)
            .cast::<i32>()
            .write_unaligned(i_var3.wrapping_mul(0x20).wrapping_add(0x28));
        i_var1 = DAT_100c61fc.byte_add(4).cast::<i32>().read_unaligned();
        if DAT_100c61fc.byte_add(4).cast::<i32>().read_unaligned() <= DAT_100a50f0 as i32 {
            i_var1 = DAT_100a50f0 as i32;
        }
        i_var3 = 0;
        DAT_100a50f0 = i_var1 as u32;
        _fread(DAT_100c61fc, 8, 1, file);
        loop {
            if DAT_100c61fc
                .byte_add(i_var3 as usize)
                .byte_add(8)
                .cast::<i32>()
                .read_unaligned()
                != -1
            {
                _fread(
                    DAT_100c61fc.byte_add(i_var3 as usize).byte_add(8),
                    0x20,
                    1,
                    file,
                );
            }
            i_var3 = i_var3.wrapping_add(0x20);
            if i_var3 >= 0x100 {
                break;
            }
        }
    }
}
