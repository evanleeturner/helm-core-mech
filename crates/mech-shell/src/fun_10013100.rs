// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10098204: u32;
    static mut DAT_100a7248: u32;
}

#[allow(non_snake_case)]
unsafe extern "thiscall" {
    fn FUN_10012590(this: *mut u8, param_1: i32, param_2: i32, param_3: i32, param_4: i32) -> u32;
    fn FUN_100124d0(this: *mut u8, param_1: i32, param_2: i32, param_3: i32, param_4: i32);
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10065b64(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
    ) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "thiscall" fn FUN_10013100(
    this: *mut u8,
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
) {
    mech_trace::enter!(
        "10013100", this, param_1, param_2, param_3, param_4, param_5, param_6
    );
    // SAFETY: the callees and globals are the game's own; DAT_100a7248 holds
    // the address of a table whose entry at +0x20 is a callable function pointer.
    unsafe {
        let call_result = mech_trace::cr!(
            "10013100",
            "10012590",
            FUN_10012590(this, param_3, param_4, param_5, param_6)
        );
        if call_result != 0 {
            let mut call_result: i32 = -1;
            if mech_trace::gr!("10013100", "10098204", DAT_10098204) != 0 {
                let f = mech_trace::rd!(
                    "10013100",
                    ((mech_trace::gr!("10013100", "100a7248", DAT_100a7248).wrapping_add(0x20))
                        as *const usize),
                    read_unaligned
                );
                let f: unsafe extern "C" fn() -> i32 = core::mem::transmute(f);
                call_result = mech_trace::cr!("10013100", "ind", f());
            }
            if call_result == 0 {
                mech_trace::cr!(
                    "10013100",
                    "10065b64",
                    FUN_10065b64(
                        this.byte_add(0x56).cast::<i32>(),
                        param_1,
                        param_2,
                        param_3,
                        param_4
                    )
                );
            }
            FUN_100124d0(this, param_3, param_4, param_5, param_6);
        }
    }
}
