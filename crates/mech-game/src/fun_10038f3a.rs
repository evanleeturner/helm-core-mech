// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c6be0: u8;
    static mut DAT_100c6ba0: u8;
    static mut DAT_100ae5e0: u8;
    static mut DAT_100ae634: u8;
    static mut DAT_100ae66c: u8;
    static mut DAT_100ae680: u8;
    static mut DAT_100ae6d4: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn sprintf(buffer: *mut i8, format: *const i8, ...) -> i32;
    fn strncpy(dest: *mut i8, src: *const i8, count: usize) -> *mut i8;
    fn FUN_10002168(format: *const i8, ...);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RegOpenKeyExA(
        hKey: *mut c_void,
        lpSubKey: *const i8,
        ulOptions: u32,
        samDesired: u32,
        phkResult: *mut *mut c_void,
    ) -> i32;
    fn RegQueryValueExA(
        hKey: *mut c_void,
        lpValueName: *const i8,
        lpReserved: *mut u32,
        lpType: *mut u32,
        lpData: *mut u8,
        lpcbData: *mut u32,
    ) -> i32;
    fn RegCloseKey(hKey: *mut c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10038f3a(
    param_1: i32,
    param_2: i32,
    param_3: *mut i8,
    param_4: usize,
) -> i8 {
    mech_trace::enter!("10038f3a", param_1, param_2, param_3, param_4);
    let mut local_14: *mut c_void = core::ptr::null_mut();
    let mut local_10: [u32; 3] = [0; 3];
    let b_var1: i8;

    // SAFETY: the globals, string tables and callees are the game's own
    // memory and imports; param_3 is the caller's destination buffer and the
    // locals outlive every call that receives their addresses.
    unsafe {
        let buf = &raw mut DAT_100c6be0 as *mut i8;
        let data = &raw mut DAT_100c6ba0;
        mech_trace::cr!(
            "10038f3a",
            "sprintf",
            sprintf(buf, &raw const DAT_100ae5e0 as *const i8, param_2)
        );
        local_10[2] = mech_trace::cr!(
            "10038f3a",
            "RegOpenKeyExA",
            RegOpenKeyExA(0x80000002usize as *mut c_void, buf, 0, 1, &raw mut local_14)
        ) as u32;
        if local_10[2] == 0 {
            mech_trace::cr!(
                "10038f3a",
                "sprintf",
                sprintf(
                    buf,
                    &raw const DAT_100ae66c as *const i8,
                    param_1.wrapping_add(1),
                )
            );
            local_10[1] = 0x40;
            local_10[0] = 1;
            local_10[2] = mech_trace::cr!(
                "10038f3a",
                "RegQueryValueExA",
                RegQueryValueExA(
                    local_14,
                    buf,
                    core::ptr::null_mut(),
                    &raw mut local_10[0],
                    data,
                    &raw mut local_10[1],
                )
            ) as u32;
            if local_10[2] == 0 {
                mech_trace::cr!("10038f3a", "RegCloseKey", RegCloseKey(local_14));
                mech_trace::cr!(
                    "10038f3a",
                    "sprintf",
                    sprintf(buf, &raw const DAT_100ae680 as *const i8, data)
                );
                local_10[2] = mech_trace::cr!(
                    "10038f3a",
                    "RegOpenKeyExA",
                    RegOpenKeyExA(0x80000002usize as *mut c_void, buf, 0, 1, &raw mut local_14)
                ) as u32;
                if local_10[2] == 0 {
                    local_10[1] = 0x40;
                    local_10[2] = mech_trace::cr!(
                        "10038f3a",
                        "RegQueryValueExA",
                        RegQueryValueExA(
                            local_14,
                            &raw const DAT_100ae6d4 as *const i8,
                            core::ptr::null_mut(),
                            &raw mut local_10[0],
                            data,
                            &raw mut local_10[1],
                        )
                    ) as u32;
                    if local_10[2] == 0 {
                        mech_trace::cr!(
                            "10038f3a",
                            "strncpy",
                            strncpy(param_3, data as *const i8, param_4)
                        );
                    }
                }
            }
            mech_trace::cr!("10038f3a", "RegCloseKey", RegCloseKey(local_14));
            b_var1 = (local_10[2] == 0) as i8;
        } else {
            FUN_10002168(&raw const DAT_100ae634 as *const i8, local_10[2]);
            b_var1 = 0;
        }
    }
    mech_trace::rt!("10038f3a", b_var1)
}
