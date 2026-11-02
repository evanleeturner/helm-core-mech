// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_000b0f00: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_100070d0();
    fn FUN_100070f2();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000712c(param_1: *mut i8) {
    mech_trace::enter!("1000712c", param_1);
    // SAFETY: the callees and the global are the game's own; param_1 points at
    // a NUL-terminated string, and the destination buffer is the game's own
    // memory, written where the original does.
    unsafe {
        let mut local_10: *mut i8 = param_1;
        let mut local_8: *mut i8 = (&raw mut DAT_000b0f00).cast::<i8>();
        let mut p: *mut i8 = param_1;
        let mut u_var2: u32 = 0xffff_ffff;
        loop {
            if u_var2 == 0 {
                break;
            }
            u_var2 = u_var2.wrapping_sub(1);
            let c_var1 = mech_trace::rd!("1000712c", p, read);
            p = p.byte_add(1);
            if c_var1 == 0 {
                break;
            }
        }
        let mut local_14: i32 = (!u_var2).wrapping_sub(1) as i32;
        if 0x50 < local_14 {
            local_14 = 0x50;
        }
        FUN_100070d0();
        FUN_100070f2();
        let mut local_c: i32 = 0;
        while local_c < local_14 {
            mech_trace::wr!(
                "1000712c",
                local_8,
                write,
                mech_trace::rd!("1000712c", local_10, read)
            );
            local_10 = local_10.byte_add(1);
            local_8 = local_8.byte_add(2);
            local_c = local_c.wrapping_add(1);
        }
    }
}
