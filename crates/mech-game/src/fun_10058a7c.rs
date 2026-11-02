// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b2b10: [u8; 256];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10058a7c(param_1: i32, param_2: i32) -> u32 {
    // SAFETY: param_1 points at the game's own table structure, and the
    // offsets, row data and translation table are read and written where the
    // original does.
    unsafe {
        let off = param_2
            .wrapping_mul(8)
            .wrapping_add(8)
            .wrapping_add(param_1);
        let i_var5 = (off as *const i32).read_unaligned().wrapping_add(param_1);
        let i_var2 = ((i_var5.wrapping_add(0x14)) as *const i32)
            .read_unaligned()
            .wrapping_add(1);
        let base = ((i_var5.wrapping_add(0xc)) as *const i32).read_unaligned();
        let mut i_var3 = i_var2.wrapping_sub(base);
        if i_var3 != 0 && base <= i_var2 {
            let tbl = (&raw mut DAT_100b2b10).cast::<u8>() as *const u8;
            let mut p = i_var5.wrapping_add(0x18) as *mut u8;
            loop {
                let mut h = p.read();
                p = p.add(1);
                loop {
                    let flag = h & 1;
                    let count = h >> 1;
                    if count != 0 {
                        if flag == 0 {
                            p.write(tbl.add(p.read() as usize).read());
                            h = p.add(1).read();
                            p = p.add(2);
                        } else {
                            let mut n = count as u32;
                            while n != 0 {
                                p.write(tbl.add(p.read() as usize).read());
                                p = p.add(1);
                                n -= 1;
                            }
                            h = p.read();
                            p = p.add(1);
                        }
                    } else if flag != 0 {
                        h = p.add(1).read();
                        p = p.add(2);
                    } else {
                        break;
                    }
                }
                i_var3 = i_var3.wrapping_sub(1);
                if i_var3 == 0 {
                    break;
                }
            }
        }
        0
    }
}
