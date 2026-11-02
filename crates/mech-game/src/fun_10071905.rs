// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf74: *mut core::ffi::c_void;
}

unsafe extern "stdcall" {
    fn HeapAlloc(h: *mut core::ffi::c_void, flags: u32, size: u32) -> *mut core::ffi::c_void;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10071905(
    param_1: *mut u32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: u32,
) -> u32 {
    mech_trace::enter!(
        "10071905", param_1, param_2, param_3, param_4, param_5, param_6
    );
    // SAFETY: the heap handle is the game's own global; param_1 points at the
    // caller's output structure and param_2/param_3 are caller-supplied
    // address integers, accessed where the original does.
    unsafe {
        let pvar1 = mech_trace::cr!(
            "10071905",
            "HeapAlloc",
            HeapAlloc(
                mech_trace::gr!("10071905", "100adf74", DAT_100adf74),
                1,
                (param_5.wrapping_mul(0xc)) as u32
            )
        );
        if pvar1.is_null() {
            return mech_trace::rt!("10071905", 0xffff_ffff);
        }
        let base = pvar1 as i32;
        mech_trace::wr!("10071905", param_1, write_unaligned, base as u32);
        mech_trace::wr!(
            "10071905",
            param_1.add(1),
            write_unaligned,
            base.wrapping_add(0x300) as u32
        );
        mech_trace::wr!(
            "10071905",
            param_1.add(2),
            write_unaligned,
            base.wrapping_add(0x600) as u32
        );
        let mut local_14 = mech_trace::rd!("10071905", param_1.add(1), read_unaligned) as *mut i8;
        let mut local_8 = mech_trace::rd!("10071905", param_1, read_unaligned) as *mut u8;
        let mut local_c = mech_trace::rd!("10071905", param_1.add(2), read_unaligned) as *mut u16;
        let mut local_10 = param_4.wrapping_mul(3);
        while local_10
            < param_4
                .wrapping_mul(3)
                .wrapping_add(param_5.wrapping_mul(3))
        {
            let a = param_2.wrapping_add(local_10) as *const u8;
            let b = param_3.wrapping_add(local_10) as *const u8;
            mech_trace::dw!("10071905", local_8, mech_trace::rd!("10071905", a, read));
            local_8 = local_8.add(1);
            mech_trace::dw!(
                "10071905",
                local_14,
                (mech_trace::rd!("10071905", b, read) as i8)
                    .wrapping_sub(mech_trace::rd!("10071905", a, read) as i8)
            );
            local_14 = local_14.add(1);
            mech_trace::wr!("10071905", local_c, write_unaligned, 0);
            local_c = local_c.add(1);
            local_10 = local_10.wrapping_add(1);
        }
        mech_trace::wr!("10071905", param_1.add(3), write_unaligned, param_6);
        mech_trace::wr!("10071905", param_1.add(4), write_unaligned, 0);
        mech_trace::wr!(
            "10071905",
            param_1.add(5).cast::<u8>(),
            write,
            param_4 as u8
        );
        mech_trace::wr!(
            "10071905",
            param_1.cast::<u8>().add(0x15).cast::<i32>(),
            write_unaligned,
            param_5
        );
        mech_trace::rt!("10071905", 1)
    }
}
