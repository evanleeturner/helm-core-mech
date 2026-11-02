// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10083d88: u8;
    static mut DAT_10083d8c: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn strncpy(dest: *mut i8, source: *const i8, count: u32) -> *mut i8;
    fn FUN_1005bbb0(source: *mut i8) -> *mut i8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000bf40(source: *mut i8) -> *mut *mut u8 {
    mech_trace::enter!("1000bf40", source);
    let mut local_20: [i8; 4] = [0; 4];
    let mut i_var5: i32 = 0;
    // SAFETY: the globals, the string helpers and the table are the game's own;
    // source points at the caller's string, and local_20 is a local buffer.
    unsafe {
        let mut ppu_var6: *mut *mut u8 = (&raw mut DAT_10083d8c).cast::<*mut u8>();
        mech_trace::cr!(
            "1000bf40",
            "strncpy",
            strncpy(local_20.as_mut_ptr(), source, 3)
        );
        local_20[3] = 0;
        mech_trace::rt!(
            "1000bf40",
            loop {
                let mut pb2 = mech_trace::cr!(
                    "1000bf40",
                    "1005bbb0",
                    FUN_1005bbb0(
                        mech_trace::rd!("1000bf40", ppu_var6, read_unaligned).cast::<i8>()
                    )
                )
                .cast::<u8>();
                let mut pb3 =
                    mech_trace::cr!("1000bf40", "1005bbb0", FUN_1005bbb0(local_20.as_mut_ptr()))
                        .cast::<u8>();
                let equal = loop {
                    let b1 = mech_trace::rd!("1000bf40", pb2, read);
                    if b1 != mech_trace::rd!("1000bf40", pb3, read) {
                        break false;
                    }
                    if b1 == 0 {
                        break true;
                    }
                    let b2 = mech_trace::rd!("1000bf40", pb2.add(1), read);
                    if b2 != mech_trace::rd!("1000bf40", pb3.add(1), read) {
                        break false;
                    }
                    pb2 = pb2.add(2);
                    pb3 = pb3.add(2);
                    if b2 == 0 {
                        break true;
                    }
                };
                if equal {
                    return mech_trace::rt!(
                        "1000bf40",
                        (&raw mut DAT_10083d88)
                            .byte_add(i_var5.wrapping_mul(10) as usize)
                            .cast::<*mut u8>()
                    );
                }
                ppu_var6 = ppu_var6.byte_add(40);
                i_var5 = i_var5.wrapping_add(1);
                if (ppu_var6 as usize) > 0x1008473b {
                    return mech_trace::rt!("1000bf40", core::ptr::null_mut());
                }
            }
        )
    }
}
