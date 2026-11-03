// SPDX-License-Identifier: MIT

use core::arch::asm;

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10077030(
    param_1: u32,
    param_2: u32,
    param_3: u32,
    param_4: u32,
) -> u64 {
    mech_trace::enter!("10077030", param_1, param_2, param_3, param_4);
    if param_4 == 0 {
        let rem1: u32;
        let rem2: u32;
        // SAFETY: the machine's own DIV is used so a zero divisor faults as in the original.
        unsafe {
            asm!(
                "div {d}",
                d = in(reg) param_3,
                inout("eax") param_2 => _,
                inout("edx") 0u32 => rem1,
                options(nomem, nostack)
            );
            asm!(
                "div {d}",
                d = in(reg) param_3,
                inout("eax") param_1 => _,
                inout("edx") rem1 => rem2,
                options(nomem, nostack)
            );
        }
        return mech_trace::rt!("10077030", rem2 as u64);
    }
    let mut ecx = param_4;
    let mut ebx = param_3;
    let mut edx = param_2;
    let mut eax = param_1;
    loop {
        ebx = (ebx >> 1) | ((ecx & 1) << 31);
        ecx >>= 1;
        eax = (eax >> 1) | ((edx & 1) << 31);
        edx >>= 1;
        if ecx == 0 {
            break;
        }
    }
    let q = ((((edx as u64) << 32) | eax as u64) / (ebx as u64)) as u32;
    let hi_part = q.wrapping_mul(param_4);
    let prod = (q as u64).wrapping_mul(param_3 as u64);
    let lo = prod as u32;
    let (hi, carry) = ((prod >> 32) as u32).overflowing_add(hi_part);
    let mut v: u64 = ((hi as u64) << 32) | lo as u64;
    if carry || hi > param_2 || (hi == param_2 && lo > param_1) {
        v = v.wrapping_sub(((param_4 as u64) << 32) | param_3 as u64);
    }
    v = v.wrapping_sub(((param_2 as u64) << 32) | param_1 as u64);
    mech_trace::rt!("10077030", v.wrapping_neg())
}
