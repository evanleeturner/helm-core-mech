// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10083508: u32;
    static mut DAT_100c11d8: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1000ce20() -> *mut i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1005bb20(param_1: *mut i8) -> *mut i8;
}

unsafe fn c_strlen(mut p: *const i8) -> i32 {
    let mut n: i32 = 0;
    // SAFETY: p points at a NUL-terminated string owned by the game or caller.
    unsafe {
        while mech_trace::rd!("10055350", p, read) != 0 {
            p = p.add(1);
            n = n.wrapping_add(1);
        }
    }
    n
}

unsafe fn append(buf: *mut i8, src: *const i8) {
    // SAFETY: buf is the game's buffer, src a NUL-terminated string; the
    // original does the same strcat.
    unsafe {
        let mut d = buf.offset(c_strlen(buf as *const i8) as isize);
        let mut s = src;
        loop {
            let c = mech_trace::rd!("10055350", s, read);
            mech_trace::wr!("10055350", d, write, c);
            s = s.add(1);
            d = d.add(1);
            if c == 0 {
                break;
            }
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10055350(param_1: *mut i8, param_2: *mut i8) {
    mech_trace::enter!("10055350", param_1, param_2);
    // SAFETY: globals and callees are the game's own; param_1 and param_2 are
    // NUL-terminated strings from the caller; the buffer is written where the
    // original writes.
    unsafe {
        let buf: *mut i8 = &raw mut DAT_100c11d8 as *mut i8;
        let len = c_strlen(param_1 as *const i8);
        let mut i_var8: i32 = 0;
        let mut i_var5: i32 = 0;
        if 0 < len {
            loop {
                let i_var1 = mech_trace::gr!("10055350", "10083508", DAT_10083508);
                let mut c_var2 = mech_trace::rd!("10055350", param_1.offset(i_var8 as isize), read);
                let mut i_var9 = i_var8;
                let mut store = true;
                if c_var2 == b'\\' as i8 {
                    i_var9 = i_var8.wrapping_add(1);
                    let n =
                        mech_trace::rd!("10055350", param_1.offset(i_var9 as isize), read) as u8;
                    match n {
                        b'M' | b'm' => {
                            i_var8 = i_var8.wrapping_add(2);
                            mech_trace::wr!("10055350", buf.offset(i_var5 as isize), write, 0);
                            append(buf, i_var1.wrapping_add(0x102) as usize as *const i8);
                            i_var5 = c_strlen(buf as *const i8);
                            store = false;
                        }
                        b'P' | b'p' => {
                            mech_trace::wr!("10055350", buf.offset(i_var5 as isize), write, 0);
                            i_var8 = i_var8.wrapping_add(2);
                            let p = mech_trace::cr!("10055350", "1000ce20", FUN_1000ce20());
                            if !p.is_null() {
                                append(buf, p as *const i8);
                            }
                            i_var5 = c_strlen(buf as *const i8);
                            store = false;
                        }
                        b'Q' | b'q' => {
                            i_var8 = i_var8.wrapping_add(2);
                            mech_trace::wr!("10055350", buf.offset(i_var5 as isize), write, 0);
                            append(buf, param_2 as *const i8);
                            i_var5 = c_strlen(buf as *const i8);
                            store = false;
                        }
                        b'R' | b'r' => {
                            mech_trace::wr!("10055350", buf.offset(i_var5 as isize), write, 0);
                            i_var8 = i_var8.wrapping_add(2);
                            store = false;
                        }
                        _ => {
                            mech_trace::wr!("10055350", buf.offset(i_var5 as isize), write, 0x5c);
                            i_var5 = i_var5.wrapping_add(1);
                            c_var2 =
                                mech_trace::rd!("10055350", param_1.offset(i_var9 as isize), read);
                        }
                    }
                }
                if store {
                    mech_trace::wr!("10055350", buf.offset(i_var5 as isize), write, c_var2);
                    i_var8 = i_var9.wrapping_add(1);
                    i_var5 = i_var5.wrapping_add(1);
                }
                if !(i_var8 < len) {
                    break;
                }
            }
        }
        mech_trace::wr!("10055350", buf.offset(i_var5 as isize), write, 0);
        mech_trace::cr!("10055350", "1005bb20", FUN_1005bb20(buf));
    }
}
