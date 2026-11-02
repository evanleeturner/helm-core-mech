// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf34: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000fc30(param_1: i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_10002570(param_1: u32, param_2: i32) -> u32;
    fn FUN_1007fcc3(param_1: *mut i32, param_2: *mut i32, param_3: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1007fc1d(param_1: *mut i32, param_2: *mut i32) {
    // SAFETY: the globals and callees are the game's own; param_1 and param_2
    // point at the caller's int arrays, read at the original's offsets.
    unsafe {
        if *((DAT_100adf34 as usize).wrapping_add(3) as *const i8) != 0 {
            let a = param_1
                .add(0x10d)
                .read_unaligned()
                .wrapping_sub(param_2.add(0x10d).read_unaligned());
            let b = param_1
                .add(0x10e)
                .read_unaligned()
                .wrapping_sub(param_2.add(0x10e).read_unaligned());
            let c = param_1
                .add(0x10f)
                .read_unaligned()
                .wrapping_sub(param_2.add(0x10f).read_unaligned());
            let i_var1 = FUN_1000fc30(a, b, c);
            if 200000 < i_var1 {
                let r = FUN_10002570(i_var1.wrapping_sub(200000) as u32, 1300000) as i32;
                FUN_1007fcc3(param_1, param_2, r.wrapping_mul(3));
            }
        }
    }
}
