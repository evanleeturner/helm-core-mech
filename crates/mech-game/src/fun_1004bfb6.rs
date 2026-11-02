// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100af098: i32;
    static mut DAT_10160c00: [i32; 1];
    static mut DAT_100aa1f4: i32;
    static mut DAT_100adf38: i32;
    static mut DAT_100b38d8: i32;
    static mut DAT_100b3904: i32;
    static mut DAT_100eef40: [i32; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1003cc75();
    fn FUN_1001cac9(param_1: i32, param_2: *mut i32, param_3: *mut i32, param_4: *mut i32) -> i32;
    fn FUN_1000fc30(param_1: i32, param_2: i32, param_3: i32) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RaiseException(a: u32, b: u32, c: u32, d: *const u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1004bfb6() {
    mech_trace::enter!("1004bfb6");
    // SAFETY: all globals, callees and the object pointers walked here are
    // the game's own memory, accessed exactly as the original does.
    unsafe {
        let mut local_38: i32 = 0;
        let mut local_20: i32 = 0;
        let mut local_24: i32 = 0;
        let mut local_28: i32 = 0;
        if mech_trace::gr!("1004bfb6", "100af098", DAT_100af098) == 0 {
            return;
        }
        FUN_1003cc75();
        let tbl = (&raw mut DAT_10160c00).cast::<i32>();
        let objs = (&raw mut DAT_100eef40).cast::<i32>();
        let mut local_8: i32 = mech_trace::rd!("1004bfb6", tbl, read);
        while local_8 != 0 {
            local_38 = local_38.wrapping_add(1);
            let mut local_40: i32 = mech_trace::rd!(
                "1004bfb6",
                ((local_8 as u32).wrapping_add(8) as *const i32),
                read_unaligned
            );
            while local_40 != 0 {
                let w = mech_trace::rd!(
                    "1004bfb6",
                    ((local_40 as u32).wrapping_add(2) as *const u16),
                    read_unaligned
                );
                if (w as u8) & 0xf0 == 0x10 {
                    let mut local_18 = mech_trace::cr!(
                        "1004bfb6",
                        "1001cac9",
                        FUN_1001cac9(
                            local_40,
                            &raw mut local_20,
                            &raw mut local_24,
                            &raw mut local_28,
                        )
                    );
                    local_18 = local_18.wrapping_mul(2);
                    let mut local_30 = mech_trace::gr!("1004bfb6", "100b38d8", DAT_100b38d8);
                    loop {
                        let mut iv2 = local_30.wrapping_sub(1);
                        if local_30 == 0 {
                            break;
                        }
                        local_30 = iv2;
                        if mech_trace::gr!("1004bfb6", "100aa1f4", DAT_100aa1f4) == 0
                            || iv2 == mech_trace::gr!("1004bfb6", "100adf38", DAT_100adf38)
                        {
                            let local_10 =
                                mech_trace::rd!("1004bfb6", objs.offset(iv2 as isize), read);
                            let f = mech_trace::rd!(
                                "1004bfb6",
                                ((local_10 as u32).wrapping_add(0x14) as *const u16),
                                read_unaligned
                            );
                            if (f & 2) == 0 && (f & 4) == 0 {
                                iv2 = mech_trace::rd!(
                                    "1004bfb6",
                                    ((local_10 as u32).wrapping_add(0x20) as *const i32),
                                    read_unaligned
                                );
                                let local_14 = mech_trace::rd!(
                                    "1004bfb6",
                                    (((iv2 as u32).wrapping_add(0x41c)) as *const i32),
                                    read_unaligned
                                )
                                .wrapping_add(local_18);
                                let x = mech_trace::rd!(
                                    "1004bfb6",
                                    ((local_10 as u32).wrapping_add(0x50) as *const i32),
                                    read_unaligned
                                );
                                let y = mech_trace::rd!(
                                    "1004bfb6",
                                    ((local_10 as u32).wrapping_add(0x54) as *const i32),
                                    read_unaligned
                                );
                                let z = mech_trace::rd!(
                                    "1004bfb6",
                                    ((local_10 as u32).wrapping_add(0x58) as *const i32),
                                    read_unaligned
                                );
                                let local_1c = mech_trace::cr!(
                                    "1004bfb6",
                                    "1000fc30",
                                    FUN_1000fc30(
                                        x.wrapping_sub(local_20),
                                        y.wrapping_sub(local_24),
                                        z.wrapping_sub(local_28),
                                    )
                                );
                                if local_1c < local_14 {
                                    let prod = local_18.wrapping_mul(local_1c);
                                    if local_14 == 0 || (local_14 == -1 && prod == i32::MIN) {
                                        RaiseException(0xC0000094, 0, 0, core::ptr::null());
                                    }
                                    let quot: i32;
                                    core::arch::asm!(
                                        "cdq",
                                        "idiv {d:e}",
                                        d = in(reg) local_14,
                                        inout("eax") prod => quot,
                                        out("edx") _,
                                    );
                                    let local_c = local_18.wrapping_sub(quot) >> 3;
                                    let p = ((iv2 as u32).wrapping_add(0x3c4)) as *mut i32;
                                    mech_trace::wr!(
                                        "1004bfb6",
                                        p,
                                        write_unaligned,
                                        mech_trace::rd!("1004bfb6", p, read_unaligned)
                                            .wrapping_add(local_c.wrapping_mul(mech_trace::gr!(
                                                "1004bfb6",
                                                "100b3904",
                                                DAT_100b3904
                                            )))
                                    );
                                }
                            }
                        }
                    }
                }
                local_40 = mech_trace::rd!(
                    "1004bfb6",
                    ((local_40 as u32).wrapping_add(8) as *const i32),
                    read_unaligned
                );
            }
            local_8 = mech_trace::rd!("1004bfb6", tbl.offset(local_38 as isize), read);
        }
    }
}
