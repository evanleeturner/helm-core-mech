// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b2f98: u8;
    static mut DAT_100b3188: u8;
    static mut DAT_101f5300: u8;
    static mut PTR_DAT_100b2cf0: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10066209(param_1: i32, param_2: i32) -> i32;
    fn FUN_10065e40(param_1: u32, param_2: i32, param_3: i32) -> i32;
    fn FUN_100660e1(param_1: i32, param_2: *mut i32, param_3: *mut i32) -> *mut i32;
    fn FUN_10065ebf(param_1: i32, param_2: i32, param_3: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1005c952() {
    mech_trace::enter!("1005c952");
    let mut local_14: i32 = 0;
    while local_14 < 0x22 {
        // SAFETY: the globals, callees and the pointer tables they lead to are
        // the game's own memory; reads and writes follow the original's order.
        unsafe {
            let pp = (&raw mut PTR_DAT_100b2cf0 as usize)
                .wrapping_add((local_14 as usize).wrapping_mul(5)) as i32;
            mech_trace::cr!("1005c952", "10066209", FUN_10066209(pp, pp));
            mech_trace::cr!(
                "1005c952",
                "10065e40",
                FUN_10065e40(&raw mut DAT_101f5300 as usize as u32, pp, pp)
            );
            let pt = (&raw mut DAT_100b2f98 as usize)
                .wrapping_add((local_14 as usize).wrapping_mul(8)) as *mut i32;
            mech_trace::cr!("1005c952", "100660e1", FUN_100660e1(pp, pt, pt));
            let i_var1 = mech_trace::rd!(
                "1005c952",
                ((&raw mut DAT_100b3188 as usize).wrapping_add((local_14 as usize).wrapping_mul(4))
                    as *const i32),
                read_unaligned
            );
            if i_var1 != 0 {
                let base = mech_trace::rd!(
                    "1005c952",
                    (i_var1 as usize as *const u8).byte_add(4).cast::<i32>(),
                    read_unaligned
                );
                let p2 = mech_trace::rd!(
                    "1005c952",
                    ((base as usize).wrapping_add(4) as *const u32),
                    read_unaligned
                ) as usize as *mut u32;
                mech_trace::wr!(
                    "1005c952",
                    p2,
                    write_unaligned,
                    &raw mut DAT_101f5300 as usize as u32
                );
                mech_trace::cr!(
                    "1005c952",
                    "10065ebf",
                    FUN_10065ebf(pp, p2 as usize as i32, p2 as usize as i32)
                );

                let base = mech_trace::rd!(
                    "1005c952",
                    (i_var1 as usize as *const u8).byte_add(4).cast::<i32>(),
                    read_unaligned
                );
                let p2 = mech_trace::rd!(
                    "1005c952",
                    ((base as usize).wrapping_add(8) as *const u32),
                    read_unaligned
                ) as usize as *mut u32;
                mech_trace::wr!(
                    "1005c952",
                    p2,
                    write_unaligned,
                    &raw mut DAT_101f5300 as usize as u32
                );
                mech_trace::cr!(
                    "1005c952",
                    "10065ebf",
                    FUN_10065ebf(pp, p2 as usize as i32, p2 as usize as i32)
                );

                let base = mech_trace::rd!(
                    "1005c952",
                    (i_var1 as usize as *const u8).byte_add(4).cast::<i32>(),
                    read_unaligned
                );
                let p3 = mech_trace::rd!(
                    "1005c952",
                    ((base as usize).wrapping_add(0xc) as *const u32),
                    read_unaligned
                ) as usize as *mut u32;
                mech_trace::wr!(
                    "1005c952",
                    p3,
                    write_unaligned,
                    &raw mut DAT_101f5300 as usize as u32
                );
            }
        }
        local_14 = local_14.wrapping_add(1);
    }
}
