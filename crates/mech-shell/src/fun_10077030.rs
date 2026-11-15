// SPDX-License-Identifier: MIT

use core::arch::asm;

#[unsafe(export_name = "FUN_10077030")]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn aullrem(
    dividend_low: u32,
    dividend_high: u32,
    divisor_low: u32,
    divisor_high: u32,
) -> u64 {
    mech_trace::enter!(
        "10077030",
        dividend_low,
        dividend_high,
        divisor_low,
        divisor_high
    );
    if divisor_high == 0 {
        let high_remainder: u32;
        let remainder: u32;
        // SAFETY: the machine's own DIV is used so a zero divisor faults as in the original.
        unsafe {
            mech_trace::log::debug!(
                f = "10077030", b = "div", a:% = mech_trace::Args([
                    mech_trace::__shown!(divisor_low),
                    mech_trace::__shown!(dividend_high),
                    mech_trace::__shown!(0u32),
                ]); "asm.in"
            );
            asm!(
                "div {d}",
                d = in(reg) divisor_low,
                inout("eax") dividend_high => _,
                inout("edx") 0u32 => high_remainder,
                options(nomem, nostack)
            );
            mech_trace::log::debug!(
                f = "10077030", b = "div", r = "edx", v:% = mech_trace::__shown!(high_remainder); "asm.out"
            );
            mech_trace::log::debug!(
                f = "10077030", b = "div", a:% = mech_trace::Args([
                    mech_trace::__shown!(divisor_low),
                    mech_trace::__shown!(dividend_low),
                    mech_trace::__shown!(high_remainder),
                ]); "asm.in"
            );
            asm!(
                "div {d}",
                d = in(reg) divisor_low,
                inout("eax") dividend_low => _,
                inout("edx") high_remainder => remainder,
                options(nomem, nostack)
            );
            mech_trace::log::debug!(
                f = "10077030", b = "div", r = "edx", v:% = mech_trace::__shown!(remainder); "asm.out"
            );
        }
        return mech_trace::rt!("10077030", remainder as u64);
    }
    let mut ecx = divisor_high;
    let mut ebx = divisor_low;
    let mut edx = dividend_high;
    let mut eax = dividend_low;
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
    let hi_part = q.wrapping_mul(divisor_high);
    let prod = (q as u64).wrapping_mul(divisor_low as u64);
    let lo = prod as u32;
    let (hi, carry) = ((prod >> 32) as u32).overflowing_add(hi_part);
    let mut v: u64 = ((hi as u64) << 32) | lo as u64;
    if carry || hi > dividend_high || (hi == dividend_high && lo > dividend_low) {
        v = v.wrapping_sub(((divisor_high as u64) << 32) | divisor_low as u64);
    }
    v = v.wrapping_sub(((dividend_high as u64) << 32) | dividend_low as u64);
    mech_trace::rt!("10077030", v.wrapping_neg())
}
