// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10094438: u8;
    static mut DAT_10098224: *mut u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapAlloc(h: *mut u8, flags: u32, size: u32) -> *mut u8;
}

unsafe extern "C" {
    fn strncpy(dest: *mut u8, src: *const u8, count: usize) -> *mut u8;
}

unsafe fn copy_cstr(mut src: *const u8, mut dst: *mut u8) {
    // SAFETY: src is a NUL-terminated string and dst has room for it,
    // as in the original.
    unsafe {
        let mut n: usize = 1;
        while mech_trace::dr!("10038160", src.add(n - 1)) != 0 {
            n += 1;
        }
        let mut i = 0;
        while i < n {
            mech_trace::dw!("10038160", dst, mech_trace::dr!("10038160", src));
            src = src.add(1);
            dst = dst.add(1);
            i += 1;
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10038160(param_1: i32) -> *mut u8 {
    mech_trace::enter!("10038160", param_1);
    let mut dest: *mut u8 = core::ptr::null_mut();
    if param_1 == 0 {
        return mech_trace::rt!("10038160", dest);
    }
    let base = param_1 as u32 as *const u8;
    let mut local_8: i32 = 0xc;
    // SAFETY: param_1 points at the game's chunk buffer; globals and callees
    // are the game's own; reads and writes follow the original's layout.
    unsafe {
        let i_var2 = mech_trace::rd!(
            "10038160",
            base.wrapping_offset(4).cast::<i32>(),
            read_unaligned
        );
        let mut p9 = base.wrapping_offset(0xc);
        if 0xc < i_var2 {
            loop {
                let tag = mech_trace::rd!("10038160", p9.cast::<i32>(), read_unaligned);
                if tag == 0x48434554 || tag == 0x544c4950 {
                    dest = mech_trace::cr!(
                        "10038160",
                        "HeapAlloc",
                        HeapAlloc(
                            mech_trace::gr!("10038160", "10098224", DAT_10098224),
                            1,
                            0x205e
                        )
                    );
                    mech_trace::fill!("10038160", core::ptr::write_bytes, dest, 0, 0x205e);
                    mech_trace::cr!(
                        "10038160",
                        "strncpy",
                        strncpy(dest, p9.wrapping_offset(8), 0x1f)
                    );
                    mech_trace::dw!("10038160", dest.add(0x20), 0);
                    mech_trace::dw!("10038160", dest.add(0x2c), 0);
                    mech_trace::dw!("10038160", dest.add(0x20), 0xff);
                    mech_trace::dw!("10038160", dest.add(0x21), 0xff);
                    mech_trace::dw!("10038160", dest.add(0x22), 0xff);
                    mech_trace::dw!("10038160", dest.add(0x23), 0xff);
                    let mut src8 = p9.wrapping_offset(0x28);
                    let mut out = dest.add(0x2d);
                    let mut k = 8;
                    while k != 0 {
                        let v = mech_trace::rd!("10038160", src8.cast::<i32>(), read_unaligned);
                        src8 = src8.wrapping_offset(2);
                        mech_trace::wr!("10038160", out.cast::<i16>(), write_unaligned, v as i16);
                        out = out.add(2);
                        k -= 1;
                    }
                    copy_cstr(p9.wrapping_offset(0x3c), dest.add(0x3d));
                    copy_cstr(p9.wrapping_offset(0x4c), dest.add(0x4d));
                    let v = mech_trace::rd!(
                        "10038160",
                        p9.wrapping_offset(0x38).cast::<i32>(),
                        read_unaligned
                    );
                    mech_trace::wr!("10038160", dest.add(0x24).cast::<i32>(), write_unaligned, v);
                    mech_trace::dw!("10038160", dest.add(0x28), 0);
                    mech_trace::dw!("10038160", dest.add(0x29), 0);
                    mech_trace::dw!("10038160", dest.add(0x2a), 0);
                    mech_trace::dw!("10038160", dest.add(0x2b), 0);
                    mech_trace::dw!("10038160", dest.add(0x5d), 0);
                    copy_cstr(&raw mut DAT_10094438 as *const u8, dest.add(0x5e));
                    local_8 = local_8.wrapping_add(0x5c);
                } else {
                    let len = mech_trace::rd!(
                        "10038160",
                        p9.wrapping_offset(4).cast::<i32>(),
                        read_unaligned
                    );
                    if tag == 0x52535344 {
                        mech_trace::cr!(
                            "10038160",
                            "strncpy",
                            strncpy(
                                dest.wrapping_offset(0x5e),
                                p9.wrapping_offset(8),
                                len.wrapping_sub(1) as usize,
                            )
                        );
                        let len2 = mech_trace::rd!(
                            "10038160",
                            p9.wrapping_offset(4).cast::<i32>(),
                            read_unaligned
                        );
                        mech_trace::dw!(
                            "10038160",
                            dest.wrapping_offset(len2.wrapping_add(0x5e) as isize),
                            0
                        );
                    }
                    let len3 = mech_trace::rd!(
                        "10038160",
                        p9.wrapping_offset(4).cast::<i32>(),
                        read_unaligned
                    );
                    local_8 = local_8.wrapping_add(len3);
                }
                p9 = base.wrapping_offset(local_8 as isize);
                if !(local_8 < i_var2) {
                    break;
                }
            }
        }
    }
    mech_trace::rt!("10038160", dest)
}
