// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100686cb: u8;
    static mut DAT_100694db: u8;
}

#[inline(always)]
unsafe fn a(i: u32) -> i32 {
    // SAFETY: reads the game's own table at base + i*4, as the original does.
    unsafe {
        let base = (&raw mut DAT_100686cb) as u32;
        mech_trace::rd!(
            "100694df",
            (base.wrapping_add(i.wrapping_mul(4)) as *const i32),
            read_unaligned
        )
    }
}

#[inline(always)]
unsafe fn b(i: u32) -> i32 {
    // SAFETY: reads the game's own table at base - i*4, as the original does.
    unsafe {
        let base = (&raw mut DAT_100694db) as u32;
        mech_trace::rd!(
            "100694df",
            (base.wrapping_add(i.wrapping_mul(0xfffffffc)) as *const i32),
            read_unaligned
        )
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100694df(mut param_1: u32, param_2: *mut i32, param_3: *mut i32) {
    mech_trace::enter!("100694df", param_1, param_2, param_3);
    while (param_1 as i32) < 0 {
        param_1 = param_1.wrapping_add(0xe10);
    }
    while 0xe10 < (param_1 as i32) {
        param_1 = param_1.wrapping_sub(0xe10);
    }
    let i_var1: i32;
    let i_var2: i32;
    // SAFETY: table reads are the game's own data; param_2 and param_3 are
    // the caller's output pointers.
    unsafe {
        if param_1 < 0x709 {
            if param_1 < 0x385 {
                i_var1 = a(param_1);
                i_var2 = b(param_1);
            } else {
                let t = 0x708u32.wrapping_sub(param_1);
                i_var1 = a(t).wrapping_neg();
                i_var2 = b(t);
            }
        } else {
            let u_var3 = 0xe10u32.wrapping_sub(param_1);
            if u_var3 < 0x385 {
                i_var1 = a(u_var3);
                i_var2 = b(u_var3).wrapping_neg();
            } else {
                let t = 0x708u32.wrapping_sub(u_var3);
                i_var1 = a(t).wrapping_neg();
                i_var2 = b(t).wrapping_neg();
            }
        }
        mech_trace::wr!("100694df", param_2, write_unaligned, i_var1);
        mech_trace::wr!("100694df", param_3, write_unaligned, i_var2);
    }
}
