// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aafdc: u32;
    static mut DAT_100aafe0: u32;
    static mut DAT_100af17c: u32;
    static mut DAT_100b351c: u32;
    static mut DAT_100b352c: *mut u16;
    static mut DAT_101f52e0: [u8; 0];
    static mut PTR_DAT_100aaf68: u32;
    static mut PTR_DAT_100af100: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1005f9f7(param_1: i32, param_2: *mut u16, param_3: *mut u16) -> *mut u16;
    fn FUN_1008668b(param_1: u32, param_2: u32, param_3: *mut i32) -> *mut u16;
    fn FUN_1001a040(param_1: u32, param_2: u32) -> u32;
    fn FUN_1001a060(param_1: i32, param_2: i32) -> i32;
    fn FUN_1000c3a0(
        param_1: *mut i32,
        param_2: *mut u8,
        param_3: i16,
        param_4: i16,
        param_5: i32,
        param_6: *mut u32,
        param_7: u32,
        param_8: u32,
        param_9: i32,
    ) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001793d(
    param_1: i32,
    param_2: *mut i32,
    param_3: i32,
    param_4: i32,
) -> u32 {
    let mut local_208: [u16; 2] = [0; 2];
    let mut local_1f4: [u32; 120] = [0; 120];
    let mut local_c: [u16; 2] = [0; 2];

    // SAFETY: the callees and globals are the game's own; param_2 points at
    // param_3 object pointers whose fields are read at the original offsets.
    unsafe {
        let local_14 = FUN_1005f9f7(
            param_1.wrapping_add(0x100),
            local_c.as_mut_ptr(),
            local_208.as_mut_ptr(),
        );
        if local_14.is_null() {
            return 0;
        }
        if param_4 != 0 {
            DAT_100aafdc = DAT_100aafdc.wrapping_add(DAT_100aafe0);
            if (DAT_100aafdc as i32) < 0 || 0xf < DAT_100aafdc {
                DAT_100aafe0 = DAT_100aafe0.wrapping_neg();
                DAT_100aafdc = DAT_100aafdc.wrapping_add(DAT_100aafe0);
            }
        }
        if DAT_100b352c.is_null() {
            DAT_100b352c = FUN_1008668b(
                DAT_100af17c,
                DAT_100b351c,
                PTR_DAT_100af100 as usize as *mut i32,
            );
        }
        let local_8 = DAT_100b352c.wrapping_add(DAT_100aafdc.wrapping_mul(0x80) as usize);
        let mut local_1f8 = param_3;
        let mut local_1fc = local_1f4.as_mut_ptr();
        let mut local_200 = param_2;
        while local_1f8 != 0 {
            let local_10 = local_200.read_unaligned();
            local_200 = local_200.wrapping_add(1);
            let p = local_10 as usize as *const u8;
            let local_204 = p.byte_add(0xc).cast::<u32>().read_unaligned();
            let u_var2 = p.byte_add(0x10).cast::<u32>().read_unaligned();
            local_1fc.write_unaligned(local_204);
            local_1fc.add(1).write_unaligned(u_var2);
            local_1fc
                .add(3)
                .write_unaligned(p.byte_add(0x14).cast::<u32>().read_unaligned());
            local_1fc
                .add(4)
                .write_unaligned(p.byte_add(0x18).cast::<u32>().read_unaligned());
            local_1fc
                .add(5)
                .write_unaligned(p.byte_add(8).cast::<u32>().read_unaligned());
            let v = p.byte_add(0x2c).cast::<i32>().read_unaligned();
            local_1fc.add(2).write_unaligned(v.wrapping_shl(16) as u32);
            local_1fc = local_1fc.add(6);
            local_1f8 = local_1f8.wrapping_sub(1);
        }
        local_1fc = local_1f4.as_mut_ptr();
        local_1f8 = 0;
        while local_1f8 < param_3 {
            let a = ((PTR_DAT_100aaf68.wrapping_add(0x94)) as usize as *const u32).read_unaligned();
            let mut u = FUN_1001a040(a, local_1fc.add(5).read_unaligned());
            local_1fc.add(5).write_unaligned(u);
            u = FUN_1001a060(
                local_1fc.add(3).read_unaligned() as i32,
                local_1fc.add(5).read_unaligned() as i32,
            ) as u32;
            local_1fc.add(3).write_unaligned(u);
            u = FUN_1001a060(
                local_1fc.add(4).read_unaligned() as i32,
                local_1fc.add(5).read_unaligned() as i32,
            ) as u32;
            local_1fc.add(4).write_unaligned(u);
            local_1fc = local_1fc.add(6);
            local_1f8 = local_1f8.wrapping_add(1);
        }
        FUN_1000c3a0(
            &raw mut DAT_101f52e0 as *mut i32,
            local_14 as *mut u8,
            local_c[0] as i16,
            local_208[0] as i16,
            param_3,
            local_1f4.as_mut_ptr(),
            0,
            local_8 as usize as u32,
            0,
        );
        1
    }
}
