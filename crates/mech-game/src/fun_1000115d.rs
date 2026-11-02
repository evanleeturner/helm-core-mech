// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a4030: u32;
    static mut DAT_100a4044: [i8; 0];
    static mut DAT_100a4050: [i8; 0];
    static mut DAT_100a4054: [i8; 0];
    static mut DAT_100a4058: [i8; 0];
    static mut DAT_100a405c: [i8; 0];
    static mut DAT_100c4be8: [i8; 0];
    static mut s_MW2_INI_100a4048: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    #[link_name = "_fopen"]
    fn _fopen(_Filename: *mut i8, _Mode: *mut i8) -> *mut c_void;
    #[link_name = "_fseek"]
    fn _fseek(_File: *mut c_void, _Offset: i32, _Origin: i32) -> i32;
    fn FUN_100012e5(param_1: *mut i8) -> *mut i8;
    #[link_name = "_fgets"]
    fn _fgets(_Buf: *mut i8, _MaxCount: i32, _File: *mut c_void) -> *mut i8;
    #[link_name = "_strspn"]
    fn _strspn(_Str: *mut i8, _Control: *mut i8) -> u32;
    #[link_name = "_strchr"]
    fn _strchr(_Str: *mut i8, _Val: i32) -> *mut i8;
    #[link_name = "__strcmpi"]
    fn __strcmpi(_Str1: *mut i8, _Str2: *mut i8) -> i32;
    #[link_name = "_fclose"]
    fn _fclose(_File: *mut c_void) -> i32;
    #[link_name = "_strcspn"]
    fn _strcspn(_Str: *mut i8, _Control: *mut i8) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000115d(param_1: *mut i8, param_2: i32) -> *mut i8 {
    mech_trace::enter!("1000115d", param_1, param_2);
    // SAFETY: globals and callees are the game's own; all pointers refer to the
    // game's buffers and the caller's string, used as the original does.
    unsafe {
        let mut local_10: *mut i8;
        let file: *mut c_void;
        if mech_trace::gr!("1000115d", "100a4030", DAT_100a4030) == 0xffff_ffff {
            return mech_trace::rt!("1000115d", &raw mut DAT_100a4050 as *mut i8);
        }
        file = mech_trace::cr!(
            "1000115d",
            "_fopen",
            _fopen(
                &raw mut s_MW2_INI_100a4048 as *mut i8,
                &raw mut DAT_100a4044 as *mut i8,
            )
        );
        if file.is_null() {
            return mech_trace::rt!("1000115d", &raw mut DAT_100a4050 as *mut i8);
        }
        mech_trace::cr!(
            "1000115d",
            "_fseek",
            _fseek(
                file,
                mech_trace::gr!("1000115d", "100a4030", DAT_100a4030) as i32,
                0
            )
        );
        let pc_var2 = mech_trace::cr!("1000115d", "100012e5", FUN_100012e5(param_1));
        let buf = &raw mut DAT_100c4be8 as *mut i8;
        loop {
            local_10 = &raw mut DAT_100a4054 as *mut i8;
            let r = mech_trace::cr!("1000115d", "_fgets", _fgets(buf, 0x85, file));
            if r.is_null() {
                break;
            }
            let s = mech_trace::cr!(
                "1000115d",
                "_strspn",
                _strspn(buf, &raw mut DAT_100a4058 as *mut i8)
            );
            let mut pc_var3 = buf.add(s as usize);
            if mech_trace::dr!("1000115d", pc_var3) == b'[' as i8 {
                break;
            }
            local_10 = mech_trace::cr!("1000115d", "_strchr", _strchr(pc_var3, 0x3d));
            if local_10.is_null() {
                continue;
            }
            mech_trace::dw!("1000115d", local_10, 0);
            local_10 = local_10.add(1);
            pc_var3 = mech_trace::cr!("1000115d", "100012e5", FUN_100012e5(pc_var3));
            if mech_trace::cr!("1000115d", "__strcmpi", __strcmpi(pc_var3, pc_var2)) == 0 {
                break;
            }
        }
        mech_trace::cr!("1000115d", "_fclose", _fclose(file));
        let mut u_var6: u32 = 0xffff_ffff;
        let mut p = local_10;
        loop {
            if u_var6 == 0 {
                break;
            }
            u_var6 = u_var6.wrapping_sub(1);
            let c = mech_trace::dr!("1000115d", p);
            p = p.add(1);
            if c == 0 {
                break;
            }
        }
        let n = !u_var6;
        if n != 1 {
            mech_trace::dw!(
                "1000115d",
                local_10.offset((n.wrapping_sub(2)) as i32 as isize),
                0
            );
        }
        if param_2 == 0 {
            let s = mech_trace::cr!(
                "1000115d",
                "_strcspn",
                _strcspn(local_10, &raw mut DAT_100a405c as *mut i8)
            );
            mech_trace::dw!("1000115d", local_10.add(s as usize), 0);
            local_10 = mech_trace::cr!("1000115d", "100012e5", FUN_100012e5(local_10));
        }
        mech_trace::rt!("1000115d", local_10)
    }
}
