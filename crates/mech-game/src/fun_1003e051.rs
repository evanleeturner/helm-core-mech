// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aefc0: u32;
    static mut DAT_100c6d48: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1003e369(param_1: i32) -> u8;
    fn FUN_1002f322(param_1: i32, param_2: *mut u32, param_3: *mut u32, param_4: *mut u32) -> u8;
    fn FUN_1003e3e3(param_1: i32, param_2: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003e051(param_1: *mut u16) {
    let mut local_18: u32 = 0;
    let mut local_14: i32 = 0;
    let mut local_10: i32 = 0;
    if param_1.is_null() {
        return;
    }
    // SAFETY: param_1 points at the caller's record; the callees and globals
    // are the game's own, accessed in the original's order.
    unsafe {
        let w0 = param_1.read_unaligned();
        if (w0 & 0x1000) == 0 {
            return;
        }
        let w1 = param_1.byte_add(2).read_unaligned();
        if ((w1 as u8) & 0xf0) == 0x70 {
            return;
        }
        param_1.write_unaligned(w0 & 0xefff);
        if param_1.byte_add(4).cast::<i32>().read_unaligned() == 0 {
            return;
        }
        FUN_1003e369(param_1 as i32);
        if DAT_100aefc0 == 0 {
            FUN_1003e3e3(param_1 as i32, 0x100c6d00);
        } else {
            let p = param_1.byte_add(0x18).cast::<i32>().read_unaligned();
            FUN_1002f322(
                p,
                (&raw mut local_14).cast::<u32>(),
                &raw mut local_18,
                (&raw mut local_10).cast::<u32>(),
            );
            let local_8 = local_14.wrapping_add(150000) / 25000;
            let local_c = local_10.wrapping_add(150000) / 25000;
            if local_8 < 0 || 0xb < local_8 || local_c < 0 || 0xb < local_c {
                param_1.byte_add(0x44).write_unaligned(9999u16);
                param_1.byte_add(0x46).write_unaligned(0u16);
                param_1.byte_add(0x48).write_unaligned(9999u16);
                param_1.byte_add(0x4a).write_unaligned(0u16);
                FUN_1003e3e3(param_1 as i32, 0x100c6d18);
            } else {
                param_1
                    .byte_add(0x44)
                    .cast::<i32>()
                    .write_unaligned(local_8);
                param_1
                    .byte_add(0x48)
                    .cast::<i32>()
                    .write_unaligned(local_c);
                let base = (&raw mut DAT_100c6d48) as i32;
                let addr = base
                    .wrapping_add(local_8.wrapping_mul(0x120))
                    .wrapping_add(local_c.wrapping_mul(0x18));
                FUN_1003e3e3(param_1 as i32, addr);
            }
        }
    }
}
