// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf74: *mut c_void;
    static mut DAT_100af17c: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn strncpy(dest: *mut i8, source: *const i8, count: u32) -> *mut i8;
    fn strncat(dest: *mut i8, source: *const i8, count: u32) -> *mut i8;
    fn FUN_1007b9d2(param_1: u32, param_2: *mut i8) -> i32;
    fn FUN_1008668b(param_1: i32, param_2: u32, source: *mut i32) -> *mut u16;
    fn FUN_1001e8a8(param_1: i32, param_2: *mut i8, source: u16) -> i32;
    fn FUN_100480d9(param_1: *mut i8) -> *mut u32;
    fn FUN_10047b36(
        param_1: *mut i8,
        param_2: *mut u32,
        source: *mut i32,
        param_4: *mut i32,
    ) -> i32;
    #[link_name = "_close"]
    fn c_close(file_handle: i32) -> i32;
    fn FUN_100814bb(param_1: i32, param_2: i32) -> i32;
    fn FUN_1008664f(param_1: u32, param_2: *mut i32) -> u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapFree(h: *mut c_void, flags: u32, mem: *mut c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1007bbed(
    param_1: *mut i16,
    param_2: *mut i32,
    source: *mut i8,
    param_4: u32,
    param_5: *mut u32,
    param_6: *mut i32,
) -> *mut u16 {
    mech_trace::enter!(
        "1007bbed", param_1, param_2, source, param_4, param_5, param_6
    );
    // SAFETY: the callees and globals are the game's own; the pointers are the
    // caller's buffers, accessed where the original does. The 16-byte buffer
    // mirrors the stack layout: local_28 at 0, local_20 at 8, local_1c at 12.
    unsafe {
        let mut buf = [0u8; 16];
        let local_28 = buf.as_mut_ptr().cast::<i8>();
        let mut local_18: *mut u16 = core::ptr::null_mut();
        let mut local_8: *mut u16 = core::ptr::null_mut();
        let mut local_c: i32 = 0;
        let mut local_14: u32 = mech_trace::rd!("1007bbed", param_1, read_unaligned) as i32 as u32;

        if local_14 == 0xffff_ffff {
            mech_trace::cr!(
                "1007bbed",
                "strncpy",
                strncpy(local_28, param_1.add(1).cast::<i8>(), 0xc)
            );
            buf[12] = 0;
            local_14 =
                mech_trace::cr!("1007bbed", "1007b9d2", FUN_1007b9d2(param_4, local_28)) as u32;
        }
        if local_14 != 0xffff_ffff {
            local_18 = mech_trace::cr!(
                "1007bbed",
                "1008668b",
                FUN_1008668b(
                    mech_trace::gr!("1007bbed", "100af17c", DAT_100af17c) as i32,
                    local_14,
                    param_2
                )
            );
            let u_var1 = mech_trace::cr!(
                "1007bbed",
                "1001e8a8",
                FUN_1001e8a8(
                    mech_trace::gr!("1007bbed", "100af17c", DAT_100af17c) as i32,
                    param_2.cast::<i8>(),
                    local_14 as u16
                )
            );
            mech_trace::wr!("1007bbed", param_5, write_unaligned, u_var1 as u32);
            if !local_18.is_null() {
                local_c = 1;
            }
        }
        if local_18.is_null() {
            local_14 = 0xffff_ffff;
            buf[8] = 0;
            mech_trace::cr!("1007bbed", "strncat", strncat(local_28, source, 4));
            let pc_var2 =
                mech_trace::cr!("1007bbed", "100480d9", FUN_100480d9(local_28)).cast::<i8>();
            let local_10 = mech_trace::cr!(
                "1007bbed",
                "10047b36",
                FUN_10047b36(
                    pc_var2,
                    param_5,
                    (&raw mut local_18).cast::<i32>(),
                    core::ptr::null_mut(),
                )
            );
            if local_10 == -1 {
                local_18 = core::ptr::null_mut();
            } else {
                mech_trace::cr!("1007bbed", "c_close", c_close(local_10));
                local_c = 0;
            }
        }
        if !local_18.is_null()
            && !param_6.is_null()
            && 0 < (mech_trace::rd!("1007bbed", param_5, read_unaligned) as i32)
        {
            local_8 = local_18;
            local_18 = mech_trace::cr!(
                "1007bbed",
                "100814bb",
                FUN_100814bb(
                    mech_trace::rd!("1007bbed", param_5, read_unaligned) as i32,
                    mech_trace::rd!("1007bbed", param_6, read_unaligned)
                )
            ) as usize as *mut u16;
            if !local_18.is_null() {
                let u_var1 = mech_trace::rd!("1007bbed", param_5, read_unaligned);
                let mut src = local_8.cast::<u8>();
                let mut dst = local_18.cast::<u8>();
                let mut n = u_var1 >> 2;
                while n != 0 {
                    mech_trace::wr!(
                        "1007bbed",
                        dst.cast::<u32>(),
                        write_unaligned,
                        mech_trace::rd!("1007bbed", src.cast::<u32>(), read_unaligned)
                    );
                    src = src.add(4);
                    dst = dst.add(4);
                    n = n.wrapping_sub(1);
                }
                let mut m = u_var1 & 3;
                while m != 0 {
                    mech_trace::dw!("1007bbed", dst, mech_trace::dr!("1007bbed", src));
                    src = src.add(1);
                    dst = dst.add(1);
                    m = m.wrapping_sub(1);
                }
            }
            if local_c == 0 {
                mech_trace::cr!(
                    "1007bbed",
                    "HeapFree",
                    HeapFree(
                        mech_trace::gr!("1007bbed", "100adf74", DAT_100adf74),
                        1,
                        local_8.cast::<c_void>()
                    )
                );
            } else {
                mech_trace::cr!("1007bbed", "1008664f", FUN_1008664f(local_14, param_2));
            }
        }
        mech_trace::wr!("1007bbed", param_1, write_unaligned, local_14 as i16);
        mech_trace::rt!("1007bbed", local_18)
    }
}
