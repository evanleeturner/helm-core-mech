// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf38: i32;
    static mut DAT_100eef40: i32;
    static mut DAT_100af17c: i32;
    static mut DAT_101686f0: i32;
    static mut DAT_100af0b8: i32;
    static mut DAT_10114e08: i32;
    static mut DAT_100eebbe: u8;
    static mut DAT_100eebb0: i32;
    static mut DAT_100ad9d0: [i8; 0];
    static mut DAT_100ad9d8: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000fc30(param_1: i32, param_2: i32, param_3: i32);
    fn __ftol() -> i64;
    fn FUN_1008668b(param_1: i32, param_2: i32, param_3: *mut i32) -> *mut u16;
    fn _sprintf(param_1: *mut i8, param_2: *const i8, ...) -> i32;
    fn FUN_1005acaa(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: *mut i8,
        param_6: i32,
    );
    fn FUN_1005aae4(param_1: i32) -> i32;
    fn FUN_1008664f(param_1: i32, param_2: *mut i32);
    fn FUN_10016801(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002e822(param_1: i32) {
    let mut local_4c: [i8; 64] = [0; 64];
    // SAFETY: all globals, callees and pointers are the game's own memory;
    // the offsets read mirror the original's structure accesses.
    unsafe {
        if ((param_1.wrapping_add(4)) as *const i16).read_unaligned() != 0 {
            let idx = DAT_100adf38;
            let base = (&raw mut DAT_100eef40).wrapping_offset(idx as isize);
            let local_c: i32 =
                (base.read_unaligned().wrapping_add(0x20) as *const i32).read_unaligned();
            FUN_1000fc30(
                (local_c.wrapping_add(0x428) as *const i32).read_unaligned(),
                (local_c.wrapping_add(0x42c) as *const i32).read_unaligned(),
                (local_c.wrapping_add(0x430) as *const i32).read_unaligned(),
            );
            let l_var3 = __ftol();
            let mut local_8 = l_var3 as i32;
            if (local_c.wrapping_add(0x2c) as *const i32).read_unaligned() < 0 {
                local_8 = local_8.wrapping_neg();
                DAT_100eebbe = 6;
            } else {
                DAT_100eebbe = 0xe;
            }
            let p_var1 = FUN_1008668b(
                DAT_100af17c,
                DAT_101686f0.wrapping_add(1),
                DAT_100af0b8 as *mut i32,
            );
            if !p_var1.is_null() {
                let buf = local_4c.as_mut_ptr();
                _sprintf(buf, (&raw const DAT_100ad9d0).cast::<i8>());
                let a = (param_1.wrapping_add(0xa4) as *const i32).read_unaligned() as *mut i32;
                let p1 = (DAT_10114e08.wrapping_add(0xa8) as *const i32).read_unaligned();
                let b = (p1 as *const i32).read_unaligned();
                let p2 = (DAT_10114e08.wrapping_add(0xa8) as *const i32).read_unaligned();
                let c = (p2.wrapping_add(4) as *const i32).read_unaligned();
                FUN_1005acaa(a, b, c, p_var1 as i32, buf, &raw mut DAT_100eebb0 as i32);
                _sprintf(buf, (&raw const DAT_100ad9d8).cast::<i8>(), local_8);
                let i_var2 = FUN_1005aae4(p_var1 as i32);
                let a = (param_1.wrapping_add(0xa4) as *const i32).read_unaligned() as *mut i32;
                let p1 = (DAT_10114e08.wrapping_add(0xa8) as *const i32).read_unaligned();
                let b = (p1 as *const i32).read_unaligned();
                let p2 = (DAT_10114e08.wrapping_add(0xa8) as *const i32).read_unaligned();
                let c = (p2.wrapping_add(4) as *const i32)
                    .read_unaligned()
                    .wrapping_add(i_var2);
                FUN_1005acaa(a, b, c, p_var1 as i32, buf, &raw mut DAT_100eebb0 as i32);
                DAT_100eebbe = 0xe;
                FUN_1008664f(DAT_101686f0.wrapping_add(1), DAT_100af0b8 as *mut i32);
                FUN_10016801(param_1);
            }
        }
    }
}
