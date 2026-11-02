// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1009b750: [i8; 1];
    static mut DAT_1009b754: u16;
    static mut DAT_1009b756: u8;
    static mut DAT_1009b774: u16;
    static mut DAT_1009b776: u8;
    static mut DAT_100a67ac: u32;
    static mut DAT_100bc9b8: [u8; 1];
    static mut PTR_s_LRM_20__C__1008517c: [u32; 1];
    static mut s__d__s_1009b758: [i8; 1];
    static mut s__d_tons___s__ammo_1009b760: [i8; 1];
    static mut s__nSalvaged_weapons__n_1009b738: [i8; 1];
    static mut s__nThere_were_no_weapons_salvaged_1009b778: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10018060(param_1: i32, param_2: i32) -> u32;
    fn sprintf(dest: *mut i8, format: *const i8, ...) -> i32;
}

unsafe fn count() -> i32 {
    // SAFETY: DAT_100a67ac points at the game's collection object.
    unsafe { ((DAT_100a67ac as usize + 4) as *const i32).read_unaligned() }
}

unsafe fn str_end(mut p: *mut i8) -> *mut i8 {
    // SAFETY: p points at a NUL-terminated string.
    unsafe {
        while *p != 0 {
            p = p.add(1);
        }
    }
    p
}

unsafe fn str_cat(dest: *mut i8, src: *const i8) {
    // SAFETY: dest and src are NUL-terminated strings, dest has room.
    unsafe {
        let mut d = str_end(dest);
        let mut s = src;
        loop {
            let c = *s;
            *d = c;
            if c == 0 {
                break;
            }
            d = d.add(1);
            s = s.add(1);
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004c9b0(param_1: *mut i8) {
    // SAFETY: globals and callees are the game's own; param_1 is the caller's
    // NUL-terminated buffer with room for the appended text.
    unsafe {
        let mut local_100: [i8; 256] = [0; 256];
        let buf = local_100.as_mut_ptr();
        let flags = (&raw mut DAT_100bc9b8).cast::<u8>();
        let mut b_var2 = false;
        let mut local_108: i32 = 0;
        let mut i: i32 = 0;
        if 0 < count() {
            loop {
                let item = FUN_10018060(DAT_100a67ac as i32, i) as usize;
                let f = *((item + 9) as *const u8);
                if f == 1 {
                    local_108 = local_108.wrapping_add(1);
                }
                *flags.offset(i as isize) = f;
                i = i.wrapping_add(1);
                if i >= count() {
                    break;
                }
            }
        }
        if local_108 < 1 {
            sprintf(
                buf,
                (&raw const s__nThere_were_no_weapons_salvaged_1009b778).cast::<i8>(),
            );
            str_cat(param_1, buf);
        } else {
            sprintf(
                buf,
                (&raw const s__nSalvaged_weapons__n_1009b738).cast::<i8>(),
            );
            str_cat(param_1, buf);
            let mut local_104: i32 = 0;
            let mut local_10c: i32 = 0;
            if 0 < count() {
                loop {
                    let p4 = FUN_10018060(DAT_100a67ac as i32, local_10c) as usize as *mut i32;
                    if *((p4 as usize + 9) as *const u8) != 0 {
                        let mut i_var8: i32 = 1;
                        local_104 = local_104.wrapping_add(1);
                        let mut i_var3 = local_10c.wrapping_add(1);
                        *((p4 as usize + 9) as *mut u8) = 0;
                        if i_var3 < count() {
                            loop {
                                let p5 =
                                    FUN_10018060(DAT_100a67ac as i32, i_var3) as usize as *mut i32;
                                if *p5 == *p4 && *((p5 as usize + 9) as *const u8) == 1 {
                                    *((p5 as usize + 9) as *mut u8) = 0;
                                    i_var8 = i_var8.wrapping_add(1);
                                }
                                i_var3 = i_var3.wrapping_add(1);
                                if i_var3 >= count() {
                                    break;
                                }
                            }
                        }
                        if -1 < *p4 {
                            // copy DAT_1009b750 into local buffer
                            let mut s = (&raw const DAT_1009b750).cast::<i8>();
                            let mut d = buf;
                            loop {
                                let c = *s;
                                *d = c;
                                if c == 0 {
                                    break;
                                }
                                s = s.add(1);
                                d = d.add(1);
                            }
                            if !b_var2 {
                                b_var2 = true;
                                let e = str_end(param_1);
                                e.cast::<u16>().write_unaligned(DAT_1009b754);
                                *e.add(2) = DAT_1009b756 as i8;
                            }
                            let kind = *p4.add(2) as u8;
                            if kind == 0 || kind == 1 {
                                let tbl = (&raw const PTR_s_LRM_20__C__1008517c).cast::<u32>();
                                let name = tbl
                                    .wrapping_offset((*p4).wrapping_mul(0x10) as isize)
                                    .read_unaligned();
                                let fmt = if kind == 0 {
                                    (&raw const s__d__s_1009b758).cast::<i8>()
                                } else {
                                    (&raw const s__d_tons___s__ammo_1009b760).cast::<i8>()
                                };
                                sprintf(buf, fmt, i_var8, name);
                            }
                            str_cat(param_1, buf);
                            if local_104 < local_108 {
                                let e = str_end(param_1);
                                e.cast::<u16>().write_unaligned(DAT_1009b774);
                                *e.add(2) = DAT_1009b776 as i8;
                            }
                        }
                    }
                    local_10c = local_10c.wrapping_add(1);
                    if local_10c >= count() {
                        break;
                    }
                }
            }
        }
        let mut i_var8: i32 = 0;
        if 0 < count() {
            loop {
                let i_var10 = i_var8.wrapping_add(1);
                let item = FUN_10018060(DAT_100a67ac as i32, i_var8) as usize;
                *((item + 9) as *mut u8) = *flags.offset(i_var8 as isize);
                i_var8 = i_var10;
                if i_var10 >= count() {
                    break;
                }
            }
        }
    }
}
