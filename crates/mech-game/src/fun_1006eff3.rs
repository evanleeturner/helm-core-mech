// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10168490: [u32; 150];
    static mut DAT_100adaa8: i32;
    static mut DAT_100adaac: i32;
    static mut DAT_100c2dec: i32;
    static mut DAT_100c2be0: *mut u8;
    static mut DAT_100b3ddc: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1004cea0();
    fn FUN_10068d2b(param_1: i32, param_2: i32, param_3: i32);
    fn __isctype(c: i32, mask: i32) -> i32;
    fn _atoi(s: *const i8) -> i32;
    fn _strncpy(dst: *mut i8, src: *const i8, n: u32) -> *mut i8;
    fn FUN_10032c28(param_1: i32);
    fn FUN_1001d9b0(param_1: *mut i16, param_2: *mut u32) -> *mut i32;
    fn FUN_1000c550() -> i32;
    fn FUN_1006c375(param_1: i32) -> u32;
    fn FUN_1001dd9f(param_1: *mut i32);
    fn FUN_1003289f();
    fn FUN_1000c73b();
    fn FUN_1000d7f5() -> u8;
    fn FUN_100108a0(param_1: i32, param_2: *mut i8);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1006eff3(param_1: *mut i8) -> u8 {
    mech_trace::enter!("1006eff3", param_1);
    let mut local_44: [u32; 8] = [0; 8];
    let mut local_24: [i16; 8] = [0; 8];
    let mut local_10: u32 = 0;
    // SAFETY: the callees and globals are the game's own; param_1 points at the
    // caller's NUL-terminated string; local buffers are used as the original does.
    unsafe {
        FUN_1004cea0();
        let mut local_14: i32 = 0;
        while local_14 < 0x96 {
            let p = (&raw mut DAT_10168490).cast::<u32>().add(local_14 as usize);
            mech_trace::wr!("1006eff3", p, write_unaligned, 0xffff_ffff);
            local_14 = local_14.wrapping_add(1);
        }
        mech_trace::gw!("1006eff3", "100adaa8", DAT_100adaa8, 0);
        mech_trace::gw!("1006eff3", "100adaac", DAT_100adaac, 0);
        FUN_10068d2b(0, 0, 0);
        let local_c: *mut i16 = local_24.as_mut_ptr();
        let local_48: u32;
        if mech_trace::gr!("1006eff3", "100c2dec", DAT_100c2dec) < 2 {
            let c = (mech_trace::dr!("1006eff3", param_1)) as i32;
            let tbl = mech_trace::gr!("1006eff3", "100c2be0", DAT_100c2be0);
            local_48 = (mech_trace::rd!(
                "1006eff3",
                tbl.byte_offset(c.wrapping_mul(2) as isize).cast::<u16>(),
                read_unaligned
            ) as u32)
                & 4;
        } else {
            local_48 = mech_trace::cr!(
                "1006eff3",
                "__isctype",
                __isctype((mech_trace::dr!("1006eff3", param_1)) as i32, 4)
            ) as u32;
        }
        if local_48 == 0 {
            mech_trace::wr!("1006eff3", local_c, write_unaligned, -1);
        } else {
            let iv = mech_trace::cr!("1006eff3", "_atoi", _atoi(param_1));
            mech_trace::wr!("1006eff3", local_c, write_unaligned, iv as i16);
        }
        mech_trace::cr!(
            "1006eff3",
            "_strncpy",
            _strncpy(local_c.add(1).cast::<i8>(), param_1, 0xc)
        );
        mech_trace::wr!("1006eff3", local_c.add(7).cast::<u8>(), write, 0);
        FUN_10032c28(0);
        let local_8 = mech_trace::cr!(
            "1006eff3",
            "1001d9b0",
            FUN_1001d9b0(local_c, local_44.as_mut_ptr())
        );
        if !local_8.is_null() && mech_trace::cr!("1006eff3", "1000c550", FUN_1000c550()) != 0 {
            local_10 = mech_trace::cr!("1006eff3", "1006c375", FUN_1006c375(local_8 as i32));
            FUN_1001dd9f(local_8);
            FUN_1003289f();
            FUN_1000c73b();
            let b = mech_trace::cr!("1006eff3", "1000d7f5", FUN_1000d7f5());
            if b == 0 {
                FUN_100108a0(0x4b, core::ptr::null_mut());
            }
        }
        let b_var1: u8 = (local_10 != 0) as u8;
        if b_var1 == 0 {
            FUN_100108a0(0xb, (&raw mut DAT_100b3ddc).cast::<i8>());
        }
        mech_trace::rt!("1006eff3", b_var1)
    }
}
