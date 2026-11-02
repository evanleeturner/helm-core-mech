// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf38: i32;
    static mut DAT_100eef40: [i32; 1];
    static mut DAT_100aa568: i32;
    static mut _DAT_100aa56c: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10011e60(a: i32, b: i32, c: i32) -> i32;
    fn FUN_10067d51(a: *mut i32) -> i32;
    fn FUN_10016cdd(a: *mut i32, b: i32, c: i32, d: i32, e: i32, f: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001648d(param_1: i32) {
    // SAFETY: param_1 is the game's object pointer; the globals and callees
    // are the game's own, accessed in the original's order.
    unsafe {
        let base = param_1 as usize;
        let pi_var1 = ((base + 0xa4) as *const i32).read_unaligned() as *mut i32;
        let mut i_var8 = ((base + 0x74) as *const i32).read_unaligned();
        let i_var2 = ((base + 0x78) as *const i32).read_unaligned();
        let i_var3 = ((base + 0x7c) as *const i32).read_unaligned();
        let i_var4 = ((base + 0x74) as *const i32).read_unaligned();
        let i_var5 = ((base + 0x78) as *const i32).read_unaligned();
        let mut i_var7 = ((base + 0x7c) as *const i32).read_unaligned();
        let i_var6 = ((base + 0x80) as *const i32)
            .read_unaligned()
            .wrapping_sub(1);

        let idx = DAT_100adf38;
        let arr = (&raw mut DAT_100eef40).cast::<i32>();
        let entry = arr.offset(idx as isize).read_unaligned();
        let inner = ((entry as usize + 0x20) as *const i32).read_unaligned();
        let val = ((inner as usize + 0x3cc) as *const i32).read_unaligned();
        _DAT_100aa56c = FUN_10011e60(val, i_var7, 100);

        let mut local_40 = FUN_10067d51(&raw mut DAT_100aa568);
        local_40 >>= 0x10;
        if local_40 < i_var7 {
            if local_40 < 1 {
                FUN_10016cdd(pi_var1, i_var8, i_var2, i_var3, i_var6, 7);
            } else {
                let local_4c;
                if local_40 < (i_var7 >> 1) {
                    local_4c = 7;
                } else {
                    local_40 = i_var7.wrapping_sub(local_40);
                    local_4c = 0xb;
                }
                i_var7 = i_var7.wrapping_add(local_40.wrapping_mul(-2));
                i_var8 = local_40.wrapping_add(i_var4).wrapping_add(1);
                FUN_10016cdd(pi_var1, i_var4, i_var5, local_40, i_var6, 3);
                FUN_10016cdd(pi_var1, i_var8, i_var5, i_var7, i_var6, local_4c);
                FUN_10016cdd(
                    pi_var1,
                    i_var8.wrapping_add(i_var7),
                    i_var5,
                    local_40,
                    i_var6,
                    3,
                );
            }
        } else {
            FUN_10016cdd(pi_var1, i_var8, i_var2, i_var3, i_var6, 0xb);
        }
    }
}
