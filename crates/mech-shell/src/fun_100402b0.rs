// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "fastcall" {
    fn FUN_10040300(text_object: *mut u32) -> u8;
}

#[allow(non_snake_case)]
unsafe extern "thiscall" {
    fn FUN_10050d90(
        this: *mut u8,
        text_object: i32,
        param_2: i32,
        param_3: *mut u8,
        param_4: *mut u8,
    ) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "fastcall" fn FUN_100402b0(text_object: *mut u32) {
    mech_trace::enter!("100402b0", text_object);
    // SAFETY: text_object points at the game's text object; fields are read
    // and written at the offsets the original uses, and the string is scanned
    // up to its terminator.
    unsafe {
        let base = text_object.cast::<u8>();
        let text = mech_trace::rd!(
            "100402b0",
            base.byte_add(0x19).cast::<*const i8>(),
            read_unaligned
        );
        if !text.is_null() {
            let mut countdown: i32 = -1;
            let mut cursor = text;
            loop {
                if countdown == 0 {
                    break;
                }
                countdown = countdown.wrapping_sub(1);
                let c = mech_trace::rd!("100402b0", cursor, read);
                cursor = cursor.add(1);
                if c == 0 {
                    break;
                }
            }
            if countdown != -2 {
                if mech_trace::rd!("100402b0", text_object.byte_add(0x10), read_unaligned) == 1 {
                    mech_trace::cr!("100402b0", "10040300", FUN_10040300(text_object));
                    return;
                }
                let this = mech_trace::rd!("100402b0", text_object, read_unaligned) as *mut u8;
                let a = mech_trace::rd!(
                    "100402b0",
                    base.byte_add(0x25).cast::<i32>(),
                    read_unaligned
                );
                let b = mech_trace::rd!(
                    "100402b0",
                    base.byte_add(0x29).cast::<i32>(),
                    read_unaligned
                );
                let s = mech_trace::rd!(
                    "100402b0",
                    base.byte_add(0x19).cast::<*mut u8>(),
                    read_unaligned
                );
                let p4 = mech_trace::rd!("100402b0", text_object.add(1), read_unaligned) as *mut u8;
                mech_trace::cr!("100402b0", "10050d90", FUN_10050d90(this, a, b, s, p4));
                mech_trace::wr!("100402b0", base.byte_add(0x35), write, 1);
            }
        }
    }
}
