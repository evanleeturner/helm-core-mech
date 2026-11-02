// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[repr(C)]
struct Frame {
    local_70: [i32; 4],
    local_60: [i8; 16],
    local_50: i32,
    local_4c: [i8; 16],
    local_3c: [i8; 22],
    local_26: [i8; 22],
    local_10: [i8; 16],
}

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a1bd0: i32;
    static mut DAT_100a1bdc: [*mut i8; 0];
    static mut DAT_100a1c40: [i16; 0];
    static mut DAT_100c6068: u32;
    static mut DAT_100c6c30: *mut c_void;
    static mut DAT_100a35f4: [i8; 0];
    static mut DAT_100a35c0: [i8; 0];
    static mut DAT_100a35c8: [i8; 0];
    static mut DAT_100a35d0: [i8; 0];
    static mut DAT_100a35f8: [i8; 0];
    static mut DAT_100a3620: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10056be0() -> i32;
    fn FUN_10057200(param_1: i32, param_2: *mut c_void) -> *mut *mut i8;
    fn FUN_1005e020(param_1: i32, param_2: *mut i8) -> i32;
    fn FUN_1005e290(
        a1: i32,
        a2: *mut i8,
        a3: *mut i8,
        a4: *mut i8,
        a5: i32,
        a6: *mut i8,
        a7: i32,
        a8: i32,
        a9: i32,
        a10: i32,
        a11: i32,
        a12: i32,
        a13: i32,
    ) -> i32;
    fn FUN_1005e670();
    fn FUN_1005e6f0(param_1: *mut i8) -> u32;
    fn _sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
    fn __strnicmp(s1: *const i8, s2: *const i8, n: u32) -> i32;
    fn FID_conflict__fwprintf(file: *mut c_void, fmt: *const i8, ...) -> i32;
    fn _fflush(file: *mut c_void) -> i32;
}

