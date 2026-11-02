// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10093b40: [u32; 1];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10036540(param_1: i32, param_2: i32, param_3: i32) -> *mut i32 {
    let mut pi_var1: *mut i32;
    // SAFETY: param_1 and param_3 are addresses of the game's own chunk data,
    // and DAT_10093b40 is the game's table; all are read where the original reads.
    unsafe {
        if param_3 == 0 {
            pi_var1 = param_1.wrapping_add(0xc) as *mut i32;
        } else {
            let off = ((param_3.wrapping_add(4)) as *const i32).read_unaligned();
            pi_var1 = off.wrapping_add(param_3) as *mut i32;
        }
        loop {
            let len = ((param_1.wrapping_add(4)) as *const i32).read_unaligned();
            let end = len.wrapping_add(param_1) as u32;
            if end <= pi_var1 as u32 {
                return core::ptr::null_mut();
            }
            let key = (&raw mut DAT_10093b40)
                .cast::<u32>()
                .offset(param_2 as isize)
                .read_unaligned();
            if (pi_var1.read_unaligned() as u32) == key {
                break;
            }
            let v = pi_var1.byte_add(4).read_unaligned();
            let adj = v.wrapping_add(((v >> 0x1f) as u32 & 3) as i32);
            pi_var1 = pi_var1.wrapping_offset((adj >> 2) as isize);
        }
    }
    pi_var1
}
