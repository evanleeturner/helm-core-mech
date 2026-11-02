// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c4c68: [*mut u8; 8];
    static mut DAT_100aa6b8: i32;
    static mut DAT_100aa6dc: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000d6f0(param_1: i32, param_2: i32) -> i32;
}

// __thiscall: `this` in ECX, stack arguments popped by the callee. A
// fastcall with an unused EDX slot gives the same stack layout.
#[allow(non_snake_case)]
unsafe extern "fastcall" {
    fn FUN_1005f7d0(this: *mut u8, _edx: u32, param_1: i32);
    fn FUN_1005f810(this: *mut u8);
    fn FUN_1005f890(this: *mut u8);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1006cfd0(param_1: i32) {
    mech_trace::enter!("1006cfd0", param_1);
    let mut i_var3: i32 = 0;
    loop {
        // SAFETY: the globals and callees are the game's own; param_1 points
        // at an object holding the field at +0x364, and the table has 8 entries.
        unsafe {
            let this = mech_trace::rd!(
                "1006cfd0",
                (&raw const DAT_100c4c68)
                    .cast::<*mut u8>()
                    .add(i_var3 as usize),
                read
            );
            let i_var2 = mech_trace::cr!("1006cfd0", "1000d6f0", FUN_1000d6f0(param_1, i_var3));
            let field = mech_trace::rd!(
                "1006cfd0",
                ((param_1 as usize + 0x364) as *const i32),
                read_unaligned
            );
            if i_var2 == 3 {
                if field == i_var3 {
                    FUN_1005f7d0(this, 0, 6);
                    FUN_1005f810(this);
                } else {
                    FUN_1005f890(this);
                }
            } else if field != i_var3 {
                FUN_1005f7d0(this, 0, i_var2);
                FUN_1005f810(this);
            } else {
                FUN_1005f7d0(this, 0, i_var2.wrapping_add(3));
                let p = (mech_trace::gr!("1006cfd0", "100aa6b8", DAT_100aa6b8) as usize)
                    .wrapping_add(0x364) as *const i32;
                mech_trace::gw!(
                    "1006cfd0",
                    "100aa6dc",
                    DAT_100aa6dc,
                    mech_trace::rd!("1006cfd0", p, read_unaligned)
                );
                FUN_1005f810(this);
            }
        }
        i_var3 = i_var3.wrapping_add(1);
        if 7 < i_var3 {
            return;
        }
    }
}
