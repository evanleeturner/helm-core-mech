// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100d08c4: u32;
    static mut DAT_100d08c8: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100421c0(param_1: i32) {
    // SAFETY: param_1 points at a game list node; the globals are the game's
    // own list head and tail, accessed in the original's order.
    unsafe {
        let node = param_1 as u32 as *mut u8;
        if node.byte_add(2).cast::<i16>().read_unaligned() != 0 {
            node.byte_add(2).cast::<u16>().write_unaligned(0);
            if DAT_100d08c8 != 0 {
                (DAT_100d08c8 as usize as *mut u8)
                    .byte_add(0xc)
                    .cast::<i32>()
                    .write_unaligned(param_1);
            }
            node.byte_add(0x10)
                .cast::<u32>()
                .write_unaligned(DAT_100d08c8);
            DAT_100d08c8 = param_1 as u32;
            node.byte_add(0xc).cast::<u32>().write_unaligned(0);
            if DAT_100d08c4 == 0 {
                DAT_100d08c4 = param_1 as u32;
            }
        }
    }
}
