// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7560: u32;
    static mut DAT_100a7564: u32;
    static mut DAT_100a7570: u32;
    static mut DAT_100a7574: u32;
    static mut DAT_100a757c: u32;
    static mut DAT_100a7580: u32;
    static mut DAT_101f5510: u32;
    static mut DAT_10215624: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1000c73b();
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002ce10(param_1: i32) -> u32;
    fn FUN_10086951(param_1: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1000c550() -> u32 {
    let mut local_8: u32 = 0;
    // SAFETY: the callees and the globals are the game's own memory; they are
    // read and written in the same order as the original.
    unsafe {
        FUN_1000c73b();
        let u_var1 = FUN_1002ce10(7);
        if u_var1 != 0 {
            DAT_100a757c = u_var1 >> 3;
            DAT_100a7580 = FUN_10086951(u_var1);
            if DAT_100a7580 != 0 {
                let u_var1 = FUN_1002ce10(8);
                if u_var1 != 0 {
                    DAT_101f5510 = u_var1 >> 2;
                    DAT_10215624 = DAT_101f5510;
                    DAT_100a7570 = FUN_10086951(u_var1);
                    if DAT_100a7570 != 0 {
                        DAT_100a7560 = FUN_10086951(u_var1);
                        if DAT_100a7560 != 0 {
                            let s_var2 = FUN_1002ce10(9);
                            if s_var2 != 0 {
                                DAT_100a7574 = FUN_10086951(s_var2);
                                if DAT_100a7574 != 0 {
                                    DAT_100a7564 = FUN_10086951(s_var2);
                                    if DAT_100a7564 != 0 {
                                        local_8 = 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    local_8
}
