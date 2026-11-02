// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002f37a(param_1: i32, param_2: u32, param_3: u32, param_4: u32) {
    let base = param_1 as *mut u8;
    // SAFETY: param_1 is the address of a caller-owned structure; the fields
    // at offsets 0x30, 0x34, 0x38 and 0x74 are written/read as the original does.
    unsafe {
        base.byte_add(0x30).cast::<u32>().write_unaligned(param_2);
        base.byte_add(0x34).cast::<u32>().write_unaligned(param_3);
        base.byte_add(0x38).cast::<u32>().write_unaligned(param_4);
        let p = base.byte_add(0x74).cast::<u32>();
        p.write_unaligned(p.read_unaligned() | 1);
    }
}
