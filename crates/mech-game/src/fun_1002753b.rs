// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38d8: i32;
    static mut DAT_100eef40: [*mut i32; 0];
    static mut DAT_10123130: [i32; 0];
    static mut DAT_1012327a: [u8; 0];
    static mut DAT_101232e1: [u8; 0];
    static mut DAT_101232e9: [u8; 0];
    static mut DAT_10123120: i32;
    static mut DAT_100adf38: i32;
    static mut DAT_100ab0a0: *mut i32;
    static mut DAT_100acdb0: i16;
    static mut DAT_100b38fc: i32;
    static mut DAT_100ad3b4: [i8; 0];
    static mut DAT_100ad3bc: [i8; 0];
    static mut DAT_100ad434: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002950f(param_1: i32, param_2: i16);
    fn FUN_1002c530();
    fn FUN_100274a5(param_1: i16, param_2: i32, param_3: i32) -> i32;
    fn sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
    fn FUN_1005dd5a(param_1: *mut i8);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
#[allow(unused_assignments)]
#[allow(unused_variables)]
pub unsafe extern "stdcall" fn FUN_1002753b() {
    // SAFETY: all globals, callees and record pointers belong to the game;
    // reads and writes happen in the original's order.
    unsafe {
        let mut local_280: i32 = 0;
        while local_280 < DAT_100b38d8 {
            let mut local_28c: i32 = 0x20;
            let rec: *mut i32 = (&raw mut DAT_100eef40)
                .cast::<*mut i32>()
                .offset(local_280 as isize)
                .read();
            let tbl = (&raw mut DAT_10123130).cast::<i32>();
            let mut n: i32 = tbl.offset(rec.add(2).read() as isize).read();
            let i_var2: i32 = (n as u32)
                .wrapping_mul(0x13f)
                .wrapping_add((rec.add(2).read() as u32).wrapping_mul(0x3c0a))
                as i32;
            let local_27c: *mut u8 = (&raw mut DAT_1012327a).cast::<u8>().offset(i_var2 as isize);
            let mut field_2e9: i32 = (&raw mut DAT_101232e9)
                .cast::<u8>()
                .offset(i_var2 as isize)
                .cast::<i32>()
                .read_unaligned();
            let local_288: i32;
            if tbl.offset(rec.add(2).read() as isize).read() == -1 || field_2e9 < 0 {
                local_288 = 0;
            } else {
                field_2e9 = (&raw mut DAT_101232e9)
                    .cast::<u8>()
                    .offset(i_var2 as isize)
                    .cast::<i32>()
                    .read_unaligned();
                let field_2e1: i32 = (&raw mut DAT_101232e1)
                    .cast::<u8>()
                    .offset(i_var2 as isize)
                    .cast::<i32>()
                    .read_unaligned();
                local_288 = (field_2e9 as u32)
                    .wrapping_sub((DAT_10123120 as u32).wrapping_sub(field_2e1 as u32))
                    as i32;
            }
            let local_8: i16 = if rec.add(1).read() == DAT_100adf38 {
                rec.add(0x37).read() as i16
            } else {
                rec.add(0x5e).cast::<i16>().read_unaligned()
            };
            let mut local_210 = [0u8; 0x208];
            core::ptr::copy_nonoverlapping(rec.cast::<u8>(), local_210.as_mut_ptr(), 0x206);
            FUN_1002950f(rec as i32, local_8);
            if rec.add(0x30).read() < 0 {
                FUN_1002c530();
            }
            let p13 = rec.add(0x13).read() as *const u8;
            if p13.byte_add(0x1d).read() != 0 {
                let l = p13;
                if l.byte_add(0x1e).read() == 0 {
                    if l.byte_add(0x1f).read() == 0 {
                        if l.byte_add(0x20).read() == 0 {
                            if l.byte_add(0x21).read() == 0 {
                                local_28c = 0x2b;
                            } else {
                                local_28c = 0x76;
                            }
                        } else {
                            local_28c = 0x5e;
                        }
                    } else {
                        local_28c = 0x3e;
                    }
                } else {
                    local_28c = 0x3c;
                }
                let s = (l.byte_add(0x1e).read() as i8 as i32)
                    .wrapping_add(l.byte_add(0x1f).read() as i8 as i32)
                    .wrapping_add(l.byte_add(0x20).read() as i8 as i32)
                    .wrapping_add(l.byte_add(0x21).read() as i8 as i32);
                if 1 < s {
                    local_28c = 0x3f;
                }
            }
            if rec.add(0x77).read() != 0 {
                let q = rec.add(0x77).read() as *const u8;
                FUN_100274a5(
                    (q.byte_add(2).cast::<u16>().read_unaligned() & 0xf00) as i16,
                    0x100acc20,
                    3,
                );
            }
            let local_2ec: *mut i8;
            if rec
                .cast::<u8>()
                .byte_add(0x176)
                .cast::<i16>()
                .read_unaligned()
                == 3
            {
                local_2ec = FUN_100274a5(
                    rec.add(0x71).cast::<i16>().read_unaligned(),
                    0x100acd38,
                    0xf,
                ) as *mut i8;
            } else {
                local_2ec = (&raw mut DAT_100ad3b4).cast::<i8>();
            }
            let r8 = rec.add(8).read() as *const u8;
            let u_var3 = FUN_100274a5(
                r8.byte_add(0x3d4).cast::<i32>().read_unaligned() as i16,
                0x100acc80,
                8,
            );
            let u_var13 = rec.add(0x13).read().wrapping_add(8) as *const i32;
            let u_var13 = u_var13.read_unaligned();
            let u_var12 = rec.add(0x30).read();
            let w17a = rec
                .cast::<u8>()
                .byte_add(0x17a)
                .cast::<u16>()
                .read_unaligned();
            let u_var9 = (w17a & 0xff) as i32;
            let u_var4 = FUN_100274a5(
                (rec.cast::<u8>()
                    .byte_add(0x17a)
                    .cast::<u16>()
                    .read_unaligned()
                    & 0xf00) as i16,
                0x100acc08,
                3,
            );
            let u_var5 = (local_8 as i32) & 0xff;
            let u_var6 = FUN_100274a5(((local_8 as u16) & 0xf00) as i16, 0x100acc08, 3);
            let u_var7 = FUN_100274a5(
                rec.cast::<u8>()
                    .byte_add(0x176)
                    .cast::<i16>()
                    .read_unaligned(),
                0x100acb48,
                0xd,
            );
            let i_var2b = local_288;
            n = local_27c.byte_add(2).cast::<i32>().read_unaligned();
            let u_var8 = FUN_100274a5(n as i16, 0x100accc0, 0xf);
            let mut local_278 = [0i8; 100];
            let first: i32 = if rec == DAT_100ab0a0 { 0x3e } else { 0x20 };
            sprintf(
                local_278.as_mut_ptr(),
                (&raw mut DAT_100ad3bc).cast::<i8>(),
                first,
                rec.add(1).read(),
                rec.add(2).read(),
                tbl.offset(rec.add(2).read() as isize).read(),
                u_var8,
                i_var2b,
                u_var7,
                u_var6,
                u_var5,
                u_var4,
                u_var9,
                local_2ec,
                u_var12,
                u_var13,
                u_var3,
            );
            let mut local_284: i32 = 7;
            if (r8.byte_add(0x3d4).read() & 4) == 0 {
                local_284 = 0xf;
            }
            let w17a2 = rec
                .cast::<u8>()
                .byte_add(0x17a)
                .cast::<u16>()
                .read_unaligned();
            if w17a2 == 0 || rec.add(0x5e).cast::<i16>().read_unaligned() == 0 {
                DAT_100acdb0 = DAT_100acdb0.wrapping_add(1);
                if DAT_100acdb0 < 10 {
                    let mut local_2e0 = [0i8; 80];
                    sprintf(
                        local_2e0.as_mut_ptr(),
                        (&raw mut DAT_100ad434).cast::<i8>(),
                        DAT_100b38fc,
                        rec.add(2).read(),
                        rec.add(1).read(),
                    );
                    FUN_1005dd5a(local_2e0.as_mut_ptr());
                }
            }
            core::ptr::copy_nonoverlapping(
                local_210.as_ptr().add(0xc0),
                rec.add(0x30).cast::<u8>(),
                0x28,
            );
            local_280 = local_280.wrapping_add(1);
        }
    }
}
