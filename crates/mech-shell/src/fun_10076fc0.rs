// SPDX-License-Identifier: MIT

#[unsafe(export_name = "FUN_10076fc0")]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn aulldiv(
    dividend_low: u32,
    dividend_high: u32,
    divisor_low: u32,
    divisor_high: u32,
) -> u64 {
    mech_trace::enter!(
        "10076fc0",
        dividend_low,
        dividend_high,
        divisor_low,
        divisor_high
    );
    if divisor_high == 0 {
        let q_hi: u32;
        let q_lo: u32;
        // SAFETY: DIV faults on a zero divisor exactly as the original does;
        // the second DIV cannot overflow because its high half is a remainder.
        unsafe {
            let high_remainder: u32;
            mech_trace::log::debug!(
                f = "10076fc0", b = "div", a:% = mech_trace::Args([
                    mech_trace::__shown!(divisor_low),
                    mech_trace::__shown!(dividend_high),
                    mech_trace::__shown!(0u32),
                ]); "asm.in"
            );
            core::arch::asm!(
                "div {d}",
                d = in(reg) divisor_low,
                inout("eax") dividend_high => q_hi,
                inout("edx") 0u32 => high_remainder,
                options(nomem, nostack)
            );
            mech_trace::log::debug!(
                f = "10076fc0", b = "div", r = "eax", v:% = mech_trace::__shown!(q_hi); "asm.out"
            );
            mech_trace::log::debug!(
                f = "10076fc0", b = "div", r = "edx", v:% = mech_trace::__shown!(high_remainder); "asm.out"
            );
            let _remainder: u32;
            mech_trace::log::debug!(
                f = "10076fc0", b = "div", a:% = mech_trace::Args([
                    mech_trace::__shown!(divisor_low),
                    mech_trace::__shown!(dividend_low),
                    mech_trace::__shown!(high_remainder),
                ]); "asm.in"
            );
            core::arch::asm!(
                "div {d}",
                d = in(reg) divisor_low,
                inout("eax") dividend_low => q_lo,
                inout("edx") high_remainder => _remainder,
                options(nomem, nostack)
            );
            mech_trace::log::debug!(
                f = "10076fc0", b = "div", r = "eax", v:% = mech_trace::__shown!(q_lo); "asm.out"
            );
            mech_trace::log::debug!(
                f = "10076fc0", b = "div", r = "edx", v:% = mech_trace::__shown!(_remainder); "asm.out"
            );
        }
        return mech_trace::rt!("10076fc0", ((q_hi as u64) << 32) | (q_lo as u64));
    }
    let mut ecx = divisor_high;
    let mut ebx = divisor_low;
    let mut edx = dividend_high;
    let mut eax = dividend_low;
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
    let ecx2 = (esi as u64).wrapping_mul(divisor_high as u64) as u32;
    let prod = (divisor_low as u64).wrapping_mul(esi as u64);
    let lo = prod as u32;
    let hi = (prod >> 32) as u32;
    let (sum, carry) = hi.overflowing_add(ecx2);
    if carry || sum > dividend_high || (sum == dividend_high && lo > dividend_low) {
        esi = esi.wrapping_sub(1);
    }
    mech_trace::rt!("10076fc0", esi as u64)
}
