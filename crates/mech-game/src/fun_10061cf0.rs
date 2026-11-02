// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100f2fb0: u8;
    static mut DAT_100f2ff0: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10061cf0(param_1: i32) {
    let mut local_c: i32 = 0;
    while local_c < 0xe {
        // SAFETY: param_1 points at the game's structure; the pointer at +0x54
        // addresses an array of 0x40-byte entries, read and written as the original does.
        unsafe {
            let base = ((param_1 as usize).wrapping_add(0x54)) as *const i32;
            let iv = base
                .read_unaligned()
                .wrapping_add(local_c.wrapping_mul(0x40));
            let p = (iv as usize).wrapping_add(8) as *mut i32;
            if p.read_unaligned() == 2 || p.read_unaligned() == 3 {
                p.write_unaligned(1);
            }
        }
        local_c = local_c.wrapping_add(1);
    }
    local_c = 0;
    while local_c < 0xe {
        // SAFETY: the globals are the game's own arrays of 14 dwords.
        unsafe {
            (&raw mut DAT_100f2ff0)
                .byte_add(local_c.wrapping_mul(4) as usize)
                .cast::<u32>()
                .write_unaligned(0);
            (&raw mut DAT_100f2fb0)
                .byte_add(local_c.wrapping_mul(4) as usize)
                .cast::<u32>()
                .write_unaligned(0);
        }
        local_c = local_c.wrapping_add(1);
    }
}
