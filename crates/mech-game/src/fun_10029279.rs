// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10041cb0(param_1: *mut i32, param_2: u16);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10029279(param_1: *mut i32, param_2: u16) {
    mech_trace::enter!("10029279", param_1, param_2);
    // SAFETY: param_1 points at the caller's int array whose element 0x13 is a
    // pointer to a record; the bytes at offsets 0x25 and 0x27 are written as in
    // the original, and the callee is the game's own.
    unsafe {
        let p = mech_trace::rd!("10029279", param_1.add(0x13), read_unaligned) as usize as *mut u8;
        mech_trace::wr!("10029279", p.byte_add(0x25), write, 0);
        let p = mech_trace::rd!("10029279", param_1.add(0x13), read_unaligned) as usize as *mut u8;
        mech_trace::wr!("10029279", p.byte_add(0x27), write, 0);
        FUN_10041cb0(param_1, param_2);
    }
}
