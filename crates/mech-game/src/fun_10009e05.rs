// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100eef40: [i32; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100354cd(param_1: *mut u16) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10009e05(param_1: *mut u16) -> u32 {
    mech_trace::enter!("10009e05", param_1);
    let mut local_10: u32 = 0;
    // SAFETY: param_1 points at the caller's record; the global table and the
    // callee are the game's own, read and called in the original's order.
    unsafe {
        let w1 = mech_trace::rd!("10009e05", param_1.add(1), read_unaligned);
        if (w1 & 0xf00) == 0x100 {
            let idx = mech_trace::rd!("10009e05", param_1.add(10), read_unaligned) as usize;
            let base = (&raw mut DAT_100eef40).cast::<i32>();
            let ptr = mech_trace::rd!("10009e05", base.add(idx), read_unaligned);
            let v = mech_trace::rd!(
                "10009e05",
                ((ptr as u32).wrapping_add(0x14) as *const u16),
                read_unaligned
            );
            local_10 = ((v & 0x16) != 0) as u32;
        } else if (w1 & 0xf0) == 0x30 || (w1 & 0xf0) == 0x70 {
            local_10 = 1;
        }
        if local_10 == 0 {
            local_10 = mech_trace::cr!("10009e05", "100354cd", FUN_100354cd(param_1));
        }
    }
    mech_trace::rt!("10009e05", local_10)
}
