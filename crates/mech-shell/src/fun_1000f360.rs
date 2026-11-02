// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000f290(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1000f360() {
    mech_trace::enter!("1000f360");
    let mut slot_index: i32 = 0;
    loop {
        // SAFETY: the callee is the game's own function; it takes a slot index in 0..0x20.
        unsafe {
            FUN_1000f290(slot_index);
        }
        slot_index = slot_index.wrapping_add(1);
        if slot_index >= 0x20 {
            break;
        }
    }
}
