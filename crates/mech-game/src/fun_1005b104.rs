// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005b104(param_1: i32) -> u32 {
    // SAFETY: param_1 is the address of a rectangle-like structure the caller
    // owns; its 16-bit fields at offsets 4, 6, 8 and 10 are read.
    unsafe {
        let p = param_1 as *const u8;
        let f4 = p.byte_add(4).cast::<i16>().read_unaligned();
        let f6 = p.byte_add(6).cast::<i16>().read_unaligned();
        let f8 = p.byte_add(8).cast::<i16>().read_unaligned();
        let f10 = p.byte_add(10).cast::<i16>().read_unaligned();
        let u_var1 = ((f8 as i32 as u32)
            .wrapping_sub(f4 as i32 as u32)
            .wrapping_add(1)) as u16;
        let u_var2 = ((f10 as i32 as u32)
            .wrapping_sub(f6 as i32 as u32)
            .wrapping_add(1)) as u16;
        (u32::from(u_var1) << 16) | u32::from(u_var2)
    }
}
