// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b9f60: i32;
    static mut DAT_100b38d8: i32;
    static mut DAT_100adf38: i32;
    static mut DAT_100eef40: [i32; 0];
    static mut DAT_10215670: [u8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1008255f(param_1: u32, param_2: u32) -> i32 {
    if param_2 & 0x100 == 0 {
        return -1;
    }
    let u_var1: u32 = param_2 & 0xff;
    // SAFETY: the globals are the game's own memory; all pointers derived from
    // them address the game's tables exactly as the original does.
    unsafe {
        if (u_var1 as i32) >= DAT_100b9f60 {
            return -1;
        }
        let base = (&raw mut DAT_10215670).cast::<u8>();
        let flags = base
            .byte_add(0x24usize.wrapping_add(u_var1.wrapping_mul(0x54) as usize))
            .cast::<u16>()
            .read_unaligned();
        if flags & 1 == 0 {
            return -1;
        }
        let key = base
            .byte_add(8usize.wrapping_add(u_var1.wrapping_mul(0x54) as usize))
            .cast::<u32>()
            .read_unaligned();
        if key != (param_1 | 0x200) {
            return -1;
        }
        let mut local_c: u32 = u_var1;
        while (local_c as i32) < (DAT_100b9f60 as u32).wrapping_sub(1) as i32 {
            let mut src = base
                .byte_add(local_c.wrapping_add(1).wrapping_mul(0x54) as usize)
                .cast::<u32>();
            let mut dst = base
                .byte_add(local_c.wrapping_mul(0x54) as usize)
                .cast::<u32>();
            let mut n: i32 = 0x15;
            while n != 0 {
                dst.write_unaligned(src.read_unaligned());
                src = src.add(1);
                dst = dst.add(1);
                n = (n as u32).wrapping_sub(1) as i32;
            }
            local_c = local_c.wrapping_add(1);
        }
        local_c = 0;
        while (local_c as i32) < DAT_100b38d8 {
            let entry = (&raw mut DAT_100eef40)
                .cast::<i32>()
                .add(local_c as usize)
                .read();
            if entry != 0 {
                let p = entry as usize as *mut u8;

                let a = p.byte_add(0x178).cast::<u16>();
                let v = a.read_unaligned();
                if (v & 0x100) != 0 && u_var1 < (a.read_unaligned() & 0xff) as u32 {
                    a.write_unaligned(a.read_unaligned().wrapping_sub(1));
                }
                let b = p.byte_add(0x17a).cast::<u16>();
                let v = b.read_unaligned();
                if (v & 0x100) != 0 && u_var1 < (b.read_unaligned() & 0xff) as u32 {
                    b.write_unaligned(b.read_unaligned().wrapping_sub(1));
                }
                let c = p.byte_add(0xdc).cast::<u32>();
                if (p.byte_add(0xdd).read() & 1) != 0
                    && u_var1 < (c.read_unaligned() & 0xff)
                    && p.byte_add(4).cast::<i32>().read_unaligned() != DAT_100adf38
                {
                    c.write_unaligned(c.read_unaligned().wrapping_sub(1));
                }
                if (p.byte_add(0x1a3).read() & 1) != 0 {
                    let d = p.byte_add(0x1a2).cast::<u32>();
                    let tmp = d.read_unaligned();
                    if u_var1 < (tmp & 0xff) {
                        let tmp = d.read_unaligned().wrapping_sub(1);
                        d.write_unaligned(tmp);
                    }
                }
            }
            local_c = local_c.wrapping_add(1);
        }
        DAT_100b9f60 = (DAT_100b9f60 as u32).wrapping_sub(1) as i32;
        DAT_100b9f60
    }
}
