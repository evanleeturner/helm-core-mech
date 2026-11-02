// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a758c: i32;
    static mut DAT_100adf74: *mut u8;
    static mut DAT_101f5490: u8;
    static mut DAT_101f5514: u32;
    static mut DAT_101f5520: u8;
    static mut DAT_101f5524: u8;
    static mut DAT_101f5528: u8;
    static mut DAT_101f552c: u8;
    static mut DAT_101f5530: u8;
    static mut DAT_101f5534: u8;
    static mut DAT_100af0d0: u32;
    static mut DAT_100af13c: *mut i8;
    static mut DAT_100a75e8: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _strncpy(dest: *mut i8, source: *const i8, count: u32) -> *mut i8;
    fn FUN_1007bbed(
        param_1: *mut i16,
        param_2: *mut i32,
        param_3: *mut i8,
        param_4: u32,
        param_5: *mut u32,
        param_6: *mut i32,
    ) -> *mut u16;
    fn FUN_10068d4e(param_1: i32, param_2: i32, param_3: i32);
    fn FUN_10068db6(param_1: u32);
    fn FUN_10068dc9(param_1: i32, param_2: *mut i32, param_3: i32, param_4: i32) -> *mut u16;
    fn FUN_100141c0(param_1: *mut u16, param_2: i32);
    fn FUN_10089cc9(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
        param_7: i32,
    );
    fn FUN_100895ca(param_1: *mut i32, param_2: *mut i32, param_3: *mut u32);
    fn FUN_1001b7cd(param_1: *mut u16, param_2: *mut i32);
    fn FUN_1003da99(param_1: *mut u16);
    fn FUN_1008664f(param_1: u32, param_2: *mut i32);
    fn FUN_1001d671(param_1: *mut u8);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapFree(h: *mut u8, flags: u32, mem: *mut u8) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1000dec3() -> u32 {
    mech_trace::enter!("1000dec3");
    let mut local_78: [i32; 12] = [0; 12];
    let mut buf: [i16; 8] = [0; 8];
    let mut local_c: u32 = 0;
    let mut local_2c: i32;
    let mut local_8: u32 = 0;
    // SAFETY: globals and callees are the game's own; all pointers derived
    // from game tables or local buffers, accessed as the original does.
    unsafe {
        if mech_trace::gr!("1000dec3", "101f5514", DAT_101f5514) == 0 {
            mech_trace::gw!("1000dec3", "101f5514", DAT_101f5514, 1);
            let local_10: *mut i16 = buf.as_mut_ptr();
            mech_trace::dw!("1000dec3", local_10, -1);
            mech_trace::cr!(
                "1000dec3",
                "_strncpy",
                _strncpy(
                    local_10.byte_add(2).cast::<i8>(),
                    &raw const DAT_100a75e8,
                    0xc,
                )
            );
            mech_trace::wr!("1000dec3", local_10.add(7).cast::<u8>(), write, 0);
            let lp_mem = mech_trace::cr!(
                "1000dec3",
                "1007bbed",
                FUN_1007bbed(
                    local_10,
                    mech_trace::gr!("1000dec3", "100af0d0", DAT_100af0d0) as *mut i32,
                    mech_trace::gr!("1000dec3", "100af13c", DAT_100af13c),
                    2,
                    &raw mut local_c,
                    core::ptr::null_mut(),
                )
            );
            if !lp_mem.is_null() {
                let mut local_38: i32 = 0;
                while local_38 < mech_trace::gr!("1000dec3", "100a758c", DAT_100a758c) {
                    let i_var1 = local_38.wrapping_mul(0x7c) as isize;
                    let local_34 = (&raw mut DAT_101f5520).offset(i_var1).cast::<i32>();
                    let p_2c = (&raw mut DAT_101f552c).offset(i_var1).cast::<i32>();
                    if mech_trace::rd!("1000dec3", local_34, read_unaligned)
                        != mech_trace::rd!("1000dec3", p_2c, read_unaligned)
                    {
                        let a = mech_trace::rd!("1000dec3", local_34, read_unaligned);
                        let b = mech_trace::rd!("1000dec3", p_2c, read_unaligned);
                        let d = b.wrapping_sub(a);
                        let u = d >> 31;
                        let local_1c = (d ^ u).wrapping_sub(u);

                        let a = mech_trace::rd!(
                            "1000dec3",
                            (&raw mut DAT_101f5524).offset(i_var1).cast::<i32>(),
                            read_unaligned
                        );
                        let b = mech_trace::rd!(
                            "1000dec3",
                            (&raw mut DAT_101f5530).offset(i_var1).cast::<i32>(),
                            read_unaligned
                        );
                        let d = b.wrapping_sub(a);
                        let u = d >> 31;
                        let local_18 = (d ^ u).wrapping_sub(u);

                        let a = mech_trace::rd!(
                            "1000dec3",
                            (&raw mut DAT_101f5528).offset(i_var1).cast::<i32>(),
                            read_unaligned
                        );
                        let b = mech_trace::rd!(
                            "1000dec3",
                            (&raw mut DAT_101f5534).offset(i_var1).cast::<i32>(),
                            read_unaligned
                        );
                        let d = b.wrapping_sub(a);
                        let u = d >> 31;
                        let local_14 = (d ^ u).wrapping_sub(u);

                        let local_28 = local_1c.wrapping_add(9) / 10;
                        let local_24 = local_18.wrapping_add(9) / 10;
                        let local_20 = local_14.wrapping_add(9) / 10;
                        FUN_10068d4e(local_28, local_24, local_20);
                        FUN_10068db6(0);
                        local_2c = 0;
                        let puv = mech_trace::cr!(
                            "1000dec3",
                            "10068dc9",
                            FUN_10068dc9(lp_mem as i32, &raw mut local_2c, local_c as i32, 0)
                        );
                        let slot = (&raw mut DAT_101f5490)
                            .offset((local_38.wrapping_mul(4)) as isize)
                            .cast::<*mut u16>();
                        mech_trace::wr!("1000dec3", slot, write_unaligned, puv);
                        let local_30 = mech_trace::rd!("1000dec3", slot, read_unaligned);
                        if !local_30.is_null() {
                            local_8 = 1;
                            FUN_100141c0(local_30, 4);
                            FUN_10089cc9(
                                local_78.as_mut_ptr(),
                                0,
                                0,
                                0,
                                mech_trace::rd!("1000dec3", local_34.add(6), read_unaligned),
                                mech_trace::rd!("1000dec3", local_34.add(7), read_unaligned),
                                mech_trace::rd!("1000dec3", local_34.add(8), read_unaligned),
                            );
                            FUN_100895ca(
                                local_34.add(0x12),
                                local_78.as_mut_ptr(),
                                local_78.as_mut_ptr().cast::<u32>(),
                            );
                            FUN_1001b7cd(local_30, local_78.as_mut_ptr());
                            FUN_1003da99(local_30);
                        }
                    }
                    local_38 = local_38.wrapping_add(1);
                }
                if mech_trace::dr!("1000dec3", local_10) == -1 {
                    mech_trace::cr!(
                        "1000dec3",
                        "HeapFree",
                        HeapFree(
                            mech_trace::gr!("1000dec3", "100adf74", DAT_100adf74),
                            1,
                            lp_mem.cast::<u8>()
                        )
                    );
                } else {
                    FUN_1008664f(
                        mech_trace::dr!("1000dec3", local_10) as i32 as u32,
                        mech_trace::gr!("1000dec3", "100af0d0", DAT_100af0d0) as *mut i32,
                    );
                }
            }
        } else {
            mech_trace::gw!("1000dec3", "101f5514", DAT_101f5514, 0);
            local_8 = 1;
            let mut local_38: i32 = 0;
            while local_38 < mech_trace::gr!("1000dec3", "100a758c", DAT_100a758c) {
                let slot = (&raw mut DAT_101f5490)
                    .offset((local_38.wrapping_mul(4)) as isize)
                    .cast::<u32>();
                if mech_trace::rd!("1000dec3", slot, read_unaligned) != 0 {
                    FUN_1001d671(mech_trace::rd!("1000dec3", slot, read_unaligned) as *mut u8);
                    mech_trace::wr!("1000dec3", slot, write_unaligned, 0);
                }
                local_38 = local_38.wrapping_add(1);
            }
        }
    }
    mech_trace::rt!("1000dec3", local_8)
}
