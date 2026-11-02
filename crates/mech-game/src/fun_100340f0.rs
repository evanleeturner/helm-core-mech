// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100eef40: [u32; 256];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1004d5e3(param_1: i32) -> u32;
    fn FUN_10034c0a(param_1: i32, param_2: i32) -> u8;
    fn FUN_1004d60d(param_1: i32) -> u32;
    fn FUN_10034e74(param_1: i32, param_2: i32, param_3: i32) -> u8;
    fn FUN_1003ba89(param_1: *mut i32, param_2: *mut i32, param_3: *mut i32) -> u32;
    fn FUN_1003510c(param_1: i32, param_2: i32, param_3: i32) -> u8;
    fn FUN_10066f36(param_1: i32, param_2: *mut i32, param_3: *mut i32) -> *mut i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10083808() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100340f0(param_1: *mut i32) {
    mech_trace::enter!("100340f0", param_1);
    // SAFETY: param_1 points at the game's object pointer; all offsets read are
    // inside that object, and the callees and globals are the game's own.
    unsafe {
        let local_c: i32 = mech_trace::rd!("100340f0", param_1, read_unaligned);
        let mut local_24: u32 = mech_trace::rd!(
            "100340f0",
            ((local_c.wrapping_add(0x104)) as usize as *const u32),
            read_unaligned
        );
        if local_24 != 0 && (local_24 & 0x1000) == 0 {
            let local_8: u32 = local_24 & 0xff;
            local_24 &= 0xf00;
            if local_24 == 0x100 {
                let local_1c: i32 = 0xc4;
                let local_20: i32 = 0xca;
                let mut local_10: i32 = mech_trace::rd!(
                    "100340f0",
                    ((local_c.wrapping_add(0xf0)) as usize as *const i32),
                    read_unaligned
                );
                let mut local_14: i32 = mech_trace::rd!(
                    "100340f0",
                    ((local_c.wrapping_add(0xf4)) as usize as *const i32),
                    read_unaligned
                );
                let mut local_18: i32 = mech_trace::rd!(
                    "100340f0",
                    ((local_c.wrapping_add(0xf8)) as usize as *const i32),
                    read_unaligned
                );
                let i_var1 = mech_trace::cr!(
                    "100340f0",
                    "1003ba89",
                    FUN_1003ba89(&raw mut local_10, &raw mut local_14, &raw mut local_18)
                );
                if i_var1 == 0 {
                    let mut loc: [i32; 2] = [local_10, local_14];
                    let p = loc.as_mut_ptr();
                    mech_trace::cr!("100340f0", "10066f36", FUN_10066f36(0x101f52e0, p, p));
                    mech_trace::cr!(
                        "100340f0",
                        "1003510c",
                        FUN_1003510c(loc[0], loc[1], local_20)
                    );
                } else {
                    mech_trace::cr!(
                        "100340f0",
                        "1003510c",
                        FUN_1003510c(local_10, local_14, local_1c)
                    );
                }
            } else if local_24 == 0x200 {
                let i_var1 = mech_trace::cr!("100340f0", "1004d5e3", FUN_1004d5e3(local_8 as i32));
                let d = mech_trace::rd!(
                    "100340f0",
                    (&raw mut DAT_100eef40).cast::<u32>().add(local_8 as usize),
                    read_unaligned
                );
                mech_trace::cr!(
                    "100340f0",
                    "10034c0a",
                    FUN_10034c0a(d as i32, i_var1 as i32)
                );
            } else if local_24 == 0x400 {
                let i_var1 = mech_trace::cr!("100340f0", "1004d60d", FUN_1004d60d(local_8 as i32));
                let i_var2 = mech_trace::cr!("100340f0", "10083808", FUN_10083808());
                mech_trace::cr!(
                    "100340f0",
                    "10034e74",
                    FUN_10034e74(local_8 as i32, i_var2 as i32, i_var1 as i32)
                );
            }
        }
    }
}
