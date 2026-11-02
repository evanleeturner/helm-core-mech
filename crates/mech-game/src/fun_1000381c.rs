// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aaf68: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10006b20(param_1: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000381c(param_1: i32) -> i32 {
    mech_trace::enter!("1000381c", param_1);
    // SAFETY: the global and the callee are the game's own; the global holds
    // a pointer to a structure whose field at offset 0xc is read.
    unsafe {
        let base = mech_trace::gr!("1000381c", "100aaf68", DAT_100aaf68);
        let v = mech_trace::rd!(
            "1000381c",
            ((base.wrapping_add(0xc)) as *const i32),
            read_unaligned
        );
        let pan_term = mech_trace::cr!(
            "1000381c",
            "10006b20",
            FUN_10006b20(v.wrapping_sub(param_1))
        );
        let mut pan = (pan_term >> 0x17).wrapping_add(0x40);
        if pan < 0xf {
            pan = 0xf;
        } else if 0x6f < pan {
            pan = 0x6f;
        }
        mech_trace::rt!("1000381c", pan)
    }
}
