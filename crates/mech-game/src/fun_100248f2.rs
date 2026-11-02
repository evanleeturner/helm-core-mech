// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38fc: i32;
    static mut DAT_1016acb8: [u8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10051f43(param_1: i32) -> i32;
    fn FUN_1002f3a7(param_1: i32, param_2: i32, param_3: i32, param_4: i32);
    fn FUN_1002fbc0(param_1: i32);
    fn __ftol() -> i64;
    fn FUN_1002f727(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100248f2(param_1: i32) -> bool {
    let base = param_1 as usize as *mut u8;
    // SAFETY: param_1 points at the game's object; the fields, globals and
    // callees are the game's own, accessed in the original's order.
    unsafe {
        let v_1de = base.byte_add(0x1de).cast::<i32>().read_unaligned();
        let i_var1 = DAT_100b38fc.wrapping_sub(v_1de);
        if i_var1 < 0x2e {
            let v_1f2 = base.byte_add(0x1f2).cast::<i32>().read_unaligned();
            let i_var4 = v_1f2;
            let i_var2 = FUN_10051f43(10);
            let v_1d2 = base.byte_add(0x1d2).cast::<i32>().read_unaligned();
            let v_1ea = base.byte_add(0x1ea).cast::<i32>().read_unaligned();
            let v_1ee = base.byte_add(0x1ee).cast::<i32>().read_unaligned();
            FUN_1002f3a7(v_1d2, v_1ea, v_1ee.wrapping_add(i_var2), i_var4);
            FUN_1002fbc0(v_1d2);
            let l_var3 = __ftol();
            let tbl = (&raw mut DAT_1016acb8).cast::<u8>();
            let v_1e6 = tbl.offset((l_var3 as i32) as isize).read() as u32 as i32;
            base.byte_add(0x1e6).cast::<i32>().write_unaligned(v_1e6);
        } else {
            let p = base.byte_add(0x10).cast::<u16>();
            p.write_unaligned(p.read_unaligned() & 0xfffe);
            let v_1d2 = base.byte_add(0x1d2).cast::<i32>().read_unaligned();
            FUN_1002f727(v_1d2);
        }
        i_var1 < 0x2e
    }
}
