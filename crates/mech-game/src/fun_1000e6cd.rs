// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b0f20: i32;
    static mut DAT_100aafc4: i32;
    static mut DAT_100aafc8: i32;
    static mut DAT_100aafcc: i32;
    static mut DAT_100aafd0: i32;
    static mut DAT_100aafd4: i32;
    static mut _DAT_100ad6b0: u8;
    static mut DAT_100aaf8c: i32;
    static mut DAT_100aaf90: i32;
    static mut DAT_100aafa0: i32;
    static mut _DAT_100aaf80: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1007af4b(param_1: *mut *mut u8, param_2: *mut u32) -> u32;
    fn FUN_100108a0(param_1: i32, param_2: *mut i8);
    fn FUN_10035e40(param_1: i32, param_2: i32);
    fn FUN_1003d47c();
    fn FUN_1000e953();
    fn FUN_1003bc5a();
    fn FUN_10080639();
    fn FUN_1002c540();
    fn FUN_10016e60();
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn GetVersion() -> u32;
    fn VirtualProtect(param_1: *mut u8, param_2: u32, param_3: u32, param_4: *mut u32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1000e6cd() {
    let mut local_14: *mut u8 = core::ptr::null_mut();
    let mut local_10: u32 = 0;
    let mut local_8: u32 = 0;
    // SAFETY: all callees and globals are the game's own; locals are passed
    // by address to the callees exactly as the original does.
    unsafe {
        let local_c = FUN_1007af4b(&raw mut local_14, &raw mut local_8);
        let d_var1 = GetVersion();
        let flag: u32 = if (d_var1 & 0x8000_0000) == 0 {
            0xffff_ffff
        } else {
            0
        };
        let prot = (flag & 0x3c).wrapping_add(4);
        let b_var2 = VirtualProtect(local_14, local_c, prot, &raw mut local_10);
        if b_var2 == 0 {
            FUN_100108a0(0x4d, core::ptr::null_mut());
        }
        FUN_10035e40(0x100, 0x5dc);
        DAT_100b0f20 = 0x578;
        FUN_1003d47c();
        DAT_100aafc4 = FUN_1000e953 as usize as i32;
        DAT_100aafc8 = FUN_1003bc5a as usize as i32;
        DAT_100aafcc = FUN_10080639 as usize as i32;
        DAT_100aafd0 = FUN_1002c540 as usize as i32;
        DAT_100aafd4 = FUN_10016e60 as usize as i32;
        _DAT_100ad6b0 = 0xff;
        if DAT_100aaf8c != 0 || DAT_100aaf90 != 0 {
            DAT_100aafa0 = 0;
        }
        _DAT_100aaf80 |= 8;
    }
}
