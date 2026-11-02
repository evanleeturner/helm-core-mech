// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10082bf0: u8;
    static mut DAT_10092000: u32;
    static mut DAT_10098224: *mut core::ffi::c_void;
    static mut DAT_100a67c8: *mut i32;
    static mut DAT_10093634: i8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapAlloc(
        param_1: *mut core::ffi::c_void,
        param_2: u32,
        param_3: u32,
    ) -> *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10017df0(param_1: *mut i32, param_2: u32) -> u32;
    fn FUN_1005b1b0(param_1: *const i8) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10032c10() {
    // SAFETY: the globals and callees are the game's own; the pointers computed
    // from DAT_10092000 and the heap block address the game's own structures,
    // accessed where the original does.
    unsafe {
        let mut local_4: i32 = 8;
        let mut i_var3: i32 = 0;
        let mut i_var2: u32 = 0;
        loop {
            let mut i_var4: i32 = 0xc;
            loop {
                let p = DAT_10092000.wrapping_add(0x368).wrapping_add(i_var2) as *mut i32;
                if p.read_unaligned() / 100 == 0x50 {
                    i_var3 = i_var3.wrapping_add(1);
                    p.write_unaligned(0);
                }
                i_var2 = i_var2.wrapping_add(4);
                i_var4 = i_var4.wrapping_sub(1);
                if i_var4 == 0 {
                    break;
                }
            }
            local_4 = local_4.wrapping_sub(1);
            if local_4 == 0 {
                break;
            }
        }
        (DAT_10092000.wrapping_add(0x228) as *mut u32).write_unaligned(1);
        if 0 < i_var3 {
            loop {
                let pi = HeapAlloc(DAT_10098224, 1, 0x16) as *mut i32;
                pi.write_unaligned(2);
                pi.byte_add(8).cast::<u8>().write(2u8);
                let idx = pi.read_unaligned();
                let src =
                    (&raw mut DAT_10082bf0).byte_add(idx.wrapping_mul(0xc) as usize) as *const i32;
                pi.add(1).write_unaligned(src.read_unaligned());
                pi.byte_add(10).write_unaligned(-1i32);
                pi.byte_add(0xe).write_unaligned(-1i32);
                pi.byte_add(0x12).write_unaligned(-1i32);
                FUN_10017df0(DAT_100a67c8, pi as u32);
                i_var3 = i_var3.wrapping_sub(1);
                if i_var3 == 0 {
                    break;
                }
            }
        }
        FUN_1005b1b0(&raw mut DAT_10093634 as *const i8);
    }
}
