// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1009b808: u32;
    static mut DAT_1009b80c: u8;
    static mut DAT_100a6798: u32;
    static mut DAT_100a67a4: u32;
    static mut DAT_100a67a8: u32;
    static mut DAT_1009b810: u8;
    static mut DAT_1009b7e8: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10018060(param_1: i32, param_2: i32) -> u32;
    fn FUN_1000ad70(param_1: i32, param_2: i32) -> i32;
    fn FUN_1000cf10(param_1: i32) -> u32;
    fn FUN_10009840(param_1: *mut i8) -> *mut u8;
    fn FUN_1000ae80(param_1: *mut i8) -> *mut i8;
    fn sprintf(dest: *mut i8, format: *const i8, ...) -> i32;
}

unsafe fn append_cstr(dst: *mut i8, src: *const i8) {
    // SAFETY: both are NUL-terminated buffers supplied by the caller; the
    // source is copied, with its terminator, over the destination's terminator.
    unsafe {
        let mut d = dst;
        while *d != 0 {
            d = d.add(1);
        }
        let mut s = src;
        loop {
            let c = *s;
            *d = c;
            if c == 0 {
                break;
            }
            s = s.add(1);
            d = d.add(1);
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004cec0(param_1: *mut i8) {
    let mut local_120: [i8; 32] = [0; 32];
    let mut local_100: [i8; 256] = [0; 256];
    // SAFETY: the globals, callees and returned item pointers are the game's
    // own memory; param_1 is the caller's NUL-terminated output buffer.
    unsafe {
        let mut i_var9: i32 = 0;
        if 0 < ((DAT_100a67a4.wrapping_add(4)) as *const i32).read_unaligned() {
            loop {
                let i_var2 = FUN_10018060(DAT_100a67a4 as i32, i_var9) as i32;
                if *(i_var2.wrapping_add(0x2064) as *const i8) == 0 {
                    let i_var3 = FUN_1000ad70(
                        DAT_100a6798 as i32,
                        (i_var2.wrapping_add(0x24) as *const i32).read_unaligned(),
                    );
                    if i_var3 != 0 {
                        let u_var4 = FUN_1000cf10(i_var3);
                        if (u_var4 as u8) == 1 {
                            let p = FUN_10009840(i_var3.wrapping_add(0x81c) as *mut i8);
                            sprintf(
                                local_100.as_mut_ptr(),
                                &raw mut DAT_1009b7e8 as *const i8,
                                i_var2,
                                p,
                            );
                            append_cstr(param_1, local_100.as_ptr());
                        }
                    }
                }
                i_var9 = i_var9.wrapping_add(1);
                if !(i_var9 < ((DAT_100a67a4.wrapping_add(4)) as *const i32).read_unaligned()) {
                    break;
                }
            }
        }
        i_var9 = 0;
        if 0 < ((DAT_100a67a8.wrapping_add(4)) as *const i32).read_unaligned() {
            loop {
                let i_var2 = FUN_10018060(DAT_100a67a8 as i32, i_var9) as i32;
                if *(i_var2.wrapping_add(0x2c) as *const u8) == 1 {
                    let mut s = i_var2.wrapping_add(0x4d) as *const i8;
                    let mut d = local_120.as_mut_ptr();
                    loop {
                        let c = *s;
                        *d = c;
                        if c == 0 {
                            break;
                        }
                        s = s.add(1);
                        d = d.add(1);
                    }
                    local_120[5] = 0x4d;
                    local_120[6] = 0x45;
                    local_120[7] = 0x52;
                    let mut e = local_120.as_mut_ptr();
                    while *e != 0 {
                        e = e.add(1);
                    }
                    (e as *mut u32).write_unaligned(DAT_1009b808);
                    *e.add(4) = DAT_1009b80c as i8;
                    let p_var6 = FUN_1000ae80(local_120.as_mut_ptr());
                    let u_var4 = FUN_1000cf10(p_var6 as i32);
                    if (u_var4 as u8) == 1 {
                        *(i_var2.wrapping_add(0x5d) as *mut u8) = 1;
                        let p = FUN_10009840(p_var6.wrapping_add(0x81c));
                        sprintf(
                            local_100.as_mut_ptr(),
                            &raw mut DAT_1009b810 as *const i8,
                            i_var2,
                            p,
                        );
                        append_cstr(param_1, local_100.as_ptr());
                    }
                }
                i_var9 = i_var9.wrapping_add(1);
                if !(i_var9 < ((DAT_100a67a8.wrapping_add(4)) as *const i32).read_unaligned()) {
                    break;
                }
            }
        }
    }
}
