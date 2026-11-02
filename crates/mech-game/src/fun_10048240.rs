// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf74: *mut core::ffi::c_void;
    static mut DAT_10160bdc: *mut u32;
    static mut DAT_10160be0: *mut u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapAlloc(
        param_1: *mut core::ffi::c_void,
        param_2: u32,
        param_3: u32,
    ) -> *mut core::ffi::c_void;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10048240(param_1: u32) -> u32 {
    mech_trace::enter!("10048240", param_1);
    // SAFETY: the heap handle and list globals are the game's own memory; the
    // allocated block is 0x18 bytes, so the six u32 writes stay in bounds, and
    // the tail pointer is only dereferenced when the list head is non-null.
    unsafe {
        let pu_var1 = mech_trace::cr!(
            "10048240",
            "HeapAlloc",
            HeapAlloc(
                mech_trace::gr!("10048240", "100adf74", DAT_100adf74),
                1,
                0x18
            )
        ) as *mut u32;
        if pu_var1.is_null() {
            return mech_trace::rt!("10048240", 0);
        }
        let mut pu_var4 = pu_var1;
        let mut i_var3: i32 = 6;
        while i_var3 != 0 {
            mech_trace::wr!("10048240", pu_var4, write_unaligned, 0);
            pu_var4 = pu_var4.add(1);
            i_var3 -= 1;
        }
        mech_trace::wr!("10048240", pu_var1, write_unaligned, param_1);
        pu_var4 = pu_var1;
        if !mech_trace::gr!("10048240", "10160be0", DAT_10160be0).is_null() {
            mech_trace::wr!(
                "10048240",
                mech_trace::gr!("10048240", "10160bdc", DAT_10160bdc).add(5),
                write_unaligned,
                pu_var1 as usize as u32
            );
            pu_var4 = mech_trace::gr!("10048240", "10160be0", DAT_10160be0);
        }
        mech_trace::gw!("10048240", "10160be0", DAT_10160be0, pu_var4);
        mech_trace::gw!("10048240", "10160bdc", DAT_10160bdc, pu_var1);
        mech_trace::rt!("10048240", 1)
    }
}
