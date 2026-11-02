// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b0578: u8;
    static mut DAT_100c7d08: u8;
    static mut s__s__s_100b06a0: i8;
}

unsafe extern "C" {
    fn strchr(_str: *mut i8, _val: i32) -> *mut i8;
    fn sprintf(_dest: *mut i8, _format: *mut i8, ...) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100480d9(param_1: *mut i8) -> *mut u32 {
    // SAFETY: the globals and callees are the game's own; param_1 points at a
    // NUL-terminated string, and the destination buffer is the game's own.
    unsafe {
        let buf = &raw mut DAT_100c7d08;
        let mut i: i32 = 0;
        while i < 0x50 {
            buf.byte_add(i as usize).write(0);
            i = i.wrapping_add(1);
        }
        if DAT_100b0578 != 0 && strchr(param_1, 0x5c).is_null() && strchr(param_1, 0x2f).is_null() {
            sprintf(
                buf as *mut i8,
                &raw mut s__s__s_100b06a0,
                &raw mut DAT_100b0578,
                param_1,
            );
            return buf as *mut u32;
        }
        let mut len: usize = 0;
        while param_1.add(len).read() != 0 {
            len += 1;
        }
        let total = len + 1;
        let mut k: usize = 0;
        while k < total {
            buf.add(k).write(param_1.add(k).read() as u8);
            k += 1;
        }
        buf as *mut u32
    }
}
