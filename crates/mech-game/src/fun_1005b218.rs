// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b17f9: [u8; 0];
    static mut DAT_100b2b02: [u8; 0];
    static mut DAT_100b2b07: [u8; 0];
    static mut DAT_100b2b0c: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1005ace1(param_1: u32, param_2: i32, param_3: *mut u32, param_4: *mut u8);
}

unsafe fn rd(p: *mut u8, off: usize) -> i32 {
    // SAFETY: p points at the caller's state object.
    unsafe { mech_trace::rd!("1005b218", p.byte_add(off).cast::<i32>(), read_unaligned) }
}

unsafe fn wr(p: *mut u8, off: usize, v: i32) {
    // SAFETY: p points at the caller's state object.
    unsafe {
        mech_trace::wr!(
            "1005b218",
            p.byte_add(off).cast::<i32>(),
            write_unaligned,
            v
        )
    }
}

unsafe extern "C" fn mercsw_inner(in_al: u8, unaff_edi: i32) {
    mech_trace::enter!("1005b218", in_al, unaff_edi);
    let p = unaff_edi as *mut u8;
    // SAFETY: the globals and the callee are the game's own; p is the
    // state object the caller passed in EDI.
    unsafe {
        let buf = (&raw mut DAT_100b17f9).cast::<u8>();
        mech_trace::wr!(
            "1005b218",
            buf.wrapping_offset(rd(p, 8) as isize),
            write,
            in_al
        );
        wr(p, 8, rd(p, 8).wrapping_add(1));
        wr(p, 0x20, rd(p, 0x20).wrapping_sub(1));
        if rd(p, 0x20) == 0 {
            mech_trace::cr!(
                "1005b218",
                "1005ace1",
                FUN_1005ace1(
                    mech_trace::gr!("1005b218", "100b2b0c", DAT_100b2b0c),
                    rd(p, 0xc),
                    buf.cast::<u32>(),
                    rd(p, 0x24) as *mut u8,
                )
            );
            wr(p, 8, 0);
            wr(p, 0x20, rd(p, 0x24));
            if mech_trace::rd!("1005b218", p.byte_add(0x2c), read) == 0 {
                wr(p, 0xc, rd(p, 0xc).wrapping_add(1));
                if rd(p, 0x28) <= rd(p, 0xc) {
                    wr(p, 0xc, 0);
                }
            } else {
                let t = (&raw mut DAT_100b2b02).cast::<u8>();
                let i = mech_trace::rd!("1005b218", p.byte_add(0x2d), read) as usize;
                let b = mech_trace::rd!("1005b218", t.add(i), read) as i32;
                wr(p, 0xc, rd(p, 0xc).wrapping_add(b));
                if rd(p, 0x28) <= rd(p, 0xc) {
                    let n = mech_trace::rd!("1005b218", p.byte_add(0x2d), read).wrapping_add(1);
                    mech_trace::wr!("1005b218", p.byte_add(0x2d), write, n);
                    let t2 = (&raw mut DAT_100b2b07).cast::<u8>();
                    let at = mech_trace::rd!("1005b218", p.byte_add(0x2d), read) as usize;
                    let v = mech_trace::rd!("1005b218", t2.add(at), read) as i32;
                    wr(p, 0xc, v);
                }
            }
        }
    }
}

#[unsafe(naked)]
#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1005b218() {
    core::arch::naked_asm!(
        "push edi",
        "movzx eax, al",
        "push eax",
        "call {inner}",
        "add esp, 8",
        "ret",
        inner = sym mercsw_inner,
    )
}
