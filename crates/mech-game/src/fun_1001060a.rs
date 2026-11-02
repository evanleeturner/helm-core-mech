// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf74: *mut c_void;
    static mut DAT_101686f0: u32;
    static mut DAT_101ec430: u8;
    static mut PTR_DAT_100af0b4: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1008664f(param_1: u32, param_2: *mut i32) -> u8;
}

unsafe extern "stdcall" {
    fn HeapFree(h: *mut c_void, flags: u32, mem: *mut c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001060a(param_1: i32) {
    mech_trace::enter!("1001060a", param_1);
    // SAFETY: the globals and callees are the game's own; the slot table and
    // the records it points to are read and written where the original does.
    unsafe {
        let slot = (&raw mut DAT_101ec430)
            .byte_offset(param_1.wrapping_mul(4) as isize)
            .cast::<i32>();
        let rec = mech_trace::rd!("1001060a", slot, read_unaligned);
        if rec != 0 {
            let flag = mech_trace::rd!(
                "1001060a",
                ((rec as usize).wrapping_add(0x18) as *const i32),
                read_unaligned
            );
            if flag != 0 {
                let off = mech_trace::rd!(
                    "1001060a",
                    ((rec as usize).wrapping_add(0x14) as *const i32),
                    read_unaligned
                );
                mech_trace::cr!(
                    "1001060a",
                    "1008664f",
                    FUN_1008664f(
                        (off as u32).wrapping_add(mech_trace::gr!(
                            "1001060a",
                            "101686f0",
                            DAT_101686f0
                        )),
                        mech_trace::gr!("1001060a", "100af0b4", PTR_DAT_100af0b4) as *mut i32,
                    )
                );
            }
            mech_trace::cr!(
                "1001060a",
                "HeapFree",
                HeapFree(
                    mech_trace::gr!("1001060a", "100adf74", DAT_100adf74),
                    1,
                    mech_trace::rd!("1001060a", slot, read_unaligned) as *mut c_void
                )
            );
            mech_trace::wr!("1001060a", slot, write_unaligned, 0);
        }
    }
}
