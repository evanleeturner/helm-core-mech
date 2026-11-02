// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf74: *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1001c8a3(mem: *mut core::ffi::c_void) -> u8;
    fn FUN_1001c110(mem: i32) -> u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapFree(h: *mut core::ffi::c_void, flags: u32, mem: *mut core::ffi::c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001c989(mem: *mut core::ffi::c_void) {
    mech_trace::enter!("1001c989", mem);
    // SAFETY: mem points at the game's own object; its list at offset 0x1c is
    // walked as the original does, and the callees and globals are the game's.
    unsafe {
        let mut local_8: *mut core::ffi::c_void = mech_trace::rd!(
            "1001c989",
            mem.byte_add(0x1c).cast::<*mut core::ffi::c_void>(),
            read_unaligned
        );
        while !local_8.is_null() {
            let pv_var1 = mech_trace::rd!(
                "1001c989",
                local_8.byte_add(0xc).cast::<*mut core::ffi::c_void>(),
                read_unaligned
            );
            mech_trace::cr!("1001c989", "1001c8a3", FUN_1001c8a3(local_8));
            local_8 = pv_var1;
        }
        mech_trace::cr!("1001c989", "1001c110", FUN_1001c110(mem as i32));
        mech_trace::cr!(
            "1001c989",
            "HeapFree",
            HeapFree(
                mech_trace::gr!("1001c989", "100adf74", DAT_100adf74),
                1,
                mem
            )
        );
    }
}
