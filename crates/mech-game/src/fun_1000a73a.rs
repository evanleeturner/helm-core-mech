// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7314: u32;
    static mut DAT_100a7b30: [u8; 0];
    static mut DAT_10114db0: u32;
    static mut DAT_10215658: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10051f43(param_1: i32) -> i32;
    fn FUN_10010683(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32) -> u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1000a5b7() -> u8;
    fn FUN_1000898c() -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1000a73a() {
    // SAFETY: globals and callees are the game's own; the pointers read from
    // the game's tables are dereferenced exactly where the original does.
    unsafe {
        let base = (&raw mut DAT_100a7b30).cast::<u8>();
        let puvar1: *mut u32 = base
            .byte_add(DAT_10215658.wrapping_mul(4) as usize)
            .cast::<u32>()
            .read_unaligned() as *mut u32;
        if puvar1.is_null() {
            return;
        }
        if DAT_10215658 == 4 {
            FUN_1000a5b7();
        } else {
            let a = puvar1.add(0x1e).read_unaligned();
            let ivar2 = ((a as usize + 8) as *const i32).read_unaligned();
            if ivar2 == -1 {
                FUN_1000898c();
            } else {
                let s = ((DAT_10114db0 as usize).wrapping_add(6) as *const i16).read_unaligned();
                if s == 1 {
                    let local_8: i32 = if DAT_100a7314 == 0 { 3 } else { 7 };
                    let ivar4 = FUN_10051f43(10);
                    DAT_100a7314 = (ivar4 < local_8) as u32;
                } else if s == 2 {
                    DAT_100a7314 = 1;
                }
                FUN_1000898c();
                if DAT_100a7314 != 0 {
                    let pivar3 = puvar1.read_unaligned() as *mut i32;
                    FUN_10010683(pivar3, ivar2, 0, 0);
                    let f = puvar1.add(0x1f).read_unaligned();
                    if f != 0 {
                        let p = puvar1.add(0x1d).read_unaligned();
                        let arg = ((p as usize).wrapping_add(0x30) as *const u32).read_unaligned();
                        let func =
                            core::mem::transmute::<u32, unsafe extern "C" fn(*mut i32, u32)>(f);
                        func(pivar3, arg);
                    }
                }
            }
        }
    }
}
