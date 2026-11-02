// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa2c8: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1006761c(param_1: i32, param_2: i32) -> u32 {
    // SAFETY: param_1 points at the game's resource block; the offsets read and
    // the bytes rewritten are those the original reads and writes, and the
    // table is the game's own.
    unsafe {
        let tbl = &raw const DAT_100aa2c8;
        let slot = param_2
            .wrapping_mul(8)
            .wrapping_add(8)
            .wrapping_add(param_1) as *const i32;
        let i_var5 = slot.read_unaligned().wrapping_add(param_1);
        let i_var2 = ((i_var5.wrapping_add(0x14)) as *const i32)
            .read_unaligned()
            .wrapping_add(1);
        let lim = ((i_var5.wrapping_add(0xc)) as *const i32).read_unaligned();
        let mut i_var3 = i_var2.wrapping_sub(lim);
        if i_var3 != 0 && lim <= i_var2 {
            let mut p = i_var5.wrapping_add(0x18) as *mut u8;
            loop {
                let mut h = p.read();
                p = p.add(1);
                loop {
                    let flag = h & 1;
                    let cnt = h >> 1;
                    if flag == 0 {
                        if cnt == 0 {
                            break;
                        }
                        p.write(tbl.add(p.read() as usize).read());
                        h = p.add(1).read();
                        p = p.add(2);
                    } else if cnt != 0 {
                        let mut c = cnt;
                        while c != 0 {
                            p.write(tbl.add(p.read() as usize).read());
                            p = p.add(1);
                            c -= 1;
                        }
                        h = p.read();
                        p = p.add(1);
                    } else {
                        h = p.add(1).read();
                        p = p.add(2);
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
