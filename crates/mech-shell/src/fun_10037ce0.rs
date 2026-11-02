// SPDX-License-Identifier: MIT

use core::ffi::c_void;
use core::mem::MaybeUninit;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10094358: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1006f780(param_1: *const c_void) -> u8;
    fn FUN_1006f590(param_1: *mut c_void) -> u32;
    #[link_name = "__findfirst"]
    fn findfirst_10037ce0(param_1: *const c_void, param_2: *mut u32) -> *mut c_void;
    #[link_name = "__findnext"]
    fn findnext_10037ce0(param_1: *mut c_void, param_2: *mut u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10037ce0() {
    // Frame: local_118 (20 bytes) followed directly by local_104 (260 bytes).
    let mut buf: MaybeUninit<[u32; 70]> = MaybeUninit::uninit();
    // SAFETY: the callees and the global are the game's own; the buffer
    // reproduces the original's stack layout, with the name at offset 20.
    unsafe {
        let local_118 = buf.as_mut_ptr().cast::<u32>();
        let local_104 = local_118.byte_add(20);
        let pv_var1 = findfirst_10037ce0((&raw mut DAT_10094358).cast::<c_void>(), local_118);
        if pv_var1 as usize != 0xffff_ffff {
            loop {
                FUN_1006f780(local_104.cast::<c_void>());
                let i_var2 = findnext_10037ce0(pv_var1, local_118);
                if i_var2 != 0 {
                    break;
                }
            }
            FUN_1006f590(pv_var1);
        }
    }
}
