// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf38: i32;
    static mut DAT_10215680: [u8; 0];
    static mut DAT_10215694: [u8; 0];
    static mut DAT_10215696: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10083b74(param_1: i32, param_2: i32, param_3: i32);
    fn FUN_1004fed5();
    fn FUN_10003742(param_1: i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32);
    fn FUN_10044054(param_1: *mut i32) -> u8;
    fn FUN_100297b4(param_1: i32);
}

#[inline(always)]
unsafe fn rd<T>(a: usize) -> T {
    // SAFETY: the caller passes an address of live game memory.
    unsafe { (a as *const T).read_unaligned() }
}

#[inline(always)]
unsafe fn wr<T>(a: usize, v: T) {
    // SAFETY: the caller passes an address of live game memory.
    unsafe { (a as *mut T).write_unaligned(v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10004190(param_1: *mut i32) {
    // SAFETY: param_1 and everything reached through it is the game's own
    // memory; globals and callees are the game's, accessed as the original does.
    unsafe {
        let a680 = (&raw mut DAT_10215680) as usize;
        let a694 = (&raw mut DAT_10215694) as usize;
        let a696 = (&raw mut DAT_10215696) as usize;
        let flag = param_1.add(0xfc);
        if flag.read_unaligned() != 0 {
            if flag.read_unaligned() == 1 {
                let b = param_1.read_unaligned() as u32 as usize;
                if (rd::<u8>(b.wrapping_add(0x105)) & 1) == 0
                    || (rd::<u8>(b.wrapping_add(0x105)) & 0x10) != 0
                {
                    FUN_10083b74(param_1.read_unaligned(), 0, 0);
                    let u2: u32 =
                        rd::<u32>((param_1.read_unaligned() as u32 as usize).wrapping_add(0x104))
                            & 0xff;
                    let mut local_8 = u2;
                    while (rd::<u16>(a694.wrapping_add(local_8.wrapping_mul(0x54) as usize)) & 0x20)
                        != 0
                    {
                        FUN_10083b74(param_1.read_unaligned(), 1, 0);
                        let bb = param_1.read_unaligned() as u32 as usize;
                        local_8 = rd::<u32>(bb.wrapping_add(0x104)) & 0xff;
                        if local_8 == u2 {
                            let bb = param_1.read_unaligned() as u32 as usize;
                            let v = rd::<u32>(bb.wrapping_add(0x104)) | 0x1000;
                            wr::<u32>(bb.wrapping_add(0x104), v);
                            return;
                        }
                    }
                }
                let b = param_1.read_unaligned() as u32 as usize;
                if (rd::<u8>(b.wrapping_add(0x105)) & 1) != 0 {
                    let u2: u32 = rd::<u32>(b.wrapping_add(0x104)) & 0xff;
                    let off = u2.wrapping_mul(0x54) as usize;
                    if rd::<i32>(b.wrapping_add(0xe8)) < rd::<i32>(a680.wrapping_add(off))
                        && rd::<i32>(b.wrapping_add(4)) == DAT_100adf38
                    {
                        if (rd::<u16>(a694.wrapping_add(off)) & 0x20) == 0 {
                            let v = rd::<u16>(a694.wrapping_add(off)) | 0x20;
                            wr::<u16>(a694.wrapping_add(off), v);
                            let sh = (rd::<u8>(b.wrapping_add(8)) & 0x1f) as u32;
                            let v =
                                rd::<u16>(a696.wrapping_add(off)) | (1u32.wrapping_shl(sh) as u16);
                            wr::<u16>(a696.wrapping_add(off), v);
                            if (rd::<u16>(a694.wrapping_add(off)) & 0x40) != 0 {
                                FUN_1004fed5();
                            }
                            FUN_10003742(0xfc, 100, 0x40, 5, 0x50);
                        }
                        FUN_10083b74(param_1.read_unaligned(), 1, 0);
                        let bb = param_1.read_unaligned() as u32 as usize;
                        if (rd::<u32>(bb.wrapping_add(0x104)) & 0xff) < u2 {
                            flag.write_unaligned(0);
                            let bb = param_1.read_unaligned() as u32 as usize;
                            let v = rd::<u32>(bb.wrapping_add(0x104)) | 0x1000;
                            wr::<u32>(bb.wrapping_add(0x104), v);
                            let c = rd::<i32>(param_1.read_unaligned() as u32 as usize + 0x4c)
                                as u32 as usize;
                            wr::<i32>(c.wrapping_add(8), 0);
                            let bb = param_1.read_unaligned() as u32 as usize;
                            let c = rd::<i32>(bb.wrapping_add(0x4c)) as u32 as usize;
                            wr::<u8>(c.wrapping_add(0x24), 1);
                            return;
                        }
                    }
                }
            }
            let bvar1 = FUN_10044054(param_1.read_unaligned() as u32 as usize as *mut i32);
            if bvar1 == 0 {
                FUN_100297b4(param_1.read_unaligned());
                let b = param_1.read_unaligned() as u32 as usize;
                if rd::<i32>(b.wrapping_add(0x1d8)) == 1 {
                    let b = param_1.read_unaligned() as u32 as usize;
                    let v = rd::<i32>(b.wrapping_add(0x1d4));
                    let c = rd::<i32>(b.wrapping_add(0x4c)) as u32 as usize;
                    wr::<i32>(c.wrapping_add(8), v);
                    let b = param_1.read_unaligned() as u32 as usize;
                    wr::<i32>(b.wrapping_add(0x1d8), 0);
                }
                let b = param_1.read_unaligned() as u32 as usize;
                let c = rd::<i32>(b.wrapping_add(0x4c)) as u32 as usize;
                let v = rd::<i32>(c.wrapping_add(8));
                let b = param_1.read_unaligned() as u32 as usize;
                wr::<i32>(b.wrapping_add(0x1d4), v);
            } else {
                let b = param_1.read_unaligned() as u32 as usize;
                wr::<i32>(b.wrapping_add(0x1d8), 1);
            }
        }
    }
}
