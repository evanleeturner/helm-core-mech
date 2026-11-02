// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf74: *mut core::ffi::c_void;
    static mut DAT_100ba7bc: *mut core::ffi::c_void;
    static mut DAT_100c9050: u32;
    static mut DAT_100c9054: u32;
    static mut DAT_100c9058: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10086336(param_1: *mut u16) -> u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapFree(h: *mut core::ffi::c_void, flags: u32, mem: *mut core::ffi::c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100861a3() {
    mech_trace::enter!("100861a3");
    // SAFETY: the globals and callees are the game's own; the table holds
    // 0x3f1 pointers to linked nodes whose next pointer sits at offset 8.
    unsafe {
        if !mech_trace::gr!("100861a3", "100ba7bc", DAT_100ba7bc).is_null() {
            let mut local_c: i32 = 0;
            while local_c < 0x3f1 {
                let mut local_8: *mut u16 = mech_trace::rd!(
                    "100861a3",
                    mech_trace::gr!("100861a3", "100ba7bc", DAT_100ba7bc)
                        .cast::<u8>()
                        .byte_add((local_c as usize).wrapping_mul(4))
                        .cast::<*mut u16>(),
                    read_unaligned
                );
                while !local_8.is_null() {
                    let p_var1: *mut u16 = mech_trace::rd!(
                        "100861a3",
                        local_8.add(4).cast::<*mut u16>(),
                        read_unaligned
                    );
                    mech_trace::cr!("100861a3", "10086336", FUN_10086336(local_8));
                    local_8 = p_var1;
                }
                local_c = local_c.wrapping_add(1);
            }
            mech_trace::gw!("100861a3", "100c9050", DAT_100c9050, 0);
            mech_trace::gw!("100861a3", "100c9054", DAT_100c9054, 0);
            mech_trace::gw!("100861a3", "100c9058", DAT_100c9058, 0);
            mech_trace::cr!(
                "100861a3",
                "HeapFree",
                HeapFree(
                    mech_trace::gr!("100861a3", "100adf74", DAT_100adf74),
                    1,
                    mech_trace::gr!("100861a3", "100ba7bc", DAT_100ba7bc)
                )
            );
        }
    }
}
