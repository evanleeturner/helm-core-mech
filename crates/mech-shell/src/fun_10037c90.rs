// SPDX-License-Identifier: MIT

use core::mem::MaybeUninit;

#[repr(C)]
struct Locals {
    local_118: [u32; 5],
    local_104: [i8; 260],
}

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10094344: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn __findfirst(param_1: *const core::ffi::c_void, param_2: *mut u32) -> *mut core::ffi::c_void;
    fn FUN_1006f780(param_1: *const core::ffi::c_void) -> u8;
    fn __findnext(param_1: *mut core::ffi::c_void, param_2: *mut u32) -> u32;
    fn FUN_1006f590(param_1: *mut core::ffi::c_void) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10037c90() {
    let mut locals: MaybeUninit<Locals> = MaybeUninit::uninit();
    let p = locals.as_mut_ptr();
    // SAFETY: the callees and the global are the game's own; the locals
    // are laid out as the original's stack frame, and the name buffer is
    // passed on without being initialised first, as the original does.
    unsafe {
        let l118 = &raw mut (*p).local_118 as *mut u32;
        let l104 = &raw mut (*p).local_104 as *const i8;
        let pv_var1 = __findfirst((&raw mut DAT_10094344).cast::<core::ffi::c_void>(), l118);
        if pv_var1 as usize != 0xffff_ffff {
            loop {
                FUN_1006f780(l104.cast::<core::ffi::c_void>());
                let i_var2 = __findnext(pv_var1, l118);
                if i_var2 != 0 {
                    break;
                }
            }
            FUN_1006f590(pv_var1);
        }
    }
}
