// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100eef40: [u32; 0];
    static mut DAT_100ef034: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000dd4e(param_1: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10044c17(param_1: u16) -> u32 {
    let mut local_10: i32 = 0;
    let idx = (param_1 & 0xff) as i16 as i32;
    // SAFETY: the globals and callee are the game's own; the table entries
    // are addresses of game structures, read where the original reads.
    unsafe {
        if (param_1 & 0xf00) == 0x200 {
            let entry = (&raw mut DAT_100eef40)
                .cast::<u32>()
                .offset(idx as isize)
                .read_unaligned();
            local_10 = (entry.wrapping_add(0x40) as usize as *const i32).read_unaligned();
        } else if (param_1 & 0xf00) == 0x400 {
            let v = (&raw mut DAT_100ef034)
                .cast::<u8>()
                .byte_offset(idx.wrapping_mul(0x40) as isize)
                .cast::<i32>()
                .read_unaligned();
            local_10 = FUN_1000dd4e(v) as i32;
        }
        if local_10 == 0 {
            0
        } else {
            ((local_10 as u32).wrapping_add(0x6c) as usize as *const u32).read_unaligned()
        }
    }
}
