// SPDX-License-Identifier: MIT

use core::arch::asm;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aefc0: i32;
    static mut DAT_100af098: i32;
    static mut DAT_100af088: i32;
    static mut DAT_100af08c: i32;
    static mut DAT_100af090: i32;
    static mut DAT_100af094: i32;
    static mut DAT_10160c00: i32;
    static mut DAT_10160c04: i32;
    static mut DAT_10160e48: i32;
    static mut DAT_10160e50: i32;
    static mut DAT_10160e54: i32;
    static mut DAT_10160e58: i32;
    static mut DAT_10160e5c: i32;
    static mut DAT_10160e60: i32;
    static mut DAT_10160e64: i32;
    static mut DAT_10160e68: i32;
    static mut DAT_10160e70: i32;
    static mut DAT_100aefc8: i32;
    static mut DAT_100aefe8: i32;
    static mut DAT_100af028: i32;
    static mut DAT_100af068: i32;
    static mut DAT_100c6d5c: u8;
}

#[inline(always)]
unsafe fn rem_idiv(a: i32, b: i32) -> i32 {
    let r: i32;
    // SAFETY: the machine's own idiv; faults on zero divisor as the original does.
    unsafe {
        asm!(
            "cdq",
            "idiv {d:e}",
            d = in(reg) b,
            inlateout("eax") a => _,
            out("edx") r,
        );
    }
    r
}

#[inline(always)]
unsafe fn cell(x: i32, y: i32) -> i32 {
    let off = (y as u32)
        .wrapping_mul(0x18)
        .wrapping_add((x as u32).wrapping_mul(0x120)) as i32;
    // SAFETY: the table is the game's own memory; the value is read where the original reads it.
    unsafe {
        (&raw mut DAT_100c6d5c)
            .byte_offset(off as isize)
            .cast::<i32>()
            .read_unaligned()
    }
}

