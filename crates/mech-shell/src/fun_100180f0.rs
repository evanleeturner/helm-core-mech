// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10098224: *mut c_void;
    static mut DAT_100ab6e8: u8;
    static mut s_Out_of_memory_in_clear_collectio_1008f4f8: u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapFree(h: *mut c_void, flags: u32, mem: *mut c_void) -> i32;
    fn HeapAlloc(h: *mut c_void, flags: u32, size: u32) -> *mut c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FID_conflict__fwprintf(file: *mut c_void, format: *const c_void, ...) -> i32;
    fn _fflush(file: *mut c_void) -> i32;
    fn FUN_1006fde0(param_1: u32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100180f0(param_1: *mut i32, param_2: i8) {
    if param_1.is_null() {
        return;
    }
    // SAFETY: param_1 points at the game's collection structure; the fields,
    // heap handle, stream and callbacks are the game's own, accessed at the
    // same offsets and in the same order as the original.
    unsafe {
        let mut i_var4: i32 = 0;
        if 0 < param_1.add(1).read() {
            let mut i_var5: i32 = 0;
            loop {
                let arr = param_1.byte_add(0x11).cast::<i32>().read_unaligned();
                let i_var1 = (arr as u32).wrapping_add(i_var5 as u32) as *const i32;
                let i_var1 = i_var1.read_unaligned();
                if i_var1 != 0 {
                    let cb = param_1.byte_add(0x15).cast::<u32>().read_unaligned();
                    if cb != 0 && param_2 == 1 {
                        let f = core::mem::transmute::<u32, unsafe extern "C" fn(i32)>(cb);
                        f(i_var1);
                    }
                }
                i_var5 = i_var5.wrapping_add(4);
                i_var4 = i_var4.wrapping_add(1);
                if !(i_var4 < param_1.add(1).read()) {
                    break;
                }
            }
        }
        let data = param_1
            .byte_add(0x11)
            .cast::<*mut c_void>()
            .read_unaligned();
        if !data.is_null() {
            HeapFree(DAT_10098224, 1, data);
            param_1.byte_add(0x11).cast::<u32>().write_unaligned(0);
        }
        param_1.add(1).write(0);
        param_1.byte_add(0xd).cast::<u32>().write_unaligned(0);
        let size = (param_1.add(2).read() as u32).wrapping_shl(2);
        let puVar2 = HeapAlloc(DAT_10098224, 1, size) as *mut u32;
        if puVar2.is_null() {
            FID_conflict__fwprintf(
                &raw mut DAT_100ab6e8 as *mut c_void,
                &raw mut s_Out_of_memory_in_clear_collectio_1008f4f8 as *const c_void,
            );
            _fflush(&raw mut DAT_100ab6e8 as *mut c_void);
            FUN_1006fde0(1);
        }
        i_var4 = 0;
        param_1.write(param_1.add(2).read());
        let mut pu_var3 = puVar2;
        if 0 < param_1.add(2).read() {
            loop {
                pu_var3.write_unaligned(0);
                i_var4 = i_var4.wrapping_add(1);
                pu_var3 = pu_var3.wrapping_add(1);
                if !(i_var4 < param_1.read()) {
                    break;
                }
            }
        }
        param_1
            .byte_add(0x11)
            .cast::<*mut u32>()
            .write_unaligned(puVar2);
    }
}
