// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000ece0(param_1: i32, param_2: u32) -> u32 {
    mech_trace::enter!("1000ece0", param_1, param_2);
    // SAFETY: param_1 is the address of a caller-owned structure with
    // fields at offsets 0x44 and 0x48, read and written as the original does.
    unsafe {
        let p48 = (param_1 as usize).wrapping_add(0x48) as *mut u32;
        if mech_trace::rd!("1000ece0", p48, read_unaligned) <= param_2 {
            let v44 = mech_trace::rd!(
                "1000ece0",
                ((param_1 as usize).wrapping_add(0x44) as *const i32),
                read_unaligned
            );
            mech_trace::wr!(
                "1000ece0",
                p48,
                write_unaligned,
                (v44 as u32).wrapping_add(param_2)
            );
            return mech_trace::rt!("1000ece0", 0);
        }
        mech_trace::rt!("1000ece0", 0xffff_ffff)
    }
}
