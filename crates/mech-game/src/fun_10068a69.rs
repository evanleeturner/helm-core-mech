// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100f3040: [u8; 0];
    static mut DAT_10114ec8: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10003d80(param_1: *mut i32) -> u8;
    fn FUN_10003fc7(param_1: i32, param_2: *mut i32) -> *mut u32;
    fn FUN_100682e7(param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10068a69(param_1: i32) {
    let mut local_34: [u32; 5] = [0; 5];
    let mut local_20: [u32; 5] = [0; 5];
    // SAFETY: the globals, callees and param_1 structure belong to the game;
    // all reads and writes mirror the original's offsets and order.
    unsafe {
        let p = param_1 as usize;
        if (((p + 4) as *const i16).read_unaligned() != 0)
            && (((p + 0x30) as *const i32).read_unaligned() != 0)
            && (((p + 0x30) as *const i32).read_unaligned() != 2)
            && (((p + 0x30) as *const i32).read_unaligned() != 1)
        {
            let local_c = ((p + 0xac) as *const *mut i32).read_unaligned();
            if !local_c.is_null() {
                if ((p + 0xb0) as *const i32).read_unaligned() != 1 {
                    FUN_10003d80(local_c);
                }
                let local_8 = FUN_10003fc7(0, local_c);
                if local_8.is_null() {
                    FUN_100682e7(param_1);
                } else {
                    let idx = ((p + 0xc) as *const i32).read_unaligned();
                    let slot = (&raw mut DAT_100f3040 as *mut u8)
                        .offset(0)
                        .wrapping_offset(idx.wrapping_mul(0x14) as isize)
                        as *mut u32;
                    let other = ((p + 0xa4) as *const *mut u32).read_unaligned();

                    for i in 0..5 {
                        local_20[i] = slot.add(i).read_unaligned();
                    }
                    for i in 0..5 {
                        local_34[i] = other.add(i).read_unaligned();
                    }
                    for i in 0..5 {
                        slot.add(i).write_unaligned(local_8.add(i).read_unaligned());
                    }
                    let other = ((p + 0xa4) as *const *mut u32).read_unaligned();
                    for i in 0..5 {
                        other
                            .add(i)
                            .write_unaligned(local_8.add(i).read_unaligned());
                    }
                    FUN_100682e7(param_1);
                    let idx = ((p + 0xc) as *const i32).read_unaligned();
                    let slot = (&raw mut DAT_100f3040 as *mut u8)
                        .wrapping_offset(idx.wrapping_mul(0x14) as isize)
                        as *mut u32;
                    for i in 0..5 {
                        slot.add(i).write_unaligned(local_20[i]);
                    }
                    let other = ((p + 0xa4) as *const *mut u32).read_unaligned();
                    for i in 0..5 {
                        other.add(i).write_unaligned(local_34[i]);
                    }
                }
            }
            ((p + 0xb0) as *mut u32).write_unaligned(DAT_10114ec8);
        }
    }
}
