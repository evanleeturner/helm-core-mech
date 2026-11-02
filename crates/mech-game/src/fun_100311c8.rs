// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10115100: u8;
    static mut DAT_10169600: u8;
    static mut DAT_101f52e0: u8;
    static mut PTR_DAT_100aaf68: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10017bc1(param_1: i32, param_2: u32, param_3: i32, param_4: i32) -> u32;
    fn FUN_1007b068(
        param_1: *mut i32,
        param_2: *mut i32,
        param_3: i32,
        param_4: u32,
        param_5: u32,
        param_6: u32,
        param_7: u32,
        param_8: u32,
    ) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100311c8(param_1: i32, param_2: u32) {
    let mut local_1f0: [i32; 120] = [0; 120];
    // SAFETY: the globals and callees are the game's own memory; the pointers
    // read and written are the game's tables and the local buffer, accessed
    // in the same order and extent as the original.
    unsafe {
        let base = PTR_DAT_100aaf68;
        let a = (base.wrapping_add(0x3c) as *const i32).read_unaligned();
        let b = (base.wrapping_add(0x40) as *const i32).read_unaligned();
        let u_var2 = FUN_10017bc1(param_1, param_2, a, b);
        if 2 < u_var2 {
            let mut local_c = (&raw mut DAT_10115100) as *const i32;
            let mut local_8 = local_1f0.as_mut_ptr();
            let mut local_10: u32 = 0;
            while local_10 < u_var2 {
                let i_var1 = local_c.read_unaligned();
                local_c = local_c.wrapping_add(1);
                let p = i_var1 as u32 as usize;
                local_8.write_unaligned(((p.wrapping_add(0xc)) as *const i32).read_unaligned());
                local_8
                    .wrapping_add(1)
                    .write_unaligned(((p.wrapping_add(0x10)) as *const i32).read_unaligned());
                local_8.wrapping_add(5).write_unaligned(1);
                local_8.wrapping_add(3).write_unaligned(1);
                local_8.wrapping_add(4).write_unaligned(1);
                local_8
                    .wrapping_add(2)
                    .write_unaligned(((p.wrapping_add(0x14)) as *const i32).read_unaligned());
                local_8 = local_8.wrapping_add(6);
                local_10 = local_10.wrapping_add(1);
            }
            FUN_1007b068(
                &raw mut DAT_101f52e0 as *mut i32,
                local_1f0.as_mut_ptr(),
                u_var2 as i32,
                0x40,
                0,
                0,
                (&raw mut DAT_10169600) as u32,
                0,
            );
        }
    }
}
