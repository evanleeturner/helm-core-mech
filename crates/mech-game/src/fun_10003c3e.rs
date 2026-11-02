// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a40f8: u32;
    static mut PTR_DAT_100af0a8: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn _AIL_end_sample_4(param_1: u32) -> u32;
    fn _AIL_release_sample_handle_4(param_1: u32) -> u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1008664f(param_1: u32, param_2: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10003c3e(param_1: i32) {
    mech_trace::enter!("10003c3e", param_1);
    if param_1 == 0 {
        return;
    }
    // SAFETY: param_1 points at the game's sample record and DAT_100a40f8 at
    // its sample tables; reads, writes and calls follow the original's order.
    unsafe {
        let rec = param_1 as u32;
        if mech_trace::rd!(
            "10003c3e",
            ((rec.wrapping_add(8)) as *const i32),
            read_unaligned
        ) != 0
        {
            let idx = mech_trace::rd!(
                "10003c3e",
                ((rec.wrapping_add(4)) as *const i32),
                read_unaligned
            );
            let base = mech_trace::gr!("10003c3e", "100a40f8", DAT_100a40f8);
            let a = base
                .wrapping_add(4)
                .wrapping_add(idx.wrapping_mul(4) as u32);
            let sample_handle = mech_trace::rd!("10003c3e", (a as *const u32), read_unaligned);
            mech_trace::wr!("10003c3e", (a as *mut u32), write_unaligned, 0);

            let idx = mech_trace::rd!(
                "10003c3e",
                ((rec.wrapping_add(4)) as *const i32),
                read_unaligned
            );
            let b = mech_trace::gr!("10003c3e", "100a40f8", DAT_100a40f8)
                .wrapping_add(0x48)
                .wrapping_add(idx.wrapping_mul(4) as u32);
            mech_trace::wr!("10003c3e", (b as *mut u32), write_unaligned, 0);

            let idx = mech_trace::rd!(
                "10003c3e",
                ((rec.wrapping_add(4)) as *const i32),
                read_unaligned
            );
            let c = mech_trace::gr!("10003c3e", "100a40f8", DAT_100a40f8)
                .wrapping_add(0x4a8)
                .wrapping_add(idx.wrapping_mul(4) as u32);
            mech_trace::wr!("10003c3e", (c as *mut u32), write_unaligned, 0);

            mech_trace::wr!(
                "10003c3e",
                ((rec.wrapping_add(4)) as *mut u32),
                write_unaligned,
                0xffffffff
            );
            mech_trace::cr!(
                "10003c3e",
                "_AIL_end_sample_4",
                _AIL_end_sample_4(sample_handle)
            );
            mech_trace::cr!(
                "10003c3e",
                "_AIL_release_sample_handle_4",
                _AIL_release_sample_handle_4(sample_handle)
            );
            let s = mech_trace::rd!(
                "10003c3e",
                ((rec.wrapping_add(0x1c)) as *const i16),
                read_unaligned
            );
            FUN_1008664f(
                s as i32 as u32,
                mech_trace::gr!("10003c3e", "100af0a8", PTR_DAT_100af0a8) as *mut i32,
            );
            mech_trace::wr!(
                "10003c3e",
                ((rec.wrapping_add(8)) as *mut u32),
                write_unaligned,
                0
            );
        }
    }
}