#[inline(always)]
unsafe fn has_item(p: i32) -> bool {
    // SAFETY: p is a pointer stored in the game's table.
    unsafe {
        (p as u32 as usize as *const u8)
            .byte_add(8)
            .cast::<i32>()
            .read_unaligned()
            != 0
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003cd85(param_1: *mut i32) {
    // SAFETY: globals and tables are the game's own; param_1 is the caller's
    // record, read where the original reads it.
    unsafe {
        let list0 = &raw mut DAT_10160c00;
        let list1 = &raw mut DAT_10160c04;
        let e70 = &raw mut DAT_10160e70;
        let mut local_18: i32 = 0;
        if DAT_100aefc0 == 0 || param_1.is_null() {
            list0.write_unaligned(DAT_100af098);
            list1.write_unaligned(0);
            return;
        }
        let mut local_c: i32 = (param_1.add(3).read_unaligned() >> 0x10) % 0x168;
        if local_c < 0 {
            local_c = (local_c as u32).wrapping_add(0x168) as i32;
        }
        let u_var1: u32 = (((local_c as u32).wrapping_add(0x16) as i32 % 0x168) / 0x2d) as u32;
        let u_var2: u32 = ((u_var1 as i32) >> 0x1f) as u32;
        if (((u_var1 ^ u_var2).wrapping_sub(u_var2) & 1) ^ u_var2) == u_var2 {
            e70.add(2).write_unaligned(3);
            e70.add(3).write_unaligned(3);
        } else {
            e70.add(2).write_unaligned(5);
            e70.add(3).write_unaligned(2);
        }
        e70.add(1).write_unaligned(4);
        e70.write_unaligned(2);
        match u_var1 {
            0 => {
                DAT_10160e5c = 0;
                DAT_10160e50 = 0;
                DAT_10160e54 = 0;
                DAT_10160e58 = 1;
            }
            1 => {
                DAT_10160e50 = 1;
                DAT_10160e54 = 1;
                DAT_10160e58 = 1;
                DAT_10160e5c = 1;
            }
            2 => {
                DAT_10160e50 = 1;
                DAT_10160e54 = 2;
                DAT_10160e58 = 3;
                DAT_10160e5c = 1;
            }
            3 => {
                DAT_10160e50 = 2;
                DAT_10160e54 = 3;
                DAT_10160e58 = 3;
                DAT_10160e5c = 2;
            }
            4 => {
                DAT_10160e50 = 2;
                DAT_10160e54 = 4;
                DAT_10160e58 = 5;
                DAT_10160e5c = 2;
            }
            5 => {
                DAT_10160e50 = 3;
                DAT_10160e54 = 5;
                DAT_10160e58 = 5;
                DAT_10160e5c = 3;
            }
            6 => {
                DAT_10160e50 = 3;
                DAT_10160e54 = 6;
                DAT_10160e58 = 7;
                DAT_10160e5c = 3;
            }
            _ => {
                DAT_10160e5c = 0;
                DAT_10160e50 = 0;
                DAT_10160e58 = 7;
                DAT_10160e54 = 7;
            }
        }
        DAT_10160e60 = ((param_1.read_unaligned() as u32).wrapping_add(150000) as i32) / 25000;
        DAT_10160e68 =
            ((param_1.add(2).read_unaligned() as u32).wrapping_add(150000) as i32) / 25000;
        DAT_10160e64 = 0;
        DAT_10160e48 = 0;
        let mut local_10: i32 = 0;
        let mut local_14: i32 = 0;
        while DAT_10160e64 < 4 {
            match DAT_10160e64 {
                0 => {
                    let idx = rem_idiv(DAT_10160e50.wrapping_add(DAT_10160e48), DAT_100af088);
                    let t = &raw mut DAT_100aefc8;
                    local_10 = t
                        .offset(idx.wrapping_mul(2) as isize)
                        .read_unaligned()
                        .wrapping_add(DAT_10160e60);
                    local_14 = t
                        .offset(idx.wrapping_mul(2).wrapping_add(1) as isize)
                        .read_unaligned()
                        .wrapping_add(DAT_10160e68);
                }
                1 => {
                    let idx = rem_idiv(DAT_10160e54.wrapping_add(DAT_10160e48), DAT_100af08c);
                    let t = &raw mut DAT_100aefe8;
                    local_10 = t
                        .offset(idx.wrapping_mul(2) as isize)
                        .read_unaligned()
                        .wrapping_add(DAT_10160e60);
                    local_14 = t
                        .offset(idx.wrapping_mul(2).wrapping_add(1) as isize)
                        .read_unaligned()
                        .wrapping_add(DAT_10160e68);
                }
                2 => {
                    let idx = rem_idiv(DAT_10160e58.wrapping_add(DAT_10160e48), DAT_100af090);
                    let t = &raw mut DAT_100af028;
                    local_10 = t
                        .offset(idx.wrapping_mul(2) as isize)
                        .read_unaligned()
                        .wrapping_add(DAT_10160e60);
                    local_14 = t
                        .offset(idx.wrapping_mul(2).wrapping_add(1) as isize)
                        .read_unaligned()
                        .wrapping_add(DAT_10160e68);
                }
                3 => {
                    let idx = rem_idiv(DAT_10160e5c.wrapping_add(DAT_10160e48), DAT_100af094);
                    let t = &raw mut DAT_100af068;
                    local_10 = t
                        .offset(idx.wrapping_mul(2) as isize)
                        .read_unaligned()
                        .wrapping_add(DAT_10160e60);
                    local_14 = t
                        .offset(idx.wrapping_mul(2).wrapping_add(1) as isize)
                        .read_unaligned()
                        .wrapping_add(DAT_10160e68);
                }
                _ => {}
            }
            DAT_10160e48 = DAT_10160e48.wrapping_add(1);
            if e70.offset(DAT_10160e64 as isize).read_unaligned() <= DAT_10160e48 {
                DAT_10160e48 = 0;
                DAT_10160e64 = DAT_10160e64.wrapping_add(1);
            }
            if -1 < local_10 && local_10 < 0xc && -1 < local_14 && local_14 < 0xc {
                let p = cell(local_10, local_14);
                if has_item(p) {
                    let p2 = cell(local_10, local_14);
                    list0.offset(local_18 as isize).write_unaligned(p2);
                    list1.offset(local_18 as isize).write_unaligned(0);
                    local_18 = local_18.wrapping_add(1);
                }
            }
        }
        let p = cell(DAT_10160e60, DAT_10160e68);
        if has_item(p) {
            let p2 = cell(DAT_10160e60, DAT_10160e68);
            list0.offset(local_18 as isize).write_unaligned(p2);
            list1.offset(local_18 as isize).write_unaligned(0);
        }
    }
}
