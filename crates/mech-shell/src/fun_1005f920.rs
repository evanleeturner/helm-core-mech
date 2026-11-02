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
    mech_trace::enter!(
        "1005f920", param_1, param_2, param_3, param_4, param_5, param_6
    );
    // SAFETY: all globals and callees belong to the game; the tables and the
    // records they point to are read and written exactly where the original does.
    unsafe {
        let tbl30 = (&raw const DAT_100a7230).cast::<*mut i32>();
        let tbl20 = (&raw const DAT_100a7220).cast::<*mut i32>();
        let tbl34 = (&raw const DAT_100a7234).cast::<*mut i32>();

        let mut pi_var4: *mut i32 = mech_trace::rd!("1005f920", tbl30, read);
        if param_1 != -1 {
            if !(0..=5).contains(&param_1) {
                return mech_trace::rt!("1005f920", 0);
            }
            pi_var4 = mech_trace::rd!("1005f920", tbl30.add(param_1 as usize), read);
        }
        mech_trace::gw!("1005f920", "100a7254", DAT_100a7254, param_2);
        let pi_var1: *mut i32 = mech_trace::rd!(
            "1005f920",
            tbl20.offset(mech_trace::rd!("1005f920", pi_var4.add(1), read) as isize),
            read
        );
        let i_var2 = mech_trace::cr!("1005f920", "GetSystemMetrics", GetSystemMetrics(0));
        let i_var3 = mech_trace::cr!("1005f920", "GetSystemMetrics", GetSystemMetrics(1));
        mech_trace::gw!("1005f920", "100c5c20", DAT_100c5c20, 0);
        mech_trace::gw!("1005f920", "100c5c24", DAT_100c5c24, 0);
        mech_trace::gw!("1005f920", "100c5c28", DAT_100c5c28, param_4);
        mech_trace::gw!("1005f920", "100c5c2c", DAT_100c5c2c, param_5);
        if param_4 < i_var2 || param_5 < i_var3 {
            mech_trace::gw!("1005f920", "1008d980", DAT_1008d980, 0xca0000);
            mech_trace::cr!(
                "1005f920",
                "AdjustWindowRect",
                AdjustWindowRect((&raw mut DAT_100c5c20).cast::<c_void>(), 0xca0000, param_6)
            );
            mech_trace::gw!(
                "1005f920",
                "100c5c28",
                DAT_100c5c28,
                mech_trace::gr!("1005f920", "100c5c28", DAT_100c5c28)
                    .wrapping_sub(mech_trace::gr!("1005f920", "100c5c20", DAT_100c5c20))
            );
            mech_trace::gw!(
                "1005f920",
                "100c5c2c",
                DAT_100c5c2c,
                mech_trace::gr!("1005f920", "100c5c2c", DAT_100c5c2c)
                    .wrapping_sub(mech_trace::gr!("1005f920", "100c5c24", DAT_100c5c24))
            );
            mech_trace::gw!(
                "1005f920",
                "100c5c24",
                DAT_100c5c24,
                i_var3.wrapping_sub(mech_trace::gr!("1005f920", "100c5c2c", DAT_100c5c2c)) / 2
            );
            mech_trace::gw!(
                "1005f920",
                "100c5c20",
                DAT_100c5c20,
                i_var2.wrapping_sub(mech_trace::gr!("1005f920", "100c5c28", DAT_100c5c28)) / 2
            );
            mech_trace::gw!(
                "1005f920",
                "100a757c",
                DAT_100a757c,
                mech_trace::rd!("1005f920", pi_var1.add(1), read)
            );
        } else {
            mech_trace::gw!("1005f920", "1008d980", DAT_1008d980, 0x80000000);
            mech_trace::gw!("1005f920", "100a757c", DAT_100a757c, 1);
        }
        if mech_trace::rd!("1005f920", pi_var1.add(1), read) == 1 {
            mech_trace::cr!("1005f920", "GetSystemMetrics", GetSystemMetrics(0));
            mech_trace::cr!("1005f920", "GetSystemMetrics", GetSystemMetrics(1));
        }
        if mech_trace::gr!("1005f920", "100a7580", DAT_100a7580) == 0 {
            if pi_var4 != mech_trace::gr!("1005f920", "100a724c", DAT_100a724c)
                || mech_trace::gr!("1005f920", "100c605c", DAT_100c605c) != param_4
                || mech_trace::gr!("1005f920", "100c6060", DAT_100c6060) != param_5
            {
                FUN_10060070(pi_var1 as i32);
                let f: unsafe extern "C" fn() =
                    core::mem::transmute::<usize, unsafe extern "C" fn()>(mech_trace::rd!(
                        "1005f920",
                        mech_trace::gr!("1005f920", "100a724c", DAT_100a724c)
                            .add(5)
                            .cast::<usize>(),
                        read
                    ));
                mech_trace::cr!("1005f920", "ind", f());
            }
        } else {
            mech_trace::gw!("1005f920", "100c605c", DAT_100c605c, param_4);
            mech_trace::gw!("1005f920", "100c6060", DAT_100c6060, param_5);
            FUN_10060070(pi_var1 as i32);
            mech_trace::gw!("1005f920", "100a7248", DAT_100a7248, pi_var1);
        }
        mech_trace::gw!(
            "1005f920",
            "100c6058",
            DAT_100c6058,
            param_5.wrapping_mul(param_4)
        );
        let mut i_var2 = mech_trace::rd!("1005f920", pi_var4.add(2), read);
        mech_trace::gw!("1005f920", "100c605c", DAT_100c605c, param_4);
        mech_trace::gw!("1005f920", "100c6060", DAT_100c6060, param_5);
        mech_trace::gw!("1005f920", "100a725c", DAT_100a725c, param_3);
        mech_trace::gw!("1005f920", "100a724c", DAT_100a724c, pi_var4);
        loop {
            let mut ok = i_var2 == 0;
            if !ok {
                let f = core::mem::transmute::<usize, unsafe extern "C" fn(u32, i32, i32) -> i32>(
                    mech_trace::rd!(
                        "1005f920",
                        mech_trace::gr!("1005f920", "100a724c", DAT_100a724c)
                            .add(4)
                            .cast::<usize>(),
                        read
                    ),
                );
                i_var2 = mech_trace::cr!("1005f920", "ind", f(param_3, param_4, param_5));
                ok = i_var2 == 0;
            }
            if ok {
                if mech_trace::gr!("1005f920", "100a7580", DAT_100a7580) != 0
                    && mech_trace::rd!("1005f920", pi_var1, read) != 2
                {
                    mech_trace::cr!(
                        "1005f920",
                        "ShowWindow",
                        ShowWindow(mech_trace::gr!("1005f920", "100d08e8", DAT_100d08e8), 10)
                    );
                    mech_trace::cr!(
                        "1005f920",
                        "UpdateWindow",
                        UpdateWindow(mech_trace::gr!("1005f920", "100d08e8", DAT_100d08e8))
                    );
                }
                mech_trace::gw!("1005f920", "100a7580", DAT_100a7580, 0);
                return mech_trace::rt!("1005f920", 1);
            }
            FUN_100640a0(
                (&raw mut DAT_100a7594).cast::<i8>(),
                mech_trace::rd!(
                    "1005f920",
                    mech_trace::gr!("1005f920", "100a724c", DAT_100a724c),
                    read
                ),
            );
            mech_trace::wr!(
                "1005f920",
                mech_trace::gr!("1005f920", "100a724c", DAT_100a724c).add(2),
                write,
                0
            );
            if mech_trace::gr!("1005f920", "100a7254", DAT_100a7254) == 0
                || mech_trace::rd!(
                    "1005f920",
                    mech_trace::gr!("1005f920", "100a724c", DAT_100a724c),
                    read
                ) == 5
            {
                break;
            }
            mech_trace::gw!(
                "1005f920",
                "100a724c",
                DAT_100a724c,
                mech_trace::rd!(
                    "1005f920",
                    tbl34.offset(mech_trace::rd!(
                        "1005f920",
                        mech_trace::gr!("1005f920", "100a724c", DAT_100a724c),
                        read
                    ) as isize),
                    read
                )
            );
            i_var2 = mech_trace::rd!(
                "1005f920",
                mech_trace::gr!("1005f920", "100a724c", DAT_100a724c).add(2),
                read
            );
        }
        mech_trace::rt!("1005f920", 0)
    }
}
