// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10098204: u32;
    static mut DAT_100a7248: *mut u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10067af3(
        param_1: *mut i32,
        pos_x: i32,
        pos_y: i32,
        width: *mut i32,
        height: i32,
        param_6: i32,
        param_7: i32,
    );
}

#[allow(non_snake_case)]
unsafe extern "thiscall" {
    fn FUN_100124d0(this: *mut u8, pos_x: i32, pos_y: i32, width: i32, height: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "thiscall" fn FUN_10012ca0(
    this: *mut u8,
    param_1: u32,
    pos_x: i32,
    pos_y: i32,
    width: i32,
    height: i32,
) {
    mech_trace::enter!("10012ca0", this, param_1, pos_x, pos_y, width, height);
    let mut record: [u32; 5] = [
        param_1,
        0,
        0,
        width.wrapping_sub(1) as u32,
        height.wrapping_sub(1) as u32,
    ];
    let mut record_ptr: *mut u32 = record.as_mut_ptr();
    let mut gate_result: i32 = -1;
    // SAFETY: the globals, callees and vtable-like table are the game's own;
    // `this` points at the caller's object, and record_ptr points at a live local.
    unsafe {
        if mech_trace::gr!("10012ca0", "10098204", DAT_10098204) != 0 {
            let f = mech_trace::rd!(
                "10012ca0",
                mech_trace::gr!("10012ca0", "100a7248", DAT_100a7248)
                    .byte_add(0x20)
                    .cast::<unsafe extern "C" fn() -> i32>(),
                read_unaligned
            );
            gate_result = mech_trace::cr!("10012ca0", "ind", f());
        }
        if gate_result == 0 {
            FUN_10067af3(
                (&raw mut record_ptr).cast::<i32>(),
                0,
                0,
                this.byte_add(0x56).cast::<i32>(),
                pos_x,
                pos_y,
                -1,
            );
        }
        FUN_100124d0(this, pos_x, pos_y, width, height);
    }
}
