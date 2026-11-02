// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _strncpy(dest: *mut i8, source: *mut i8, count: usize) -> *mut i8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1006a2a5(source: *mut i8) {
    mech_trace::enter!("1006a2a5", source);
    let mut local_54: [i8; 80] = [0; 80];
    let mut local_58: i32 = 0;
    while local_58 < 0x50 {
        local_54[local_58 as usize] = 0;
        local_58 = local_58.wrapping_add(1);
    }
    local_54[0] = 0x20;
    // SAFETY: local_54 has 80 bytes; writing at most 0x4e bytes from offset 1 stays in bounds;
    // source is the caller's string.
    unsafe {
        mech_trace::cr!(
            "1006a2a5",
            "_strncpy",
            _strncpy(local_54.as_mut_ptr().add(1), source, 0x4e)
        );
    }
    local_58 = 0;
    while local_58 < 0x4f {
        if local_54[local_58 as usize] == 0 {
            local_54[local_58 as usize] = 0x20;
        }
        local_58 = local_58.wrapping_add(1);
    }
}
