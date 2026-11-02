// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1008375c: i32;
    static mut DAT_10083760: i32;
    static mut DAT_10083768: i32;
    static mut DAT_1008376c: i32;
    static mut DAT_10083c78: i32;
    static mut DAT_10083c7c: i32;
    static mut DAT_10083d0c: i32;
    static mut DAT_10082d10: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn __ftol() -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10004850() {
    // SAFETY: the globals are the game's own memory; the table pointers stay
    // within the game's array at the offsets the original uses.
    unsafe {
        let i_var2: i32 = if DAT_1008375c == 0 {
            DAT_10083760.wrapping_shl(4)
        } else {
            __ftol()
        };
        DAT_10083768 = i_var2.wrapping_add(0x31) / 100;
        let mut pi_var3: *mut i32 = &raw mut DAT_10083c7c;
        DAT_1008376c = 0;
        loop {
            let prev = pi_var3.byte_offset(-4).read_unaligned();
            DAT_1008376c = DAT_1008376c.wrapping_add(prev);
            let cur = pi_var3.read_unaligned();
            if -1 < cur {
                DAT_1008376c = DAT_1008376c.wrapping_add(cur);
            }
            pi_var3 = pi_var3.byte_add(0x12);
            if !((pi_var3 as usize) < (&raw mut DAT_10083d0c as usize)) {
                break;
            }
        }
        if DAT_10083768 < DAT_1008376c {
            loop {
                let i_var1 = DAT_10082d10.wrapping_mul(0x12);
                let p78: *mut i32 = (&raw mut DAT_10083c78).byte_offset(i_var1 as isize);
                let i_var2 = p78.read_unaligned();
                if 0 < i_var2 {
                    p78.write_unaligned(i_var2.wrapping_sub(1));
                    DAT_1008376c = DAT_1008376c.wrapping_sub(1);
                }
                if DAT_1008376c <= DAT_10083768 {
                    return;
                }
                let p7c: *mut i32 = (&raw mut DAT_10083c7c).byte_offset(i_var1 as isize);
                if 0 < p7c.read_unaligned() {
                    p7c.write_unaligned(p7c.read_unaligned().wrapping_sub(1));
                    DAT_1008376c = DAT_1008376c.wrapping_sub(1);
                }
                DAT_10082d10 = DAT_10082d10.wrapping_add(1);
                if 7 < DAT_10082d10 {
                    DAT_10082d10 = 0;
                }
                if !(DAT_10083768 < DAT_1008376c) {
                    break;
                }
            }
        }
    }
}
