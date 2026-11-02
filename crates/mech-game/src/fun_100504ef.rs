// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_101155c0: [u8; 0];
    static mut DAT_100eef40: [i32; 0];
    static mut DAT_100af17c: i32;
    static mut DAT_100af0d0: *mut i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1001ca17(param_1: *mut u16) -> u16;
    fn FUN_1001d671(param_1: *mut core::ffi::c_void);
    fn FUN_1008668b(param_1: i32, param_2: u32, param_3: *mut i32) -> *mut u16;
    fn FUN_1001e8a8(param_1: i32, param_2: *mut i32, param_3: u16) -> i32;
    fn FUN_10068d4e(param_1: i32, param_2: i32, param_3: i32);
    fn FUN_10068db6(param_1: i32);
    fn FUN_10068dc9(param_1: i32, param_2: *mut i32, param_3: i32, param_4: i32) -> *mut u16;
    fn FUN_1008691a() -> u8;
    fn FUN_1008664f(param_1: u32, param_2: *mut i32);
    fn FUN_1001c9e8(param_1: *mut u16, param_2: u16);
    fn FUN_1002f010(param_1: i32, param_2: i32) -> *mut i32;
    fn FUN_1002f19a(param_1: i32, param_2: *mut i32) -> *mut i32;
    fn FUN_1001d5cb(param_1: i32) -> i32;
    fn FUN_1002f272(param_1: i32, param_2: i32);
    fn FUN_1001d5e1(param_1: i32, param_2: i32);
    fn FUN_1001ca37(param_1: i32, param_2: u16);
    fn FUN_1001ca62(param_1: i32, param_2: i16);
    fn FUN_1001ca4c(param_1: i32, param_2: i16);
    fn FUN_1003da99(param_1: *mut u16);
    fn FUN_1003dfde(param_1: *mut u16);
    fn FUN_1003df3a(param_1: *mut u16);
    fn FUN_1003e051(param_1: *mut u16);
    fn FUN_100141c0(param_1: *mut u16, param_2: i32);
    fn FUN_1002f727(param_1: i32);
    fn FUN_1002f7f7(param_1: i32);
    fn FUN_1002f90b(param_1: i32, param_2: i32);
    fn FUN_1002f37a(param_1: i32, param_2: i32, param_3: i32, param_4: i32);
    fn FUN_1002fbc0(param_1: *mut i32);
}

unsafe fn ri(rec: *mut u8, off: i32) -> i32 {
    // SAFETY: rec points at a game record; the caller guarantees the offset is valid.
    unsafe { rec.byte_offset(off as isize).cast::<i32>().read_unaligned() }
}

unsafe fn wi(rec: *mut u8, off: i32, v: i32) {
    // SAFETY: rec points at a game record; the caller guarantees the offset is valid.
    unsafe {
        rec.byte_offset(off as isize)
            .cast::<i32>()
            .write_unaligned(v)
    }
}

unsafe fn rp(rec: *mut u8, off: i32) -> *mut u16 {
    // SAFETY: as for ri.
    unsafe { ri(rec, off) as usize as *mut u16 }
}

