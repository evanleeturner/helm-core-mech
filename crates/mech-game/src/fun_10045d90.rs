// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10045edc();
    fn FUN_10045ee7();
    fn FUN_10045efb();
    fn FUN_10045f0f();
    fn FUN_10045f36();
    fn FUN_10045f9b();
    fn FUN_10045faf();
    fn FUN_10045fc6();
    fn FUN_10045fdd();
    fn FUN_1004601c();
    fn FUN_10046031();
    fn FUN_10046045();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10045d90(param_1: i32) {
    mech_trace::enter!("10045d90", param_1);
    let base = param_1 as usize as *mut u8;
    // SAFETY: param_1 is the address of a caller-owned object of at least
    // 0x100 bytes; fields are written at the same offsets and in the same
    // order as the original.
    unsafe {
        mech_trace::wr!("10045d90", base.cast::<i32>(), write_unaligned, param_1);
        mech_trace::wr!(
            "10045d90",
            base.byte_add(6).cast::<i16>(),
            write_unaligned,
            0
        );
        mech_trace::wr!(
            "10045d90",
            base.byte_add(8).cast::<i32>(),
            write_unaligned,
            0
        );
        mech_trace::wr!(
            "10045d90",
            base.byte_add(0xc).cast::<i32>(),
            write_unaligned,
            -1
        );
        let mut p = base.byte_add(0x10).cast::<i32>();
        let mut i: i32 = 8;
        while i != 0 {
            mech_trace::wr!("10045d90", p, write_unaligned, 0);
            p = p.wrapping_add(1);
            i = i.wrapping_sub(1);
        }
        mech_trace::wr!(
            "10045d90",
            base.byte_add(0xb4).cast::<i16>(),
            write_unaligned,
            0
        );
        mech_trace::wr!(
            "10045d90",
            base.byte_add(0xb6).cast::<i16>(),
            write_unaligned,
            0
        );
        mech_trace::wr!(
            "10045d90",
            base.byte_add(0xb8).cast::<i16>(),
            write_unaligned,
            0
        );
        mech_trace::wr!(
            "10045d90",
            base.byte_add(0xba).cast::<i16>(),
            write_unaligned,
            0
        );
        mech_trace::wr!(
            "10045d90",
            base.byte_add(0xbc).cast::<u32>(),
            write_unaligned,
            FUN_10045d90 as usize as u32
        );
        mech_trace::wr!(
            "10045d90",
            base.byte_add(0xc0).cast::<u32>(),
            write_unaligned,
            FUN_10045edc as usize as u32
        );
        mech_trace::wr!(
            "10045d90",
            base.byte_add(0xc4).cast::<u32>(),
            write_unaligned,
            FUN_10045ee7 as usize as u32
        );
        mech_trace::wr!(
            "10045d90",
            base.byte_add(0xc8).cast::<u32>(),
            write_unaligned,
            FUN_10045efb as usize as u32
        );
        mech_trace::wr!(
            "10045d90",
            base.byte_add(0xcc).cast::<u32>(),
            write_unaligned,
            FUN_10045f0f as usize as u32
        );
        mech_trace::wr!(
            "10045d90",
            base.byte_add(0xd0).cast::<u32>(),
            write_unaligned,
            FUN_10045f36 as usize as u32
        );
        mech_trace::wr!(
            "10045d90",
            base.byte_add(0xd4).cast::<u32>(),
            write_unaligned,
            FUN_10045f9b as usize as u32
        );
        mech_trace::wr!(
            "10045d90",
            base.byte_add(0xd8).cast::<u32>(),
            write_unaligned,
            FUN_10045faf as usize as u32
        );
        mech_trace::wr!(
            "10045d90",
            base.byte_add(0xdc).cast::<u32>(),
            write_unaligned,
            FUN_10045fc6 as usize as u32
        );
        mech_trace::wr!(
            "10045d90",
            base.byte_add(0xe0).cast::<u32>(),
            write_unaligned,
            FUN_10045fdd as usize as u32
        );
        mech_trace::wr!(
            "10045d90",
            base.byte_add(0xe4).cast::<u32>(),
            write_unaligned,
            FUN_1004601c as usize as u32
        );
        mech_trace::wr!(
            "10045d90",
            base.byte_add(0xe8).cast::<u32>(),
            write_unaligned,
            FUN_10046031 as usize as u32
        );
        mech_trace::wr!(
            "10045d90",
            base.byte_add(0xec).cast::<u32>(),
            write_unaligned,
            FUN_10046045 as usize as u32
        );
        mech_trace::wr!(
            "10045d90",
            base.byte_add(0xf0).cast::<i32>(),
            write_unaligned,
            0
        );
        mech_trace::wr!(
            "10045d90",
            base.byte_add(0xf4).cast::<i32>(),
            write_unaligned,
            0
        );
        mech_trace::wr!(
            "10045d90",
            base.byte_add(0xf8).cast::<i32>(),
            write_unaligned,
            0
        );
        mech_trace::wr!(
            "10045d90",
            base.byte_add(0xfc).cast::<i32>(),
            write_unaligned,
            0
        );
    }
}
