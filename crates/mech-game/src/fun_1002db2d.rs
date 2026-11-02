// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ad6e4: i32;
    static mut DAT_100b0710: i32;
    static mut DAT_100af17c: i32;
    static mut DAT_101686f0: i32;
    static mut DAT_100af0b8: *mut u8;
    static mut DAT_100eebbe: u8;
    static mut DAT_100b38fc: i32;
    static mut DAT_100b38d8: i32;
    static mut DAT_100eef40: [i32; 0];
    static mut DAT_101eb4b0: [i16; 0];
    static mut DAT_10123244: [u8; 0];
    static mut DAT_10123248: [u8; 0];
    static mut DAT_1012324c: [u8; 0];
    static mut DAT_10123278: [u8; 0];

    static mut DAT_100ad784: [i8; 0];
    static mut DAT_100ad798: [i8; 0];
    static mut DAT_100ad7ac: [i8; 0];
    static mut DAT_100ad7c0: [i8; 0];
    static mut DAT_100ad7d4: [i8; 0];
    static mut DAT_100ad7e4: [i8; 0];
    static mut DAT_100ad7f8: [i8; 0];
    static mut DAT_100ad80c: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1008668b(param_1: i32, param_2: i32, param_3: *mut i32) -> *mut u16;
    fn FUN_1005aae4(param_1: i32) -> i32;
    fn FUN_1005acaa(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: *mut u8,
        param_6: i32,
    );
    fn FUN_1006670a(
        param_1: *mut i32,
        param_2: *mut i8,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
    );
    fn FUN_1002d4f1(param_1: i32, param_2: *mut i32, param_3: i32, param_4: i8);
    fn FUN_1002d360(param_1: i32) -> *mut u8;
    fn FUN_1002d415(param_1: i32) -> *mut u8;
    fn FUN_1008664f(param_1: i32, param_2: *mut i32);
    fn sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002db2d(param_1: i32) {
    // SAFETY: all globals, callees and the structure at param_1 are the
    // game's own memory; accesses mirror the original's reads and writes.
    unsafe {
        let base = param_1 as usize as *const u8;
        let s_tmp = base.byte_add(4).cast::<i16>().read_unaligned();
        if s_tmp == 0 || DAT_100ad6e4 == 0 {
            return;
        }
        let i_var1 = DAT_100b0710.wrapping_mul(0x3c0a);
        let puv2 = FUN_1008668b(
            DAT_100af17c,
            DAT_101686f0.wrapping_add(1),
            DAT_100af0b8 as *mut i32,
        );
        if puv2.is_null() {
            return;
        }
        let mut local_108 = [0u8; 256];
        let buf = local_108.as_mut_ptr();
        let local_10c = FUN_1005aae4(puv2 as i32);
        let local_8 = local_10c / 2;
        let p_a8 = base.byte_add(0xa8).cast::<*mut i32>().read_unaligned();
        let mut local_118: i32 = p_a8.read_unaligned();
        let mut local_114: i32 = p_a8.add(1).read_unaligned();
        let p_a4 = base.byte_add(0xa4).cast::<*mut i32>().read_unaligned();
        let l118 = &raw mut local_118;
        if DAT_100ad6e4 == 1 {
            DAT_100eebbe = 6;
            FUN_1005acaa(
                p_a4,
                local_118,
                local_114,
                puv2 as i32,
                (&raw mut DAT_100ad784).cast::<u8>(),
                0x100eebb0,
            );
            DAT_100eebbe = 0xe;
            FUN_1006670a(
                p_a4,
                (&raw mut DAT_100ad798).cast::<i8>(),
                local_118,
                local_114,
                puv2 as i32,
                6,
            );
            local_114 = local_114.wrapping_add(local_8).wrapping_add(local_10c);
            FUN_1002d4f1(param_1, l118, puv2 as i32, 1);
            local_114 = local_114.wrapping_add(local_8);
            FUN_1002d4f1(param_1, l118, puv2 as i32, 2);
            local_114 = local_114.wrapping_add(local_8);
            FUN_1002d4f1(param_1, l118, puv2 as i32, 4);
            FUN_1002d4f1(param_1, l118, puv2 as i32, 0);
            local_114 = local_114.wrapping_add(local_8);
            FUN_1002d4f1(param_1, l118, puv2 as i32, 8);
            local_118 =
                (*(base.byte_add(0xa8).cast::<*mut i32>().read_unaligned())).wrapping_add(0);
            local_114 = local_114.wrapping_add(local_8).wrapping_add(local_10c);
            let off = i_var1 as isize;
            let sel = (&raw mut DAT_10123278).cast::<u8>().offset(off).read();
            let pa = (&raw mut DAT_10123244).cast::<u8>().offset(off);
            let pb = (&raw mut DAT_1012324c).cast::<u8>().offset(off);
            let pc = (&raw mut DAT_10123248).cast::<u8>().offset(off);
            match sel {
                0 => {
                    let b = pb.cast::<i32>().read_unaligned();
                    if b < 1 {
                        let a = pa.cast::<i32>().read_unaligned();
                        let local_110 = DAT_100b38fc.wrapping_add(a.wrapping_mul(-0xb5));
                        let p3 = FUN_1002d360(local_110);
                        sprintf(buf.cast::<i8>(), (&raw const DAT_100ad7c0).cast::<i8>(), p3);
                    } else {
                        let a = pa.cast::<i32>().read_unaligned();
                        let b = pb.cast::<i32>().read_unaligned();
                        let local_110 = a
                            .wrapping_add(b)
                            .wrapping_mul(0xb5)
                            .wrapping_sub(DAT_100b38fc);
                        let p3 = FUN_1002d360(local_110);
                        sprintf(buf.cast::<i8>(), (&raw const DAT_100ad7ac).cast::<i8>(), p3);
                    }
                }
                2 => {
                    let a = pc.cast::<i32>().read_unaligned();
                    let p3 = FUN_1002d415(a);
                    sprintf(buf.cast::<i8>(), (&raw const DAT_100ad7d4).cast::<i8>(), p3);
                }
                3 => {
                    let a = pc.cast::<i32>().read_unaligned();
                    let p3 = FUN_1002d415(a);
                    sprintf(buf.cast::<i8>(), (&raw const DAT_100ad7f8).cast::<i8>(), p3);
                }
                4 => {
                    let a = pc.cast::<i32>().read_unaligned();
                    let p3 = FUN_1002d415(a);
                    sprintf(buf.cast::<i8>(), (&raw const DAT_100ad7e4).cast::<i8>(), p3);
                }
                _ => {}
            }
            DAT_100eebbe = 6;
            FUN_1005acaa(p_a4, local_118, local_114, puv2 as i32, buf, 0x100eebb0);
            DAT_100eebbe = 0xe;
        } else {
            let mut i: i32 = 0;
            while i < DAT_100b38d8 {
                let item = (&raw mut DAT_100eef40)
                    .cast::<i32>()
                    .offset(i as isize)
                    .read();
                if item == 0 {
                    break;
                }
                let ip = item as usize as *const u8;
                if (ip.byte_add(0x14).cast::<u16>().read_unaligned() & 0x4800) == 0 {
                    let sv = (&raw mut DAT_101eb4b0)
                        .cast::<i16>()
                        .offset(i as isize)
                        .read();
                    sprintf(
                        buf.cast::<i8>(),
                        (&raw const DAT_100ad80c).cast::<i8>(),
                        ip.byte_add(0x110).cast::<i8>(),
                        sv as i32,
                    );
                    local_114 = local_114.wrapping_add(local_10c);
                    DAT_100eebbe = 0xe;
                    FUN_1005acaa(p_a4, local_118, local_114, puv2 as i32, buf, 0x100eebb0);
                    DAT_100eebbe = 0xe;
                }
                i = i.wrapping_add(1);
            }
        }
        FUN_1008664f(DAT_101686f0.wrapping_add(1), DAT_100af0b8 as *mut i32);
    }
}
