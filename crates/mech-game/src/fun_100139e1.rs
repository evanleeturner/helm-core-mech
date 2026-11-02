// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf38: i32;
    static mut DAT_100eef40: [i32; 0];
    static mut DAT_100aa1cc: i32;
    static mut DAT_10165922: i32;
    static mut DAT_100aa2c4: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _sprintf(dst: *mut i8, fmt: *const i8, ...) -> i32;
    fn FUN_1005be24(msg: *const i8, p2: i32, p3: i32, p4: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100139e1(param_1: i32, param_2: i32) {
    mech_trace::enter!("100139e1", param_1, param_2);
    let _ = param_1;
    let mut local_54: [i8; 80] = [0; 80];
    // SAFETY: the globals and callees are the game's own memory and code;
    // local_54 is a valid buffer passed to the original's sprintf and logger.
    unsafe {
        if param_2 != mech_trace::gr!("100139e1", "100adf38", DAT_100adf38) {
            let v = mech_trace::rd!(
                "100139e1",
                (&raw const DAT_100eef40)
                    .cast::<i32>()
                    .offset(param_2 as isize),
                read_unaligned
            );
            mech_trace::cr!(
                "100139e1",
                "_sprintf",
                _sprintf(
                    local_54.as_mut_ptr(),
                    (&raw const DAT_100aa2c4).cast::<i8>(),
                    v.wrapping_add(0x110),
                )
            );
            FUN_1005be24(local_54.as_ptr(), 1, 0x712, 0x50);
            mech_trace::gw!("100139e1", "10165922", DAT_10165922, param_2);
            if mech_trace::gr!("100139e1", "100aa1cc", DAT_100aa1cc) == 0 {
                mech_trace::gw!("100139e1", "100aa1cc", DAT_100aa1cc, 1);
            }
        }
    }
}
