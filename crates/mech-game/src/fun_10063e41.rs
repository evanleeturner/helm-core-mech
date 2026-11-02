// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf38: u32;
    static mut DAT_100b3788: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10067af1(param_1: *mut i32) -> i32;
    fn FUN_10063fbd(param_1: i32, param_2: *mut i32) -> u8;
    fn FUN_10014768(param_1: *mut i32, param_2: *mut i32, param_3: u32) -> u32;
    fn FUN_1002634e(param_1: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10063e41(param_1: i32) -> i32 {
    let mut local_4c: [i32; 14] = [0; 14];
    let mut local_10: i32 = 0;

    // SAFETY: param_1 is the caller's object pointer held as an integer; the
    // offsets are read and written where the original does, and the callees
    // and globals are the game's own.
    unsafe {
        FUN_10067af1(param_1.wrapping_add(0xa8) as usize as *mut i32);
        FUN_10063fbd(param_1, local_4c.as_mut_ptr());
        let key = ((param_1.wrapping_add(4)) as usize as *const i32).read_unaligned();
        let local_8 = FUN_10014768(local_4c.as_mut_ptr(), &raw mut local_10, key as u32);
        if local_8 != 0 && local_10 != 0 {
            let local_c = ((local_10 as usize).wrapping_add(2) as *const u16).read_unaligned();
            if (local_c & 0x100) != 0 || (local_c & 0x200) != 0 {
                let local_14 = FUN_1002634e(local_4c.as_mut_ptr() as usize as i32) as i32;
                let mut i_var1 = local_14;
                if local_14 < 0x7d1 {
                    i_var1 = 2000;
                }
                (param_1.wrapping_add(0xb0) as usize as *mut i32).write_unaligned(i_var1);
                let key2 = ((param_1.wrapping_add(4)) as usize as *const i32).read_unaligned();
                if (key2 as u32) == DAT_100adf38 {
                    DAT_100b3788 = local_10 as u32;
                }
            }
        }
        let v = (param_1.wrapping_add(0xb0) as usize as *const u32).read_unaligned();
        (param_1.wrapping_add(0x9c) as usize as *mut u32).write_unaligned(v);
        FUN_10067af1(param_1.wrapping_add(0x98) as usize as *mut i32);
    }
    local_10
}
