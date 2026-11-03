// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10076fc0(
    param_1: u32,
    param_2: u32,
    param_3: u32,
    param_4: u32,
) -> u64 {
    mech_trace::enter!("10076fc0", param_1, param_2, param_3, param_4);
    if param_4 == 0 {
        let q_hi: u32;
        let q_lo: u32;
        // SAFETY: DIV faults on a zero divisor exactly as the original does;
        // the second DIV cannot overflow because its high half is a remainder.
        unsafe {
            let rem: u32;
            core::arch::asm!(
                "div {d}",
                d = in(reg) param_3,
                inout("eax") param_2 => q_hi,
                inout("edx") 0u32 => rem,
                options(nomem, nostack)
            );
            let _rem2: u32;
            core::arch::asm!(
                "div {d}",
                d = in(reg) param_3,
                inout("eax") param_1 => q_lo,
                inout("edx") rem => _rem2,
                options(nomem, nostack)
            );
        }
        return mech_trace::rt!("10076fc0", ((q_hi as u64) << 32) | (q_lo as u64));
    }
    let mut ecx = param_4;
    let mut ebx = param_3;
    let mut edx = param_2;
    let mut eax = param_1;
    loop {
        let c1 = ecx & 1;
        ecx >>= 1;
        ebx = (c1 << 31) | (ebx >> 1);
        let c2 = edx & 1;
        edx >>= 1;
        eax = (c2 << 31) | (eax >> 1);
        if ecx == 0 {
            break;
        }
    }
    let dividend = ((edx as u64) << 32) | (eax as u64);
    let mut esi = (dividend / (ebx as u64)) as u32;
    let ecx2 = (esi as u64).wrapping_mul(param_4 as u64) as u32;
    let prod = (param_3 as u64).wrapping_mul(esi as u64);
    let lo = prod as u32;
    let hi = (prod >> 32) as u32;
    let (sum, carry) = hi.overflowing_add(ecx2);
    if carry || sum > param_2 || (sum == param_2 && lo > param_1) {
        esi = esi.wrapping_sub(1);
    }
    mech_trace::rt!("10076fc0", esi as u64)
}
