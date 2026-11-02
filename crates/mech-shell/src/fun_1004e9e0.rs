// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10083508: u32;
    static mut DAT_1009b368: u32;
    static mut DAT_100a6838: u32;
    static mut DAT_100d08e8: *mut core::ffi::c_void;
    static mut DAT_1009bdc0: u8;
    static mut DAT_1009bdc8: u8;
    static mut DAT_1009bdd0: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1005bbb0(param_1: *mut i8) -> *mut i8;
    fn FUN_1000e970(param_1: *mut u8, param_2: u32, param_3: u32) -> u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn PostMessageA(hwnd: *mut core::ffi::c_void, msg: u32, wparam: u32, lparam: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004e9e0(param_1: *mut u32) {
    // SAFETY: globals and callees are the game's own; DAT_10083508 holds the
    // address of the game's state block, and param_1 points at the caller's UINT.
    unsafe {
        let rd =
            |off: u32| -> i32 { (DAT_10083508.wrapping_add(off) as *const i32).read_unaligned() };
        let rb = |off: u32| -> u8 { (DAT_10083508.wrapping_add(off) as *const u8).read() };

        if rd(0x23bb) != 0 {
            if rd(0x23bb) != 1 {
                return;
            }
            if 1 < rd(0x1fe) && rd(0x1fe) < 4 {
                let mut p = FUN_1005bbb0(DAT_10083508.wrapping_add(0x2a2) as *mut i8) as *const u8;
                let mut q = &raw mut DAT_1009bdc0 as *const u8;
                let mut equal = true;
                loop {
                    let a = p.read();
                    let b = q.read();
                    if a != b {
                        equal = false;
                        break;
                    }
                    if a == 0 {
                        break;
                    }
                    p = p.add(1);
                    q = q.add(1);
                }
                if equal && rb(0x239c) == 0 {
                    param_1.write(0xc02b);
                    return;
                }
                if DAT_100a6838 != 1 {
                    PostMessageA(DAT_100d08e8, param_1.read(), 0xc00a, 0);
                    return;
                }
                DAT_1009b368 = 0;
                FUN_1000e970(&raw mut DAT_1009bdc8, param_1.read(), 0xc00a);
                return;
            }
        }
        if 1 < rd(0x1fe) && rd(0x1fe) < 4 && rb(0x239c) == 1 {
            if DAT_100a6838 == 1 {
                DAT_1009b368 = 0;
                FUN_1000e970(&raw mut DAT_1009bdd0, param_1.read(), 0xc00a);
                return;
            }
            PostMessageA(DAT_100d08e8, param_1.read(), 0xc00a, 0);
        }
    }
}
