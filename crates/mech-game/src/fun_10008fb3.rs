// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100af17c: u32;
    static mut DAT_101686f0: u32;
    static mut DAT_100af0b4: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100357e3(param_1: *mut i32) -> u32;
    fn FUN_1008668b(param_1: i32, param_2: u32, param_3: *mut i32) -> *mut u16;
    fn FUN_10056fc4(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
    ) -> u32;
    fn FUN_1008664f(param_1: u32, param_2: *mut i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10008fb3(
    param_1: *mut u32,
    mut param_2: i32,
    param_3: i32,
    _param_4: u32,
    param_5: i32,
) {
    // SAFETY: param_1 points at the caller's object; the globals and callees
    // are the game's own; the callback slot holds a cdecl function pointer.
    unsafe {
        let pi_var1 = param_1.read_unaligned() as *mut i32;
        let mut local_8: i32 = FUN_100357e3(&raw mut param_2) as i32;
        let cb = param_1.add(0x20).read_unaligned();
        if local_8 != 0 && cb != 0 {
            let f: unsafe extern "C" fn(*mut i32, i32, i32) -> i32 =
                core::mem::transmute(cb as usize);
            local_8 = f(pi_var1, param_2, param_3);
        }
        if local_8 != 0 {
            let p_var2 = FUN_1008668b(
                DAT_100af17c as i32,
                (param_5 as u32).wrapping_add(DAT_101686f0),
                DAT_100af0b4 as *mut i32,
            );
            if !p_var2.is_null() {
                FUN_10056fc4(pi_var1, p_var2 as i32, 0, param_2, param_3);
                FUN_1008664f(
                    (param_5 as u32).wrapping_add(DAT_101686f0),
                    DAT_100af0b4 as *mut i32,
                );
            }
        }
    }
}
