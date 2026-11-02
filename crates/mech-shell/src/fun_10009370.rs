// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ae180: i32;
    static mut DAT_100ad6e0: [i8; 0];
    static mut DAT_100ad660: [i8; 0];
    static mut DAT_100ae108: [i8; 0];
    static mut DAT_10083510: [i8; 0];
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

unsafe fn c_len(p: *const i8) -> u32 {
    let mut n: u32 = 0;
    // SAFETY: p points at a NUL-terminated string owned by the game.
    unsafe {
        while *p.add(n as usize) != 0 {
            n = n.wrapping_add(1);
        }
    }
    n
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10009370() {
    // SAFETY: all globals and callees are the game's own memory; pointers
    // are used exactly as the original does.
    unsafe {
        let mut i_var2: i32 = (DAT_100ae180 as u32).wrapping_add(1) as i32;
        if i_var2 < 200 {
            let mut pc: *const i8 = (DAT_100ae180 as u32)
                .wrapping_add(0x100ad6e1)
                .wrapping_add((i_var2 as u32).wrapping_mul(0xc))
                as *const i8;
            DAT_100ae180 = i_var2;
            loop {
                i_var2 = DAT_100ae180;
                if *pc != 0 {
                    break;
                }
                pc = pc.wrapping_add(0xd);
                DAT_100ae180 = (DAT_100ae180 as u32).wrapping_add(1) as i32;
                i_var2 = DAT_100ae180;
                if !(pc < (&raw mut DAT_100ae108).cast::<i8>() as *const i8) {
                    break;
                }
            }
        }
        DAT_100ae180 = i_var2;
        if 199 < DAT_100ae180 {
            DAT_100ae180 = 0;
        }
        FUN_10006d80(
            (&raw mut DAT_100ad6e0)
                .cast::<i8>()
                .wrapping_add((DAT_100ae180 as u32).wrapping_mul(0xd) as usize),
        );
        (&raw mut DAT_10083510).cast::<i8>().write(0x7e);

        let tbl =
            (DAT_100ae148 as u32).wrapping_add((DAT_10082d6c as u32).wrapping_mul(4)) as *const i32;
        let obj = tbl.read_unaligned();
        let src = (obj as u32).wrapping_add(0xc) as *const *const i8;
        let src: *const i8 = src.read_unaligned();

        let buf = (&raw mut DAT_100ad660).cast::<i8>();
        let n = c_len(src).wrapping_add(1);
        let mut i: u32 = 0;
        while i < n {
            *buf.add(i as usize) = *src.add(i as usize);
            i = i.wrapping_add(1);
        }

        let mut u5: u32 = 0;
        let len = c_len(buf);
        if len != 0 {
            loop {
                if *buf.add(u5 as usize) == 0x20 && *buf.add(u5 as usize + 1) == 0x20 {
                    break;
                }
                u5 = u5.wrapping_add(1);
                let l = c_len(buf);
                if !(u5 < l) {
                    break;
                }
            }
        }
        *buf.add(u5 as usize) = 0;

        let dst = (&raw mut DAT_10083510).cast::<i8>().add(1);
        let n = c_len(buf).wrapping_add(1);
        let mut i: u32 = 0;
        while i < n {
            *dst.add(i as usize) = *buf.add(i as usize);
            i = i.wrapping_add(1);
        }

        FUN_10063ea0(DAT_10082d08);
        DAT_10082d08 = core::ptr::null_mut();
        FUN_10063de0(DAT_10082d0c);
    }
}
