// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10098224: *mut core::ffi::c_void;
    static mut DAT_100ab6e8: u8;
    static mut DAT_1008f490: u8;
    static mut DAT_1008f4b0: u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapAlloc(h: *mut core::ffi::c_void, flags: u32, size: u32) -> *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FID_conflict__fwprintf(file: *mut u8, format: *mut u8, ...) -> i32;
    fn _fflush(file: *mut u8) -> i32;
    fn FUN_1006fde0(param_1: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10017c90(
    param_1: *mut u32,
    param_2: i32,
    param_3: u32,
    param_4: u8,
    param_5: u32,
) -> u32 {
    mech_trace::enter!("10017c90", param_1, param_2, param_3, param_4, param_5);
    // SAFETY: the globals, the heap and the callees are the game's own; the
    // allocated block is 0x1d bytes and every access stays within it, as in
    // the original.
    unsafe {
        let pi_var1 = mech_trace::cr!(
            "10017c90",
            "HeapAlloc",
            HeapAlloc(
                mech_trace::gr!("10017c90", "10098224", DAT_10098224),
                1,
                0x1d
            )
        ) as *mut i32;
        if pi_var1.is_null() {
            mech_trace::cr!(
                "10017c90",
                "FID_conflict__fwprintf",
                FID_conflict__fwprintf(&raw mut DAT_100ab6e8, &raw mut DAT_1008f490)
            );
            mech_trace::cr!("10017c90", "_fflush", _fflush(&raw mut DAT_100ab6e8));
            FUN_1006fde0(1);
        }
        mech_trace::wr!(
            "10017c90",
            param_1,
            write_unaligned,
            pi_var1 as usize as u32
        );
        mech_trace::wr!("10017c90", pi_var1.byte_add(8), write_unaligned, param_2);
        mech_trace::wr!("10017c90", pi_var1.byte_add(4), write_unaligned, 0);
        mech_trace::wr!(
            "10017c90",
            pi_var1.byte_add(0x15).cast::<u32>(),
            write_unaligned,
            param_3
        );
        let t = mech_trace::rd!("10017c90", pi_var1.byte_add(8), read_unaligned);
        mech_trace::wr!("10017c90", pi_var1, write_unaligned, t);
        mech_trace::wr!(
            "10017c90",
            pi_var1.byte_add(0xc).cast::<u8>(),
            write_unaligned,
            param_4
        );
        mech_trace::wr!(
            "10017c90",
            pi_var1.byte_add(0xd).cast::<u32>(),
            write_unaligned,
            0
        );
        mech_trace::wr!(
            "10017c90",
            pi_var1.byte_add(0x19).cast::<u32>(),
            write_unaligned,
            param_5
        );
        let size = (mech_trace::rd!("10017c90", pi_var1.byte_add(8), read_unaligned) as u32)
            .wrapping_shl(2);
        let pu_var2 = mech_trace::cr!(
            "10017c90",
            "HeapAlloc",
            HeapAlloc(
                mech_trace::gr!("10017c90", "10098224", DAT_10098224),
                1,
                size
            )
        ) as *mut u32;
        if pu_var2.is_null() {
            mech_trace::cr!(
                "10017c90",
                "FID_conflict__fwprintf",
                FID_conflict__fwprintf(&raw mut DAT_100ab6e8, &raw mut DAT_1008f4b0)
            );
            mech_trace::cr!("10017c90", "_fflush", _fflush(&raw mut DAT_100ab6e8));
            FUN_1006fde0(1);
        }
        let mut i_var3: i32 = 0;
        let mut pu_var4 = pu_var2;
        if 0 < mech_trace::rd!("10017c90", pi_var1, read_unaligned) {
            loop {
                mech_trace::wr!("10017c90", pu_var4, write_unaligned, 0);
                i_var3 = i_var3.wrapping_add(1);
                pu_var4 = pu_var4.wrapping_add(1);
                if !(i_var3 < mech_trace::rd!("10017c90", pi_var1, read_unaligned)) {
                    break;
                }
            }
        }
        mech_trace::wr!(
            "10017c90",
            pi_var1.byte_add(0x11).cast::<u32>(),
            write_unaligned,
            pu_var2 as usize as u32
        );
        mech_trace::rt!("10017c90", 0)
    }
}
