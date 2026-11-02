// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002f3a7(param_1: i32, param_2: i32, param_3: i32, param_4: i32) {
    mech_trace::enter!("1002f3a7", param_1, param_2, param_3, param_4);
    // SAFETY: param_1 is the caller's object address; fields are read and
    // written at the same offsets as the original.
    unsafe {
        let p = param_1 as usize as *mut u8;
        let a = p.byte_add(0x30).cast::<u32>();
        mech_trace::wr!(
            "1002f3a7",
            a,
            write_unaligned,
            mech_trace::rd!("1002f3a7", a, read_unaligned).wrapping_add(param_2 as u32)
        );
        let b = p.byte_add(0x34).cast::<u32>();
        mech_trace::wr!(
            "1002f3a7",
            b,
            write_unaligned,
            mech_trace::rd!("1002f3a7", b, read_unaligned).wrapping_add(param_3 as u32)
        );
        let c = p.byte_add(0x38).cast::<u32>();
        mech_trace::wr!(
            "1002f3a7",
            c,
            write_unaligned,
            mech_trace::rd!("1002f3a7", c, read_unaligned).wrapping_add(param_4 as u32)
        );
        let d = p.byte_add(0x74).cast::<u32>();
        mech_trace::wr!(
            "1002f3a7",
            d,
            write_unaligned,
            mech_trace::rd!("1002f3a7", d, read_unaligned) | 1
        );
    }
}
