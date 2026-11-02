// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ae180: i32;
    static mut DAT_100ad6e0: [i8; 0];
    static mut DAT_10083510: [u32; 0x215];
    static mut DAT_100ad508: [i8; 0];
    static mut DAT_100ae148: i32;
    static mut DAT_10082d6c: i32;
    static mut DAT_10082d08: *mut i32;
    static mut DAT_10082d0c: *mut i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10006d80(param_1: *mut i8);
    fn FUN_10063ea0(param_1: *mut i32);
    fn FUN_10063de0(param_1: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100094c0() {
    // SAFETY: all globals and callees are the game's own memory; pointers
    // are formed and dereferenced exactly where the original does.
    unsafe {
        if DAT_100ae180 == 0 {
            DAT_100ae180 = 200;
        }
        let mut i_var2: i32 = (DAT_100ae180 as u32).wrapping_sub(1) as i32;
        if -1 < i_var2 {
            let mut pc: u32 = (DAT_100ae180 as u32)
                .wrapping_add(0x100ad6df)
                .wrapping_add((i_var2 as u32).wrapping_mul(0xc));
            DAT_100ae180 = i_var2;
            loop {
                i_var2 = DAT_100ae180;
                if *(pc as *const i8) != 0 {
                    break;
                }
                pc = pc.wrapping_sub(0xd);
                DAT_100ae180 = (DAT_100ae180 as u32).wrapping_sub(1) as i32;
                i_var2 = DAT_100ae180;
                if !(0x100ad6df < pc) {
                    break;
                }
            }
        }
        DAT_100ae180 = i_var2;
        if DAT_100ae180 < 0 {
            DAT_100ae180 = 0;
        }
        FUN_10006d80(
            ((&raw mut DAT_100ad6e0) as *mut u8 as u32)
                .wrapping_add((DAT_100ae180 as u32).wrapping_mul(0xd)) as *mut i8,
        );
        let tbl = (&raw mut DAT_10083510) as *mut u8;
        *tbl = 0x7e;

        let src = ((DAT_100ae148.wrapping_add(DAT_10082d6c.wrapping_mul(4))) as *const i32)
            .read_unaligned();
        let src = ((src as u32).wrapping_add(0xc) as *const u32).read_unaligned() as *const u8;
        let buf = (&raw mut DAT_100ad508) as *mut u8;

        let mut n: usize = 0;
        while *src.add(n) != 0 {
            n += 1;
        }
        n += 1;
        let mut k = 0;
        while k < n {
            *buf.add(k) = *src.add(k);
            k += 1;
        }

        let mut len: usize = 0;
        while *buf.add(len) != 0 {
            len += 1;
        }
        let mut u5: usize = 0;
        if len != 0 {
            loop {
                if *buf.add(u5) == b' ' && *buf.add(u5 + 1) == b' ' {
                    break;
                }
                u5 += 1;
                let mut l = 0usize;
                while *buf.add(l) != 0 {
                    l += 1;
                }
                if !(u5 < l) {
                    break;
                }
            }
        }
        *buf.add(u5) = 0;

        let mut n: usize = 0;
        while *buf.add(n) != 0 {
            n += 1;
        }
        n += 1;
        let mut k = 0;
        while k < n {
            *tbl.add(1 + k) = *buf.add(k);
            k += 1;
        }

        FUN_10063ea0(DAT_10082d08);
        DAT_10082d08 = core::ptr::null_mut();
        FUN_10063de0(DAT_10082d0c);
    }
}
