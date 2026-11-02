// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa1ac: u32;
    static mut DAT_100adf74: *mut core::ffi::c_void;
    static mut DAT_100b0688: u32;
    static mut DAT_100b068c: u32;
    static mut DAT_100b0690: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100480d9(param_1: *mut i8) -> *mut u32;
    fn FUN_10047b36(
        param_1: *mut i8,
        param_2: *mut u32,
        param_3: *mut i32,
        param_4: *mut i32,
    ) -> i32;
    fn __close(file_handle: i32) -> i32;
}

unsafe extern "stdcall" {
    fn HeapAlloc(h: *mut core::ffi::c_void, flags: u32, size: u32) -> *mut core::ffi::c_void;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10047e94(param_1: *mut i8, param_2: *mut i32) -> u32 {
    let mut local_10: i32 = 0;
    let mut local_8: u32 = 0;
    // SAFETY: callees and globals are the game's own; param_1 and param_2 are
    // the caller's pointers, used as the original does.
    unsafe {
        let pc_var1 = FUN_100480d9(param_1) as *mut i8;
        let local_c = FUN_10047b36(
            pc_var1,
            &raw mut local_8,
            &raw mut local_10,
            core::ptr::null_mut(),
        );
        if local_c == -1 {
            let mut buf: [u32; 21] = [0; 21];
            buf[0] = 0x10000;
            buf[1] = 0x10000;
            buf[2] = 0x10000;
            buf[3] = 0x10000;
            buf[4] = 0xb;
            buf[5] = 1;
            buf[6] = 1;
            buf[7] = 1;
            buf[8] = 1;
            buf[9] = 1;
            buf[14] = DAT_100b0688;
            buf[15] = DAT_100b068c;
            buf[16] = DAT_100b0690 as u32;
            let pv_var2 = HeapAlloc(DAT_100adf74, 1, 0x54);
            param_2.write_unaligned(pv_var2 as i32);
            let dst = param_2.read_unaligned() as *mut u32;
            let mut i = 0usize;
            while i < 0x15 {
                dst.add(i).write_unaligned(buf[i]);
                i += 1;
            }
            0xffff_ffff
        } else {
            param_2.write_unaligned(local_10);
            if DAT_100aa1ac != 0 {
                let base = param_2.read_unaligned() as u32;
                (base.wrapping_add(0x20) as *mut u32).write_unaligned(1);
            }
            __close(local_c);
            1
        }
    }
}
