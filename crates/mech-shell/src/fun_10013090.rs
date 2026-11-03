// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10098204: u32;
    static mut DAT_100a7248: u32;
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

#[allow(non_snake_case)]
unsafe extern "thiscall" {
    fn FUN_100124d0(this: *mut u8, param_1: i32, param_2: i32, param_3: i32, param_4: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "thiscall" fn FUN_10013090(
    this: *mut u8,
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
) {
    mech_trace::enter!(
        "10013090", this, param_1, param_2, param_3, param_4, param_5, param_6
    );
    // SAFETY: globals, callees and the vtable-like table are the game's own;
    // `this` points at the caller's object, offsets used as the original does.
    unsafe {
        let mut status: i32 = -1;
        if mech_trace::gr!("10013090", "10098204", DAT_10098204) != 0 {
            let f = mech_trace::rd!(
                "10013090",
                ((mech_trace::gr!("10013090", "100a7248", DAT_100a7248).wrapping_add(0x20))
                    as *const u32),
                read_unaligned
            );
            let func: unsafe extern "C" fn() -> i32 = core::mem::transmute(f as usize);
            status = mech_trace::cr!("10013090", "ind", func());
        }
        if status == 0 {
            mech_trace::cr!(
                "10013090",
                "10065b64",
                FUN_10065b64(
                    this.byte_add(0x56).cast::<i32>(),
                    param_1,
                    param_2,
                    param_3,
                    param_4,
                )
            );
        }
        mech_trace::cr!(
            "10013090",
            "100124d0",
            FUN_100124d0(this, param_3, param_4, param_5, param_6)
        );
    }
}
