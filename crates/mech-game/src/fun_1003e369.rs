// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aefc0: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003e369(param_1: i32) {
    let p = param_1 as usize as *mut u8;
    // SAFETY: param_1 points at the caller's node; linked nodes and the
    // global are the game's own memory, accessed in the original's order.
    unsafe {
        let next = p.byte_add(8).cast::<i32>().read_unaligned();
        if next != 0 {
            let prev = p.byte_add(4).cast::<u32>().read_unaligned();
            (next as usize as *mut u8)
                .byte_add(4)
                .cast::<u32>()
                .write_unaligned(prev);
        }
        let prev = p.byte_add(4).cast::<i32>().read_unaligned();
        if prev != 0 {
            let next = p.byte_add(8).cast::<u32>().read_unaligned();
            (prev as usize as *mut u8)
                .byte_add(8)
                .cast::<u32>()
                .write_unaligned(next);
        }
        p.byte_add(4).cast::<u32>().write_unaligned(0);
        let v = p.byte_add(4).cast::<u32>().read_unaligned();
        p.byte_add(8).cast::<u32>().write_unaligned(v);
        if DAT_100aefc0 != 0 {
            p.byte_add(0x44).cast::<u32>().write_unaligned(9999);
            p.byte_add(0x48).cast::<u32>().write_unaligned(9999);
        }
    }
}
