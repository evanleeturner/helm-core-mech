// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a3638: [i8; 1];
    static mut DAT_100a363c: u16;
}

unsafe extern "C" {
    fn sprintf(dest: *mut i8, format: *mut i8, ...) -> i32;
}

unsafe fn len_of(p: *const i8) -> i32 {
    let mut n: i32 = 0;
    // SAFETY: p points at a NUL-terminated buffer.
    unsafe {
        while *p.offset(n as isize) != 0 {
            n = n.wrapping_add(1);
        }
    }
    n
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10057440(number_likely: u32, out_buffer: i32) -> i32 {
    if out_buffer == 0 {
        return 0;
    }
    let mut reversed: [i8; 32] = [0; 32];
    let mut printed: [i8; 32] = [0; 32];
    let rp = reversed.as_mut_ptr();
    let pp = printed.as_mut_ptr();
    let out = out_buffer as *mut i8;
    // SAFETY: the format global and sprintf are the game's own; the local
    // buffers are 32 bytes and the output buffer is the caller's.
    unsafe {
        sprintf(pp, (&raw mut DAT_100a3638) as *mut i8, number_likely);
        let len = len_of(pp);
        let mut digits: i32 = 0;
        let mut w: i32 = 0;
        let mut src: i32 = len.wrapping_sub(1);
        while ((printed[0] == b'-' as i8) as i32) <= src {
            if digits < 3 {
                digits = digits.wrapping_add(1);
            } else {
                digits = 1;
                *rp.offset(w as isize) = b',' as i8;
                w = w.wrapping_add(1);
            }
            *rp.offset(w as isize) = *pp.offset(src as isize);
            w = w.wrapping_add(1);
            src = src.wrapping_sub(1);
        }
        *rp.offset(w as isize) = 0;
        if printed[0] == b'-' as i8 {
            let l = len_of(rp);
            rp.offset(l as isize)
                .cast::<u16>()
                .write_unaligned(DAT_100a363c);
        }
        let l = len_of(rp);
        let mut cc: i32 = 0;
        if l != 0 {
            loop {
                let l = len_of(rp);
                let idx = l.wrapping_sub(cc);
                cc = cc.wrapping_add(1);
                *out.offset(cc.wrapping_sub(1) as isize) = *rp.offset(idx.wrapping_sub(1) as isize);
                let l2 = len_of(rp);
                if !(cc < l2) {
                    break;
                }
            }
        }
        *out.offset(cc as isize) = 0;
    }
    out_buffer
}
