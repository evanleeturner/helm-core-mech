// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aaf68: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10002570(param_1: u32, param_2: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1006764b(param_1: i32, param_2: i32, param_3: i32) -> bool {
    // SAFETY: param_1 points at the caller's rect-like structure, and the
    // global holds a pointer to the game's own table; all reads happen in the
    // original's order.
    unsafe {
        let p = param_1 as u32;
        let r_c = ((p.wrapping_add(0xc)) as *const i32).read_unaligned();
        let r_4 = ((p.wrapping_add(4)) as *const i32).read_unaligned();
        let i_var1 = (r_c.wrapping_sub(r_4).wrapping_add(1) / 2).wrapping_sub(1);

        let r_c2 = ((p.wrapping_add(0xc)) as *const i32).read_unaligned();
        let r_42 = ((p.wrapping_add(4)) as *const i32).read_unaligned();
        let w = r_c2.wrapping_sub(r_42).wrapping_add(1) >> 1;
        let i_var2 = param_2.wrapping_sub(w);

        let r_10 = ((p.wrapping_add(0x10)) as *const i32).read_unaligned();
        let r_8 = ((p.wrapping_add(8)) as *const i32).read_unaligned();
        let h = r_10.wrapping_sub(r_8).wrapping_add(1) >> 1;
        let arg1 = param_3.wrapping_sub(h) as u32;
        let arg2 = ((DAT_100aaf68.wrapping_add(0x44)) as *const i32).read_unaligned();
        let i_var3 = FUN_10002570(arg1, arg2) as i32;

        i_var3
            .wrapping_mul(i_var3)
            .wrapping_add(i_var2.wrapping_mul(i_var2))
            <= i_var1.wrapping_mul(i_var1)
    }
}
