// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa1c0: u32;
    static mut DAT_100aa1e8: *mut u16;
    static mut DAT_100aa2bc: u16;
    static mut DAT_100ef030: [u8; 0];
    static mut DAT_101eb51c: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10006e00(param_1: u32, param_2: u32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10013817() {
    // SAFETY: the globals and the callee are the game's own; the buffer
    // pointed to by DAT_100aa1e8 and the table at DAT_100ef030 are the
    // game's memory, accessed at the same offsets as the original.
    unsafe {
        let p_var1 = DAT_100aa1e8;
        let mut local_10: i32 = 0;
        let mut local_14: u8 = 0;
        p_var1.write_unaligned(DAT_100aa2bc);
        p_var1
            .byte_add(2)
            .cast::<u32>()
            .write_unaligned(DAT_100aa1c0);
        let mut local_8: *mut u8 = p_var1.byte_add(6).cast::<u8>();
        let mut local_c: i32 = 0;
        while local_c < 0xfe {
            local_14 = local_14.wrapping_shl(1);
            let v = (&raw mut DAT_100ef030)
                .cast::<u8>()
                .byte_add((local_c as usize).wrapping_mul(0x40))
                .cast::<u16>()
                .read_unaligned();
            if (v & 4) != 0 {
                local_14 |= 1;
            }
            local_10 = local_10.wrapping_add(1);
            if local_10 == 8 {
                local_10 = 0;
                local_8.write(local_14);
                local_8 = local_8.add(1);
                local_14 = 0;
            }
            local_c = local_c.wrapping_add(1);
        }
        FUN_10006e00(DAT_100aa1e8 as u32, DAT_101eb51c);
    }
}
