// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf38: i32;
    static mut DAT_100b38d8: i32;
    static mut DAT_100b38dc: i32;
    static mut DAT_100eef40: [u32; 0];
    static mut DAT_100ef030: [u8; 0];
    static mut DAT_100ef034: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1004d5e3(param_1: i32) -> u32;
    fn FUN_10008fb3(param_1: *mut u32, param_2: i32, param_3: i32, param_4: u32, param_5: i32);
    fn FUN_1000dde1(param_1: i32, param_2: *mut u32, param_3: *mut u32, param_4: *mut u32);
    fn FUN_1004d60d(param_1: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10009076(param_1: *mut u32) {
    mech_trace::enter!("10009076", param_1);
    let mut local_20: i32 = 0;
    let mut local_1c: i32 = 0;
    let mut local_18: u32 = 0;
    let mut local_14: i32 = 0;
    // SAFETY: globals and callees are the game's own; all pointers refer to
    // the game's tables and the caller's structure, accessed as the original does.
    unsafe {
        while local_14 < mech_trace::gr!("10009076", "100b38d8", DAT_100b38d8) {
            if local_14 != mech_trace::gr!("10009076", "100adf38", DAT_100adf38) {
                let local_8 = mech_trace::rd!(
                    "10009076",
                    (&raw mut DAT_100eef40).cast::<u32>().add(local_14 as usize),
                    read_unaligned
                );
                let base = local_8 as *const u8;
                let flags = mech_trace::rd!(
                    "10009076",
                    base.byte_add(0x14).cast::<u16>(),
                    read_unaligned
                );
                if (flags & 0x1400) != 0
                    && (flags & 0x800) == 0
                    && (flags & 0x4000) == 0
                    && (flags & 0x16) == 0
                {
                    local_20 = mech_trace::rd!(
                        "10009076",
                        base.byte_add(0x50).cast::<i32>(),
                        read_unaligned
                    );
                    local_1c = mech_trace::rd!(
                        "10009076",
                        base.byte_add(0x54).cast::<i32>(),
                        read_unaligned
                    );
                    local_18 = mech_trace::rd!(
                        "10009076",
                        base.byte_add(0x58).cast::<u32>(),
                        read_unaligned
                    );
                    let i_var1 =
                        mech_trace::cr!("10009076", "1004d5e3", FUN_1004d5e3(local_14)) as i32;
                    let p = mech_trace::rd!("10009076", param_1.add(0x1c), read_unaligned);
                    let addr = p
                        .wrapping_add(0xc)
                        .wrapping_add((i_var1 as u32).wrapping_mul(4));
                    let local_c = mech_trace::rd!("10009076", (addr as *const i32), read_unaligned);
                    FUN_10008fb3(param_1, local_20, local_1c, local_18, local_c);
                }
            }
            local_14 = local_14.wrapping_add(1);
        }
        local_14 = 0;
        while local_14 < mech_trace::gr!("10009076", "100b38dc", DAT_100b38dc) {
            let off = (local_14 as usize).wrapping_mul(0x40);
            let local_10 = (&raw mut DAT_100ef030).cast::<u8>().wrapping_add(off);
            let flags = mech_trace::rd!("10009076", local_10.cast::<u16>(), read_unaligned);
            if (flags & 0x1400) != 0 && (flags & 0x1e) == 0 {
                let arg = mech_trace::rd!(
                    "10009076",
                    (&raw mut DAT_100ef034)
                        .cast::<u8>()
                        .wrapping_add(off)
                        .cast::<i32>(),
                    read_unaligned
                );
                FUN_1000dde1(
                    arg,
                    &raw mut local_20 as *mut u32,
                    &raw mut local_1c as *mut u32,
                    &raw mut local_18,
                );
                let i_var1 = mech_trace::cr!("10009076", "1004d60d", FUN_1004d60d(local_14)) as i32;
                let p = mech_trace::rd!("10009076", param_1.add(0x1c), read_unaligned);
                let addr = p
                    .wrapping_add(0xc)
                    .wrapping_add((i_var1 as u32).wrapping_mul(4));
                let local_c = mech_trace::rd!("10009076", (addr as *const i32), read_unaligned);
                FUN_10008fb3(param_1, local_20, local_1c, local_18, local_c);
            }
            local_14 = local_14.wrapping_add(1);
        }
    }
}
