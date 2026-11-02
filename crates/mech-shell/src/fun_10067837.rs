// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a8a79: u32;
    static mut DAT_100a8a7d: u32;
    static mut DAT_100a8a81: u32;
    static mut DAT_100a8a85: u32;
    static mut DAT_100a8a89: u32;
    static mut DAT_100a8a8d: u32;
    static mut DAT_100a8aa1: u32;
    static mut DAT_100a8aa9: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10067837(param_1: i32, param_2: i32, param_3: i32) -> u32 {
    // SAFETY: the globals are the game's own memory; the output and source
    // pointers they hold are the caller's buffers, accessed where the original does.
    unsafe {
        if param_1 == 2 || param_1 == 1 {
            if DAT_100a8a7d != 0 {
                loop {
                    let mut p1 = DAT_100a8a7d;
                    if 0xfe < (p1 as i32) {
                        p1 = 0xff;
                    }
                    DAT_100a8a7d = DAT_100a8a7d.wrapping_sub(p1);
                    if DAT_100a8a79 != 0 {
                        let q = DAT_100a8a89 as *mut u8;
                        q.write(1);
                        q.add(1).write(p1 as u8);
                    }
                    DAT_100a8a89 = DAT_100a8a89.wrapping_add(2);
                    DAT_100a8a8d = DAT_100a8a8d.wrapping_add(p1);
                    if DAT_100a8a7d == 0 {
                        break;
                    }
                }
                DAT_100a8a7d = 0;
            }
            let q = DAT_100a8a85;
            let r = DAT_100a8a8d;
            let mut p3 = q.wrapping_add(0u32.wrapping_sub(param_2 as u32).wrapping_sub(r));
            let q = DAT_100a8a81;
            let mut p1 = r.wrapping_add((param_3 as u32).wrapping_sub(q));
            if (p1 as i32) < (DAT_100a8aa1 as i32) {
                DAT_100a8aa1 = p1;
            }
            let mut p2 = p1.wrapping_add(p3).wrapping_sub(1);
            let mut first = (p2 as i32) <= (DAT_100a8aa9 as i32);
            loop {
                if !first {
                    DAT_100a8aa9 = p2;
                    if p3 == 0 {
                        break;
                    }
                    p3 = p3.wrapping_sub(p1);
                }
                first = false;
                p1 = p3;
                if 0x7e < (p3 as i32) {
                    p1 = 0x7f;
                }
                if param_1 == 2 {
                    if DAT_100a8a79 != 0 {
                        let q = DAT_100a8a89 as *mut u8;
                        let r = DAT_100a8a8d as *const u8;
                        q.write(p1.wrapping_mul(2) as u8);
                        q.add(1).write(r.read());
                    }
                    DAT_100a8a89 = DAT_100a8a89.wrapping_add(2);
                    DAT_100a8a8d = DAT_100a8a8d.wrapping_add(p1);
                } else if DAT_100a8a79 == 0 {
                    DAT_100a8a8d = DAT_100a8a8d.wrapping_add(p1);
                    DAT_100a8a89 = DAT_100a8a89.wrapping_add(p1.wrapping_add(1));
                } else {
                    let q = DAT_100a8a89 as *mut u8;
                    q.write(p1.wrapping_mul(2).wrapping_add(1) as u8);
                    p2 = p1;
                    loop {
                        DAT_100a8a89 = DAT_100a8a89.wrapping_add(1);
                        if p2 == 0 {
                            break;
                        }
                        let q = DAT_100a8a89 as *mut u8;
                        let r = DAT_100a8a8d as *const u8;
                        q.write(r.read());
                        DAT_100a8a8d = DAT_100a8a8d.wrapping_add(1);
                        p2 = p2.wrapping_sub(1);
                    }
                }
                p2 = DAT_100a8aa9;
            }
        } else if param_1 == 3 {
            let q = DAT_100a8a85;
            let r = DAT_100a8a8d;
            DAT_100a8a7d = q.wrapping_add(0u32.wrapping_sub(param_2 as u32).wrapping_sub(r));
        } else if param_1 == 4 {
            if DAT_100a8a79 != 0 {
                (DAT_100a8a89 as *mut u8).write(0);
            }
            DAT_100a8a89 = DAT_100a8a89.wrapping_add(1);
        } else if param_1 == 0 {
            DAT_100a8a7d = 0;
            DAT_100a8a8d = DAT_100a8a85;
        }
    }
    0
}
