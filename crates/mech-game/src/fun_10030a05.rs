// SPDX-License-Identifier: MIT

use core::arch::asm;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ada50: u32;
    static mut DAT_100ada58: *mut u32;
    static mut DAT_100ada5c: u32;
    static mut DAT_100b3904: u32;
    static mut DAT_10168700: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002f37a(param_1: i32, param_2: u32, param_3: u32, param_4: u32) -> u8;
    fn FUN_1002fbc0(param_1: *mut i32) -> u8;
    fn FUN_1002f322(param_1: i32, param_2: *mut u32, param_3: *mut u32, param_4: *mut u32) -> u8;
    fn FUN_10030fd2(param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10030a05() {
    let mut local_20: i32 = 0;
    let mut local_10: u32 = 0;
    let mut local_18: u32 = 0;
    let mut local_1c: u32 = 0;
    // SAFETY: all globals and callees are the game's own; the list nodes and
    // the records they point at are read and written where the original does.
    unsafe {
        if !DAT_100ada58.is_null() && DAT_100ada5c != 0 && DAT_100ada50 != 0 {
            let mut c: *mut u32 = DAT_100ada58;
            while !c.is_null() {
                match c.add(1).read_unaligned() {
                    1 => {
                        let local_8 = c.add(2).read_unaligned();
                        c.add(5).write_unaligned(
                            ((local_8.wrapping_add(0x50)) as *const u32).read_unaligned(),
                        );
                        c.add(6).write_unaligned(
                            ((local_8.wrapping_add(0x74)) as *const u32).read_unaligned(),
                        );
                        c.add(7).write_unaligned(
                            ((local_8.wrapping_add(0x58)) as *const u32).read_unaligned(),
                        );
                        FUN_1002f37a(
                            c.add(0xc).read_unaligned() as i32,
                            c.add(5).read_unaligned(),
                            c.add(6).read_unaligned(),
                            c.add(7).read_unaligned(),
                        );
                        FUN_1002fbc0(c.add(0xc).read_unaligned() as *mut i32);
                    }
                    2 => {
                        let p = c.add(2).read_unaligned().wrapping_add(0x30) as *const i32;
                        FUN_1002f322(
                            p.read_unaligned(),
                            &raw mut local_10,
                            &raw mut local_18,
                            &raw mut local_1c,
                        );
                        c.add(5).write_unaligned(local_10);
                        c.add(6).write_unaligned(0);
                        c.add(7).write_unaligned(local_1c);
                        FUN_1002f37a(
                            c.add(0xc).read_unaligned() as i32,
                            c.add(5).read_unaligned(),
                            c.add(6).read_unaligned(),
                            c.add(7).read_unaligned(),
                        );
                        FUN_1002fbc0(c.add(0xc).read_unaligned() as *mut i32);
                    }
                    3 | 4 | 5 => {}
                    6 => {
                        let v = c.add(10).read_unaligned().wrapping_add(DAT_100b3904);
                        c.add(10).write_unaligned(v);
                        let c8 = c.add(8).read_unaligned();
                        if (v as i32) < (c8 as i32) {
                            let c9 = c.add(9).read_unaligned();
                            let num = c8
                                .wrapping_add(c.add(10).read_unaligned().wrapping_mul(0xFFFF_FFFE))
                                .wrapping_mul(c9);
                            let q: u32;
                            asm!(
                                "div {d:e}",
                                d = in(reg) c8,
                                inout("eax") num => q,
                                inout("edx") 0u32 => _,
                                options(nostack)
                            );
                            let u2 = ((q as i32) >> 0x1f) as u32;
                            c.add(0xb)
                                .write_unaligned(c9.wrapping_sub((q ^ u2).wrapping_sub(u2)));
                        } else if local_20 < 0x40 {
                            let dst = (&raw mut DAT_10168700)
                                .byte_add((local_20 as usize).wrapping_mul(4));
                            dst.cast::<u32>().write_unaligned(c.read_unaligned());
                            local_20 = local_20.wrapping_add(1);
                        }
                    }
                    _ => {}
                }
                c = c.add(4).read_unaligned() as *mut u32;
            }
            if local_20 != 0 {
                let mut i: i32 = 0;
                while i < local_20 {
                    let src = (&raw mut DAT_10168700).byte_add((i as usize).wrapping_mul(4));
                    FUN_10030fd2(src.cast::<i32>().read_unaligned());
                    i = i.wrapping_add(1);
                }
            }
        }
    }
}
