// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    #[link_name = "FUN_1000a800"]
    fn FUN_1000a800(param_1: i32);
    fn FUN_1000a890(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10031950(param_1: i32) {
    mech_trace::enter!("10031950", param_1);
    let p = (param_1 as u32).wrapping_add(0x288) as *mut i32;
    // SAFETY: param_1 is the address of the game's own object; the field at
    // offset 0x288 is read and written as the original does, and the callees
    // are the game's own functions.
    unsafe {
        if mech_trace::rd!("10031950", p, read_unaligned) != 0 {
            mech_trace::wr!(
                "10031950",
                p,
                write_unaligned,
                mech_trace::rd!("10031950", p, read_unaligned).wrapping_sub(1)
            );
        }
        FUN_1000a800(param_1);
        FUN_1000a890(param_1);
    }
}
