// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa1ac: i32;
    static mut DAT_100adf98: u32;
    static mut DAT_100aa1d8: *mut i32;
    static mut DAT_101eb4d0: [i32; 8];
    static mut DAT_101eb520: [u8; 8];
    static mut DAT_101eb510: [u8; 8];
    static mut DAT_101658a0: u16;
    static mut DAT_100b38d8: u32;
    static mut DAT_100aa1e0: u32;
    static mut DAT_100aa1dc: *mut u32;
    static mut DAT_100adf74: *mut core::ffi::c_void;
    static mut DAT_100aa1e4: *mut u32;
    static mut DAT_101eb51c: u32;
    static mut DAT_100aa1e8: *mut u32;
    static mut DAT_100aa1ec: *mut u32;
    static mut DAT_101eb4c4: *mut core::ffi::c_void;
    static mut DAT_101eb4c0: *mut core::ffi::c_void;
    static mut DAT_101eb4f0: [i32; 8];
    static mut DAT_101eb530: [i32; 8];
    static mut DAT_100adf28: i32;
    static mut DAT_100adf2c: i32;
    static mut DAT_100aa20c: i32;
    static mut DAT_100adf38: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapAlloc(
        h_heap: *mut core::ffi::c_void,
        dw_flags: u32,
        dw_bytes: u32,
    ) -> *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10013be9(param_1: *mut i32) -> i32;
    fn FUN_10011e80(param_1: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10011f03(param_1: *mut i32) {
    // SAFETY: globals and callees are the game's own; the heap blocks are
    // freshly allocated with sizes covering the zeroing writes below.
    unsafe {
        if DAT_100aa1ac == 0 {
            DAT_100adf98 |= 1;
        } else {
            DAT_100aa1d8 = param_1;
            let mut local_8: usize = 0;
            while local_8 < 8 {
                DAT_101eb4d0[local_8] = 0;
                DAT_101eb520[local_8] = 0;
                DAT_101eb510[local_8] = 0;
                local_8 += 1;
            }
            DAT_101658a0 = DAT_100b38d8 as u16;
            DAT_100aa1e0 = 0x85;
            DAT_100aa1dc = HeapAlloc(DAT_100adf74, 1, 0x85) as *mut u32;
            let mut u_var2 = DAT_100aa1e0;
            let mut puvar4 = DAT_100aa1dc;
            let mut u_var1 = DAT_100aa1e0 >> 2;
            while u_var1 != 0 {
                puvar4.write_unaligned(0);
                puvar4 = puvar4.add(1);
                u_var1 = u_var1.wrapping_sub(1);
            }
            u_var2 &= 3;
            while u_var2 != 0 {
                (puvar4 as *mut u8).write(0);
                puvar4 = (puvar4 as *mut u8).add(1) as *mut u32;
                u_var2 = u_var2.wrapping_sub(1);
            }
            DAT_100aa1e4 = DAT_100aa1dc;
            DAT_101eb51c = 0x27;
            DAT_100aa1e8 = HeapAlloc(DAT_100adf74, 1, 0x27) as *mut u32;
            u_var2 = DAT_101eb51c;
            puvar4 = DAT_100aa1e8;
            u_var1 = DAT_101eb51c >> 2;
            while u_var1 != 0 {
                puvar4.write_unaligned(0);
                puvar4 = puvar4.add(1);
                u_var1 = u_var1.wrapping_sub(1);
            }
            u_var2 &= 3;
            while u_var2 != 0 {
                (puvar4 as *mut u8).write(0);
                puvar4 = (puvar4 as *mut u8).add(1) as *mut u32;
                u_var2 = u_var2.wrapping_sub(1);
            }
            DAT_100aa1ec = HeapAlloc(DAT_100adf74, 1, 0x52) as *mut u32;
            puvar4 = DAT_100aa1ec;
            let mut i_var3: i32 = 0x14;
            while i_var3 != 0 {
                puvar4.write_unaligned(0);
                puvar4 = puvar4.add(1);
                i_var3 = i_var3.wrapping_sub(1);
            }
            (puvar4 as *mut u16).write_unaligned(0);
            DAT_101eb4c4 = HeapAlloc(DAT_100adf74, 1, 0x100);
            DAT_101eb4c0 = HeapAlloc(DAT_100adf74, 1, 0x100);
            local_8 = 0;
            while local_8 < 8 {
                DAT_101eb4f0[local_8] = 0;
                DAT_101eb530[local_8] = 0;
                local_8 += 1;
            }
            i_var3 = FUN_10013be9(param_1);
            if i_var3 == 0 {
                DAT_100adf28 = 1;
                DAT_100adf2c = DAT_100adf2c.wrapping_add(0x29a);
            } else if DAT_100aa20c != 0 {
                DAT_100adf38 = FUN_10011e80(DAT_100aa20c);
            }
        }
    }
}
