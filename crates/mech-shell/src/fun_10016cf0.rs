// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a6860: u8;
    static mut DAT_100a687c: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _toupper(_C: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10016cf0() {
    mech_trace::enter!("10016cf0");
    // SAFETY: the globals are the game's own table of string pointers; each
    // string is NUL-terminated and modified in place as the original does.
    unsafe {
        let mut pp: *mut *mut u8 = (&raw mut DAT_100a6860) as *mut *mut u8;
        let limit = ((&raw mut DAT_100a687c) as u32).wrapping_add(3);
        loop {
            let mut p: *const u8 = mech_trace::rd!("10016cf0", pp, read_unaligned) as *const u8;
            let mut len: i32 = 0;
            while mech_trace::rd!("10016cf0", p, read) != 0 {
                p = p.add(1);
                len = len.wrapping_add(1);
            }
            let mut i: i32 = 0;
            if 0 < len {
                loop {
                    let s = mech_trace::rd!("10016cf0", pp, read_unaligned);
                    let q = s.byte_add(i as usize);
                    mech_trace::wr!(
                        "10016cf0",
                        q,
                        write,
                        mech_trace::rd!("10016cf0", q, read) ^ 7
                    );
                    let q = mech_trace::rd!("10016cf0", pp, read_unaligned).byte_add(i as usize);
                    i = i.wrapping_add(1);
                    let r = mech_trace::cr!(
                        "10016cf0",
                        "_toupper",
                        _toupper(mech_trace::rd!("10016cf0", q, read) as i8 as i32)
                    );
                    mech_trace::wr!("10016cf0", q, write, r as u8);
                    if i >= len {
                        break;
                    }
                }
            }
            pp = pp.add(1);
            if limit < (pp as u32) {
                return;
            }
        }
    }
}
