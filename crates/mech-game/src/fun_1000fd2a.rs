// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RaiseException(a: u32, b: u32, c: u32, d: *const u32);
}

unsafe fn checked_div(prod: i64, d: i32) -> i32 {
    // SAFETY: RaiseException is the OS routine, called with the original's arguments.
    unsafe {
        if d == 0 {
            RaiseException(0xC0000094, 0, 0, core::ptr::null());
        }
        if d == -1 && prod == i64::MIN {
            RaiseException(0xC0000094, 0, 0, core::ptr::null());
        }
        let q: i64 = if d == 0 {
            0
        } else {
            prod.wrapping_div(d as i64)
        };
        if q > 0x7FFF_FFFF || q < -0x8000_0000 {
            RaiseException(0xC0000094, 0, 0, core::ptr::null());
        }
        q as i32
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000fd2a(param_1: i32, param_2: i32) -> i32 {
    mech_trace::enter!("1000fd2a", param_1, param_2);
    let base = param_2 as *const u8;
    // SAFETY: param_2 is the address of the caller's struct; reads happen at
    // the original's offsets and in its order.
    unsafe {
        let a = param_1.wrapping_add(
            mech_trace::rd!(
                "1000fd2a",
                base.byte_add(0x8c).cast::<i32>(),
                read_unaligned
            )
            .wrapping_neg(),
        );
        let mut d = mech_trace::rd!(
            "1000fd2a",
            base.byte_add(0x94).cast::<i32>(),
            read_unaligned
        );
        let mut prod = (a as i64).wrapping_mul(mech_trace::rd!(
            "1000fd2a",
            base.byte_add(0x58).cast::<i32>(),
            read_unaligned
        ) as i64);
        let t1 = checked_div(prod, d);

        let t2 = t1.wrapping_add(
            mech_trace::rd!(
                "1000fd2a",
                base.byte_add(0x70).cast::<i32>(),
                read_unaligned
            ) >> 0x10,
        );
        let t2 = t2.wrapping_neg();

        d = mech_trace::rd!("1000fd2a", base.byte_add(100).cast::<i32>(), read_unaligned);
        prod = (t2 as i64).wrapping_mul(mech_trace::rd!(
            "1000fd2a",
            base.byte_add(0x98).cast::<i32>(),
            read_unaligned
        ) as i64);
        let t3 = checked_div(prod, d);

        mech_trace::rt!(
            "1000fd2a",
            mech_trace::rd!(
                "1000fd2a",
                base.byte_add(0x90).cast::<i32>(),
                read_unaligned
            )
            .wrapping_add(t3.wrapping_neg())
        )
    }
}
