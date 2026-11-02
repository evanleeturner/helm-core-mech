// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf38: i32;
    static mut DAT_100eef40: i32;
    static mut DAT_100aa598: i32;
    static mut DAT_100aa59c: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10056489(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
        param_7: i32,
    );
    fn FUN_10067d51(param_1: *mut i32) -> i32;
    fn FUN_10011e60(param_1: i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_10016b61(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
    );
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100169fb(param_1: i32) {
    // SAFETY: param_1 is the address of the caller's structure; the globals
    // and callees are the game's own, and reads and calls keep the original order.
    unsafe {
        let rd = |off: i32| -> i32 { ((param_1.wrapping_add(off)) as *const i32).read_unaligned() };
        let pi_var1 = rd(0xa4) as *mut i32;
        let table = (&raw mut DAT_100eef40).add(DAT_100adf38 as usize);
        let mut i_var2 = (table.read().wrapping_add(0x20) as *const i32).read_unaligned();
        FUN_10056489(pi_var1, rd(0x64), rd(0x68), rd(0x64), rd(0x70), 0, 10);
        FUN_10056489(pi_var1, rd(0x6c), rd(0x68), rd(0x6c), rd(0x70), 0, 10);
        FUN_10056489(pi_var1, rd(0x64), rd(0x68), rd(0x6c), rd(0x68), 0, 10);
        FUN_10056489(pi_var1, rd(0x64), rd(0x70), rd(0x6c), rd(0x70), 0, 10);
        let v = ((i_var2.wrapping_add(0x3f4)) as *const i32).read_unaligned();
        if -1 < v {
            DAT_100aa59c = ((i_var2.wrapping_add(0x3f4)) as *const i32).read_unaligned();
            i_var2 = FUN_10067d51(&raw mut DAT_100aa598);
            let mut local_8 = FUN_10011e60(rd(0x60).wrapping_add(1), i_var2, 0x712);
            if rd(0x60) < local_8 {
                local_8 = rd(0x60);
            }
            FUN_10016b61(
                pi_var1,
                rd(0x54),
                rd(0x58),
                rd(0x5c).wrapping_sub(1),
                local_8,
                0xb,
            );
        }
    }
}
