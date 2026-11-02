// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1008fba4: u32;
    static mut DAT_1008fba8: u8;
    static mut DAT_1008fbac: u8;
    static mut DAT_1008fbb0: u8;
    static mut DAT_100a6738: *mut core::ffi::c_void;
    static mut DAT_100a6b90: u8;
}

#[allow(non_snake_case)]
unsafe extern "thiscall" {
    fn FUN_100364c0(
        this: *mut core::ffi::c_void,
        param_1: *mut i8,
        param_2: u32,
        param_3: *mut i32,
    ) -> i32;
    fn FUN_10036510(
        this: *mut core::ffi::c_void,
        param_1: *mut i8,
        param_2: u32,
        param_3: *mut i32,
    ) -> u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10036540(param_1: i32, param_2: i32, param_3: i32) -> *mut i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1001a910(param_1: i32, param_2: *mut i8) -> u8;
    fn FUN_100361e0(param_1: i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32) -> u8;
    fn FUN_10035f30(param_1: i32, param_2: *mut i8, param_3: *mut i8, param_4: *mut i8) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001a980(param_1: *mut i8, param_2: i32, param_3: i32) {
    let mut local_10 = [0u8; 260];
    // SAFETY: the callees and globals are the game's own; param_1 is a
    // NUL-terminated string from the caller, copied into the local buffer
    // and the returned records are read where the original reads them.
    unsafe {
        let mut len: usize = 0;
        while *param_1.add(len) != 0 {
            len += 1;
        }
        let total = len + 1;
        let mut i = 0;
        while i < total {
            local_10[i] = *param_1.add(i) as u8;
            i += 1;
        }
        local_10[4] = 0;
        let mut n = 0usize;
        while local_10[n] != 0 {
            n += 1;
        }
        let p = local_10.as_mut_ptr();
        p.add(n).cast::<u32>().write_unaligned(DAT_1008fba4);
        *p.add(n + 4) = DAT_1008fba8;
        let name = p.cast::<i8>();
        let i_var6 = FUN_100364c0(DAT_100a6738, name, 1, (&raw mut DAT_1008fbac).cast::<i32>());
        if i_var6 != 0 {
            let pi = FUN_10036540(i_var6, 0x47, 0);
            if !pi.is_null() {
                let src = pi.add(2).cast::<u8>();
                let dst = &raw mut DAT_100a6b90;
                let mut k = 0usize;
                loop {
                    let c = *src.add(k);
                    *dst.add(k) = c;
                    k += 1;
                    if c == 0 {
                        break;
                    }
                }
            }
            if param_2 != 0 {
                let mut pi = FUN_10036540(i_var6, 0x46, 0);
                if !pi.is_null() {
                    FUN_1001a910(0, pi.add(2).cast::<i8>());
                }
                pi = FUN_10036540(i_var6, 0x46, pi as i32);
                if !pi.is_null() {
                    FUN_1001a910(1, pi.add(2).cast::<i8>());
                }
            }
            FUN_100361e0(0, -1, -1, -1, -1);
            FUN_10035f30(
                0,
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
            );
            if param_3 != 0 {
                FUN_10036540(i_var6, 0x45, 0);
            }
            FUN_10036510(DAT_100a6738, name, 1, (&raw mut DAT_1008fbb0).cast::<i32>());
        }
    }
}