#[inline(always)]
unsafe fn copy_str(dst: *mut i8, src: *const i8) {
    let mut i = 0usize;
    // SAFETY: caller guarantees src is NUL-terminated and dst has room, as in the original.
    unsafe {
        loop {
            let c = mech_trace::dr!("10056dd0", src.add(i));
            mech_trace::dw!("10056dd0", dst.add(i), c);
            i += 1;
            if c == 0 {
                break;
            }
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10056dd0(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: *mut i16,
) -> u32 {
    mech_trace::enter!("10056dd0", param_1, param_2, param_3, param_4);
    let p2 = param_2 as usize as *mut u8;
    let mut f = Frame {
        local_70: [0; 4],
        local_60: [0; 16],
        local_50: 0,
        local_4c: [0; 16],
        local_3c: [0; 22],
        local_26: [0; 22],
        local_10: [0; 16],
    };
    // SAFETY: all globals and callees are the game's own; param_2 and param_4 point at
    // the caller's memory; the frame is accessed through raw pointers as the original does.
    unsafe {
        let l70 = (&raw mut f.local_70) as *mut i32;
        let l60 = (&raw mut f.local_60) as *mut i8;
        let l50 = &raw mut f.local_50;
        let l4c = (&raw mut f.local_4c) as *mut i8;
        let l3c = (&raw mut f.local_3c) as *mut i8;
        let l26 = (&raw mut f.local_26) as *mut i8;
        let l10 = (&raw mut f.local_10) as *mut i8;

        let mut iv3 = mech_trace::gr!("10056dd0", "100a1bd0", DAT_100a1bd0);
        mech_trace::cp!(
            "10056dd0",
            core::ptr::copy_nonoverlapping,
            p2.byte_add(0xdc) as *const u8,
            l50 as *mut u8,
            23
        );
        mech_trace::dw!("10056dd0", param_4, 0);
        let mut local_84: i32 = iv3.wrapping_sub(2);
        let b = f.local_4c[1];
        if b == 1 {
            local_84 = 6;
        } else if b == 2 {
            local_84 = 5;
        } else if b == 3 {
            local_84 = 1;
        }
        mech_trace::dw!("10056dd0", l70.add(3), local_84);
        let mut local_80: u32 = 1;
        let mut u9: u32 = local_84 as u32;
        if 0 < param_1 {
            loop {
                mech_trace::dw!("10056dd0", l70.add(0), -1);
                mech_trace::dw!("10056dd0", l70.add(1), -1);
                mech_trace::dw!("10056dd0", l70.add(2), -1);
                if local_84 < 1 {
                    local_84 = 1;
                }
                if 8 < local_84 {
                    local_84 = 8;
                }
                FUN_1005e670();
                iv3 =
                    (mech_trace::cr!("10056dd0", "10056be0", FUN_10056be0()) / 10).wrapping_add(1);
                let mut local_7c: i32 = 0;
                let local_78 = iv3 as i16;
                mech_trace::dw!(
                    "10056dd0",
                    param_4,
                    (mech_trace::dr!("10056dd0", param_4)).wrapping_add(local_78)
                );
                if 0 < iv3 {
                    let mut pi11: *mut i32 = l70;
                    let mut local_74: *mut u8 = p2.byte_add(0x28);
                    loop {
                        let tmp: i32 = mech_trace::rd!(
                            "10056dd0",
                            p2.byte_add(10).cast::<i32>(),
                            read_unaligned
                        );
                        let ppu4 = mech_trace::cr!(
                            "10056dd0",
                            "10057200",
                            FUN_10057200(tmp, l50 as *mut c_void)
                        );
                        let mut iv7: i32;
                        loop {
                            iv7 = mech_trace::cr!("10056dd0", "10056be0", FUN_10056be0()) / 10;
                            let mut iv8: i32 = 0;
                            let mut pi5: *mut i32 = l70;
                            if pi5 < pi11 {
                                loop {
                                    if mech_trace::dr!("10056dd0", pi5) == iv7 {
                                        break;
                                    }
                                    pi5 = pi5.add(1);
                                    iv8 = iv8.wrapping_add(1);
                                    if !(iv8 < local_7c) {
                                        break;
                                    }
                                }
                            }
                            if mech_trace::dr!("10056dd0", l70.offset(iv8 as isize)) == iv7 {
                                iv7 = -1;
                            }
                            if iv7 != -1 {
                                break;
                            }
                        }
                        mech_trace::dw!("10056dd0", pi11, iv7);
                        mech_trace::cr!(
                            "10056dd0",
                            "_sprintf",
                            _sprintf(
                                l3c,
                                (&raw const DAT_100a35c0) as *const i8,
                                param_3,
                                mech_trace::dr!("10056dd0", ppu4.add(3)),
                            )
                        );

                        let names = (&raw mut DAT_100a1bdc) as *mut *mut i8;
                        let nm =
                            mech_trace::dr!("10056dd0", names.offset(iv7.wrapping_mul(2) as isize));
                        copy_str(l26, nm);

                        mech_trace::cr!(
                            "10056dd0",
                            "_sprintf",
                            _sprintf(
                                l4c,
                                (&raw const DAT_100a35c8) as *const i8,
                                mech_trace::dr!("10056dd0", ppu4.add(1))
                            )
                        );
                        if -1 < mech_trace::dr!("10056dd0", l50) {
                            copy_str(l60, l4c);

                            if (mech_trace::dr!("10056dd0", ppu4.add(1))).is_null() {
                                mech_trace::cr!(
                                    "10056dd0",
                                    "FID_conflict__fwprintf",
                                    FID_conflict__fwprintf(
                                        mech_trace::gr!("10056dd0", "100c6c30", DAT_100c6c30),
                                        (&raw const DAT_100a35d0) as *const i8,
                                        l60,
                                        mech_trace::dr!("10056dd0", l50),
                                    )
                                );
                                let r = mech_trace::cr!(
                                    "10056dd0",
                                    "_fflush",
                                    _fflush(mech_trace::gr!("10056dd0", "100c6c30", DAT_100c6c30))
                                );
                                return mech_trace::rt!("10056dd0", ((r as u32) & 0xffff0000) | 2);
                            }
                            let r7 = mech_trace::cr!(
                                "10056dd0",
                                "__strnicmp",
                                __strnicmp(l60.add(5), (&raw const DAT_100a35f4) as *const i8, 3)
                            );
                            if r7 == 0 {
                                mech_trace::cr!("10056dd0", "1005e020", FUN_1005e020(8, l60));
                            }
                            mech_trace::gw!("10056dd0", "100c6068", DAT_100c6068, local_80);
                            mech_trace::cr!(
                                "10056dd0",
                                "FID_conflict__fwprintf",
                                FID_conflict__fwprintf(
                                    mech_trace::gr!("10056dd0", "100c6c30", DAT_100c6c30),
                                    (&raw const DAT_100a35f8) as *const i8,
                                    mech_trace::dr!("10056dd0", ppu4.add(2)),
                                    local_80,
                                )
                            );
                            let sh = (&raw mut DAT_100a1c40) as *mut i16;
                            let base = local_84.wrapping_mul(7) as isize;
                            mech_trace::cr!(
                                "10056dd0",
                                "1005e290",
                                FUN_1005e290(
                                    0,
                                    l3c,
                                    l26,
                                    mech_trace::dr!("10056dd0", ppu4.add(2)),
                                    0,
                                    l4c,
                                    mech_trace::dr!("10056dd0", sh.offset(base)) as i32,
                                    mech_trace::dr!("10056dd0", sh.offset(base + 1)) as i32,
                                    mech_trace::dr!("10056dd0", sh.offset(base + 2)) as i32,
                                    mech_trace::dr!("10056dd0", sh.offset(base + 3)) as i32,
                                    mech_trace::dr!("10056dd0", sh.offset(base + 4)) as i32,
                                    0,
                                    0,
                                )
                            );
                            if local_80 == 1 {
                                copy_str(local_74 as *mut i8, l4c);
                                copy_str(p2.byte_add(0x58) as *mut i8, l3c);
                                copy_str(p2.byte_add(0x9a) as *mut i8, l26);
                                mech_trace::wr!(
                                    "10056dd0",
                                    p2.byte_add(0x24).cast::<i32>(),
                                    write_unaligned,
                                    mech_trace::dr!("10056dd0", l70.add(3))
                                );
                            }
                        }
                        pi11 = pi11.add(1);
                        local_74 = local_74.byte_add(0x10);
                        local_7c = local_7c.wrapping_add(1);
                        if !(local_7c < iv3) {
                            break;
                        }
                    }
                }
                mech_trace::cr!(
                    "10056dd0",
                    "_sprintf",
                    _sprintf(l10, (&raw const DAT_100a3620) as *const i8, local_80)
                );
                let u6 = mech_trace::cr!("10056dd0", "1005e6f0", FUN_1005e6f0(l10));
                if (u6 as i16) != 0 {
                    return mech_trace::rt!("10056dd0", u6);
                }
                u9 = local_80.wrapping_add(1);
                local_84 = local_84.wrapping_add(1);
                local_80 = u9;
                if !((u9 as i32) <= param_1) {
                    break;
                }
            }
        }
        mech_trace::rt!("10056dd0", u9 & 0xffff0000)
    }
}
