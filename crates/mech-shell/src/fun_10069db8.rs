// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa2c4: u32;
    static mut DAT_100a8fb1: [u8; 1];
    static mut DAT_100aa2ba: [u8; 1];
    static mut DAT_100aa2bf: [u8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10069881(param_1: u32, param_2: i32, param_3: *mut u32, param_4: *mut u8);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10069db8() {
    mech_trace::enter!("10069db8");
    let mut eax_tmp: u32 = 0;
    let mut edi: u32 = 0;
    // SAFETY: captures the register inputs (AL, EDI) the caller set up.
    unsafe {
        core::arch::asm!("", out("eax") eax_tmp, out("edi") edi, options(nomem, nostack, preserves_flags));
    }
    let in_al = eax_tmp as u8;
    let p = edi as usize as *mut u8;
    // SAFETY: EDI points at the caller's object; the globals are the game's own
    // buffers and tables, accessed at the offsets the original uses.
    unsafe {
        let buf = (&raw mut DAT_100a8fb1).cast::<u8>();
        let step = (&raw mut DAT_100aa2ba).cast::<u8>();
        let start = (&raw mut DAT_100aa2bf).cast::<u8>();

        let idx = mech_trace::rd!("10069db8", p.byte_add(8).cast::<i32>(), read_unaligned);
        mech_trace::dw!("10069db8", buf.offset(idx as isize), in_al);
        let v = mech_trace::rd!("10069db8", p.byte_add(8).cast::<i32>(), read_unaligned);
        mech_trace::wr!(
            "10069db8",
            p.byte_add(8).cast::<i32>(),
            write_unaligned,
            v.wrapping_add(1)
        );
        let v = mech_trace::rd!("10069db8", p.byte_add(0x20).cast::<i32>(), read_unaligned);
        mech_trace::wr!(
            "10069db8",
            p.byte_add(0x20).cast::<i32>(),
            write_unaligned,
            v.wrapping_sub(1)
        );
        if mech_trace::rd!("10069db8", p.byte_add(0x20).cast::<i32>(), read_unaligned) == 0 {
            FUN_10069881(
                mech_trace::gr!("10069db8", "100aa2c4", DAT_100aa2c4),
                mech_trace::rd!("10069db8", p.byte_add(0xc).cast::<i32>(), read_unaligned),
                buf.cast::<u32>(),
                mech_trace::rd!("10069db8", p.byte_add(0x24).cast::<u32>(), read_unaligned) as usize
                    as *mut u8,
            );
            mech_trace::wr!("10069db8", p.byte_add(8).cast::<u32>(), write_unaligned, 0);
            let v = mech_trace::rd!("10069db8", p.byte_add(0x24).cast::<u32>(), read_unaligned);
            mech_trace::wr!(
                "10069db8",
                p.byte_add(0x20).cast::<u32>(),
                write_unaligned,
                v
            );
            if mech_trace::rd!("10069db8", p.byte_add(0x2c), read) == 0 {
                let v = mech_trace::rd!("10069db8", p.byte_add(0xc).cast::<i32>(), read_unaligned);
                mech_trace::wr!(
                    "10069db8",
                    p.byte_add(0xc).cast::<i32>(),
                    write_unaligned,
                    v.wrapping_add(1)
                );
                if mech_trace::rd!("10069db8", p.byte_add(0x28).cast::<i32>(), read_unaligned)
                    <= mech_trace::rd!("10069db8", p.byte_add(0xc).cast::<i32>(), read_unaligned)
                {
                    mech_trace::wr!(
                        "10069db8",
                        p.byte_add(0xc).cast::<u32>(),
                        write_unaligned,
                        0
                    );
                }
            } else {
                let v = mech_trace::rd!("10069db8", p.byte_add(0xc).cast::<i32>(), read_unaligned);
                let s = mech_trace::dr!(
                    "10069db8",
                    step.add(mech_trace::rd!("10069db8", p.byte_add(0x2d), read) as usize)
                );
                mech_trace::wr!(
                    "10069db8",
                    p.byte_add(0xc).cast::<i32>(),
                    write_unaligned,
                    v.wrapping_add(s as i32)
                );
                if mech_trace::rd!("10069db8", p.byte_add(0x28).cast::<i32>(), read_unaligned)
                    <= mech_trace::rd!("10069db8", p.byte_add(0xc).cast::<i32>(), read_unaligned)
                {
                    let b = mech_trace::rd!("10069db8", p.byte_add(0x2d), read);
                    mech_trace::wr!("10069db8", p.byte_add(0x2d), write, b.wrapping_add(1));
                    let st = mech_trace::dr!(
                        "10069db8",
                        start.add(mech_trace::rd!("10069db8", p.byte_add(0x2d), read) as usize)
                    );
                    mech_trace::wr!(
                        "10069db8",
                        p.byte_add(0xc).cast::<u32>(),
                        write_unaligned,
                        st as u32
                    );
                }
            }
        }
    }
}
