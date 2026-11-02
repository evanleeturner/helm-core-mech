// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38fc: i32;
    static mut _DAT_100b3950: i32;
    static mut DAT_100b394c: i32;
    static mut DAT_100b3954: i32;
    static mut DAT_100b3948: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10056e8b(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i8,
    );
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10066cf1(param_1: *mut i32) {
    // SAFETY: the globals and the callee are the game's own; param_1 points
    // at the caller's int array, read at indices 3 and 4 as the original does.
    unsafe {
        if 0x59 < DAT_100b38fc.wrapping_sub(_DAT_100b3950) {
            _DAT_100b3950 = DAT_100b38fc;
            DAT_100b394c = DAT_100b394c.wrapping_add(DAT_100b3954);
            if DAT_100b394c != 0xf0 {
                DAT_100b3954 = 1;
            }
            if DAT_100b394c != 0xf7 {
                DAT_100b3954 = DAT_100b3954.wrapping_neg();
            }
        }
        let u_var1: i32 = DAT_100b3948 >> 0x1f;
        DAT_100b3948 = ((((DAT_100b3948 ^ u_var1).wrapping_sub(u_var1)) & 1) ^ u_var1)
            .wrapping_sub(u_var1)
            .wrapping_add(1);
        let a = param_1.byte_add(12).read_unaligned();
        let b = param_1.byte_add(16).read_unaligned();
        FUN_10056e8b(
            param_1,
            DAT_100b3948,
            DAT_100b3948,
            a.wrapping_sub(DAT_100b3948),
            b.wrapping_sub(DAT_100b3948),
            DAT_100b394c as i8,
        );
    }
}
