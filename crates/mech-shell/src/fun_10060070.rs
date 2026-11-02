// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100d08e8: i32;
    static mut DAT_100a7248: *mut i32;
    static mut DAT_100d08cc: i32;
    static mut DAT_100d08e0: i32;
    static mut DAT_100d08e4: i32;
    static mut DAT_100c5c20: i32;
    static mut DAT_100c5c24: i32;
    static mut DAT_100c5c28: i32;
    static mut DAT_100c5c2c: i32;
    static mut DAT_10098208: i32;
    static mut DAT_100c605c: i32;
    static mut DAT_100c6060: i32;
    static mut DAT_100a757c: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100406b0(param_1: i32) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn SetWindowLongA(hWnd: i32, nIndex: i32, dwNewLong: i32) -> i32;
    fn timeGetTime() -> u32;
    fn SetWindowPos(
        hWnd: i32,
        hWndInsertAfter: i32,
        X: i32,
        Y: i32,
        cx: i32,
        cy: i32,
        uFlags: u32,
    ) -> i32;
    fn ShowCursor(bShow: i32) -> i32;
    fn GetSystemMetrics(nIndex: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10060070(param_1: i32) {
    if param_1 == 0 {
        return;
    }
    // SAFETY: param_1 points at the caller's structure; the globals and the
    // callees are the game's own.
    unsafe {
        let p = param_1 as usize as *const u8;
        let mut u_var1: u32 = p.byte_add(8).cast::<u32>().read_unaligned();
        if p.byte_add(4).cast::<i32>().read_unaligned() != 2 {
            u_var1 &= 0xfff7ffff;
        }
        SetWindowLongA(DAT_100d08e8, -0x10, (u_var1 | 0x10000000) as i32);
        if p.byte_add(4).cast::<i32>().read_unaligned() == 2 {
            if !DAT_100a7248.is_null() && DAT_100a7248.read_unaligned() == 0 {
                DAT_100d08cc = timeGetTime() as i32;
                DAT_100d08e0 = 1;
                DAT_100d08e4 = DAT_100d08cc.wrapping_add(3000);
            }
            SetWindowPos(
                DAT_100d08e8,
                -2,
                DAT_100c5c20,
                DAT_100c5c24,
                DAT_100c5c28,
                DAT_100c5c2c,
                0x10,
            );
            if DAT_10098208 != 0 && FUN_100406b0(DAT_100d08e8) == 0 {
                let mut i_var2 = ShowCursor(1);
                while i_var2 < 0 {
                    i_var2 = ShowCursor(1);
                }
            }
        } else if DAT_10098208 != 0 {
            loop {
                let i_var2 = ShowCursor(0);
                if i_var2 < 0 {
                    break;
                }
            }
        }
        let i_var2 = GetSystemMetrics(0);
        if i_var2 <= DAT_100c605c {
            let i_var2 = GetSystemMetrics(1);
            if i_var2 <= DAT_100c6060 {
                DAT_100a757c = 1;
                return;
            }
        }
        DAT_100a757c = p.byte_add(4).cast::<i32>().read_unaligned();
    }
}
