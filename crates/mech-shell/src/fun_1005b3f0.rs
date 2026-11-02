// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a50e8: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005b3f0(param_1: *const u8, param_2: *mut u8) {
    mech_trace::enter!("1005b3f0", param_1, param_2);
    // SAFETY: the global is the game's own; param_1 points at three readable
    // bytes and param_2 at three writable bytes, and the table at the computed
    // address is the game's own memory.
    unsafe {
        let i_var1 = (mech_trace::gr!("1005b3f0", "100a50e8", DAT_100a50e8).wrapping_mul(0x40))
            .wrapping_add(0x100c66a0);
        mech_trace::dw!(
            "1005b3f0",
            param_2,
            mech_trace::rd!(
                "1005b3f0",
                ((i_var1.wrapping_add(u32::from(mech_trace::dr!("1005b3f0", param_1))))
                    as *const u8),
                read
            )
        );
        mech_trace::dw!(
            "1005b3f0",
            param_2.add(1),
            mech_trace::rd!(
                "1005b3f0",
                ((i_var1.wrapping_add(u32::from(mech_trace::dr!("1005b3f0", param_1.add(1)))))
                    as *const u8),
                read
            )
        );
        mech_trace::dw!(
            "1005b3f0",
            param_2.add(2),
            mech_trace::rd!(
                "1005b3f0",
                ((i_var1.wrapping_add(u32::from(mech_trace::dr!("1005b3f0", param_1.add(2)))))
                    as *const u8),
                read
            )
        );
    }
}
