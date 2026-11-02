// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004318b(param_1: i32, param_2: i32) -> u32 {
    mech_trace::enter!("1004318b", param_1, param_2);
    let mut local_8: u32 = 0;
    if param_2 > 0x50000 || param_2 < -0x50000 {
        local_8 = 1;
        // SAFETY: param_1 points at the game's object; its field at 0x4c is a
        // pointer to a structure whose dword at offset 8 is read and written.
        unsafe {
            let p = mech_trace::rd!(
                "1004318b",
                (param_1.wrapping_add(0x4c) as *const i32),
                read_unaligned
            );
            let q = p.wrapping_add(8) as *mut i32;
            if mech_trace::rd!("1004318b", q, read_unaligned) == 0 {
                mech_trace::wr!("1004318b", q, write_unaligned, 0x66);
            }
        }
    }
    mech_trace::rt!("1004318b", local_8)
}
