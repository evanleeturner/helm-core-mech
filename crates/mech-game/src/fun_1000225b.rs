// SPDX-License-Identifier: MIT

use core::arch::asm;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a40b4: u32;
    static mut DAT_100a40b8: u32;
    static mut DAT_100a40bc: u32;
    static mut DAT_100a40c0: i32;
    static mut DAT_100a40c4: u32;
    static mut DAT_100b38fc: u32;
    static mut DAT_100c4c70: u32;
    static mut DAT_100c4c78: u8;
    static mut DAT_10114db0: u32;
    static mut DAT_10218070: u32;
    static mut DAT_10218074: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000242a(param_1: u32, param_2: i32) -> u8;
    fn FUN_10002383(param_1: i32) -> u8;
}

#[inline(always)]
unsafe fn urem(a: u32, b: u32) -> u32 {
    let rem: u32;
    // SAFETY: the machine's own `div` is used so a zero divisor faults as in the original.
    unsafe {
        asm!(
            "div {d}",
            d = in(reg) b,
            inout("eax") a => _,
            inout("edx") 0u32 => rem,
            options(nostack, pure, nomem)
        );
    }
    rem
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1000225b() {
    // SAFETY: the globals and callees are the game's own memory and code;
    // DAT_10114db0 holds a pointer to a structure whose short at +6 is read.
    unsafe {
        let mut local_14: i32 = 3;
        let p = DAT_10114db0 as *const u8;
        if 0 < p.byte_add(6).cast::<i16>().read_unaligned() && DAT_10218074 == 1 {
            FUN_1000242a(0, 0);
            DAT_10218074 = 0;
        }
        if DAT_100c4c70 < 2 {
            DAT_100c4c70 = DAT_100c4c70.wrapping_add(1);
            DAT_10218070 = 0;
        } else if DAT_10218070 <= DAT_100b38fc {
            DAT_10218070 = DAT_100b38fc.wrapping_add(0x712);
            let ivar1 = urem(DAT_100a40c4.wrapping_add(DAT_100b38fc / 0xb5), DAT_100a40b4) as i32;
            if ivar1 < DAT_100a40c0 {
                DAT_100a40bc = DAT_100a40bc.wrapping_add(1);
                DAT_100a40bc = urem(DAT_100a40bc, DAT_100a40b8);
            }
            let mut local_8: i32 = 0;
            while local_8 < 4 {
                let v = (&raw mut DAT_100c4c78)
                    .byte_add((local_8 as usize) * 4)
                    .cast::<i32>()
                    .read_unaligned();
                if v <= ivar1 {
                    local_14 = local_8;
                }
                local_8 += 1;
            }
            DAT_100a40c0 = ivar1;
            FUN_10002383(local_14);
        }
    }
}
