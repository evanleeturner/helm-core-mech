// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10018060(param_1: i32, param_2: i32) -> u32;
    fn FUN_1006ec30(param_1: *mut core::ffi::c_void);
}

#[allow(non_snake_case)]
unsafe extern "fastcall" {
    fn FUN_10040680(param_1: i32);
    fn FUN_10040640(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "thiscall" fn FUN_1001e650(this: *mut core::ffi::c_void, param_1: i8) {
    mech_trace::enter!("1001e650", this, param_1);
    // SAFETY: `this` points at the game's object whose first field is a pointer
    // to a container; the callees are the game's own and are called in the
    // original order.
    unsafe {
        let mut i_var1 = mech_trace::rd!("1001e650", this.cast::<i32>(), read_unaligned)
            .wrapping_add(4) as *const i32;
        let mut count = mech_trace::rd!("1001e650", i_var1, read_unaligned);
        while count != 0 {
            let base = mech_trace::rd!("1001e650", this.cast::<i32>(), read_unaligned);
            let inner = mech_trace::rd!(
                "1001e650",
                ((base as usize).wrapping_add(4) as *const i32),
                read_unaligned
            );
            let pv_var2 = mech_trace::cr!(
                "1001e650",
                "10018060",
                FUN_10018060(base, inner.wrapping_sub(1))
            );
            if pv_var2 != 0 {
                if param_1 == 1 {
                    FUN_10040680(pv_var2 as i32);
                    FUN_1006ec30(pv_var2 as usize as *mut core::ffi::c_void);
                } else {
                    FUN_10040640(pv_var2 as i32);
                }
            }
            i_var1 = (mech_trace::rd!("1001e650", this.cast::<i32>(), read_unaligned) as usize)
                .wrapping_add(4) as *const i32;
            count = mech_trace::rd!("1001e650", i_var1, read_unaligned);
        }
    }
}
