// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100eecb0: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10070b00(param_1: i32) {
    mech_trace::enter!("10070b00", param_1);
    // SAFETY: param_1 is the address of the caller's game object, and
    // DAT_100eecb0 is the game's own table; both are accessed at the same
    // offsets and in the same order as the original.
    unsafe {
        let p = param_1 as usize as *mut u8;
        mech_trace::wr!(
            "10070b00",
            p.byte_add(0x8c).cast::<u32>(),
            write_unaligned,
            0
        );
        mech_trace::wr!(
            "10070b00",
            p.byte_add(0x88).cast::<u32>(),
            write_unaligned,
            0xffffffff
        );
        mech_trace::wr!(
            "10070b00",
            p.byte_add(0x84).cast::<u32>(),
            write_unaligned,
            0xffffffff
        );
        let flags = mech_trace::rd!("10070b00", p.byte_add(0x80).cast::<u32>(), read_unaligned);
        mech_trace::wr!(
            "10070b00",
            p.byte_add(0x80).cast::<u32>(),
            write_unaligned,
            flags | 0x10
        );
        let idx = mech_trace::rd!("10070b00", p.byte_add(4).cast::<i32>(), read_unaligned);
        let off = idx.wrapping_mul(4) as isize;
        mech_trace::wr!(
            "10070b00",
            (&raw mut DAT_100eecb0).byte_offset(off).cast::<u32>(),
            write_unaligned,
            0xfffffffe
        );
    }
}
