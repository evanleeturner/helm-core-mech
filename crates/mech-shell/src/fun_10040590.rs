// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1005bb20(param_1: *mut i8) -> *mut i8;
}

#[allow(non_snake_case)]
unsafe extern "thiscall" {
    fn FUN_10050b20(this: *mut u8, param_1: *mut i8) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "thiscall" fn FUN_10040590(
    this: *mut u8,
    param_1: *mut i8,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
) -> *mut i32 {
    mech_trace::enter!(
        "10040590", this, param_1, param_2, param_3, param_4, param_5
    );
    let mut param_2 = param_2;
    // SAFETY: `this` points at the caller's object, param_5 at the game's
    // owner structure, param_1 at a string; all accessed as the original does.
    unsafe {
        mech_trace::wr!("10040590", this.cast::<i32>(), write_unaligned, param_5);
        let v = mech_trace::rd!(
            "10040590",
            ((param_5 as u32).wrapping_add(0x410) as *const i32),
            read_unaligned
        );
        mech_trace::wr!(
            "10040590",
            this.byte_add(0xc).cast::<i32>(),
            write_unaligned,
            v
        );
        if param_1.is_null() {
            core::ptr::write_volatile(core::ptr::null_mut::<u8>(), 0);
        }
        let mut pc = param_1;
        if mech_trace::dr!("10040590", param_1) == b'~' as i8 {
            pc = param_1.add(1);
        }
        pc = mech_trace::cr!("10040590", "1005bb20", FUN_1005bb20(pc));
        mech_trace::wr!(
            "10040590",
            this.byte_add(0x19).cast::<*mut i8>(),
            write_unaligned,
            pc
        );

        let pc = mech_trace::rd!(
            "10040590",
            this.byte_add(0x19).cast::<*mut i8>(),
            read_unaligned
        );
        let owner = mech_trace::rd!("10040590", this.cast::<*mut u8>(), read_unaligned);
        let width = mech_trace::cr!("10040590", "10050b20", FUN_10050b20(owner, pc));
        mech_trace::wr!(
            "10040590",
            this.byte_add(0x21).cast::<i32>(),
            write_unaligned,
            width
        );

        let owner = mech_trace::rd!("10040590", this.cast::<u32>(), read_unaligned);
        let i_var2 = mech_trace::rd!(
            "10040590",
            (owner.wrapping_add(0x40c) as *const i32),
            read_unaligned
        );
        mech_trace::wr!(
            "10040590",
            this.byte_add(0x1d).cast::<i32>(),
            write_unaligned,
            i_var2
        );

        if mech_trace::dr!("10040590", param_1) == b'~' as i8 {
            let t = mech_trace::rd!(
                "10040590",
                this.byte_add(0x21).cast::<i32>(),
                read_unaligned
            );
            param_2 = param_2.wrapping_sub(t / 2);
        }
        mech_trace::wr!(
            "10040590",
            this.byte_add(0x25).cast::<i32>(),
            write_unaligned,
            param_2
        );
        mech_trace::wr!(
            "10040590",
            this.byte_add(0x29).cast::<i32>(),
            write_unaligned,
            param_3
        );
        mech_trace::wr!(
            "10040590",
            this.byte_add(0x31).cast::<i32>(),
            write_unaligned,
            param_3.wrapping_add(i_var2)
        );

        let a = mech_trace::rd!(
            "10040590",
            this.byte_add(0x25).cast::<i32>(),
            read_unaligned
        );
        let b = mech_trace::rd!(
            "10040590",
            this.byte_add(0x21).cast::<i32>(),
            read_unaligned
        );
        mech_trace::wr!(
            "10040590",
            this.byte_add(0x2d).cast::<i32>(),
            write_unaligned,
            a.wrapping_add(b)
        );

        mech_trace::wr!(
            "10040590",
            this.byte_add(0x36).cast::<i32>(),
            write_unaligned,
            -1
        );
        mech_trace::wr!("10040590", this.byte_add(0x35), write, 0);
        mech_trace::wr!(
            "10040590",
            this.byte_add(0x3a).cast::<i32>(),
            write_unaligned,
            -1
        );
        mech_trace::wr!(
            "10040590",
            this.byte_add(0x10).cast::<i32>(),
            write_unaligned,
            0
        );
        mech_trace::wr!("10040590", this.byte_add(0x14), write, 0);
        mech_trace::wr!(
            "10040590",
            this.byte_add(0x15).cast::<i32>(),
            write_unaligned,
            0
        );
        mech_trace::wr!(
            "10040590",
            this.byte_add(4).cast::<i32>(),
            write_unaligned,
            param_4
        );
        mech_trace::wr!(
            "10040590",
            this.byte_add(8).cast::<i32>(),
            write_unaligned,
            param_4
        );
    }
    mech_trace::rt!("10040590", this.cast::<i32>())
}
