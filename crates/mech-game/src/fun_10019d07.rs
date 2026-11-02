// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aafd4: unsafe extern "C" fn(i32, *mut u32, u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10019d07(
    param_1: u32,
    param_2: u32,
    param_3: u32,
    param_4: u32,
    param_5: u32,
    param_6: u32,
    param_7: u32,
    param_8: u32,
    param_9: u32,
) {
    mech_trace::enter!(
        "10019d07", param_1, param_2, param_3, param_4, param_5, param_6, param_7, param_8, param_9
    );
    let mut local_64: [u32; 23] = [0; 23];
    local_64[3] = param_1;
    local_64[0] = param_1;
    local_64[4] = param_2;
    local_64[1] = param_2;
    local_64[9] = param_3;
    local_64[6] = param_3;
    local_64[10] = param_4;
    local_64[7] = param_4;
    local_64[15] = param_5;
    local_64[12] = param_5;
    local_64[16] = param_6;
    local_64[13] = param_6;
    local_64[21] = param_7;
    local_64[18] = param_7;
    local_64[22] = param_8;
    local_64[19] = param_8;
    // SAFETY: the function pointer global is the game's own; the buffer is a
    // live local passed to the callee as in the original.
    unsafe {
        let f = mech_trace::gr!("10019d07", "100aafd4", DAT_100aafd4);
        mech_trace::cr!("10019d07", "ind", f(4, local_64.as_mut_ptr(), param_9));
    }
}
