// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b681c: *mut i32;
    static mut DAT_100b6820: *mut i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1007cc83(param_1: *mut i32) {
    // SAFETY: the globals are the game's own list head and tail; param_1 and
    // the linked nodes are the game's list nodes, accessed as the original does.
    unsafe {
        if !param_1.is_null() && !DAT_100b681c.is_null() && !DAT_100b6820.is_null() {
            if param_1 == DAT_100b681c {
                if param_1 == DAT_100b6820 {
                    DAT_100b681c = core::ptr::null_mut();
                    DAT_100b6820 = core::ptr::null_mut();
                } else {
                    DAT_100b681c = param_1.read_unaligned() as usize as *mut i32;
                    DAT_100b681c.add(1).write_unaligned(0);
                }
            } else if param_1 == DAT_100b6820 {
                DAT_100b6820 = param_1.add(1).read_unaligned() as usize as *mut i32;
                DAT_100b6820.write_unaligned(0);
            } else {
                let pi_var1 = param_1.add(1).read_unaligned() as usize as *mut i32;
                let i_var2 = param_1.read_unaligned();
                pi_var1.write_unaligned(i_var2);
                (i_var2 as usize as *mut i32)
                    .add(1)
                    .write_unaligned(pi_var1 as usize as i32);
            }
            param_1.add(1).write_unaligned(0);
            param_1.write_unaligned(0);
        }
    }
}
