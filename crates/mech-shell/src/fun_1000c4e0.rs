// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1008d038: [i8; 0];
    static mut DAT_1008d03c: u16;
}

unsafe extern "C" {
    fn sprintf(dest: *mut i8, format: *const i8, ...) -> i32;
}

unsafe fn c_strlen(p: *const i8) -> usize {
    let mut n = 0usize;
    // SAFETY: p points at a NUL-terminated buffer.
    unsafe {
        while mech_trace::dr!("1000c4e0", p.add(n)) != 0 {
            n += 1;
        }
    }
    n
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000c4e0(value: u32, out_buffer: i32) -> i32 {
    mech_trace::enter!("1000c4e0", value, out_buffer);
    if out_buffer == 0 {
        return mech_trace::rt!("1000c4e0", 0);
    }
    let mut number_text = [0i8; 32];
    let mut reversed = [0i8; 32];
    // SAFETY: the format string is the game's own; the local buffers are
    // the original's scratch space and the output buffer is the caller's.
    unsafe {
        mech_trace::cr!(
            "1000c4e0",
            "sprintf",
            sprintf(
                number_text.as_mut_ptr(),
                (&raw const DAT_1008d038).cast::<i8>(),
                value,
            )
        );
        let len = c_strlen(number_text.as_ptr()) as i32;
        let neg = number_text[0] == b'-' as i8;
        let mut digits_in_group: i32 = 0;
        let mut grouped_len: usize = 0;
        let mut idx = len.wrapping_sub(1);
        while (neg as i32) <= idx {
            if digits_in_group < 3 {
                digits_in_group += 1;
            } else {
                digits_in_group = 1;
                reversed[grouped_len] = b',' as i8;
                grouped_len += 1;
            }
            reversed[grouped_len] = number_text[idx as usize];
            grouped_len += 1;
            idx = idx.wrapping_sub(1);
        }
        reversed[grouped_len] = 0;
        if neg {
            let n = c_strlen(reversed.as_ptr());
            mech_trace::wr!(
                "1000c4e0",
                reversed.as_mut_ptr().add(n).cast::<u16>(),
                write_unaligned,
                mech_trace::gr!("1000c4e0", "1008d03c", DAT_1008d03c)
            );
        }
        let out = out_buffer as usize as *mut u8;
        let rlen = c_strlen(reversed.as_ptr());
        let mut count: usize = 0;
        if rlen != 0 {
            loop {
                let l = c_strlen(reversed.as_ptr());
                let src = l - count;
                mech_trace::dw!("1000c4e0", out.add(count), reversed[src - 1] as u8);
                count += 1;
                if count >= c_strlen(reversed.as_ptr()) {
                    break;
                }
            }
        }
        mech_trace::dw!("1000c4e0", out.add(count), 0);
    }
    mech_trace::rt!("1000c4e0", out_buffer)
}
