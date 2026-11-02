// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10092000: u32;
    static mut DAT_100938d4: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1005b1b0(param_1: *mut i8) -> u8;
    fn FUN_100315b0(param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10035da0(param_1: i32) {
    mech_trace::enter!("10035da0", param_1);
    // SAFETY: the globals and callees are the game's own; param_1 and the
    // table at DAT_10092000 point at the game's structures, read where the
    // original reads them.
    unsafe {
        let base = mech_trace::gr!("10035da0", "10092000", DAT_10092000);
        let sel = mech_trace::rd!(
            "10035da0",
            ((base.wrapping_add(0x364)) as *const i32),
            read_unaligned
        );
        if -1 < sel {
            let field = mech_trace::rd!(
                "10035da0",
                ((param_1 as u32).wrapping_add(0x24) as *const i32),
                read_unaligned
            );
            let idx = field.wrapping_add(sel.wrapping_mul(0xc));
            let addr = base
                .wrapping_add(0x368)
                .wrapping_add((idx as u32).wrapping_mul(4));
            let i_var1 = mech_trace::rd!("10035da0", (addr as *const i32), read_unaligned);
            if -1 < i_var1 {
                let i_var2 = (i_var1 / 100).wrapping_mul(100);
                if 0x1c83 < i_var2 && i_var2 < 8000 {
                    mech_trace::cr!("10035da0", "1005b1b0", FUN_1005b1b0(&raw mut DAT_100938d4));
                    return;
                }
                mech_trace::cr!("10035da0", "100315b0", FUN_100315b0(i_var1));
            }
        }
    }
}
