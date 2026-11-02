// SPDX-License-Identifier: MIT

use core::arch::asm;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a40ac: u32;
    static mut DAT_100cded4: u8;
    static mut DAT_100cded8: u8;
    static mut DAT_100cdedc: u8;
    static mut DAT_100cdee0: u8;
    static mut DAT_100cdee4: u8;
    static mut DAT_100cdee8: u8;
    static mut DAT_100cdeec: u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10051f89() -> u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10003d60(param_1: i32, param_2: i32) -> i32;
}

#[inline(always)]
unsafe fn udiv(n: u32, d: u32) -> u32 {
    let q: u32;
    // SAFETY: the machine's own unsigned divide; faults on zero as the original does.
    unsafe {
        asm!("div {d:e}", d = in(reg) d, inout("eax") n => q, inout("edx") 0u32 => _, options(nomem, nostack));
    }
    q
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1007bf28(param_1: i32) {
    let i_var1 = param_1.wrapping_mul(0x24) as usize;
    // SAFETY: the globals and callees are the game's own; the arrays are
    // indexed by the record offset exactly as the original does.
    unsafe {
        if (&raw mut DAT_100cded4)
            .byte_add(i_var1)
            .cast::<i32>()
            .read_unaligned()
            != 0
        {
            let p = |base: *mut u8| base.byte_add(i_var1).cast::<i32>();

            let r = FUN_10051f89();
            let d = DAT_100a40ac.wrapping_shl(10).wrapping_shr(16);
            let q = udiv(r.wrapping_shl(16), d) as i32;
            let v = FUN_10003d60(q, 0x57e98);
            p(&raw mut DAT_100cded8).write_unaligned(v);

            let r = FUN_10051f89();
            let d = DAT_100a40ac.wrapping_shl(10).wrapping_shr(16);
            let q = udiv(r.wrapping_shl(16), d) as i32;
            let v = FUN_10003d60(q, 0x57e98);
            p(&raw mut DAT_100cdee0).write_unaligned(v);

            let r = FUN_10051f89() as i32;
            let n = r.wrapping_add(0x400).wrapping_mul(0x10000) as u32;
            let d = DAT_100a40ac.wrapping_shl(10).wrapping_shr(16);
            let q = udiv(n, d) as i32;
            let v = FUN_10003d60(q, 0x57e98);
            p(&raw mut DAT_100cdedc).write_unaligned(v);

            let r = FUN_10051f89() as i32;
            let m = r.wrapping_mul(0x7e98);
            let v = m.wrapping_add((m >> 31) & 0x3ff) >> 10;
            p(&raw mut DAT_100cdee4).write_unaligned(v);

            let r = FUN_10051f89() as i32;
            let m = r.wrapping_mul(0x7e98);
            let v = m.wrapping_add((m >> 31) & 0x3ff) >> 10;
            p(&raw mut DAT_100cdee8).write_unaligned(v);

            let r = FUN_10051f89() as i32;
            let m = r.wrapping_mul(0x7e98);
            let v = m.wrapping_add((m >> 31) & 0x3ff) >> 10;
            p(&raw mut DAT_100cdeec).write_unaligned(v);
        }
    }
}
