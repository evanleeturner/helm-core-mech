// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10098224: *mut c_void;
    static mut DAT_100ab6e8: u8;
    static mut s_Out_of_memory_in_expand_collecti_1008f4d4: u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapReAlloc(h: *mut c_void, flags: u32, mem: *mut c_void, size: u32) -> *mut c_void;
    fn HeapAlloc(h: *mut c_void, flags: u32, size: u32) -> *mut c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FID_conflict__fwprintf(file: *mut c_void, format: *mut c_void, ...) -> i32;
    fn _fflush(file: *mut c_void) -> i32;
    fn FUN_1006fde0(param_1: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10017df0(param_1: *mut i32, param_2: u32) -> u32 {
    if param_1.is_null() {
        return 0xffffffff;
    }
    // SAFETY: param_1 points at the caller's collection struct; the globals,
    // heap and callees are the game's own, accessed where the original does.
    unsafe {
        let p0 = param_1;
        let p1 = param_1.add(1);
        let p2 = param_1.add(2);
        let data_slot = param_1.byte_add(0x11).cast::<*mut c_void>();

        if p0.read().wrapping_sub(1) < p1.read() {
            let mut i_var1 = p2.read().wrapping_add(p0.read());
            p0.write(i_var1);
            let old = data_slot.read_unaligned();
            let pv_var2: *mut c_void = if old.is_null() {
                HeapAlloc(DAT_10098224, 1, i_var1.wrapping_mul(4) as u32)
            } else {
                HeapReAlloc(DAT_10098224, 1, old, i_var1.wrapping_mul(4) as u32)
            };
            if pv_var2.is_null() {
                FID_conflict__fwprintf(
                    &raw mut DAT_100ab6e8 as *mut c_void,
                    &raw mut s_Out_of_memory_in_expand_collecti_1008f4d4 as *mut c_void,
                );
                _fflush(&raw mut DAT_100ab6e8 as *mut c_void);
                FUN_1006fde0(1);
            }
            i_var1 = p0.read().wrapping_sub(p2.read());
            if i_var1 < p0.read() {
                let mut puvar3 = (pv_var2 as i32).wrapping_add(i_var1.wrapping_mul(4)) as *mut u32;
                loop {
                    puvar3.write_unaligned(0);
                    puvar3 = (puvar3 as i32).wrapping_add(4) as *mut u32;
                    i_var1 = i_var1.wrapping_add(1);
                    if !(i_var1 < p0.read()) {
                        break;
                    }
                }
            }
            data_slot.write_unaligned(pv_var2);
        }
        let base = data_slot.read_unaligned() as i32;
        let dst = base.wrapping_add(p1.read().wrapping_mul(4)) as *mut u32;
        dst.write_unaligned(param_2);
        param_1.byte_add(0xd).cast::<u32>().write_unaligned(1);
        p1.write(p1.read().wrapping_add(1));
    }
    0
}
