// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100afaf8: u8;
    static mut DAT_100afb40: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10042289(param_1: *mut i32) {
    let p = param_1 as *mut u8;
    // SAFETY: param_1 points at the caller's game object, accessed at the
    // same offsets the original uses; the globals are the game's own.
    unsafe {
        p.byte_add(0x1c4).cast::<u16>().write_unaligned(0xffff);
        p.byte_add(0x1c6).cast::<u16>().write_unaligned(0xffff);
        p.byte_add(0x1c8).cast::<i32>().write_unaligned(0);
        p.byte_add(0x1dc).cast::<i32>().write_unaligned(0);
        p.byte_add(0x1e4).cast::<u16>().write_unaligned(0);
        p.byte_add(0x1e6).cast::<u32>().write_unaligned(1);
        p.byte_add(0x1e0).cast::<i32>().write_unaligned(0);
        p.byte_add(0x1f6).cast::<u32>().write_unaligned(0);
        p.byte_add(0x1fa).cast::<u32>().write_unaligned(0);

        let cp = p.byte_add(0x187).cast::<i8>();
        let fp = p.byte_add(0x1f2).cast::<u32>();

        if cp.read() < 1 {
            cp.write(1);
        }
        if 6 < cp.read() {
            cp.write(6);
        }
        if cp.read() < 4 {
            fp.write_unaligned(fp.read_unaligned() | 1);
        } else {
            fp.write_unaligned(fp.read_unaligned() & 0xfffffffe);
        }
        if cp.read() < 4 {
            fp.write_unaligned(fp.read_unaligned() | 2);
        } else {
            fp.write_unaligned(fp.read_unaligned() & 0xfffffffd);
        }
        if cp.read() < 3 {
            fp.write_unaligned(fp.read_unaligned() | 4);
        } else {
            fp.write_unaligned(fp.read_unaligned() & 0xfffffffb);
        }
        if param_1.read() == 1 && cp.read() < 6 {
            fp.write_unaligned(fp.read_unaligned() | 8);
        } else {
            fp.write_unaligned(fp.read_unaligned() & 0xfffffff7);
        }
        if cp.read() < 5 {
            fp.write_unaligned(fp.read_unaligned() | 0x10);
        } else {
            fp.write_unaligned(fp.read_unaligned() & 0xffffffef);
        }
        if param_1.read() == 1 && cp.read() < 2 {
            fp.write_unaligned(fp.read_unaligned() | 0x20);
        } else {
            fp.write_unaligned(fp.read_unaligned() & 0xffffffdf);
        }
        if cp.read() < 6 {
            fp.write_unaligned(fp.read_unaligned() | 0x40);
        } else {
            fp.write_unaligned(fp.read_unaligned() & 0xffffffbf);
        }
        if param_1.read() == 1 && ((fp.read_unaligned() >> 3) & 1) == 0 {
            let base = param_1.add(8).read() as u32;
            (base.wrapping_add(0x414) as *mut u32).write_unaligned(0);
        }
        let sp = p.byte_add(0x202).cast::<u32>();
        if sp.read_unaligned() == 0 {
            let v =
                (&raw mut DAT_100afaf8 as u32).wrapping_add(param_1.read().wrapping_mul(8) as u32);
            sp.write_unaligned(v);
            let base = param_1.add(8).read() as u32;
            if (base.wrapping_add(0x418) as *const i32).read_unaligned() == 1 {
                sp.write_unaligned(&raw mut DAT_100afb40 as u32);
            }
        }
        let ap = p.byte_add(0x1fe).cast::<u32>();
        if ap.read_unaligned() == 0 {
            ap.write_unaligned(15000);
        }
    }
}
