// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1011515c: u32;
    static mut DAT_100aaf68: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10003d60(param_1: i32, param_2: i32) -> i32;
    fn FUN_1002f6e8(param_1: i32) -> u8;
    fn FUN_10080919(param_1: *mut u16, param_2: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10054330(param_1: i32, param_2: i32) -> u32 {
    // SAFETY: all pointers come from the game's own structures and are
    // read and written at the offsets the original uses, in the same order.
    unsafe {
        let mut local_8: *mut u8 = core::ptr::null_mut();
        let mut local_24: *mut u8 = core::ptr::null_mut();
        if param_1 == 0 {
            return 1;
        }
        let mut local_10: *mut u8 = ((param_1 as usize).wrapping_add(0x1c) as *const u32)
            .read_unaligned() as usize as *mut u8;
        if local_10.is_null() {
            return 1;
        }
        let mut piVar1: *mut u8 = local_10;
        if local_10.cast::<i32>().read_unaligned() != 0 {
            loop {
                local_24 = piVar1;
                piVar1 = local_8;
                if local_10.is_null() {
                    break;
                }
                let base = DAT_100aaf68.wrapping_add(0xb8) as usize as *const i32;
                let iVar3 = FUN_10003d60(
                    base.read_unaligned(),
                    local_10.cast::<i32>().read_unaligned(),
                );
                if param_2 < iVar3 {
                    local_8 = local_24;
                    piVar1 = local_8;
                    break;
                }
                local_24 = local_10;
                piVar1 = local_24;
                local_10 = local_10.byte_add(12).cast::<u32>().read_unaligned() as usize as *mut u8;
            }
        }
        local_8 = piVar1;
        if local_8.is_null() {
            local_8 = local_24;
        }
        if local_8.is_null() {
            return 1;
        }
        ((param_1 as usize).wrapping_add(0x20) as *mut u32)
            .write_unaligned(local_8 as usize as u32);
        if local_8.byte_add(16).cast::<i32>().read_unaligned()
            != ((param_1 as usize).wrapping_add(0x50) as *const i32).read_unaligned()
        {
            FUN_1002f6e8(param_1);
        }
        let mut local_20: i32 = local_8.byte_add(4).cast::<i16>().read_unaligned() as i32;
        let mut local_18: *mut u8 = local_8.byte_add(24);
        while local_20 != 0 {
            local_18.byte_add(36).cast::<i32>().write_unaligned(0);
            let b = local_18.byte_add(40).read();
            local_18.byte_add(40).write(b & 0xfb);
            local_18 = local_18.byte_add(0x44);
            local_20 = local_20.wrapping_sub(1);
        }
        let off = local_8.byte_add(8).cast::<i32>().read_unaligned();
        let mut local_c: *mut u16 = (off as usize).wrapping_add(local_8 as usize) as *mut u16;
        local_20 = local_8.byte_add(6).cast::<i16>().read_unaligned() as i32;
        loop {
            if local_20 == 0 {
                break;
            }
            FUN_10080919(local_c, local_8.byte_add(24) as usize as i32);
            local_c = local_c.byte_add(0x24);
            local_20 = local_20.wrapping_sub(1);
            if DAT_1011515c == 0 {
                break;
            }
        }
        0
    }
}
