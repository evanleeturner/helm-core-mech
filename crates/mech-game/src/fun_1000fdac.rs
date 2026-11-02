// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RaiseException(a: u32, b: u32, c: u32, d: *const u32);
}

#[inline(always)]
unsafe fn fault_div_zero() {
    // SAFETY: executes a divide by zero so the machine raises its own fault,
    // as the original 64-bit divide would after a continued exception.
    unsafe {
        core::arch::asm!(
            "div {0:e}",
            in(reg) 0u32,
            inout("eax") 0u32 => _,
            inout("edx") 0u32 => _,
        );
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000fdac(param_1: i32, param_2: i32) -> i32 {
    mech_trace::enter!("1000fdac", param_1, param_2);
    let base = param_2 as *const u8;
    // SAFETY: param_2 is the address of the caller's structure; fields are
    // read at the original's offsets, and the callee is the system's own.
    unsafe {
        let d1 = mech_trace::rd!(
            "1000fdac",
            base.byte_add(0x98).cast::<i32>(),
            read_unaligned
        );
        let n1 = (mech_trace::rd!(
            "1000fdac",
            base.byte_add(0x90).cast::<i32>(),
            read_unaligned
        ) as u32)
            .wrapping_sub(param_1 as u32) as i32 as i64
            * (mech_trace::rd!("1000fdac", base.byte_add(100).cast::<i32>(), read_unaligned)
                as i64);
        if d1 == 0 {
            RaiseException(0xC0000094, 0, 0, core::ptr::null());
            fault_div_zero();
        }
        let q1 = n1 / (d1 as i64);
        if q1 < -2147483648i64 || q1 > 2147483647i64 {
            RaiseException(0xC0000094, 0, 0, core::ptr::null());
        }

        let mut r1 = (q1 as i32 as u32).wrapping_add(
            (mech_trace::rd!(
                "1000fdac",
                base.byte_add(0x70).cast::<i32>(),
                read_unaligned
            ) >> 0x10) as u32,
        ) as i32;
        r1 = 0u32.wrapping_sub(r1 as u32) as i32;

        let d2 = mech_trace::rd!(
            "1000fdac",
            base.byte_add(0x58).cast::<i32>(),
            read_unaligned
        );
        let n2 = (r1 as i64)
            * (mech_trace::rd!(
                "1000fdac",
                base.byte_add(0x94).cast::<i32>(),
                read_unaligned
            ) as i64);
        if d2 == 0 {
            RaiseException(0xC0000094, 0, 0, core::ptr::null());
            fault_div_zero();
        }
        let q2 = n2 / (d2 as i64);
        if q2 < -2147483648i64 || q2 > 2147483647i64 {
            RaiseException(0xC0000094, 0, 0, core::ptr::null());
        }

        mech_trace::rt!(
            "1000fdac",
            (mech_trace::rd!(
                "1000fdac",
                base.byte_add(0x8c).cast::<i32>(),
                read_unaligned
            ) as u32)
                .wrapping_add(q2 as i32 as u32) as i32
        )
    }
}
