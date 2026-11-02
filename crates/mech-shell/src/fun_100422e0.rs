// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100984cc: *mut core::ffi::c_void;
    static mut DAT_100d08c0: u32;
    static mut DAT_100d08c4: u32;
    static mut DAT_100d08c8: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100423d0(param_1: *mut i16) -> u8;
    fn FUN_1006e790(param_1: *mut core::ffi::c_void) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100422e0() {
    mech_trace::enter!("100422e0");
    // SAFETY: the globals and callees are the game's own; the table at
    // DAT_100984cc holds 0x3f1 list heads whose nodes link through offset 8.
    unsafe {
        if !mech_trace::gr!("100422e0", "100984cc", DAT_100984cc).is_null() {
            let mut i_var3: i32 = 0;
            loop {
                let mut ps_var2: *mut i16 = mech_trace::rd!(
                    "100422e0",
                    mech_trace::gr!("100422e0", "100984cc", DAT_100984cc)
                        .byte_add(i_var3 as usize)
                        .cast::<*mut i16>(),
                    read_unaligned
                );
                while !ps_var2.is_null() {
                    let ps_var1: *mut i16 = mech_trace::rd!(
                        "100422e0",
                        ps_var2.add(4).cast::<*mut i16>(),
                        read_unaligned
                    );
                    mech_trace::cr!("100422e0", "100423d0", FUN_100423d0(ps_var2));
                    ps_var2 = ps_var1;
                }
                i_var3 = i_var3.wrapping_add(4);
                if i_var3 >= 0xfc4 {
                    break;
                }
            }
            mech_trace::gw!("100422e0", "100d08c0", DAT_100d08c0, 0);
            mech_trace::gw!("100422e0", "100d08c4", DAT_100d08c4, 0);
            mech_trace::gw!("100422e0", "100d08c8", DAT_100d08c8, 0);
            mech_trace::cr!(
                "100422e0",
                "1006e790",
                FUN_1006e790(mech_trace::gr!("100422e0", "100984cc", DAT_100984cc))
            );
        }
    }
}
