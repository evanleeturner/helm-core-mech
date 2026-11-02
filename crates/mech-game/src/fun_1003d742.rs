// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c6d48: [u8; 1];
    static mut DAT_100c6d18: i16;
    static mut DAT_100c6d30: i16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002f322(param_1: i32, param_2: *mut i32, param_3: *mut u32, param_4: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003d742(param_1: *mut u16) -> *mut u8 {
    let mut local_1c: [u32; 3] = [0; 3];
    let mut local_10: i32 = 0;
    let mut local_8: i32 = 0;
    // SAFETY: param_1 points at the caller's record; fields are read and
    // written at the original's offsets, and the callee and globals are the
    // game's own.
    unsafe {
        if (param_1.read() & 0x1000) == 0 {
            let arg = param_1.byte_add(0x18).cast::<i32>().read_unaligned();
            FUN_1002f322(
                arg,
                &raw mut local_10,
                local_1c.as_mut_ptr(),
                &raw mut local_8,
            );
            let i_var1 = ((local_10 as u32).wrapping_add(150000) as i32) / 25000;
            let i_var2 = ((local_8 as u32).wrapping_add(150000) as i32) / 25000;
            param_1.byte_add(0x44).cast::<i32>().write_unaligned(i_var1);
            param_1.byte_add(0x48).cast::<i32>().write_unaligned(i_var2);
            if i_var1 < 0 || i_var1 > 0xb || i_var2 < 0 || i_var2 > 0xb {
                (&raw mut DAT_100c6d18).cast::<u8>()
            } else {
                let off = i_var1
                    .wrapping_mul(0x120)
                    .wrapping_add(i_var2.wrapping_mul(0x18));
                (&raw mut DAT_100c6d48)
                    .cast::<u8>()
                    .byte_offset(off as isize)
            }
        } else {
            (&raw mut DAT_100c6d30).cast::<u8>()
        }
    }
}
