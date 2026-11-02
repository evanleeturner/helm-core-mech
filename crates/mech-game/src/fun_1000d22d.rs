// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7584: i32;
    static mut DAT_101f64ad: [u8; 0];
    static mut DAT_101f6518: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1005db35(param_1: *mut i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1000d22d() {
    mech_trace::enter!("1000d22d");
    // SAFETY: the globals and the callee are the game's own; offsets index
    // the game's record table, read where the original does.
    unsafe {
        let mut local_8: i32 = 0;
        while local_8 < mech_trace::gr!("1000d22d", "100a7584", DAT_100a7584) {
            let i_var1 = local_8.wrapping_mul(0x7c) as isize;
            let flags = mech_trace::rd!(
                "1000d22d",
                (&raw mut DAT_101f64ad).cast::<u8>().offset(i_var1),
                read
            );
            if (flags & 8) == 0 {
                let p = (&raw mut DAT_101f6518).cast::<u8>().offset(i_var1);
                if mech_trace::rd!("1000d22d", p.cast::<i32>(), read_unaligned) != 0 {
                    mech_trace::cr!("1000d22d", "1005db35", FUN_1005db35(p.cast::<i32>()));
                }
            }
            local_8 = local_8.wrapping_add(1);
        }
    }
}
