// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a6744: *mut c_void;
    static mut DAT_1009faa8: u8;
    static mut DAT_1009fac0: u8;
    static mut in_ZF: i32;
    static mut in_stack_00000104: *mut i32;
}

#[allow(non_snake_case)]
unsafe extern "thiscall" {
    fn FUN_10050b80(
        this: *mut c_void,
        param_1: i32,
        param_2: i32,
        param_3: *mut i8,
        param_4: i32,
    ) -> *mut i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100530ae() {
    let mut local_buf = [0u8; 0x104];
    // SAFETY: the globals and the callee are the game's own; the source
    // strings are NUL-terminated and fit the local buffer, and the enclosing
    // frame's pointer addresses at least six ints.
    unsafe {
        let src: *const u8 = if in_ZF != 0 {
            &raw mut DAT_1009faa8 as *const u8
        } else {
            &raw mut DAT_1009fac0 as *const u8
        };

        let mut len: usize = 0;
        loop {
            let c = src.add(len).read();
            len += 1;
            if c == 0 {
                break;
            }
        }

        let mut i = 0;
        while i < len {
            local_buf[i] = src.add(i).read();
            i += 1;
        }

        let p = in_stack_00000104;
        let a = p.read_unaligned();
        let b = p.add(1).read_unaligned();
        let d = p.add(5).read_unaligned();
        FUN_10050b80(DAT_100a6744, a, b, local_buf.as_mut_ptr().cast::<i8>(), d);
    }
}
