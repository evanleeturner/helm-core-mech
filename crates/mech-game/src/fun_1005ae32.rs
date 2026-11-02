// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1005ade4: [i16; 1];
    static mut DAT_1005adec: [i16; 1];
    static mut DAT_100b24f9: [u8; 1];
    static mut DAT_100b17f9: [u8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1005adf0(param_1: *mut i16, param_2: i32) -> *mut u16;
    fn FUN_1005ace1(param_1: *mut i32, param_2: i32, param_3: *mut c_void, param_4: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005ae32(param_1: *mut i32, param_2: i32) -> u32 {
    // SAFETY: all pointers are the game's own memory; reads, writes and calls
    // happen in the same order as the original.
    unsafe {
        let i_var6: i32 = param_1
            .add(3)
            .read_unaligned()
            .wrapping_sub(param_1.add(1).read_unaligned())
            .wrapping_add(1);
        let mut u_var7: u32 = (param_1.add(4).read_unaligned() as u32)
            .wrapping_sub(param_1.add(2).read_unaligned() as u32)
            .wrapping_add(1);
        let mut local_2c: i32 = 0;
        let i_var2: i32 =
            ((param_2 as u32 as usize).wrapping_add(8) as *const i32).read_unaligned();
        let mut local_3c: u32 = 0;

        let buf = (&raw mut DAT_100b24f9).cast::<u8>();
        let chunky = (&raw mut DAT_100b17f9).cast::<u8>();

        let puv8 = FUN_1005adf0((&raw mut DAT_1005ade4).cast::<i16>(), param_2);
        let u_var1: u16 = puv8.read_unaligned();
        let u_var3: u16 = u_var1.swap_bytes();
        let u_var9: u32 = u_var3 as u32;
        let h: u16 = puv8.add(1).read_unaligned();
        let mut local_c: u32 = h.swap_bytes() as u32;
        if (u_var7 as i32) <= (local_c as i32) {
            local_c = u_var7;
        }

        let hdr = puv8.cast::<u8>();
        if hdr.add(9).read() != 1 {
            let u_var4: u8 = hdr.add(10).read();
            local_3c = hdr.add(0xd).read() as u32;

            u_var7 = ((u_var3 as u32) >> 3)
                .wrapping_add(1)
                .wrapping_sub(((u_var1 & 0x700) == 0) as u32);
            let i_var15: i32 = u_var7.wrapping_add(u_var7 & 1) as i32;

            let mut i_var10: i32 = u_var9 as i32;
            if i_var6 <= u_var9 as i32 {
                i_var10 = i_var6;
            }

            let mut local_14: *mut u8 =
                FUN_1005adf0((&raw mut DAT_1005adec).cast::<i16>(), param_2).cast::<u8>();

            loop {
                let mut pbvar18: *mut u8;
                if u_var4 == 1 {
                    pbvar18 = buf;
                    let limit = buf.wrapping_add(u_var9.wrapping_add(u_var9 & 1) as usize);
                    while pbvar18 < limit {
                        let pbvar17 = local_14;
                        local_14 = pbvar17.wrapping_add(1);
                        let n: u32 = pbvar17.read() as u32;
                        if n != 0x80 {
                            if n < 0x81 {
                                let mut cnt = n.wrapping_add(1) & 3;
                                while cnt != 0 {
                                    pbvar18.write(local_14.read());
                                    local_14 = local_14.wrapping_add(1);
                                    pbvar18 = pbvar18.wrapping_add(1);
                                    cnt -= 1;
                                }
                                cnt = n.wrapping_add(1) >> 2;
                                while cnt != 0 {
                                    let v = local_14.cast::<u32>().read_unaligned();
                                    pbvar18.cast::<u32>().write_unaligned(v);
                                    local_14 = local_14.wrapping_add(4);
                                    pbvar18 = pbvar18.wrapping_add(4);
                                    cnt -= 1;
                                }
                            } else {
                                let b14: u8 = local_14.read();
                                let b12: u8 = 1u8.wrapping_sub(n as u8);
                                let mut cnt = (b12 & 3) as u32;
                                while cnt != 0 {
                                    pbvar18.write(b14);
                                    pbvar18 = pbvar18.wrapping_add(1);
                                    cnt -= 1;
                                }
                                let fill = u32::from_le_bytes([b14, b14, b14, b14]);
                                cnt = (b12 >> 2) as u32;
                                while cnt != 0 {
                                    pbvar18.cast::<u32>().write_unaligned(fill);
                                    pbvar18 = pbvar18.wrapping_add(4);
                                    cnt -= 1;
                                }
                                local_14 = pbvar17.wrapping_add(2);
                            }
                        }
                    }
                    pbvar18 = buf;
                } else {
                    pbvar18 = local_14;
                    local_14 = local_14.wrapping_add(u_var9.wrapping_add(u_var9 & 1) as usize);
                }

                if i_var2 == 0x4d424c49 {
                    let mut out = chunky;
                    let mut local_1c: u32 = i_var15 as u32;
                    let mut local_18: u32 = i_var10 as u32;
                    'planes: loop {
                        let mut b14: u8 = 0x80;
                        loop {
                            let mut i_var16: i32 = 0;
                            let mut lo: u8 = 0;
                            let mut hi: u8 = 1;
                            loop {
                                if (pbvar18.offset(i_var16 as isize).read() & b14) != 0 {
                                    lo |= hi;
                                }
                                i_var16 = i_var16.wrapping_add(i_var15);
                                hi = hi.wrapping_shl(1);
                                if hi == 0 {
                                    break;
                                }
                            }
                            out.write(lo);
                            out = out.wrapping_add(1);
                            local_18 = local_18.wrapping_sub(1);
                            if local_18 == 0 {
                                break 'planes;
                            }
                            b14 >>= 1;
                            if b14 == 0 {
                                break;
                            }
                        }
                        pbvar18 = pbvar18.wrapping_add(1);
                        local_1c = local_1c.wrapping_sub(1);
                        if local_1c == 0 {
                            break;
                        }
                    }
                    FUN_1005ace1(param_1, local_2c, chunky.cast::<c_void>(), i_var10);
                } else {
                    FUN_1005ace1(param_1, local_2c, pbvar18.cast::<c_void>(), i_var10);
                }
                local_2c = local_2c.wrapping_add(1);
                local_c = local_c.wrapping_sub(1);
                if local_c == 0 {
                    break;
                }
            }
        }
        local_3c
    }
}
