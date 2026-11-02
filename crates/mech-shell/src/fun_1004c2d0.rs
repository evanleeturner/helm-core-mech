// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1009b6f4: u32;
    static mut DAT_1009b6f8: u8;
    static mut DAT_1009b6fc: [i8; 0];
    static mut DAT_100a678c: i32;
    static mut DAT_100a6798: i32;
    static mut DAT_100a67a4: i32;
    static mut DAT_100a67a8: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10018060(param_1: i32, param_2: i32) -> u32;
    fn FUN_1000ad70(param_1: i32, param_2: i32) -> i32;
    fn FUN_1000cf10(param_1: i32) -> u32;
    fn FUN_1000ad30(param_1: i32, param_2: i32) -> i32;
    fn FUN_10017ec0(param_1: i32, param_2: i32, param_3: i8);
    fn sprintf(dest: *mut i8, format: *const i8, ...) -> i32;
    fn FUN_1006f780(param_1: *const u8);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1004c2d0() {
    // SAFETY: all globals, callees and object pointers belong to the game;
    // the local buffer is the original's 8-byte stack area accessed through
    // raw pointers exactly as the original does.
    unsafe {
        let mut local_20: [u8; 8] = [0; 8];
        let buf = local_20.as_mut_ptr();

        let mut i_var8: i32 = 0;
        if 0 < ((DAT_100a67a4 as u32).wrapping_add(4) as *const i32).read_unaligned() {
            loop {
                let i_var2 = FUN_10018060(DAT_100a67a4, i_var8) as i32;
                if *((i_var2 as u32).wrapping_add(0x2064) as *const i8) == 0 {
                    let i_var3 = FUN_1000ad70(
                        DAT_100a6798,
                        ((i_var2 as u32).wrapping_add(0x24) as *const i32).read_unaligned(),
                    );
                    if i_var3 != 0 {
                        let u_var4 = FUN_1000cf10(i_var3);
                        if (u_var4 as u8 as i8) == 1 {
                            let i_var5 = FUN_1000ad30(
                                DAT_100a678c,
                                ((i_var3 as u32).wrapping_add(0x810) as *const i32)
                                    .read_unaligned(),
                            );
                            FUN_10017ec0(DAT_100a6798, i_var3, 1);
                            FUN_10017ec0(DAT_100a678c, i_var5, 1);
                            FUN_10017ec0(DAT_100a67a4, i_var2, 1);
                        }
                    }
                }
                i_var8 = i_var8.wrapping_add(1);
                if !(i_var8
                    < ((DAT_100a67a4 as u32).wrapping_add(4) as *const i32).read_unaligned())
                {
                    break;
                }
            }
        }

        i_var8 = 0;
        if 0 < ((DAT_100a67a8 as u32).wrapping_add(4) as *const i32).read_unaligned() {
            loop {
                let i_var2 = FUN_10018060(DAT_100a67a8, i_var8) as i32;
                if *((i_var2 as u32).wrapping_add(0x2c) as *const i8) == 1 {
                    // strlen including the terminator
                    let src = (i_var2 as u32).wrapping_add(0x4d) as *const u8;
                    let mut len: u32 = 0;
                    loop {
                        let c = *src.add(len as usize);
                        len = len.wrapping_add(1);
                        if c == 0 {
                            break;
                        }
                    }
                    let mut s = src;
                    let mut d = buf;
                    let mut n = len >> 2;
                    while n != 0 {
                        (d as *mut u32).write_unaligned((s as *const u32).read_unaligned());
                        s = s.add(4);
                        d = d.add(4);
                        n -= 1;
                    }
                    let mut n = len & 3;
                    while n != 0 {
                        *d = *s;
                        s = s.add(1);
                        d = d.add(1);
                        n -= 1;
                    }
                    *buf.add(5) = 0x4d;
                    *buf.add(6) = 0x45;
                    *buf.add(7) = 0x52;
                    let mut p = buf;
                    while *p != 0 {
                        p = p.add(1);
                    }
                    (p as *mut u32).write_unaligned(DAT_1009b6f4);
                    *p.add(3) = DAT_1009b6f8;
                    sprintf(
                        buf as *mut i8,
                        (&raw mut DAT_1009b6fc) as *const i8,
                        buf as *mut i8,
                    );
                    FUN_1006f780(buf as *const u8);
                    if *((i_var2 as u32).wrapping_add(0x5d) as *const i8) == 1 {
                        FUN_10017ec0(DAT_100a67a8, i_var2, 1);
                    }
                }
                i_var8 = i_var8.wrapping_add(1);
                if !(i_var8
                    < ((DAT_100a67a8 as u32).wrapping_add(4) as *const i32).read_unaligned())
                {
                    break;
                }
            }
        }
    }
}
