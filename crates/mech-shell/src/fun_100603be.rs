// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7850: i32;
    static mut DAT_100a6740: *mut core::ffi::c_void;
    static mut DAT_10082cf0: [i32; 0];
    static mut DAT_100a8000: [i8; 0];
    static mut DAT_100a8018: [i8; 0];
    static mut in_ZF: i32;
    static mut in_stack_0000010c: *mut i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1006e7b0(buffer: *mut i8, format: *const i8, ...) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "thiscall" {
    fn FUN_10050c30(
        this: *mut core::ffi::c_void,
        param_1: i32,
        param_2: i32,
        param_3: *mut i8,
        param_4: i32,
    ) -> *mut i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100603be() -> *mut i32 {
    let mut local_buf = [0i8; 0x104];
    // SAFETY: globals and callees are the game's own memory; the slot table
    // and the backwards-walked table are read where the original reads them.
    unsafe {
        if in_ZF != 0 {
            return core::ptr::null_mut();
        }
        let mut i_var3: i32 = 0;
        let mut i_stack4: i32 = 8;
        let mut pi_var2 = DAT_100a7850.wrapping_add(0x368) as usize as *mut i32;
        loop {
            let mut i_var4: i32 = 0xc;
            loop {
                let v2 = pi_var2.read_unaligned();
                if v2 < 10000 {
                    let mut pi_var1 = (&raw mut DAT_10082cf0).cast::<i32>();
                    loop {
                        let v1 = pi_var1.read_unaligned();
                        if (v1 / 0x32 == v2 / 0x32) && (v1 == 0x2134) {
                            i_var3 = i_var3.wrapping_add(1);
                        }
                        pi_var1 = pi_var1.wrapping_sub(3);
                        if !(0x10082be7usize < pi_var1 as usize) {
                            break;
                        }
                    }
                }
                pi_var2 = pi_var2.wrapping_add(1);
                i_var4 = i_var4.wrapping_sub(1);
                if i_var4 == 0 {
                    break;
                }
            }
            i_stack4 = i_stack4.wrapping_sub(1);
            if i_stack4 == 0 {
                break;
            }
        }

        if i_var3 < 1 {
            FUN_1006e7b0(
                local_buf.as_mut_ptr(),
                (&raw const DAT_100a8018).cast::<i8>(),
            );
        } else {
            FUN_1006e7b0(
                local_buf.as_mut_ptr(),
                (&raw const DAT_100a8000).cast::<i8>(),
                i_var3,
            );
        }
        let rec = in_stack_0000010c;
        FUN_10050c30(
            DAT_100a6740,
            rec.read_unaligned(),
            rec.wrapping_add(1).read_unaligned(),
            local_buf.as_mut_ptr(),
            rec.wrapping_add(5).read_unaligned(),
        )
    }
}
