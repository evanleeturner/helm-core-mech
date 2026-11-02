// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa1ac: u32;
    static mut DAT_100adf74: *mut c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100480d9(param_1: *mut i8) -> *mut u32;
    fn FUN_10047b36(
        param_1: *mut i8,
        param_2: *mut u32,
        param_3: *mut i32,
        param_4: *mut i32,
    ) -> i32;
    fn __close(file_handle: i32) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapAlloc(h: *mut c_void, flags: u32, size: u32) -> *mut c_void;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10047d14(param_1: *mut i8, param_2: *mut i32) -> u32 {
    mech_trace::enter!("10047d14", param_1, param_2);
    let mut local_10: *mut u32 = core::ptr::null_mut();
    let mut local_8: u32 = 0;
    // SAFETY: globals and callees are the game's own; param_2 points at the
    // caller's slot holding the allocated block, accessed where the original does.
    unsafe {
        let pv_var1 = mech_trace::cr!(
            "10047d14",
            "HeapAlloc",
            HeapAlloc(
                mech_trace::gr!("10047d14", "100adf74", DAT_100adf74),
                9,
                0x17
            )
        );
        mech_trace::wr!("10047d14", param_2, write_unaligned, pv_var1 as i32);
        let pc_var2 = mech_trace::cr!("10047d14", "100480d9", FUN_100480d9(param_1)) as *mut i8;
        let local_c = mech_trace::cr!(
            "10047d14",
            "10047b36",
            FUN_10047b36(
                pc_var2,
                &raw mut local_8,
                (&raw mut local_10).cast::<i32>(),
                core::ptr::null_mut(),
            )
        );
        if local_c == -1 {
            if mech_trace::rd!("10047d14", param_2, read_unaligned) == 0 {
                return mech_trace::rt!("10047d14", 0xffff_ffff);
            }
        } else {
            let mut puvar4 = mech_trace::rd!("10047d14", param_2, read_unaligned) as *mut u32;
            let mut u_var3 = local_8 >> 2;
            while u_var3 != 0 {
                mech_trace::wr!(
                    "10047d14",
                    puvar4,
                    write_unaligned,
                    mech_trace::rd!("10047d14", local_10, read_unaligned)
                );
                local_10 = local_10.add(1);
                puvar4 = puvar4.add(1);
                u_var3 = u_var3.wrapping_sub(1);
            }
            local_8 &= 3;
            let mut d = puvar4 as *mut u8;
            let mut s = local_10 as *mut u8;
            while local_8 != 0 {
                mech_trace::wr!("10047d14", d, write, mech_trace::rd!("10047d14", s, read));
                s = s.byte_add(1);
                d = d.byte_add(1);
                local_8 = local_8.wrapping_sub(1);
            }
        }
        mech_trace::cr!("10047d14", "__close", __close(local_c));
        if mech_trace::gr!("10047d14", "100aa1ac", DAT_100aa1ac) == 0 {
            let b = mech_trace::rd!("10047d14", param_2, read_unaligned) as *mut u8;
            mech_trace::wr!(
                "10047d14",
                b.byte_add(0xf).cast::<u32>(),
                write_unaligned,
                0
            );
            let b = mech_trace::rd!("10047d14", param_2, read_unaligned) as *mut u8;
            mech_trace::wr!(
                "10047d14",
                b.byte_add(0xb).cast::<u32>(),
                write_unaligned,
                0
            );
            let b = mech_trace::rd!("10047d14", param_2, read_unaligned) as *mut u8;
            mech_trace::wr!(
                "10047d14",
                b.byte_add(0x13).cast::<u32>(),
                write_unaligned,
                0
            );
            let b = mech_trace::rd!("10047d14", param_2, read_unaligned) as *mut u8;
            mech_trace::wr!("10047d14", b.byte_add(9), write, 1);
            let b = mech_trace::rd!("10047d14", param_2, read_unaligned) as *mut u8;
            mech_trace::wr!("10047d14", b, write, 0);
            let b = mech_trace::rd!("10047d14", param_2, read_unaligned) as *mut u8;
            mech_trace::wr!("10047d14", b.byte_add(2), write, 1);
            let b = mech_trace::rd!("10047d14", param_2, read_unaligned) as *mut u8;
            mech_trace::wr!("10047d14", b.byte_add(3), write, 1);
            let b = mech_trace::rd!("10047d14", param_2, read_unaligned) as *mut u8;
            mech_trace::wr!("10047d14", b.byte_add(1), write, 0);
        } else {
            let b = mech_trace::rd!("10047d14", param_2, read_unaligned) as *mut u8;
            mech_trace::wr!("10047d14", b.byte_add(5), write, 2);
            let b = mech_trace::rd!("10047d14", param_2, read_unaligned) as *mut u8;
            mech_trace::wr!("10047d14", b.byte_add(1), write, 0);
        }
    }
    mech_trace::rt!("10047d14", 1)
}
