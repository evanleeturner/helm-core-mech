// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    #[link_name = "\u{1}_HC6_AIL_set_sample_loop_count@8"]
    fn AIL_set_sample_loop_count(handle: i32, count: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "thiscall" fn FUN_10010e80(sample_object: *mut u8, loop_count: i32) {
    mech_trace::enter!("10010e80", sample_object, loop_count);
    // SAFETY: sample_object points at the caller's sample object; fields are
    // read and written at the original's offsets, and the callee is the game's own.
    unsafe {
        mech_trace::wr!(
            "10010e80",
            sample_object.byte_add(0x10).cast::<i32>(),
            write_unaligned,
            loop_count
        );
        let field_29 = mech_trace::rd!(
            "10010e80",
            sample_object.byte_add(0x29).cast::<i32>(),
            read_unaligned
        );
        if field_29 != 0 {
            AIL_set_sample_loop_count(field_29, loop_count);
        }
    }
}
