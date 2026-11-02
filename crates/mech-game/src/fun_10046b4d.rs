// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b04b8: i32;
    static mut DAT_100b04bc: i32;
    static mut DAT_100b04c0: u32;
    static mut DAT_100b04c4: u32;
    static mut DAT_100c7cf0: u32;
    static mut DAT_100c7cf4: u32;
    static mut DAT_100c7cf8: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10046563() -> u32;
    #[link_name = "FUN_100463c4"]
    fn StopMciDevice();
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10046824(param_1: *mut u32);
    #[link_name = "FUN_100463ee"]
    fn PlayMciDeviceFromTo(param_1: u32, param_2: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10046b4d(param_1: i32) {
    let mut local_14: [u32; 4] = [0; 4];
    // SAFETY: the globals and callees are the game's own; the table at
    // DAT_100b04c0 is read at the same offsets as the original.
    unsafe {
        if DAT_100b04c4 == 0 {
            return;
        }
        let i_var1 = FUN_10046563() as i32;
        if i_var1 == 1 || !(1 < i_var1) || !(i_var1 < 5) {
            return;
        }
        FUN_10046824(local_14.as_mut_ptr());
        if !(DAT_100b04b8 <= param_1 && param_1 <= DAT_100b04bc) {
            return;
        }
        if i_var1 == 3 {
            StopMciDevice();
        }
        let a = DAT_100b04c0.wrapping_add(param_1.wrapping_sub(DAT_100b04b8).wrapping_mul(4) as u32)
            as *const u32;
        let a_val = a.read_unaligned();
        let b = DAT_100b04c0.wrapping_add(
            param_1
                .wrapping_add(1)
                .wrapping_sub(DAT_100b04b8)
                .wrapping_mul(4) as u32,
        ) as *const i32;
        let b_val = b.read_unaligned();
        PlayMciDeviceFromTo(a_val, b_val);
        if i_var1 == 4 {
            DAT_100c7cf0 = param_1 as u32;
            DAT_100c7cf4 = 0;
            DAT_100c7cf8 = 0;
        }
    }
}
