// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10095a30: u32;
    static mut DAT_10095a34: u32;
    static mut DAT_100a50f0: i32;
    static mut DAT_100c61e0: u8;
    static mut DAT_100c61e4: u32;
    static mut DAT_100c6220: u8;
    static mut DAT_100c6224: u32;
    static mut DAT_100c6160: u8;
    static mut DAT_100c6164: u32;
    static mut DAT_100c6200: u8;
    static mut DAT_100c6204: u32;
    static mut DAT_100c6110: u8;
    static mut DAT_100c6114: u32;
    static mut DAT_100c6130: u8;
    static mut DAT_100c6134: u32;
    static mut DAT_100c61fc: u32;
    static mut DAT_100c61c0: u8;
    static mut DAT_100c61c4: u32;
    static mut DAT_100c6398: *mut u8;
    static mut DAT_100c61b4: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1006ece0(dst: *mut c_void, size: u32, count: u32, file: *mut c_void) -> u32;
    fn _fseek(file: *mut c_void, offset: i32, origin: i32) -> i32;
    fn FUN_10011670(param_1: *mut c_void);
    fn FUN_10017b70() -> u32;
    fn FUN_100115c0(param_1: *mut u8, param_2: i32) -> u32;
    fn FUN_100115d0(param_1: *mut c_void);
    fn FUN_100701d0(param_1: *mut c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10011260(param_1: *mut c_void) -> u32 {
    mech_trace::enter!("10011260", param_1);
    // SAFETY: the globals, callees and the file handle are the game's own;
    // locals are passed to fread as buffers of the sizes given.
    unsafe {
        let mut buf1: [u32; 3] = [
            mech_trace::gr!("10011260", "10095a30", DAT_10095a30),
            0,
            0xc,
        ];
        let mut v = mech_trace::gr!("10011260", "100a50f0", DAT_100a50f0);
        if v <= 0xc {
            v = 0xc;
        }
        mech_trace::gw!("10011260", "100a50f0", DAT_100a50f0, v);
        mech_trace::cr!(
            "10011260",
            "1006ece0",
            FUN_1006ece0(buf1.as_mut_ptr().cast(), 0xc, 1, param_1)
        );
        FUN_10011670(param_1);

        let mut buf2: [u32; 3] = [0; 3];
        buf2[1] = 0xc;
        buf2[0] = mech_trace::gr!("10011260", "10095a34", DAT_10095a34);
        buf2[2] = mech_trace::cr!("10011260", "10017b70", FUN_10017b70());
        let mut v = mech_trace::gr!("10011260", "100a50f0", DAT_100a50f0);
        if v <= buf2[1] as i32 {
            v = buf2[1] as i32;
        }
        mech_trace::gw!("10011260", "100a50f0", DAT_100a50f0, v);
        mech_trace::cr!(
            "10011260",
            "1006ece0",
            FUN_1006ece0(buf2.as_mut_ptr().cast(), 0xc, 1, param_1)
        );
        FUN_10011670(param_1);

        let mut v = mech_trace::gr!("10011260", "100a50f0", DAT_100a50f0);
        mech_trace::gw!("10011260", "100c61e4", DAT_100c61e4, 0x1c);
        if v <= 0x1c {
            v = 0x1c;
        }
        mech_trace::gw!("10011260", "100a50f0", DAT_100a50f0, v);
        mech_trace::cr!(
            "10011260",
            "1006ece0",
            FUN_1006ece0((&raw mut DAT_100c61e0).cast(), 0x1c, 1, param_1)
        );
        FUN_10011670(param_1);

        let mut edi = mech_trace::cr!(
            "10011260",
            "100115c0",
            FUN_100115c0(&raw mut DAT_100c6220, 0x50)
        ) & 1;
        if mech_trace::gr!("10011260", "100c6224", DAT_100c6224) != 0 {
            mech_trace::gw!("10011260", "100c6224", DAT_100c6224, 0x50);
            let mut v = mech_trace::gr!("10011260", "100a50f0", DAT_100a50f0);
            if v <= 0x50 {
                v = 0x50;
            }
            mech_trace::gw!("10011260", "100a50f0", DAT_100a50f0, v);
            mech_trace::cr!(
                "10011260",
                "1006ece0",
                FUN_1006ece0((&raw mut DAT_100c6220).cast(), 0x50, 1, param_1)
            );
            FUN_10011670(param_1);
        }
        if mech_trace::gr!("10011260", "100c6164", DAT_100c6164) != 0 {
            edi &= mech_trace::cr!(
                "10011260",
                "100115c0",
                FUN_100115c0(&raw mut DAT_100c6160, 0x30)
            );
            mech_trace::gw!("10011260", "100c6164", DAT_100c6164, 0x30);
            let mut v = mech_trace::gr!("10011260", "100a50f0", DAT_100a50f0);
            if v <= 0x30 {
                v = 0x30;
            }
            mech_trace::gw!("10011260", "100a50f0", DAT_100a50f0, v);
            mech_trace::cr!(
                "10011260",
                "1006ece0",
                FUN_1006ece0((&raw mut DAT_100c6160).cast(), 0x30, 1, param_1)
            );
            FUN_10011670(param_1);
        }
        if mech_trace::gr!("10011260", "100c6204", DAT_100c6204) != 0 {
            edi &= mech_trace::cr!(
                "10011260",
                "100115c0",
                FUN_100115c0(&raw mut DAT_100c6200, 0x18)
            );
            mech_trace::gw!("10011260", "100c6204", DAT_100c6204, 0x18);
            let mut v = mech_trace::gr!("10011260", "100a50f0", DAT_100a50f0);
            if v <= 0x18 {
                v = 0x18;
            }
            mech_trace::gw!("10011260", "100a50f0", DAT_100a50f0, v);
            mech_trace::cr!(
                "10011260",
                "1006ece0",
                FUN_1006ece0((&raw mut DAT_100c6200).cast(), 0x18, 1, param_1)
            );
            FUN_10011670(param_1);
        }
        if mech_trace::gr!("10011260", "100c6114", DAT_100c6114) != 0 {
            edi &= mech_trace::cr!(
                "10011260",
                "100115c0",
                FUN_100115c0(&raw mut DAT_100c6110, 0x1e)
            );
            mech_trace::gw!("10011260", "100c6114", DAT_100c6114, 0x20);
            let mut v = mech_trace::gr!("10011260", "100a50f0", DAT_100a50f0);
            if v <= 0x20 {
                v = 0x20;
            }
            mech_trace::gw!("10011260", "100a50f0", DAT_100a50f0, v);
            mech_trace::cr!(
                "10011260",
                "1006ece0",
                FUN_1006ece0((&raw mut DAT_100c6110).cast(), 0x1e, 1, param_1)
            );
            FUN_10011670(param_1);
        }
        if mech_trace::gr!("10011260", "100c6134", DAT_100c6134) != 0 {
            edi &= mech_trace::cr!(
                "10011260",
                "100115c0",
                FUN_100115c0(&raw mut DAT_100c6130, 0x18)
            );
            mech_trace::gw!("10011260", "100c6134", DAT_100c6134, 0x18);
            let mut v = mech_trace::gr!("10011260", "100a50f0", DAT_100a50f0);
            if v <= 0x18 {
                v = 0x18;
            }
            mech_trace::gw!("10011260", "100a50f0", DAT_100a50f0, v);
            mech_trace::cr!(
                "10011260",
                "1006ece0",
                FUN_1006ece0((&raw mut DAT_100c6130).cast(), 0x18, 1, param_1)
            );
            FUN_10011670(param_1);
        }
        if mech_trace::rd!(
            "10011260",
            ((mech_trace::gr!("10011260", "100c61fc", DAT_100c61fc) as usize + 4) as *const i32),
            read_unaligned
        ) != 0
        {
            FUN_100115d0(param_1);
            FUN_10011670(param_1);
        }
        if mech_trace::gr!("10011260", "100c61c4", DAT_100c61c4) != 0 {
            edi &= mech_trace::cr!(
                "10011260",
                "100115c0",
                FUN_100115c0(&raw mut DAT_100c61c0, 0x1a)
            );
            mech_trace::gw!("10011260", "100c61c4", DAT_100c61c4, 0x1c);
            let mut v = mech_trace::gr!("10011260", "100a50f0", DAT_100a50f0);
            if v <= 0x1c {
                v = 0x1c;
            }
            mech_trace::gw!("10011260", "100a50f0", DAT_100a50f0, v);
            mech_trace::cr!(
                "10011260",
                "1006ece0",
                FUN_1006ece0((&raw mut DAT_100c61c0).cast(), 0x1a, 1, param_1)
            );
            FUN_10011670(param_1);
        }
        let dst = mech_trace::gr!("10011260", "100c6398", DAT_100c6398);
        let n = mech_trace::gr!("10011260", "100c61b4", DAT_100c61b4).wrapping_sub(dst as u32);
        mech_trace::cr!(
            "10011260",
            "1006ece0",
            FUN_1006ece0(dst.cast(), n, 1, param_1)
        );
        let mut local_1c: i32 = mech_trace::cr!("10011260", "100701d0", FUN_100701d0(param_1));
        mech_trace::cr!("10011260", "_fseek", _fseek(param_1, 4, 0));
        mech_trace::cr!(
            "10011260",
            "1006ece0",
            FUN_1006ece0((&raw mut local_1c).cast(), 4, 1, param_1)
        );
        mech_trace::cr!(
            "10011260",
            "1006ece0",
            FUN_1006ece0((&raw mut DAT_100a50f0).cast(), 4, 1, param_1)
        );
        mech_trace::cr!("10011260", "_fseek", _fseek(param_1, 2, 0));
        mech_trace::rt!("10011260", edi)
    }
}
