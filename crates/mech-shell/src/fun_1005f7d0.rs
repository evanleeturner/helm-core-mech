// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "thiscall" {
    fn FUN_1005f8c0(this: *mut u8, param_1: u32) -> bool;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "thiscall" fn FUN_1005f7d0(this: *mut u8, new_index: i32) {
    mech_trace::enter!("1005f7d0", this, new_index);
    // SAFETY: the callee is the game's own; `this` points at the game's
    // object, whose fields at 0x128, 0x12c and 0x130 are read and written
    // where the original does.
    unsafe {
        if !mech_trace::cr!("1005f7d0", "1005f8c0", FUN_1005f8c0(this, 0x40000000)) {
            let mut new_index = new_index;
            if mech_trace::rd!(
                "1005f7d0",
                this.byte_add(0x130).cast::<i32>(),
                read_unaligned
            ) <= new_index
            {
                new_index = 0;
            }
            mech_trace::wr!(
                "1005f7d0",
                this.byte_add(0x128).cast::<u32>(),
                write_unaligned,
                0xffffffff
            );
            mech_trace::wr!(
                "1005f7d0",
                this.byte_add(0x12c).cast::<i32>(),
                write_unaligned,
                new_index
            );
        }
    }
}
