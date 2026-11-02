// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c2dec: u32;
    static mut PTR_DAT_100c2be0: u32;
}

unsafe extern "C" {
    fn __isctype(_c: i32, _type: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10070ba0(mut param_1: i32, mut param_2: *const i8) -> u32 {
    mech_trace::enter!("10070ba0", param_1, param_2);
    // SAFETY: globals and the ctype table are the game's own memory; param_2
    // points at the caller's string, read where the original reads.
    unsafe {
        loop {
            let local_c: u32 = if mech_trace::gr!("10070ba0", "100c2dec", DAT_100c2dec) < 2 {
                let idx = (mech_trace::dr!("10070ba0", param_2) as i32).wrapping_mul(2);
                let p = mech_trace::gr!("10070ba0", "100c2be0", PTR_DAT_100c2be0)
                    .wrapping_add(idx as u32) as *const u16;
                (mech_trace::rd!("10070ba0", p, read_unaligned) & 8) as u32
            } else {
                mech_trace::cr!(
                    "10070ba0",
                    "__isctype",
                    __isctype(mech_trace::dr!("10070ba0", param_2) as i32, 8)
                ) as u32
            };
            if local_c == 0 {
                break;
            }
            param_2 = param_2.add(1);
            param_1 = param_1.wrapping_sub(1);
            if param_1 < 1 {
                return mech_trace::rt!("10070ba0", 0xffff_ffff);
            }
        }
        let mut local_8: u32 = 0;
        while 0 < param_1
            && mech_trace::dr!("10070ba0", param_2) > b'/' as i8
            && mech_trace::dr!("10070ba0", param_2) < b'8' as i8
        {
            local_8 = ((mech_trace::dr!("10070ba0", param_2) as i32).wrapping_sub(0x30) as u32)
                | local_8.wrapping_shl(3);
            param_1 = param_1.wrapping_sub(1);
            param_2 = param_2.add(1);
        }
        if param_1 < 1 {
            return mech_trace::rt!("10070ba0", local_8);
        }
        if mech_trace::dr!("10070ba0", param_2) == 0 {
            return mech_trace::rt!("10070ba0", local_8);
        }
        let local_10: u32 = if mech_trace::gr!("10070ba0", "100c2dec", DAT_100c2dec) < 2 {
            let idx = (mech_trace::dr!("10070ba0", param_2) as i32).wrapping_mul(2);
            let p = mech_trace::gr!("10070ba0", "100c2be0", PTR_DAT_100c2be0)
                .wrapping_add(idx as u32) as *const u16;
            (mech_trace::rd!("10070ba0", p, read_unaligned) & 8) as u32
        } else {
            mech_trace::cr!(
                "10070ba0",
                "__isctype",
                __isctype(mech_trace::dr!("10070ba0", param_2) as i32, 8)
            ) as u32
        };
        if local_10 != 0 {
            return mech_trace::rt!("10070ba0", local_8);
        }
        mech_trace::rt!("10070ba0", 0xffff_ffff)
    }
}
