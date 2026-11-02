// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38fc: i32;
    static mut DAT_100a1208: f64;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002950f(param_1: i32, param_2: i16);
    fn FUN_1002a5d4(param_1: i32);
    fn FUN_1002a6ac(param_1: i32);
    fn FUN_1002a132(param_1: i32) -> i32;
    fn FUN_10022b70(param_1: i32, param_2: i32);
    fn FUN_10044054(param_1: *mut i32) -> i32;
    fn FUN_100295f4(param_1: i32, param_2: i32) -> u64;
    fn FUN_100297b4(param_1: i32);
    fn FUN_10044a83(param_1: i32);
    fn FUN_1002a89b(param_1: i32, param_2: i16);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10043bff(param_1: *mut i32) -> i32 {
    // SAFETY: param_1 points at the game's object; the globals and callees
    // are the game's own, accessed in the original order.
    unsafe {
        let p = param_1 as i32;
        FUN_1002950f(p, param_1.byte_add(0x17a).cast::<i16>().read_unaligned());
        if param_1.add(0x74).read_unaligned() < DAT_100b38fc {
            if DAT_100a1208 < (param_1.add(0x30).read_unaligned() as f64) {
                FUN_1002a5d4(p);
                FUN_1002a6ac(p);
            }
            param_1
                .add(0x74)
                .write_unaligned(DAT_100b38fc.wrapping_add(0x5a8));
        }
        let i_var2 = FUN_1002a132(p);
        FUN_10022b70(p, i_var2);
        FUN_1002950f(p, param_1.add(0x5e).read_unaligned() as i16);
        if FUN_10044054(param_1) == 0 {
            let u_var3 = FUN_100295f4(p, 3000);
            let base = param_1.add(0x13).read_unaligned();
            (base.wrapping_add(8) as *mut i32).write_unaligned(u_var3 as i32);
            FUN_100297b4(p);
        }
        FUN_10044a83(p);
        if param_1.add(0x30).read_unaligned() < 0xbb9 {
            FUN_1002a89b(p, param_1.add(0x5e).read_unaligned() as i16);
        }
        0
    }
}
