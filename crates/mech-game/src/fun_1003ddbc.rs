// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aefc0: u32;
    static mut DAT_100af098: u32;
    static mut DAT_100c6d5c: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003ddbc(param_1: *mut u16) {
    if param_1.is_null() {
        return;
    }
    // SAFETY: param_1 points at the caller's record; the globals and the
    // linked records they point to are the game's own memory, accessed at
    // the same offsets and in the same order as the original.
    unsafe {
        let p = param_1.cast::<u8>();
        if (param_1.read_unaligned() & 0x800) == 0 {
            return;
        }
        if p.byte_add(0x24).cast::<i32>().read_unaligned() == 4 {
            return;
        }
        param_1.write_unaligned(param_1.read_unaligned() & 0xf7ff);
        if p.byte_add(4).cast::<i32>().read_unaligned() == 0 {
            return;
        }
        if DAT_100af098 == 0 {
            return;
        }
        let me = p as usize as u32;
        if DAT_100aefc0 == 0 {
            let g = DAT_100af098 as usize as *mut u8;
            let head = g.byte_add(0x10).cast::<u32>().read_unaligned();
            if head != 0 {
                (head as usize as *mut u8)
                    .byte_add(0xc)
                    .cast::<u32>()
                    .write_unaligned(me);
            }
            p.byte_add(0x10)
                .cast::<u32>()
                .write_unaligned(g.byte_add(0x10).cast::<u32>().read_unaligned());
            g.byte_add(0x10).cast::<u32>().write_unaligned(me);
            p.byte_add(0xc).cast::<u32>().write_unaligned(DAT_100af098);
        } else {
            let base = (&raw mut DAT_100c6d5c) as *mut u8;
            let idx = |q: *mut u8| -> usize {
                let a = q.byte_add(0x48).cast::<i32>().read_unaligned();
                let b = q.byte_add(0x44).cast::<i32>().read_unaligned();
                a.wrapping_mul(0x18).wrapping_add(b.wrapping_mul(0x120)) as usize
            };
            let entry = |q: *mut u8| -> *mut u8 {
                base.wrapping_add(idx(q)).cast::<u32>().read_unaligned() as usize as *mut u8
            };
            let head = entry(p).byte_add(0x10).cast::<u32>().read_unaligned();
            if head != 0 {
                (head as usize as *mut u8)
                    .byte_add(0xc)
                    .cast::<u32>()
                    .write_unaligned(me);
            }
            let v = entry(p).byte_add(0x10).cast::<u32>().read_unaligned();
            p.byte_add(0x10).cast::<u32>().write_unaligned(v);
            entry(p).byte_add(0x10).cast::<u32>().write_unaligned(me);
            let e = base.wrapping_add(idx(p)).cast::<u32>().read_unaligned();
            p.byte_add(0xc).cast::<u32>().write_unaligned(e);
        }
    }
}
