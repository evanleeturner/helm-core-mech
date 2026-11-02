// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b9a18: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100717ee(param_1: i32) {
    // SAFETY: param_1 is the address of the game's own structure; the bytes it
    // points at are read and written where the original does, and the callback
    // table address comes from the game's global.
    unsafe {
        let base = (param_1.wrapping_add(4) as *const u32).read_unaligned();
        let idx = ((param_1.wrapping_add(8) as *const u8).read()) as u32;
        let puvar1 = idx.wrapping_mul(3).wrapping_add(base) as *mut u8;
        let mut aux: [u8; 4] = [0; 4];
        let mut local_8 = puvar1;
        let mut local_10: i32 = 0;
        while local_10 < 3 {
            aux[local_10 as usize] = local_8.read();
            local_8 = local_8.wrapping_add(1);
            local_10 = local_10.wrapping_add(1);
        }
        local_8 = puvar1;
        local_10 = 0;
        let count = (param_1.wrapping_add(9) as *const i32)
            .read_unaligned()
            .wrapping_mul(3)
            .wrapping_sub(3);
        while local_10 < count {
            local_8.write(local_8.wrapping_add(3).read());
            local_8 = local_8.wrapping_add(1);
            local_10 = local_10.wrapping_add(1);
        }
        local_10 = 0;
        while local_10 < 3 {
            local_8.write(aux[local_10 as usize]);
            local_8 = local_8.wrapping_add(1);
            local_10 = local_10.wrapping_add(1);
        }
        let fp_addr = DAT_100b9a18.wrapping_add(0x18) as *const u32;
        let fp: unsafe extern "C" fn(u32) =
            core::mem::transmute::<u32, unsafe extern "C" fn(u32)>(fp_addr.read_unaligned());
        let arg = (param_1.wrapping_add(4) as *const u32).read_unaligned();
        fp(arg);
    }
}
