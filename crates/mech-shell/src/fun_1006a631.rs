// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a9cb1: [u8; 0x300];
    static mut DAT_100a92b1: [u8; 0x100];
    static mut DAT_100a8fb1: [u8; 0x300];
    static mut DAT_100a93b1: [u8; 0x300];
    static mut DAT_100a9fb1: [u8; 0x300];
    static mut DAT_100a8a58: unsafe extern "C" fn(u32, *mut u8);
    static mut DAT_100a8a5c: unsafe extern "C" fn(u32, *mut u8);
    static mut DAT_100a8a4c: unsafe extern "C" fn();
}

unsafe extern "stdcall" {
    fn RaiseException(code: u32, flags: u32, nargs: u32, args: *const u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1006a631(param_1: *mut u32, param_2: i32, param_3: u32) {
    // SAFETY: all globals are the game's own buffers; param_1 points at the
    // caller's three dwords and its pixel data; param_2 is an address of a
    // byte table, accessed exactly as the original does.
    unsafe {
        let cb = (&raw mut DAT_100a9cb1).cast::<u8>();
        let a2 = (&raw mut DAT_100a92b1).cast::<u8>();
        let f8 = (&raw mut DAT_100a8fb1).cast::<u8>();
        let a3 = (&raw mut DAT_100a93b1).cast::<u8>();
        let af = (&raw mut DAT_100a9fb1).cast::<u8>();

        for i in 0..0x100usize {
            cb.add(i).write(0);
        }

        let mut i_var6: i32 = param_1
            .byte_add(8)
            .read_unaligned()
            .wrapping_add(1)
            .wrapping_mul(param_1.byte_add(4).read_unaligned().wrapping_add(1))
            as i32;
        let mut local_8: u32 = 0xffff_ffff;
        let mut pb9 = param_1.read_unaligned() as *const u8;
        let mut pb12 = a2;
        loop {
            let b3 = pb9.read();
            let u7 = b3 as usize;
            let mut pb13 = pb12;
            if cb.add(u7).read() == 0 {
                cb.add(u7).write(cb.add(u7).read() | 1);
                pb13 = pb12.add(1);
                pb12.write(b3);
                local_8 = local_8.wrapping_add(1);
                let f = DAT_100a8a58;
                f(u7 as u32, f8.add(u7 * 3));
            }
            i_var6 = i_var6.wrapping_sub(1);
            pb9 = pb9.add(1);
            pb12 = pb13;
            if i_var6 == 0 {
                break;
            }
        }

        let mut local_c: u32 = 0;
        let mut u_var7: u32 = local_8;
        loop {
            let mut i6: i32 = (a2.add(u_var7 as usize).read() as i32).wrapping_mul(3);
            let mut i8: i32 = 2;
            loop {
                let x = f8.add(i6 as usize).read();
                let y = (i6.wrapping_add(param_2) as *const u8).read();
                let mut b3 = x.wrapping_sub(y);
                let mut u5: u8 = 0xff;
                if (x as i8) < (y as i8) {
                    b3 = 0u8.wrapping_sub(b3);
                    u5 = 1;
                }
                cb.add(i6 as usize).write(u5);
                a3.add(i6 as usize).write(b3);
                if (local_c as u8 as i8) < (b3 as i8) {
                    local_c = b3 as u32;
                }
                i6 = i6.wrapping_add(1);
                i8 = i8.wrapping_sub(1);
                if !(-1 < i8) {
                    break;
                }
            }
            u_var7 = u_var7.wrapping_sub(1);
            if !(-1 < (u_var7 as i32)) {
                break;
            }
        }

        let b3 = (local_c as u8) >> 1;
        let mut i: u32 = 0;
        while i < local_8 {
            af.add(i as usize).write(b3);
            i = i.wrapping_add(1);
        }

        if local_c != 0 {
            let mut local_14: u32 = 0x8000;
            u_var7 = local_8;
            let mut u_var10: u32 = local_c;
            loop {
                loop {
                    let i6: i32 = (a2.add(u_var7 as usize).read() as i32).wrapping_mul(3);
                    let mut i8: i32 = 2;
                    loop {
                        let idx = i6.wrapping_add(i8) as usize;
                        let mut c4 = af.add(idx).read().wrapping_add(a3.add(idx).read()) as i8;
                        if (local_c as u8 as i8) <= c4 {
                            c4 = (c4 as u8).wrapping_sub(local_c as u8) as i8;
                            f8.add(idx)
                                .write(f8.add(idx).read().wrapping_add(cb.add(idx).read()));
                        }
                        af.add(idx).write(c4 as u8);
                        i8 = i8.wrapping_sub(1);
                        if !(-1 < i8) {
                            break;
                        }
                    }
                    let f = DAT_100a8a5c;
                    f(a2.add(u_var7 as usize).read() as u32, f8.add(i6 as usize));
                    u_var7 = u_var7.wrapping_sub(1);
                    if !(-1 < (u_var7 as i32)) {
                        break;
                    }
                }

                let hi = param_3 >> 16;
                if local_c == 0 || hi >= local_c {
                    RaiseException(0xC000_0094, 0, 0, core::ptr::null());
                }
                local_14 =
                    local_14.wrapping_add((((param_3 as u64) << 16) / (local_c as u64)) as u32);
                if 0 < ((local_14 >> 16) as i16) {
                    loop {
                        let f = DAT_100a8a4c;
                        f();
                        let s1 = ((local_14 >> 16) as i16).wrapping_sub(1);
                        local_14 = ((s1 as u16 as u32) << 16) | (local_14 & 0xffff);
                        if s1 == 0 {
                            break;
                        }
                    }
                }
                u_var10 = u_var10.wrapping_sub(1);
                u_var7 = local_8;
                if u_var10 == 0 {
                    break;
                }
            }
        }
    }
}
