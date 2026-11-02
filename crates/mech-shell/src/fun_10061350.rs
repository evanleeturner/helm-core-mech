// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut LAB_10061430: u8;
    static mut LAB_10061510: u8;
    static mut DAT_10098224: *mut c_void;
    static mut DAT_100a6744: u32;
    static mut DAT_100c49d0: u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapAlloc(h: *mut c_void, flags: u32, size: u32) -> *mut c_void;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10061350(
    param_1: u32,
    param_2: i32,
    param_3: i32,
    param_4: u32,
) -> *mut u32 {
    mech_trace::enter!("10061350", param_1, param_2, param_3, param_4);
    if param_3 == 0 {
        return mech_trace::rt!("10061350", core::ptr::null_mut());
    }
    // SAFETY: the globals, the heap and the labels are the game's own;
    // param_3 points at the caller's structure and the allocated block holds
    // (count + 1) entries of 0x38 bytes, written where the original does.
    unsafe {
        let count_ptr = param_3.wrapping_add(4) as *const i32;
        let mut i_var3: i32 = 0;
        let mut i_var4: i32 = 0;
        let size = (mech_trace::rd!("10061350", count_ptr, read_unaligned).wrapping_add(1))
            .wrapping_mul(0x38) as u32;
        let p_var1 = mech_trace::cr!(
            "10061350",
            "HeapAlloc",
            HeapAlloc(
                mech_trace::gr!("10061350", "10098224", DAT_10098224),
                1,
                size
            )
        ) as *mut u32;
        let mut p_var2 = p_var1;
        if 0 < mech_trace::rd!("10061350", count_ptr, read_unaligned) {
            loop {
                let stride = mech_trace::rd!(
                    "10061350",
                    (mech_trace::gr!("10061350", "100a6744", DAT_100a6744).wrapping_add(0x40c)
                        as *const i32),
                    read_unaligned
                );
                mech_trace::wr!("10061350", p_var2, write_unaligned, param_1);
                let i_var5 = stride.wrapping_mul(i_var3);
                i_var3 = i_var3.wrapping_add(1);
                mech_trace::wr!(
                    "10061350",
                    p_var2.add(1),
                    write_unaligned,
                    i_var5.wrapping_add(param_2) as u32
                );
                mech_trace::wr!("10061350", p_var2.add(2), write_unaligned, 100);
                let v = mech_trace::rd!(
                    "10061350",
                    (mech_trace::gr!("10061350", "100a6744", DAT_100a6744).wrapping_add(0x40c)
                        as *const u32),
                    read_unaligned
                );
                mech_trace::wr!("10061350", p_var2.add(3), write_unaligned, v);
                mech_trace::wr!("10061350", p_var2.add(4), write_unaligned, 0);
                mech_trace::wr!(
                    "10061350",
                    p_var2.add(5),
                    write_unaligned,
                    (&raw mut DAT_100c49d0) as u32
                );
                mech_trace::wr!("10061350", p_var2.add(6), write_unaligned, 0);
                mech_trace::wr!(
                    "10061350",
                    p_var2.add(7),
                    write_unaligned,
                    (&raw mut LAB_10061430) as u32
                );
                mech_trace::wr!(
                    "10061350",
                    p_var2.add(8),
                    write_unaligned,
                    (&raw mut LAB_10061510) as u32
                );
                mech_trace::wr!("10061350", p_var2.add(9), write_unaligned, i_var4 as u32);
                i_var4 = i_var4.wrapping_add(1);
                mech_trace::wr!("10061350", p_var2.add(10), write_unaligned, param_4);
                mech_trace::wr!("10061350", p_var2.add(0xb), write_unaligned, 0xffffffff);
                p_var2 = p_var2.add(0xe);
                if !(i_var4 < mech_trace::rd!("10061350", count_ptr, read_unaligned)) {
                    break;
                }
            }
        }
        mech_trace::wr!(
            "10061350",
            p_var1.offset((i_var3.wrapping_mul(0xe)) as isize),
            write_unaligned,
            0xffffffff
        );
        mech_trace::rt!("10061350", p_var1)
    }
}
