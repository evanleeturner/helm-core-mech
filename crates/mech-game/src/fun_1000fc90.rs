// SPDX-License-Identifier: MIT

use core::arch::asm;

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RaiseException(code: u32, flags: u32, n: u32, args: *const u32);
}

unsafe fn machine_idiv(prod: i64, d: i32) -> i32 {
    let q: i32;
    // SAFETY: only reached for a zero divisor or an overflowing quotient, where the
    // machine's idiv raises its own divide fault, as the original does.
    unsafe {
        let input_2 = prod as u64 as u32;
        let input_3 = (prod as u64 >> 32) as u32;
        mech_trace::log::debug!(
            f = "1000fc90", b = "idiv", a:% = mech_trace::Args([
                mech_trace::__shown!(d),
                mech_trace::__shown!(input_2),
                mech_trace::__shown!(input_3),
            ]); "asm.in"
        );
        asm!(
            "idiv {d:e}",
            d = in(reg) d,
            inout("eax") (prod as u64 as u32) => q,
            inout("edx") ((prod as u64 >> 32) as u32) => _,
            options(nostack)
        );
        mech_trace::log::debug!(
            f = "1000fc90", b = "idiv", r = "eax", v:% = mech_trace::__shown!(q); "asm.out"
        );
    }
    q
}

unsafe fn div_step(prod: i64, d: i32) -> i32 {
    // SAFETY: the callee is the system's own; the division faults like the machine's where unchecked.
    unsafe {
        if d == 0 {
            RaiseException(0xC0000094, 0, 0, core::ptr::null());
        }
        if d == -1 && prod == i64::MIN {
            RaiseException(0xC0000094, 0, 0, core::ptr::null());
        }
        let q: i64 = if d == 0 || (d == -1 && prod == i64::MIN) {
            machine_idiv(prod, d) as i64
        } else {
            prod / (d as i64)
        };
        if q > 0x7FFFFFFF || q < -0x80000000 {
            RaiseException(0xC0000094, 0, 0, core::ptr::null());
        }
        q as i32
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000fc90(param_1: i32, param_2: i32, param_3: i32) -> i32 {
    mech_trace::enter!("1000fc90", param_1, param_2, param_3);
    // SAFETY: param_3 is the caller's struct address; fields are read at the original's offsets.
    unsafe {
        let base = param_3 as usize as *const u8;
        let rd = |off: usize| -> i32 {
            mech_trace::rd!("1000fc90", base.byte_add(off).cast::<i32>(), read_unaligned)
        };

        let a = (param_1 as u32).wrapping_sub(rd(0x8c) as u32) as i32;
        let d = rd(0x94);
        let prod = (a as i64).wrapping_mul(rd(0x58) as i64);
        let t1 = div_step(prod, d);

        let b = (rd(0x90) as u32).wrapping_sub(param_2 as u32) as i32;
        let d = rd(0x98);
        let prod = (b as i64).wrapping_mul(rd(100) as i64);
        let t2 = div_step(prod, d);

        let lhs = (0u32.wrapping_sub(rd(0x70) as u32) as i32) >> 16;
        let rhs = (t1 as u32).wrapping_add(t2 as u32) as i32;
        mech_trace::rt!("1000fc90", (lhs <= rhs) as i32)
    }
}
