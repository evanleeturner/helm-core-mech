// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b04b8: u32;
    static mut DAT_100b04c0: u32;
    static mut DAT_100b04c4: u32;
    static mut DAT_100c7cf0: u32;
    static mut DAT_100c7cf4: u32;
    static mut DAT_100c7cf8: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10046563() -> u32;
    fn FUN_100463c4();
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10046824(param_1: *mut u32);
    fn FUN_100463ee(param_1: u32, param_2: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10046a29(param_1: *mut i32, param_2: i32) {
    // SAFETY: the globals and callees are the game's own; param_1 points at
    // a two-int range and the table address comes from the game's globals.
    unsafe {
        if DAT_100b04c4 != 0 {
            let i_var1 = FUN_10046563() as i32;
            if i_var1 == 3 || i_var1 == 4 {
                let mut local_14: [u32; 4] = [0; 4];
                FUN_10046824(local_14.as_mut_ptr());
                let i_var2 = (local_14[0] as i32).wrapping_add(param_2);
                if param_1.read_unaligned() <= i_var2
                    && i_var2 <= param_1.byte_add(4).read_unaligned()
                {
                    let addr = DAT_100b04c0.wrapping_add(
                        (i_var2.wrapping_sub(DAT_100b04b8 as i32) as u32).wrapping_mul(4),
                    );
                    if i_var1 == 3 {
                        FUN_100463c4();
                        FUN_100463ee((addr as *const u32).read_unaligned(), 0);
                    } else if i_var1 == 4 {
                        FUN_100463ee((addr as *const u32).read_unaligned(), 0);
                        FUN_100463c4();
                        DAT_100c7cf4 = 0;
                        DAT_100c7cf8 = 0;
                        DAT_100c7cf0 = i_var2 as u32;
                    }
                }
            }
        }
    }
}
