// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_101f52d0: u32;
    static mut DAT_101f52e0: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10003ea2(param_1: i32, param_2: *mut i32) -> *mut u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1000898c();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000a3fd(
    param_1: i32,
    param_2: *mut u32,
    param_3: *mut i32,
    param_4: i32,
) -> u32 {
    // SAFETY: all pointers are the game's own memory; reads, writes and calls
    // happen in the same order as the original.
    unsafe {
        let mut puv_ar4: *mut u32 = param_2.read_unaligned() as usize as *mut u32;
        let mut local_8: u32 = 1;
        let mut local_c: *mut u32 = FUN_10003ea2(param_1, param_3);
        if local_c.is_null() {
            local_8 = 0;
            if param_4 != 0 {
                let base = param_3.add(1).read_unaligned() as usize;
                if param_1 == 0 {
                    local_c = ((base + 8) as *const u32).read_unaligned() as usize as *mut u32;
                } else {
                    local_c = ((base + 4) as *const u32).read_unaligned() as usize as *mut u32;
                }
            }
        }
        if !local_c.is_null() {
            let mut p2: *mut u32 = puv_ar4;
            let mut p3: *mut u32 = param_2.add(1).read_unaligned() as usize as *mut u32;
            let mut i = 5;
            while i != 0 {
                p3.write_unaligned(p2.read_unaligned());
                p2 = p2.add(1);
                p3 = p3.add(1);
                i -= 1;
            }
            p2 = local_c;
            p3 = puv_ar4;
            i = 5;
            while i != 0 {
                p3.write_unaligned(p2.read_unaligned());
                p2 = p2.add(1);
                p3 = p3.add(1);
                i -= 1;
            }
            FUN_1000898c();
            p2 = param_2.add(1).read_unaligned() as usize as *mut u32;
            i = 5;
            while i != 0 {
                puv_ar4.write_unaligned(p2.read_unaligned());
                p2 = p2.add(1);
                puv_ar4 = puv_ar4.add(1);
                i -= 1;
            }
            if param_4 != 0 {
                puv_ar4 = (&raw mut DAT_101f52e0).cast::<u32>();
                i = 5;
                while i != 0 {
                    puv_ar4.write_unaligned(local_c.read_unaligned());
                    local_c = local_c.add(1);
                    puv_ar4 = puv_ar4.add(1);
                    i -= 1;
                }
                DAT_101f52d0 = 1;
            }
        }
        local_8
    }
}
