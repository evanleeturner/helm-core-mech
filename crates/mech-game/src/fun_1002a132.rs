// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf38: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002a132(param_1: i32) -> i32 {
    // SAFETY: param_1 points at the caller's game object; the offsets read
    // are the original's, and the global is the game's own.
    unsafe {
        let base = param_1 as usize as *const u8;
        let mut local_8: i32;
        if (base.byte_add(4).cast::<i32>().read_unaligned() as u32) == DAT_100adf38 {
            let p = base.byte_add(0x20).cast::<i32>().read_unaligned() as usize as *const u8;
            if p.byte_add(0x3f0).cast::<i32>().read_unaligned() != 0 {
                local_8 = base
                    .byte_add(0xfc)
                    .cast::<i32>()
                    .read_unaligned()
                    .wrapping_sub(base.byte_add(0x60).cast::<i32>().read_unaligned());
            } else {
                local_8 = base
                    .byte_add(0xd4)
                    .cast::<i32>()
                    .read_unaligned()
                    .wrapping_sub(base.byte_add(0x60).cast::<i32>().read_unaligned());
            }
        } else {
            local_8 = base
                .byte_add(0xd4)
                .cast::<i32>()
                .read_unaligned()
                .wrapping_sub(base.byte_add(0x60).cast::<i32>().read_unaligned());
        }
        if local_8 < 0xb40001 {
            if local_8 < -0xb40000 {
                local_8 = local_8.wrapping_add(0x1680000);
            }
        } else {
            local_8 = local_8.wrapping_add(-0x1680000);
        }
        local_8
    }
}
