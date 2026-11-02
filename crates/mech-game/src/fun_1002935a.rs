// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002a4e0(param_1: i32, param_2: i16, param_3: u16, param_4: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002935a(param_1: i32) -> bool {
    // SAFETY: param_1 is the address of the game's own object; the fields
    // are read and written where the original does, and the callee is the
    // game's own.
    unsafe {
        let base = param_1 as *mut u8;
        let cnt = base.byte_add(0x16c).cast::<i16>();
        let b_var2 = cnt.read_unaligned() != 0;
        if b_var2 {
            cnt.write_unaligned(cnt.read_unaligned().wrapping_sub(1));
            let idx = base.byte_add(0x16c).cast::<u16>().read_unaligned() as usize;
            let u_var1 = base
                .byte_add(0x154)
                .byte_add(idx.wrapping_mul(4))
                .cast::<u32>()
                .read_unaligned();
            let u_stack_6 = (u_var1 >> 0x10) as u16;
            FUN_1002a4e0(param_1, u_var1 as i16, u_stack_6, 0);
        }
        !b_var2
    }
}
