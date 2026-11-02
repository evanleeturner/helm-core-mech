// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100baa0c: u32;
    static mut DAT_100c2dec: u32;
    static mut DAT_100c2be0: *const u8;
}

unsafe extern "C" {
    fn fgets(buf: *mut i8, max_count: i32, file: *mut u8) -> *mut i8;
    fn _isctype(c: i32, ctype: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10087076(buf: *mut i8, max_count: i32, file: *mut u8) -> u32 {
    mech_trace::enter!("10087076", buf, max_count, file);
    let mut b_var1 = false;
    // SAFETY: the callees and globals are the game's own; buf points at the
    // caller's buffer, which fgets fills with a NUL-terminated string.
    unsafe {
        loop {
            let pc_var2 = mech_trace::cr!("10087076", "fgets", fgets(buf, max_count, file));
            if pc_var2.is_null() {
                return mech_trace::rt!("10087076", 0);
            }
            mech_trace::gw!(
                "10087076",
                "100baa0c",
                DAT_100baa0c,
                mech_trace::gr!("10087076", "100baa0c", DAT_100baa0c).wrapping_add(1)
            );
            let mut local_8 = buf;
            while mech_trace::dr!("10087076", local_8) != 0 {
                if mech_trace::dr!("10087076", local_8) == 10
                    || mech_trace::dr!("10087076", local_8) == 35
                {
                    mech_trace::dw!("10087076", local_8, 0);
                    break;
                }
                let local_10: u32;
                if mech_trace::gr!("10087076", "100c2dec", DAT_100c2dec) < 2 {
                    let off =
                        (mech_trace::dr!("10087076", local_8) as i32).wrapping_mul(2) as isize;
                    local_10 = (mech_trace::rd!(
                        "10087076",
                        mech_trace::gr!("10087076", "100c2be0", DAT_100c2be0)
                            .byte_offset(off)
                            .cast::<u16>(),
                        read_unaligned
                    ) & 8) as u32;
                } else {
                    local_10 = mech_trace::cr!(
                        "10087076",
                        "_isctype",
                        _isctype(mech_trace::dr!("10087076", local_8) as i32, 8)
                    ) as u32;
                }
                if local_10 == 0 {
                    b_var1 = true;
                }
                local_8 = local_8.add(1);
            }
            if b_var1 {
                return mech_trace::rt!("10087076", 1);
            }
        }
    }
}
