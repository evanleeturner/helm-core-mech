// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ab4c4: i32;
    static mut DAT_100ab390: i32;
    static mut DAT_100ab4c8: [i8; 40];
    static mut DAT_100ab24c: i32;
    static mut DAT_100ab588: i32;
    static mut DAT_100ab590: i32;
    static mut DAT_1016a638: i16;
    static mut DAT_1016a63a: i16;
    static mut DAT_1016a63c: i16;
    static mut DAT_1016a63e: i16;
    static mut DAT_1016a640: i16;
    static mut DAT_1016a642: i16;
    static mut DAT_1016a644: i16;
    static mut DAT_1016a646: i16;
    static mut DAT_1016a648: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1001dce2(param_1: i32) -> *mut i32;
    fn FUN_100108a0(param_1: i32, param_2: *mut i8);
    fn FUN_1004718e(param_1: *mut u8) -> *mut u32;
    fn FUN_100470e0(param_1: *mut u16);
    fn FUN_1001deb6(param_1: i32);
    fn FUN_10031fca(param_1: *mut i32) -> u32;
    fn FUN_100324f8(param_1: i32, param_2: unsafe extern "C" fn(i32) -> u32) -> u32;
    fn FUN_100327f6(param_1: i32, param_2: unsafe extern "C" fn(i32) -> u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002cf43(param_1: i32) -> u32 {
    // SAFETY: the globals and callees are the game's own; the pointers come
    // from the game's lookup function and are read where the original reads.
    unsafe {
        let mut local_8: u32 = 1;
        let pi_var2 = FUN_1001dce2(param_1);
        if pi_var2.is_null() || DAT_100ab4c4 != pi_var2.read_unaligned() {
            local_8 = 0;
            FUN_100108a0(0x48, core::ptr::null_mut());
        } else {
            let a = pi_var2.add(2).read_unaligned() as u32;
            let b = DAT_100ab390 as u32;
            if a.swap_bytes() < b.swap_bytes() {
                local_8 = 0;
                FUN_100108a0(0x49, core::ptr::null_mut());
            } else {
                let mut local_10 = FUN_1001dce2(param_1);
                let sig0 = (&raw mut DAT_100ab4c8).cast::<i32>().read_unaligned();
                if local_10.is_null() || sig0 != local_10.read_unaligned() {
                    local_8 = 0;
                    FUN_100108a0(0x3e, core::ptr::null_mut());
                } else {
                    let pu_var3 = FUN_1004718e((param_1.wrapping_add(4)) as *mut u8);
                    if pu_var3.is_null() {
                        FUN_100470e0((param_1.wrapping_add(4)) as *mut u16);
                    }
                    DAT_1016a638 =
                        (local_10.add(2).read_unaligned() as i16).wrapping_add(DAT_1016a638);
                    DAT_1016a63a = local_10
                        .byte_add(10)
                        .cast::<i16>()
                        .read_unaligned()
                        .wrapping_add(DAT_1016a63a);
                    DAT_1016a63c =
                        (local_10.add(3).read_unaligned() as i16).wrapping_add(DAT_1016a63c);
                    DAT_1016a63e = local_10
                        .byte_add(0xe)
                        .cast::<i16>()
                        .read_unaligned()
                        .wrapping_add(DAT_1016a63e);
                    DAT_1016a640 =
                        (local_10.add(4).read_unaligned() as i16).wrapping_add(DAT_1016a640);
                    DAT_1016a642 = local_10
                        .byte_add(0x12)
                        .cast::<i16>()
                        .read_unaligned()
                        .wrapping_add(DAT_1016a642);
                    DAT_1016a644 =
                        (local_10.add(5).read_unaligned() as i16).wrapping_add(DAT_1016a644);
                    if pu_var3.is_null() {
                        DAT_1016a646 = local_10
                            .byte_add(0x16)
                            .cast::<i16>()
                            .read_unaligned()
                            .wrapping_add(DAT_1016a646);
                        DAT_1016a648 = DAT_1016a648.wrapping_add(local_10.add(6).read_unaligned());
                    }
                    while !local_10.is_null() {
                        let i_var1 = local_10.read_unaligned();
                        if DAT_100ab24c != 0 {
                            FUN_1001deb6(i_var1);
                        }
                        let sig36 = (&raw mut DAT_100ab4c8)
                            .cast::<u8>()
                            .add(36)
                            .cast::<i32>()
                            .read_unaligned();
                        if sig36 == i_var1 {
                            let u_var4 = FUN_10031fca(local_10);
                            local_8 &= u_var4;
                        } else if DAT_100ab588 == i_var1 {
                            let u_var4 = FUN_100324f8(local_10 as i32, FUN_1002cf43);
                            local_8 &= u_var4;
                        } else if DAT_100ab590 == i_var1 {
                            let u_var4 = FUN_100327f6(local_10 as i32, FUN_1002cf43);
                            local_8 &= u_var4;
                        }
                        local_10 = FUN_1001dce2(param_1);
                    }
                }
            }
        }
        local_8
    }
}
