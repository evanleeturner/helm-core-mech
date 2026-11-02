// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38e0: u32;
    static mut DAT_100b38e4: u32;
    static mut DAT_100b38e8: u32;
    static mut DAT_100c7fa8: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100655f6(param_1: u32, param_2: u8) -> u32 {
    // SAFETY: the globals are the game's own; the tables they point at are the
    // game's buffers, accessed at the same offsets as the original.
    unsafe {
        if DAT_100c7fa8 < 0x1000 {
            let mut local_8: u32 = (u32::from(param_2) << 8) ^ param_1;
            loop {
                local_8 = local_8 % 0x26f5;
                let p = DAT_100b38e8.wrapping_add(local_8.wrapping_mul(4)) as *const i32;
                if p.read_unaligned() == 0xffff {
                    break;
                }
                local_8 = local_8.wrapping_add(0x7f7);
            }
            let p = DAT_100b38e8.wrapping_add(local_8.wrapping_mul(4)) as *mut u32;
            p.write_unaligned(DAT_100c7fa8);
            let b = DAT_100c7fa8.wrapping_add(DAT_100b38e0) as *mut u8;
            b.write(param_2);
            let q = DAT_100b38e4.wrapping_add(DAT_100c7fa8.wrapping_mul(4)) as *mut u32;
            if param_1 == 0xffff {
                q.write_unaligned(0xffff);
            } else {
                q.write_unaligned(param_1);
            }
            let u_var1 = DAT_100c7fa8;
            DAT_100c7fa8 = DAT_100c7fa8.wrapping_add(1);
            u_var1
        } else {
            0xffff
        }
    }
}
