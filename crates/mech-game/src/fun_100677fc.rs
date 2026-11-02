// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aaf68: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10006b9a(param_1: i32) -> i32;
    fn FUN_10006b20(param_1: i32) -> i32;
    fn FUN_10003d60(param_1: i32, param_2: i32) -> i32;
    fn FUN_10067951(
        param_1: i32,
        param_2: *mut i32,
        param_3: u32,
        param_4: i32,
        param_5: *mut i32,
    ) -> *mut i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10006d3e() -> i32;
}

#[inline(always)]
unsafe fn idiv(a: i32, d: i32) -> i32 {
    let q: i32;
    // SAFETY: machine signed divide; faults like the original on overflow.
    unsafe {
        core::arch::asm!(
            "cdq",
            "idiv {d:e}",
            d = in(reg) d,
            inout("eax") a => q,
            out("edx") _,
        );
    }
    q
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100677fc(param_1: i32, param_2: i32, param_3: *mut i32) -> *mut i32 {
    mech_trace::enter!("100677fc", param_1, param_2, param_3);
    // SAFETY: param_1 points at the caller's rect, the globals and callees are
    // the game's own, and param_3 is the caller's output buffer.
    unsafe {
        let mut i_var2 = (param_2 % 0x1680000).wrapping_add(0x1680000) % 0x1680000;
        let mut local: [i32; 2] = [0; 2];
        let r4 = mech_trace::rd!(
            "100677fc",
            ((param_1 as u32).wrapping_add(4) as *const i32),
            read_unaligned
        );
        let rc = mech_trace::rd!(
            "100677fc",
            ((param_1 as u32).wrapping_add(0xc) as *const i32),
            read_unaligned
        );
        let r8 = mech_trace::rd!(
            "100677fc",
            ((param_1 as u32).wrapping_add(8) as *const i32),
            read_unaligned
        );
        let r10 = mech_trace::rd!(
            "100677fc",
            ((param_1 as u32).wrapping_add(0x10) as *const i32),
            read_unaligned
        );
        local[0] = (rc.wrapping_sub(r4).wrapping_add(1)) >> 1;
        local[1] = (r10.wrapping_sub(r8).wrapping_add(1)) >> 1;
        let mut local_c: u32 = if i_var2 < 0x5a0000 || 0x10e0000 < i_var2 {
            0
        } else {
            1
        };
        if i_var2 < 0xb40001 {
            local_c |= 2;
        }
        let i_var1 = mech_trace::cr!("100677fc", "10006b9a", FUN_10006b9a(i_var2)) >> 0xd;
        i_var2 = mech_trace::cr!("100677fc", "10006b20", FUN_10006b20(i_var2));
        let p =
            mech_trace::gr!("100677fc", "100aaf68", DAT_100aaf68).wrapping_add(0x44) as *const i32;
        i_var2 = mech_trace::cr!(
            "100677fc",
            "10003d60",
            FUN_10003d60(
                i_var2 >> 0xd,
                mech_trace::rd!("100677fc", p, read_unaligned)
            )
        );
        if i_var1 == 0 || 0x7fff < idiv(i_var2, i_var1) || idiv(i_var2, i_var1) < -0x8000 {
            local_c |= 4;
        } else {
            local_c &= 0xfffffffb;
        }
        let local_8 = mech_trace::cr!("100677fc", "10006d3e", FUN_10006d3e());
        mech_trace::cr!(
            "100677fc",
            "10067951",
            FUN_10067951(
                param_1,
                (&raw mut local) as *mut i32,
                local_c,
                local_8,
                param_3,
            )
        );
        mech_trace::rt!("100677fc", param_3)
    }
}
