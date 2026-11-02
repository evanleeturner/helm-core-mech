// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000e730(param_1: u32) -> *mut u8;
    fn FUN_1006a0e7(param_1: i32, param_2: i32) -> u32;
    fn FUN_1006a267(param_1: i32) -> u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn _FILE_read_8(a: u32, b: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000f640(param_1: i32, param_2: u32) -> u32 {
    let base = param_1 as usize as *mut u8;
    // SAFETY: the callees are the game's own; param_1 points at the caller's
    // object, read and written at the same offsets as the original.
    unsafe {
        let p_var1 = FUN_1000e730(param_2);
        let i_var2 = _FILE_read_8(p_var1 as usize as u32, 0) as i32;
        base.byte_add(0x14).cast::<i32>().write_unaligned(i_var2);
        if i_var2 == 0 {
            return 0;
        }
        let u_var3 = FUN_1006a0e7(i_var2, 0);
        let i_var4 = ((u_var3 as i32) >> 0x10).wrapping_add(1);
        base.byte_add(0x28).cast::<i32>().write_unaligned(i_var4);
        let i_var2 = ((u_var3 & 0xffff) as i32).wrapping_add(1);
        base.byte_add(0x2c).cast::<i32>().write_unaligned(i_var2);
        if (base.byte_add(0x1c).cast::<u32>().read_unaligned() & 0x80) != 0 {
            let v = base.byte_add(0x20).cast::<i32>().read_unaligned();
            base.byte_add(0x20)
                .cast::<i32>()
                .write_unaligned(v.wrapping_sub(i_var4 / 2));
            let v = base.byte_add(0x24).cast::<i32>().read_unaligned();
            base.byte_add(0x24)
                .cast::<i32>()
                .write_unaligned(v.wrapping_sub(i_var2));
        }
        base.byte_add(0x3c).cast::<u32>().write_unaligned(0);
        let u_var5 = FUN_1006a267(base.byte_add(0x14).cast::<i32>().read_unaligned());
        base.byte_add(0x40).cast::<u32>().write_unaligned(u_var5);
        base.byte_add(0x18).cast::<u32>().write_unaligned(0);
        let v = base.byte_add(0x3c).cast::<u32>().read_unaligned();
        base.byte_add(0x38).cast::<u32>().write_unaligned(v);
        base.byte_add(0x48).cast::<u32>().write_unaligned(0);
        1
    }
}
