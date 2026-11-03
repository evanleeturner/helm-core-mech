// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100d08c0: i32;
    static mut DAT_100d08c4: *mut i16;
    static mut DAT_100984cc: *mut u8;
    static mut DAT_10098588: u8;
    static mut s_symlog_txt_1009858c: u8;
    static mut s_Couldn_t_load_ID__d_Type__s_10098598: u8;
    static mut DAT_100985b8: u8;
    static mut s_symlog_txt_100985bc: u8;
    static mut s_Couldn_t_load_ID__d_Type__s_100985c8: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10042370(param_1: i32, param_2: *mut i32) -> *mut i16;
    fn FUN_10042210(param_1: i32);
    fn FUN_100423d0(param_1: *mut i16);
    fn FUN_1005a3c0(param_1: i32, param_2: *mut i8, param_3: u16) -> i32;
    fn __wfopen(filename: *const u8, mode: *const u8) -> *mut u8;
    fn _fwprintf(file: *mut u8, format: *const u8, ...) -> i32;
    fn _fclose(file: *mut u8) -> i32;
    fn FUN_1006fab0(param_1: u32) -> *mut u16;
    fn FUN_1005a570(param_1: i32, param_2: *mut i8, param_3: u32, param_4: *mut u8) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100425b0(param_1: i32, param_2: u32, param_3: *mut i32) -> *mut u16 {
    mech_trace::enter!("100425b0", param_1, param_2, param_3);
    if (param_2 as i32) < 0 {
        return mech_trace::rt!("100425b0", core::ptr::null_mut());
    }
    // SAFETY: callees and globals are the game's own; param_3 points at the
    // caller's key bytes, and returned blocks are the allocator's.
    unsafe {
        let psv = mech_trace::cr!(
            "100425b0",
            "10042370",
            FUN_10042370(param_2 as i32, param_3)
        );
        if !psv.is_null() {
            FUN_10042210(psv as i32);
            return mech_trace::rt!("100425b0", psv.byte_add(0x14) as *mut u16);
        }
        if mech_trace::gr!("100425b0", "100d08c0", DAT_100d08c0) >= 1000 {
            if mech_trace::gr!("100425b0", "100d08c4", DAT_100d08c4).is_null() {
                return mech_trace::rt!("100425b0", core::ptr::null_mut());
            }
            FUN_100423d0(mech_trace::gr!("100425b0", "100d08c4", DAT_100d08c4));
        }
        let i_var2 = mech_trace::cr!(
            "100425b0",
            "1005a3c0",
            FUN_1005a3c0(param_1, param_3 as *mut i8, param_2 as u16)
        );
        if i_var2 < 1 {
            let f = mech_trace::cr!(
                "100425b0",
                "__wfopen",
                __wfopen(&raw mut s_symlog_txt_1009858c, &raw mut DAT_10098588)
            );
            if !f.is_null() {
                mech_trace::cr!(
                    "100425b0",
                    "_fwprintf",
                    _fwprintf(
                        f,
                        &raw mut s_Couldn_t_load_ID__d_Type__s_10098598 as *const u8,
                        param_2,
                        param_3,
                    )
                );
            }
            mech_trace::cr!("100425b0", "_fclose", _fclose(f));
            return mech_trace::rt!("100425b0", core::ptr::null_mut());
        }
        let size = (i_var2 as u32).wrapping_add(0x14);
        let p: *mut u16 = loop {
            let q = mech_trace::cr!("100425b0", "1006fab0", FUN_1006fab0(size));
            if !q.is_null() {
                break q;
            }
            if mech_trace::gr!("100425b0", "100d08c4", DAT_100d08c4).is_null() {
                return mech_trace::rt!("100425b0", core::ptr::null_mut());
            }
            FUN_100423d0(mech_trace::gr!("100425b0", "100d08c4", DAT_100d08c4));
        };
        let data = p.byte_add(0x14) as *mut u8;
        let r = mech_trace::cr!(
            "100425b0",
            "1005a570",
            FUN_1005a570(param_1, param_3 as *mut i8, param_2, data)
        );
        if r == -1 {
            let f = mech_trace::cr!(
                "100425b0",
                "__wfopen",
                __wfopen(&raw mut s_symlog_txt_100985bc, &raw mut DAT_100985b8)
            );
            if !f.is_null() {
                mech_trace::cr!(
                    "100425b0",
                    "_fwprintf",
                    _fwprintf(
                        f,
                        &raw mut s_Couldn_t_load_ID__d_Type__s_100985c8 as *const u8,
                        param_2,
                        param_3,
                    )
                );
            }
            mech_trace::cr!("100425b0", "_fclose", _fclose(f));
            return mech_trace::rt!("100425b0", core::ptr::null_mut());
        }
        let k = param_3 as *const i8;
        let sum = (mech_trace::rd!("100425b0", k.byte_add(2), read) as i32)
            .wrapping_add(mech_trace::rd!("100425b0", k.byte_add(3), read) as i32)
            .wrapping_add(mech_trace::rd!("100425b0", k.byte_add(1), read) as i32)
            .wrapping_add(mech_trace::rd!("100425b0", k, read) as i32)
            .wrapping_add(param_2 as i32);
        let idx = sum % 0x3f1;
        let off = idx.wrapping_mul(4) as isize;
        let slot =
            mech_trace::gr!("100425b0", "100984cc", DAT_100984cc).byte_offset(off) as *mut u32;
        let old = mech_trace::rd!("100425b0", slot, read_unaligned);
        mech_trace::wr!(
            "100425b0",
            p.byte_add(8).cast::<u32>(),
            write_unaligned,
            old
        );
        let slot =
            mech_trace::gr!("100425b0", "100984cc", DAT_100984cc).byte_offset(off) as *mut u32;
        mech_trace::wr!("100425b0", slot, write_unaligned, p as u32);
        mech_trace::wr!("100425b0", p.byte_add(2), write_unaligned, 1);
        mech_trace::wr!("100425b0", p, write_unaligned, param_2 as u16);
        mech_trace::wr!(
            "100425b0",
            p.byte_add(4).cast::<i32>(),
            write_unaligned,
            mech_trace::rd!("100425b0", param_3, read_unaligned)
        );
        mech_trace::wr!(
            "100425b0",
            p.byte_add(0xc).cast::<u32>(),
            write_unaligned,
            0
        );
        mech_trace::wr!(
            "100425b0",
            p.byte_add(0x10).cast::<u32>(),
            write_unaligned,
            0
        );
        mech_trace::gw!(
            "100425b0",
            "100d08c0",
            DAT_100d08c0,
            mech_trace::gr!("100425b0", "100d08c0", DAT_100d08c0).wrapping_add(1)
        );
        mech_trace::rt!("100425b0", data as *mut u16)
    }
}