unsafe fn ent(idx: i32) -> i32 {
    // SAFETY: DAT_100eef40 is the game's table; idx comes from the record.
    unsafe {
        (&raw mut DAT_100eef40)
            .cast::<i32>()
            .offset(idx as isize)
            .read_unaligned()
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100504ef(param_1: i32, param_2: i32, param_3: *mut i32) -> u32 {
    // SAFETY: all globals and callees are the game's own; the record pointers
    // are derived from the game's record table and used as the original does.
    unsafe {
        let mut local_c: i32 = 0;
        let mut u_var3: u16 = 0;
        let i_var1 = (param_1 as u32).wrapping_mul(0x48) as i32;
        let i_stride4 = (param_2 as u32).wrapping_mul(4) as i32;
        let i_stride2 = (param_2 as u32).wrapping_mul(2) as i32;
        let base = (&raw mut DAT_101155c0).cast::<u8>();
        let rec = base.byte_offset(i_var1 as isize);
        let u_var5: u32;

        if rec.byte_offset(0x46).cast::<i16>().read_unaligned() == 0 {
            if ri(rec, 0x24) != 0 {
                let w = rec_u16(ri(rec, 0x24) as usize as *mut u8, 2);
                if (w as u8) & 0xf0 == 0x50 {
                    return 1;
                }
                u_var3 = FUN_1001ca17(rp(rec, 0x24));
                FUN_1001d671(ri(rec, 0x24) as usize as *mut core::ffi::c_void);
                wi(rec, 0x24, 0);
            }
            let off8 = 8i32.wrapping_add(i_stride4);
            let puvar6 = FUN_1008668b(DAT_100af17c, ri(rec, off8) as u32, DAT_100af0d0);
            if puvar6.is_null() {
                u_var5 = 0;
            } else {
                let i_var7 = FUN_1001e8a8(DAT_100af17c, DAT_100af0d0, ri(rec, off8) as u16);
                wi(rec, 4, -1);
                FUN_10068d4e(1, 1, 1);
                FUN_10068db6(ri(rec, 0x2c));
                loop {
                    if ri(rec, 0x24) != 0 {
                        break;
                    }
                    let puvar8 = FUN_10068dc9(puvar6 as usize as i32, &raw mut local_c, i_var7, 0);
                    wi(rec, 0x24, puvar8 as usize as i32);
                    if ri(rec, 0x24) != 0 {
                        continue;
                    }
                    let b = FUN_1008691a();
                    if b == 0 {
                        break;
                    }
                }
                FUN_1008664f(ri(rec, off8) as u32, DAT_100af0d0);
                if ri(rec, 0x24) == 0 {
                    u_var5 = 0;
                } else {
                    let mut b_var2: u8 = 0;
                    FUN_1001c9e8(rp(rec, 0x24), u_var3);
                    wi(rec, 4, param_2);
                    if ri(rec, 0x28) == 0 {
                        if ri(rec, 0x1c) == -2 {
                            if param_3.is_null() {
                                let p = FUN_1002f010(
                                    ent(ri(rec, 0)).wrapping_add(0x40).pipe_read(),
                                    10,
                                );
                                wi(rec, 0x28, p as usize as i32);
                            } else {
                                let p = FUN_1002f19a(
                                    ent(ri(rec, 0)).wrapping_add(0x40).pipe_read(),
                                    param_3,
                                );
                                wi(rec, 0x28, p as usize as i32);
                            }
                        } else {
                            let o = (ri(rec, 0x1c) as u32).wrapping_mul(0x48) as i32;
                            let mut local_20 = base
                                .byte_offset(0x28)
                                .byte_offset(o as isize)
                                .cast::<i32>()
                                .read_unaligned();
                            if local_20 == 0 {
                                local_20 = FUN_1001d5cb(
                                    base.byte_offset(0x24)
                                        .byte_offset(o as isize)
                                        .cast::<i32>()
                                        .read_unaligned(),
                                );
                            }
                            let local_24 = if param_3.is_null() {
                                FUN_1002f010(local_20, 10)
                            } else {
                                FUN_1002f19a(local_20, param_3)
                            };
                            if !local_24.is_null() {
                                wi(rec, 0x28, local_24 as usize as i32);
                                b_var2 = 1;
                            }
                        }
                    }
                    FUN_1002f272(ri(rec, 0x28), ri(rec, 0x24));
                    FUN_1001d5e1(ri(rec, 0x24), ri(rec, 0x28));
                    let local_8: u32 = (rec
                        .byte_offset(0x3c_i32.wrapping_add(i_stride2) as isize)
                        .cast::<u16>()
                        .read_unaligned() as u32)
                        & 0xf0;
                    FUN_1001ca37(ri(rec, 0x24), (local_8 as u16) | 0x100);
                    FUN_1001ca62(ri(rec, 0x24), ri(rec, 0) as i16);
                    FUN_1001ca4c(ri(rec, 0x24), ri(rec, 0x20) as i16);
                    FUN_1003da99(rp(rec, 0x24));
                    if local_8 == 0x70 {
                        FUN_1003dfde(rp(rec, 0x24));
                        FUN_1003df3a(rp(rec, 0x24));
                    } else {
                        FUN_1003e051(rp(rec, 0x24));
                        FUN_100141c0(rp(rec, 0x24), 6);
                    }
                    let e = ent(ri(rec, 0)) as usize as *mut u8;
                    if (e.byte_add(0x14).cast::<u16>().read_unaligned() & 0x4000) != 0 {
                        let e = ent(ri(rec, 0)) as usize as *mut u8;
                        FUN_1002f727(e.byte_add(0x40).cast::<i32>().read_unaligned());
                        let e = ent(ri(rec, 0)) as usize as *mut u8;
                        FUN_1002f7f7(e.byte_add(0x40).cast::<i32>().read_unaligned());
                        let e = ent(ri(rec, 0)) as usize as *mut u8;
                        let cur = e.byte_add(0x14).cast::<u16>().read_unaligned();
                        let e = ent(ri(rec, 0)) as usize as *mut u8;
                        e.byte_add(0x14).cast::<u16>().write_unaligned(cur | 0x800);
                    }
                    if local_8 == 0xa0 {
                        FUN_1002f90b(ri(rec, 0x28), 0x1a0);
                    }
                    if b_var2 != 0 {
                        FUN_1002f37a(ri(rec, 0x28), ri(rec, 0x30), ri(rec, 0x34), ri(rec, 0x38));
                    }
                    FUN_1002fbc0(ri(rec, 0x28) as usize as *mut i32);
                    u_var5 = 1;
                }
            }
        } else {
            u_var5 = 1;
        }
        u_var5
    }
}

unsafe fn rec_u16(p: *mut u8, off: i32) -> u16 {
    // SAFETY: p points at the game's object; offset is valid per the original.
    unsafe { p.byte_offset(off as isize).cast::<u16>().read_unaligned() }
}

trait PipeRead {
    unsafe fn pipe_read(self) -> i32;
}

impl PipeRead for i32 {
    unsafe fn pipe_read(self) -> i32 {
        // SAFETY: self is the address of a field in the game's object (entry + 0x40).
        unsafe { (self as usize as *const i32).read_unaligned() }
    }
}
