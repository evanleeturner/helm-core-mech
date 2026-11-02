// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a67a4: *mut i32;
    static mut DAT_100a67e4: *mut i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10018060(param_1: i32, param_2: i32) -> u32;
    fn FUN_10017df0(param_1: *mut i32, param_2: u32) -> u32;
    fn FUN_100180f0(param_1: *mut i32, param_2: i8);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1000be10();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1000c250() {
    mech_trace::enter!("1000c250");
    // SAFETY: the globals and callees are the game's own; the collection
    // pointers and the items they return are valid game memory.
    unsafe {
        let mut i_var1: i32 = 0;
        if 0 < mech_trace::rd!(
            "1000c250",
            mech_trace::gr!("1000c250", "100a67e4", DAT_100a67e4).byte_add(4),
            read_unaligned
        ) {
            loop {
                let i_var3 = i_var1.wrapping_add(1);
                i_var1 = mech_trace::cr!(
                    "1000c250",
                    "10018060",
                    FUN_10018060(
                        mech_trace::gr!("1000c250", "100a67e4", DAT_100a67e4) as i32,
                        i_var1
                    )
                ) as i32;
                FUN_1000be10();
                let u_var2: u32;
                core::arch::asm!("", out("eax") u_var2, options(nomem, nostack, preserves_flags));
                (i_var1 as usize + 0x24)..;
                i_var1 = i_var3;
                if !(i_var3
                    < mech_trace::rd!(
                        "1000c250",
                        mech_trace::gr!("1000c250", "100a67e4", DAT_100a67e4).byte_add(4),
                        read_unaligned
                    ))
                {
                    break;
                }
            }
        }
        FUN_100180f0(mech_trace::gr!("1000c250", "100a67e4", DAT_100a67e4), 0);
    }
}
