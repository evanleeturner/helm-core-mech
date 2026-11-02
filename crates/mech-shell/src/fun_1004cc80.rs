// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1009b7b0: u16;
    static mut DAT_1009b7b2: u8;
    static mut DAT_1009b7b4: i8;
    static mut DAT_1009b7b8: i8;
    static mut DAT_1009b7bc: u16;
    static mut DAT_1009b7be: u8;
    static mut DAT_1009b7c0: u16;
    static mut DAT_1009b7c2: u8;
    static mut DAT_100a67b0: u32;
    static mut s__nSalvaged_Mechs__n_1009b79c: i8;
    static mut s__nThere_were_no_mechs_salvaged___1009b7c4: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10018060(param_1: i32, param_2: i32) -> u32;
    fn FUN_10009840(param_1: *mut i8) -> *mut u8;
    fn sprintf(dest: *mut i8, format: *const i8, ...) -> i32;
}

unsafe fn end_of(p: *mut i8) -> *mut i8 {
    let mut q = p;
    // SAFETY: p points at a NUL-terminated string.
    unsafe {
        while mech_trace::dr!("1004cc80", q) != 0 {
            q = q.add(1);
        }
    }
    q
}

unsafe fn append(dst: *mut i8, src: *const i8) {
    // SAFETY: src is NUL-terminated; dst is NUL-terminated with room for src.
    unsafe {
        let mut n = 0usize;
        while mech_trace::dr!("1004cc80", src.add(n)) != 0 {
            n += 1;
        }
        n += 1;
        let e = end_of(dst);
        mech_trace::cp!("1004cc80", core::ptr::copy_nonoverlapping, src, e, n);
    }
}

unsafe fn append3(dst: *mut i8, a: u16, b: u8) {
    // SAFETY: dst is NUL-terminated with room for three more bytes.
    unsafe {
        let e = end_of(dst);
        mech_trace::wr!("1004cc80", e.cast::<u16>(), write_unaligned, a);
        mech_trace::wr!("1004cc80", e.add(2).cast::<u8>(), write, b);
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004cc80(param_1: *mut i8) {
    mech_trace::enter!("1004cc80", param_1);
    let mut local_100 = [0i8; 256];
    let buf = local_100.as_mut_ptr();
    let mut i_var7: i32 = 0;
    let mut i_var8: i32 = 0;
    let mut b_var2 = false;
    let mut local_108: i32 = 0;
    // SAFETY: the globals and callees are the game's own; param_1 is the
    // caller's NUL-terminated buffer with room for the appended text, and
    // collection items are read at the original's offsets.
    unsafe {
        let count = || {
            mech_trace::rd!(
                "1004cc80",
                ((mech_trace::gr!("1004cc80", "100a67b0", DAT_100a67b0) as usize + 4)
                    as *const i32),
                read_unaligned
            )
        };
        if 0 < count() {
            loop {
                i_var7 = mech_trace::cr!(
                    "1004cc80",
                    "10018060",
                    FUN_10018060(
                        mech_trace::gr!("1004cc80", "100a67b0", DAT_100a67b0) as i32,
                        i_var8
                    )
                ) as i32;
                if mech_trace::dr!("1004cc80", ((i_var7 as usize + 0x84e) as *const i8)) == 1 {
                    local_108 = local_108.wrapping_add(1);
                }
                i_var8 = i_var8.wrapping_add(1);
                if !(i_var8 < count()) {
                    break;
                }
            }
        }
        if local_108 < 1 {
            mech_trace::cr!(
                "1004cc80",
                "sprintf",
                sprintf(
                    buf,
                    &raw mut s__nThere_were_no_mechs_salvaged___1009b7c4 as *const i8,
                )
            );
            append(param_1, buf);
            return;
        }
        mech_trace::cr!(
            "1004cc80",
            "sprintf",
            sprintf(buf, &raw mut s__nSalvaged_Mechs__n_1009b79c as *const i8)
        );
        append(param_1, buf);
        i_var8 = 0;
        let mut local_104: i32 = 0;
        if 0 < count() {
            loop {
                if mech_trace::dr!("1004cc80", ((i_var7 as usize + 0x84e) as *const i8)) != 0 {
                    local_104 = local_104.wrapping_add(1);
                    i_var7 = mech_trace::cr!(
                        "1004cc80",
                        "10018060",
                        FUN_10018060(
                            mech_trace::gr!("1004cc80", "100a67b0", DAT_100a67b0) as i32,
                            i_var8
                        )
                    ) as i32;
                    if !b_var2 {
                        b_var2 = true;
                        append3(
                            param_1,
                            mech_trace::gr!("1004cc80", "1009b7b0", DAT_1009b7b0),
                            mech_trace::gr!("1004cc80", "1009b7b2", DAT_1009b7b2),
                        );
                    }
                    let mut arg = (i_var7 as usize + 0x100) as *mut i8;
                    let fmt: *const i8;
                    if mech_trace::dr!("1004cc80", arg) == 0 {
                        arg = mech_trace::cr!(
                            "1004cc80",
                            "10009840",
                            FUN_10009840((i_var7 as usize + 0x823) as *mut i8)
                        ) as *mut i8;
                        fmt = &raw mut DAT_1009b7b8 as *const i8;
                    } else {
                        fmt = &raw mut DAT_1009b7b4 as *const i8;
                    }
                    mech_trace::cr!("1004cc80", "sprintf", sprintf(buf, fmt, arg as *const i8));
                    append(param_1, buf);
                    if local_104 < local_108 {
                        append3(
                            param_1,
                            mech_trace::gr!("1004cc80", "1009b7bc", DAT_1009b7bc),
                            mech_trace::gr!("1004cc80", "1009b7be", DAT_1009b7be),
                        );
                    }
                }
                i_var8 = i_var8.wrapping_add(1);
                if !(i_var8 < count()) {
                    break;
                }
            }
        }
        append3(
            param_1,
            mech_trace::gr!("1004cc80", "1009b7c0", DAT_1009b7c0),
            mech_trace::gr!("1004cc80", "1009b7c2", DAT_1009b7c2),
        );
    }
}
