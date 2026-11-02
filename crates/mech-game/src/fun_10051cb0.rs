// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b09dc: u32;
    static mut DAT_101f52c8: *mut u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10064ad0(
        param_1: *mut i8,
        param_2: u32,
        param_3: u32,
        param_4: u32,
        param_5: u32,
    ) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10051cb0(param_1: *mut i8) -> u32 {
    let mut local_8: u32 = 1;
    // SAFETY: the globals and the callee are the game's own; DAT_101f52c8
    // points at an array of at least four u32 values.
    unsafe {
        if !param_1.is_null() && DAT_100b09dc == 3 {
            let p = DAT_101f52c8;
            let a = p.read_unaligned();
            let b = p.byte_add(4).read_unaligned();
            let c = p.byte_add(12).read_unaligned();
            local_8 = FUN_10064ad0(param_1, a, b, c, 6);
            DAT_100b09dc = 0;
        }
    }
    local_8
}
