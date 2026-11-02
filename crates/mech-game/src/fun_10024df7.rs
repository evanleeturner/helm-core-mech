// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38fc: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002f3a7(param_1: i32, param_2: i32, param_3: i32, param_4: i32) -> u8;
    fn FUN_1002fbc0(param_1: *mut i32) -> u8;
    fn FUN_1002f727(param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10024df7(param_1: i32) -> bool {
    // SAFETY: param_1 is the caller's object address; the globals and callees
    // are the game's own, accessed in the original's order.
    unsafe {
        let base = param_1 as usize as *mut u8;
        let i_var1 =
            (DAT_100b38fc as i32).wrapping_sub(base.byte_add(0x1de).cast::<i32>().read_unaligned());
        if i_var1 < 0x2e {
            let a = base.byte_add(0x1d2).cast::<i32>().read_unaligned();
            let b = base.byte_add(0x1ea).cast::<i32>().read_unaligned();
            let c = base.byte_add(0x1ee).cast::<i32>().read_unaligned();
            let d = base.byte_add(0x1f2).cast::<i32>().read_unaligned();
            FUN_1002f3a7(a, b, c, d);
            let p = base.byte_add(0x1d2).cast::<i32>().read_unaligned();
            FUN_1002fbc0(p as usize as *mut i32);
            base.byte_add(0x1e6).cast::<u32>().write_unaligned(3);
        } else {
            let p = base.byte_add(0x10).cast::<u16>();
            p.write_unaligned(p.read_unaligned() & 0xfffe);
            let q = base.byte_add(0x1d2).cast::<i32>().read_unaligned();
            FUN_1002f727(q);
        }
        i_var1 < 0x2e
    }
}
