// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b0574: *mut core::ffi::c_void;
    static mut DAT_100adf74: *mut core::ffi::c_void;
    static mut DAT_10160be8: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapFree(hHeap: *mut core::ffi::c_void, dwFlags: u32, lpMem: *mut core::ffi::c_void);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1004726c() {
    mech_trace::enter!("1004726c");
    // SAFETY: the globals and HeapFree are the game's own; each node is a
    // heap block whose first word links to the next, read before it is freed.
    unsafe {
        let mut p_node = mech_trace::gr!("1004726c", "100b0574", DAT_100b0574);
        while !p_node.is_null() {
            let p_next = mech_trace::rd!(
                "1004726c",
                p_node.cast::<*mut core::ffi::c_void>(),
                read_unaligned
            );
            HeapFree(
                mech_trace::gr!("1004726c", "100adf74", DAT_100adf74),
                1,
                p_node,
            );
            p_node = p_next;
        }
        mech_trace::gw!("1004726c", "100b0574", DAT_100b0574, core::ptr::null_mut());
        mech_trace::gw!("1004726c", "10160be8", DAT_10160be8, 0);
    }
}
