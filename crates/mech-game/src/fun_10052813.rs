// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10052813(param_1: *mut i8) {
    mech_trace::enter!("10052813", param_1);
    let ac_stack_40: [core::mem::MaybeUninit<i8>; 32] = [core::mem::MaybeUninit::uninit(); 32];
    // SAFETY: param_1 points at the caller's byte. The table read at an index
    // up to 127 runs past the 32-byte local, as the original does, and reads
    // whatever the stack holds there.
    unsafe {
        let c = mech_trace::rd!("10052813", param_1, read);
        if c == b'\'' as i8 {
            mech_trace::wr!("10052813", param_1, write, b'"' as i8);
        } else if c < b'<' as i8 {
            if c < b'[' as i8 {
                if c < b'^' as i8 {
                } else {
                    mech_trace::wr!("10052813", param_1, write, c.wrapping_add(b' ' as i8));
                }
            }
        } else {
            let idx = (c as u8) as usize;
            let v = mech_trace::rd!("10052813", ac_stack_40.as_ptr().add(idx), read_volatile);
            mech_trace::wr!("10052813", param_1, write, v.assume_init());
        }

        if mech_trace::rd!("10052813", param_1, read) == b'`' as i8 {
            mech_trace::wr!("10052813", param_1, write, b'~' as i8);
        }
    }
}
