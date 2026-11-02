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
    mech_trace::enter!("1001a980", param_1, param_2, param_3);
    let mut local_10 = [0u8; 260];
    // SAFETY: the callees and globals are the game's own; param_1 is a
    // NUL-terminated string from the caller, copied into the local buffer
    // and the returned records are read where the original reads them.
    unsafe {
        let mut len: usize = 0;
        while mech_trace::dr!("1001a980", param_1.add(len)) != 0 {
            len += 1;
        }
        let total = len + 1;
        let mut i = 0;
        while i < total {
            local_10[i] = mech_trace::dr!("1001a980", param_1.add(i)) as u8;
            i += 1;
        }
        local_10[4] = 0;
        let mut n = 0usize;
        while local_10[n] != 0 {
            n += 1;
        }
        let p = local_10.as_mut_ptr();
        mech_trace::wr!(
            "1001a980",
            p.add(n).cast::<u32>(),
            write_unaligned,
            mech_trace::gr!("1001a980", "1008fba4", DAT_1008fba4)
        );
        mech_trace::dw!(
            "1001a980",
            p.add(n + 4),
            mech_trace::gr!("1001a980", "1008fba8", DAT_1008fba8)
        );
        let name = p.cast::<i8>();
        let i_var6 = mech_trace::cr!(
            "1001a980",
            "100364c0",
            FUN_100364c0(
                mech_trace::gr!("1001a980", "100a6738", DAT_100a6738),
                name,
                1,
                (&raw mut DAT_1008fbac).cast::<i32>()
            )
        );
        if i_var6 != 0 {
            let pi = mech_trace::cr!("1001a980", "10036540", FUN_10036540(i_var6, 0x47, 0));
            if !pi.is_null() {
                let src = pi.add(2).cast::<u8>();
                let dst = &raw mut DAT_100a6b90;
                let mut k = 0usize;
                loop {
                    let c = mech_trace::dr!("1001a980", src.add(k));
                    mech_trace::dw!("1001a980", dst.add(k), c);
                    k += 1;
                    if c == 0 {
                        break;
                    }
                }
            }
            if param_2 != 0 {
                let mut pi = mech_trace::cr!("1001a980", "10036540", FUN_10036540(i_var6, 0x46, 0));
                if !pi.is_null() {
                    mech_trace::cr!(
                        "1001a980",
                        "1001a910",
                        FUN_1001a910(0, pi.add(2).cast::<i8>())
                    );
                }
                pi = mech_trace::cr!(
                    "1001a980",
                    "10036540",
                    FUN_10036540(i_var6, 0x46, pi as i32)
                );
                if !pi.is_null() {
                    mech_trace::cr!(
                        "1001a980",
                        "1001a910",
                        FUN_1001a910(1, pi.add(2).cast::<i8>())
                    );
                }
            }
            mech_trace::cr!("1001a980", "100361e0", FUN_100361e0(0, -1, -1, -1, -1));
            mech_trace::cr!(
                "1001a980",
                "10035f30",
                FUN_10035f30(
                    0,
                    core::ptr::null_mut(),
                    core::ptr::null_mut(),
                    core::ptr::null_mut(),
                )
            );
            if param_3 != 0 {
                mech_trace::cr!("1001a980", "10036540", FUN_10036540(i_var6, 0x45, 0));
            }
            mech_trace::cr!(
                "1001a980",
                "10036510",
                FUN_10036510(
                    mech_trace::gr!("1001a980", "100a6738", DAT_100a6738),
                    name,
                    1,
                    (&raw mut DAT_1008fbb0).cast::<i32>()
                )
            );
        }
    }
}
