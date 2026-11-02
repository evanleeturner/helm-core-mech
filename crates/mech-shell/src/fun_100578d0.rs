// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10098224: *mut core::ffi::c_void;
    static mut DAT_100a3660: u32;
    static mut DAT_100a3664: *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapAlloc(h: *mut core::ffi::c_void, flags: u32, size: u32) -> *mut core::ffi::c_void;
    fn HeapReAlloc(
        h: *mut core::ffi::c_void,
        flags: u32,
        p: *mut core::ffi::c_void,
        size: u32,
    ) -> *mut core::ffi::c_void;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100578d0() -> i32 {
    mech_trace::enter!("100578d0");
    // SAFETY: the globals and heap functions are the game's own; the cleared
    // region lies within the block just allocated or reallocated.
    unsafe {
        if mech_trace::gr!("100578d0", "100a3664", DAT_100a3664).is_null() {
            mech_trace::gw!(
                "100578d0",
                "100a3664",
                DAT_100a3664,
                mech_trace::cr!(
                    "100578d0",
                    "HeapAlloc",
                    HeapAlloc(
                        mech_trace::gr!("100578d0", "10098224", DAT_10098224),
                        1,
                        6000
                    )
                )
            );
        } else {
            mech_trace::gw!(
                "100578d0",
                "100a3664",
                DAT_100a3664,
                mech_trace::cr!(
                    "100578d0",
                    "HeapReAlloc",
                    HeapReAlloc(
                        mech_trace::gr!("100578d0", "10098224", DAT_10098224),
                        1,
                        mech_trace::gr!("100578d0", "100a3664", DAT_100a3664),
                        (mech_trace::gr!("100578d0", "100a3660", DAT_100a3660)
                            .wrapping_mul(8)
                            .wrapping_add(400))
                        .wrapping_mul(0xf),
                    )
                )
            );
        }
        if mech_trace::gr!("100578d0", "100a3664", DAT_100a3664).is_null() {
            return mech_trace::rt!("100578d0", 0);
        }
        let new_capacity = mech_trace::gr!("100578d0", "100a3660", DAT_100a3660).wrapping_add(0x32);
        let mut cursor = mech_trace::gr!("100578d0", "100a3664", DAT_100a3664)
            .cast::<u8>()
            .add(mech_trace::gr!("100578d0", "100a3660", DAT_100a3660).wrapping_mul(0x78) as usize)
            .cast::<u32>();
        let mut left: i32 = 0x5dc;
        while left != 0 {
            mech_trace::wr!("100578d0", cursor, write_unaligned, 0);
            cursor = cursor.add(1);
            left -= 1;
        }
        mech_trace::gw!("100578d0", "100a3660", DAT_100a3660, new_capacity);
        mech_trace::rt!("100578d0", new_capacity as i32)
    }
}
