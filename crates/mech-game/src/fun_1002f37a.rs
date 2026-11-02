// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002f37a(param_1: i32, param_2: u32, param_3: u32, param_4: u32) {
    mech_trace::enter!("1002f37a", param_1, param_2, param_3, param_4);
    let base = param_1 as *mut u8;
    // SAFETY: param_1 is the address of a caller-owned structure; the fields
    // at offsets 0x30, 0x34, 0x38 and 0x74 are written/read as the original does.
    unsafe {
        mech_trace::wr!(
            "1002f37a",
            base.byte_add(0x30).cast::<u32>(),
            write_unaligned,
            param_2
        );
        mech_trace::wr!(
            "1002f37a",
            base.byte_add(0x34).cast::<u32>(),
            write_unaligned,
            param_3
        );
        mech_trace::wr!(
            "1002f37a",
            base.byte_add(0x38).cast::<u32>(),
            write_unaligned,
            param_4
        );
        let p = base.byte_add(0x74).cast::<u32>();
        mech_trace::wr!(
            "1002f37a",
            p,
            write_unaligned,
            mech_trace::rd!("1002f37a", p, read_unaligned) | 1
        );
    }
}
