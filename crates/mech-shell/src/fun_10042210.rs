// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100d08c4: i32;
    static mut DAT_100d08c8: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10042210(param_1: i32) {
    // SAFETY: param_1 is the address of a list node owned by the game; the
    // fields and linked neighbours are read and written where the original
    // does, and the globals are the game's own.
    unsafe {
        let node = param_1 as *mut u8;
        if node.byte_add(2).cast::<i16>().read_unaligned() != 1 {
            node.byte_add(2).cast::<i16>().write_unaligned(1);
            let prev = node.byte_add(0xc).cast::<i32>().read_unaligned();
            if prev != 0 {
                let v = node.byte_add(0x10).cast::<i32>().read_unaligned();
                (prev as *mut u8)
                    .byte_add(0x10)
                    .cast::<i32>()
                    .write_unaligned(v);
            }
            let next = node.byte_add(0x10).cast::<i32>().read_unaligned();
            if next != 0 {
                let v = node.byte_add(0xc).cast::<i32>().read_unaligned();
                (next as *mut u8)
                    .byte_add(0xc)
                    .cast::<i32>()
                    .write_unaligned(v);
            }
            if param_1 == DAT_100d08c4 {
                DAT_100d08c4 = node.byte_add(0xc).cast::<i32>().read_unaligned();
            }
            if param_1 == DAT_100d08c8 {
                DAT_100d08c8 = node.byte_add(0x10).cast::<i32>().read_unaligned();
            }
            node.byte_add(0xc).cast::<i32>().write_unaligned(0);
            node.byte_add(0x10).cast::<i32>().write_unaligned(0);
        }
    }
}
