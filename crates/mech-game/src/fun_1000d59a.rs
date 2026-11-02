// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ef030: u8;
    static mut DAT_101f64ac: u8;
    static mut DAT_101f64b8: u8;
    static mut DAT_101f64c0: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000d403(param_1: i32) -> i32;
    fn FUN_1007c203(param_1: *mut i32, param_2: *mut u8, param_3: i32) -> u8;
    fn FUN_1000e3ec(param_1: u32) -> u32;
    fn FUN_1000e4a8(param_1: *mut i32) -> u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1000d659() -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000d59a(param_1: i32) {
    mech_trace::enter!("1000d59a", param_1);
    // SAFETY: the globals and callees are the game's own; param_1 points at
    // the caller's structure, and table entries are read and written at the
    // same offsets as the original.
    unsafe {
        let mut local_c: u32 = 0xffffffff;
        let idx = mech_trace::rd!(
            "1000d59a",
            (param_1.wrapping_add(4) as usize as *const i32),
            read_unaligned
        );
        let entry = ((&raw mut DAT_101f64ac) as usize as u32)
            .wrapping_add(idx.wrapping_mul(0x7c) as u32) as usize as *mut u32;
        let old = mech_trace::rd!("1000d59a", entry, read_unaligned);
        mech_trace::wr!("1000d59a", entry, write_unaligned, old | 0x200);
        let idx2 = mech_trace::rd!(
            "1000d59a",
            (param_1.wrapping_add(4) as usize as *const i32),
            read_unaligned
        );
        if idx2 != -1 {
            local_c = mech_trace::cr!("1000d59a", "1000d403", FUN_1000d403(idx2)) as u32;
        }
        if local_c != 0xffffffff {
            let a = ((&raw mut DAT_101f64b8) as usize as u32)
                .wrapping_add(local_c.wrapping_mul(0x7c)) as usize
                as *const i32;
            let v = mech_trace::rd!("1000d59a", a, read_unaligned);
            let b = ((&raw mut DAT_100ef030) as usize as u32)
                .wrapping_add(v.wrapping_mul(0x40) as u32) as usize
                as *const i16;
            let s = mech_trace::rd!("1000d59a", b, read_unaligned) as i32;
            if (s as u32 & 0x8000) != 0 {
                let c = ((&raw mut DAT_101f64c0) as usize as u32)
                    .wrapping_add(local_c.wrapping_mul(0x7c)) as usize
                    as *const i32;
                let p = mech_trace::rd!("1000d59a", c, read_unaligned) as usize as *mut i32;
                mech_trace::cr!(
                    "1000d59a",
                    "1007c203",
                    FUN_1007c203(p, FUN_1000e4a8 as usize as *mut u8, 0)
                );
                mech_trace::cr!("1000d59a", "1000e3ec", FUN_1000e3ec(local_c));
            }
        }
        mech_trace::cr!("1000d59a", "1000d659", FUN_1000d659());
    }
}
