// SPDX-License-Identifier: MIT

use core::arch::asm;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a410c: i32;
    static mut DAT_100a9ee0: u8;
    static mut DAT_100a9f44: u32;
    static mut DAT_100af17c: u32;
    static mut DAT_100c6a08: u8;
    static mut DAT_100c6a0e: u8;
    static mut DAT_101686f0: u32;
    static mut DAT_101eb550: u8;
    static mut DAT_101eb5f0: u8;
    static mut DAT_101ec000: u8;
    static mut DAT_101ec004: u8;
    static mut DAT_101ec400: u32;
    static mut PTR_DAT_100af0b4: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1008668b(param_1: i32, param_2: u32, param_3: *mut i32) -> *mut u16;
    fn FUN_10056fc4(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
    ) -> u32;
    fn FUN_10066609(param_1: *mut i32, param_2: i32);
    fn FUN_1005748f(param_1: *mut u32);
    fn FUN_100574ca(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
    ) -> u32;
    fn FUN_1008664f(param_1: u32, param_2: *mut i32);
}

#[inline(always)]
unsafe fn idiv(a: i32, d: i32) -> i32 {
    let q: i32;
    // SAFETY: the machine's own idiv, faulting as the original does.
    unsafe {
        asm!("cdq", "idiv ecx", inout("eax") a => q, in("ecx") d, out("edx") _);
    }
    q
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100117f2(param_1: i32, _param_2: u32, param_3: i32) {
    // SAFETY: globals and callees are the game's own; pointers are read and
    // written where the original does.
    unsafe {
        if !((-1 < param_3) && (param_3 < 8)) {
            return;
        }
        let key = DAT_101ec400.wrapping_add(DAT_101686f0);
        let puv3 = FUN_1008668b(
            DAT_100af17c as i32,
            key,
            PTR_DAT_100af0b4 as usize as *mut i32,
        );
        if puv3.is_null() {
            return;
        }
        FUN_10056fc4(
            (&raw mut DAT_101eb550)
                .byte_add(param_3.wrapping_mul(0x14) as usize)
                .cast::<i32>(),
            puv3 as usize as i32,
            0,
            0,
            0,
        );
        let mut local_30: i32 = 0;
        while local_30 < 0x10 {
            let mut local_8: i32 = 0;
            let mut local_10: i32 = 0;
            let base = (&raw mut DAT_101eb5f0).byte_add(
                local_30
                    .wrapping_mul(0x14)
                    .wrapping_add(param_3.wrapping_mul(0x140)) as usize,
            );
            let a = base.byte_add(0xc).cast::<i32>().read_unaligned();
            let b = base.byte_add(4).cast::<i32>().read_unaligned();
            let ivar4 = a.wrapping_sub(b);
            if ivar4 != -1 && -1 < ivar4.wrapping_add(1) {
                if DAT_100a9f44 != 0 {
                    FUN_10066609(base.cast::<i32>(), 0xe);
                }
                let mut cvar2: i8 = 6;
                let idx = (&raw mut DAT_100a9ee0)
                    .byte_add(local_30.wrapping_mul(4) as usize)
                    .cast::<i32>()
                    .read_unaligned();
                let p = ((param_1.wrapping_add(0x58) as usize as *const i32)
                    .read_unaligned()
                    .wrapping_add(idx.wrapping_sub(1).wrapping_mul(0x28)))
                    as usize as *mut i32;
                let flags = p.byte_add(0x26).cast::<i16>().read_unaligned();
                let mut u5 = ((flags as i32 as u32) & 0xf0) >> 4;
                if u5 != 0 {
                    let p1 = p.byte_add(4).read_unaligned();
                    let q = idiv(p1, DAT_100a410c);
                    let p2 = p.byte_add(8).read_unaligned();
                    let t = p2.wrapping_add(q).wrapping_mul(3);
                    local_10 = 0xf - t / ((u5 << 16) as i32);
                }
                if local_10 < 1 {
                    local_10 = 0;
                } else if 0xf < local_10 {
                    local_10 = 0xf;
                }
                u5 = (p.byte_add(0x26).cast::<i16>().read_unaligned() as i32 as u32) & 0xf;
                if u5 != 0 {
                    let p0 = p.read_unaligned();
                    let q = idiv(p0, DAT_100a410c);
                    let p2 = p.byte_add(8).read_unaligned();
                    let t = p2.wrapping_add(q).wrapping_mul(3);
                    local_8 = 0xf - t / ((u5 << 16) as i32);
                }
                if local_8 < 1 {
                    local_8 = 0;
                } else if 0xf < local_8 {
                    local_8 = 0xf;
                }
                if local_10 <= local_8 {
                    local_10 = local_8;
                }
                if (p.byte_add(0x26).cast::<u16>().read_unaligned() & 0x2000) == 0 {
                    if local_10 < 0xc {
                        if 0 < local_10 {
                            cvar2 = 3;
                        }
                    } else {
                        cvar2 = 0xb;
                    }
                } else {
                    cvar2 = 0;
                }
                if cvar2 != 6 {
                    DAT_100c6a0e = cvar2 as u8;
                    FUN_1005748f((&raw mut DAT_100c6a08).cast::<u32>());
                    let o = local_30
                        .wrapping_mul(8)
                        .wrapping_add(param_3.wrapping_mul(0x80))
                        as usize;
                    let x = (&raw mut DAT_101ec000)
                        .byte_add(o)
                        .cast::<i32>()
                        .read_unaligned();
                    let y = (&raw mut DAT_101ec004)
                        .byte_add(o)
                        .cast::<i32>()
                        .read_unaligned();
                    FUN_100574ca(base.cast::<i32>(), puv3 as usize as i32, 0, x, y);
                }
            }
            local_30 = local_30.wrapping_add(1);
        }
        FUN_1008664f(key, PTR_DAT_100af0b4 as usize as *mut i32);
    }
}
