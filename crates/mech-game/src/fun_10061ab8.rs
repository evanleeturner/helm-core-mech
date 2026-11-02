// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b36fc: u32;
    static mut DAT_100f30cc: [u8; 20];
    static mut DAT_10114ec8: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10003d80(param_1: *mut i32) -> u8;
    fn FUN_10003fc7(param_1: i32, param_2: *mut i32) -> *mut u32;
    fn FUN_1006147f(param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10061ab8(param_1: i32) {
    let mut local_34: [u32; 5] = [0; 5];
    let mut local_20: [u32; 5] = [0; 5];
    // SAFETY: all pointers are the game's own memory; param_1 is the caller's
    // object, and the copies of five dwords happen in the original's order.
    unsafe {
        if (((param_1 as u32).wrapping_add(4) as *const i16).read_unaligned() != 0)
            && DAT_100b36fc != 0
        {
            let local_c = ((param_1 as u32).wrapping_add(0xac) as *const *mut i32).read_unaligned();
            if !local_c.is_null() {
                if ((param_1 as u32).wrapping_add(0xb0) as *const i32).read_unaligned() != 1 {
                    FUN_10003d80(local_c);
                }
                let local_8 = FUN_10003fc7(0, local_c);
                if local_8.is_null() {
                    FUN_1006147f(param_1);
                } else {
                    let glob = &raw mut DAT_100f30cc as *mut u32;
                    let obj =
                        ((param_1 as u32).wrapping_add(0xa4) as *const *mut u32).read_unaligned();
                    for i in 0..5 {
                        local_20[i] = glob.add(i).read_unaligned();
                    }
                    for i in 0..5 {
                        local_34[i] = obj.add(i).read_unaligned();
                    }
                    for i in 0..5 {
                        glob.add(i).write_unaligned(local_8.add(i).read_unaligned());
                    }
                    let obj =
                        ((param_1 as u32).wrapping_add(0xa4) as *const *mut u32).read_unaligned();
                    for i in 0..5 {
                        obj.add(i).write_unaligned(local_8.add(i).read_unaligned());
                    }
                    FUN_1006147f(param_1);
                    for i in 0..5 {
                        glob.add(i).write_unaligned(local_20[i]);
                    }
                    let obj =
                        ((param_1 as u32).wrapping_add(0xa4) as *const *mut u32).read_unaligned();
                    for i in 0..5 {
                        obj.add(i).write_unaligned(local_34[i]);
                    }
                }
            }
            ((param_1 as u32).wrapping_add(0xb0) as *mut u32).write_unaligned(DAT_10114ec8);
        }
    }
}
