// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7230: [*mut i32; 6];
    static mut DAT_100a7220: [*mut i32; 1];
    static mut DAT_100a7234: [*mut i32; 1];
    static mut DAT_100a7254: i32;
    static mut DAT_100c5c20: i32;
    static mut DAT_100c5c24: i32;
    static mut DAT_100c5c28: i32;
    static mut DAT_100c5c2c: i32;
    static mut DAT_1008d980: u32;
    static mut DAT_100a757c: i32;
    static mut DAT_100a7580: i32;
    static mut DAT_100a724c: *mut i32;
    static mut DAT_100c605c: i32;
    static mut DAT_100c6060: i32;
    static mut DAT_100c6058: i32;
    static mut DAT_100a7248: *mut i32;
    static mut DAT_100a725c: u32;
    static mut DAT_100d08e8: *mut c_void;
    static mut DAT_100a7594: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn GetSystemMetrics(nIndex: i32) -> i32;
    fn AdjustWindowRect(lpRect: *mut c_void, dwStyle: u32, bMenu: i32) -> i32;
    fn ShowWindow(hWnd: *mut c_void, nCmdShow: i32) -> i32;
    fn UpdateWindow(hWnd: *mut c_void) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10060070(param_1: i32);
    fn FUN_100640a0(param_1: *mut i8, param_2: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005f920(
    param_1: i32,
    param_2: i32,
    param_3: u32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
) -> u32 {
    // SAFETY: all globals and callees belong to the game; the tables and the
    // records they point to are read and written exactly where the original does.
    unsafe {
        let tbl30 = (&raw const DAT_100a7230).cast::<*mut i32>();
        let tbl20 = (&raw const DAT_100a7220).cast::<*mut i32>();
        let tbl34 = (&raw const DAT_100a7234).cast::<*mut i32>();

        let mut pi_var4: *mut i32 = tbl30.read();
        if param_1 != -1 {
            if !(0..=5).contains(&param_1) {
                return 0;
            }
            pi_var4 = tbl30.add(param_1 as usize).read();
        }
        DAT_100a7254 = param_2;
        let pi_var1: *mut i32 = tbl20.offset(pi_var4.add(1).read() as isize).read();
        let i_var2 = GetSystemMetrics(0);
        let i_var3 = GetSystemMetrics(1);
        DAT_100c5c20 = 0;
        DAT_100c5c24 = 0;
        DAT_100c5c28 = param_4;
        DAT_100c5c2c = param_5;
        if param_4 < i_var2 || param_5 < i_var3 {
            DAT_1008d980 = 0xca0000;
            AdjustWindowRect((&raw mut DAT_100c5c20).cast::<c_void>(), 0xca0000, param_6);
            DAT_100c5c28 = DAT_100c5c28.wrapping_sub(DAT_100c5c20);
            DAT_100c5c2c = DAT_100c5c2c.wrapping_sub(DAT_100c5c24);
            DAT_100c5c24 = i_var3.wrapping_sub(DAT_100c5c2c) / 2;
            DAT_100c5c20 = i_var2.wrapping_sub(DAT_100c5c28) / 2;
            DAT_100a757c = pi_var1.add(1).read();
        } else {
            DAT_1008d980 = 0x80000000;
            DAT_100a757c = 1;
        }
        if pi_var1.add(1).read() == 1 {
            GetSystemMetrics(0);
            GetSystemMetrics(1);
        }
        if DAT_100a7580 == 0 {
            if pi_var4 != DAT_100a724c || DAT_100c605c != param_4 || DAT_100c6060 != param_5 {
                FUN_10060070(pi_var1 as i32);
                let f: unsafe extern "C" fn() = core::mem::transmute::<usize, unsafe extern "C" fn()>(
                    DAT_100a724c.add(5).cast::<usize>().read(),
                );
                f();
            }
        } else {
            DAT_100c605c = param_4;
            DAT_100c6060 = param_5;
            FUN_10060070(pi_var1 as i32);
            DAT_100a7248 = pi_var1;
        }
        DAT_100c6058 = param_5.wrapping_mul(param_4);
        let mut i_var2 = pi_var4.add(2).read();
        DAT_100c605c = param_4;
        DAT_100c6060 = param_5;
        DAT_100a725c = param_3;
        DAT_100a724c = pi_var4;
        loop {
            let mut ok = i_var2 == 0;
            if !ok {
                let f = core::mem::transmute::<usize, unsafe extern "C" fn(u32, i32, i32) -> i32>(
                    DAT_100a724c.add(4).cast::<usize>().read(),
                );
                i_var2 = f(param_3, param_4, param_5);
                ok = i_var2 == 0;
            }
            if ok {
                if DAT_100a7580 != 0 && pi_var1.read() != 2 {
                    ShowWindow(DAT_100d08e8, 10);
                    UpdateWindow(DAT_100d08e8);
                }
                DAT_100a7580 = 0;
                return 1;
            }
            FUN_100640a0((&raw mut DAT_100a7594).cast::<i8>(), DAT_100a724c.read());
            DAT_100a724c.add(2).write(0);
            if DAT_100a7254 == 0 || DAT_100a724c.read() == 5 {
                break;
            }
            DAT_100a724c = tbl34.offset(DAT_100a724c.read() as isize).read();
            i_var2 = DAT_100a724c.add(2).read();
        }
        0
    }
}
