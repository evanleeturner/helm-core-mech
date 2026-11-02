// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a9df0: i32;
    static mut DAT_101ec450: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1001046c() -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100103e0(param_1: u32, param_2: u16, param_3: u16) -> i32 {
    // SAFETY: the globals and the callee are the game's own; the table write
    // offset is bounded by the index check (at most 0x5f * 8 + 7).
    unsafe {
        if DAT_100a9df0 == -1 {
            FUN_1001046c();
        }
        if DAT_100a9df0 == -1 || 0x5f < DAT_100a9df0 {
            -1
        } else {
            let i_var1 = (DAT_100a9df0 as u32).wrapping_mul(8) as i32;
            let base = (&raw mut DAT_101ec450).cast::<u8>();
            base.byte_add(i_var1 as usize)
                .cast::<u32>()
                .write_unaligned(param_1 & 0xffff);
            base.byte_add(i_var1 as usize + 4)
                .cast::<u16>()
                .write_unaligned(param_2);
            base.byte_add(i_var1 as usize + 6)
                .cast::<u16>()
                .write_unaligned(param_3);
            let r = DAT_100a9df0;
            DAT_100a9df0 = (DAT_100a9df0 as u32).wrapping_add(1) as i32;
            r
        }
    }
}
