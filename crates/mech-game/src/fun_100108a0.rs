// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a9e4c: [i8; 0];
    static mut DAT_100a9e54: [i8; 0];
    static mut DAT_100a9e60: [i8; 0];
    static mut DAT_100a9e6c: u16;
    static mut DAT_100a9e6e: i8;
    static mut DAT_100a9e1c: u32;
    static mut DAT_100c6188: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
    fn vsprintf(buf: *mut i8, fmt: *const i8, args: *const u8) -> i32;
    fn FUN_10001000(param_1: *mut i8);
    fn FUN_1000115d(param_1: *mut i8, format: i32) -> *mut i8;
    fn FUN_10010b8f(param_1: *mut i8);
    fn FUN_10010a8e();
    fn FUN_10010ac1(param_1: *mut i8);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100108a0(param_1: u32, format: *mut i8) {
    mech_trace::enter!("100108a0", param_1, format);
    let mut local_10: [i8; 12] = [0; 12];
    // SAFETY: globals and callees are the game's own; the buffer DAT_100c6188
    // is the game's message buffer; the varargs follow `format` on the cdecl
    // stack, so a pointer just past it is the 32-bit va_list.
    unsafe {
        let buf = &raw mut DAT_100c6188 as *mut i8;
        mech_trace::cr!(
            "100108a0",
            "sprintf",
            sprintf(
                local_10.as_mut_ptr(),
                &raw const DAT_100a9e4c as *const i8,
                param_1,
            )
        );
        FUN_10001000(&raw mut DAT_100a9e54 as *mut i8);
        let pc2 = mech_trace::cr!(
            "100108a0",
            "1000115d",
            FUN_1000115d(local_10.as_mut_ptr(), 1)
        );
        mech_trace::cr!(
            "100108a0",
            "sprintf",
            sprintf(buf, &raw const DAT_100a9e60 as *const i8, param_1, pc2)
        );
        if !format.is_null() {
            let mut n: usize = 0;
            while mech_trace::rd!("100108a0", buf.add(n), read) != 0 {
                n += 1;
            }
            let end = buf.add(n);
            mech_trace::wr!(
                "100108a0",
                end.cast::<u16>(),
                write_unaligned,
                mech_trace::gr!("100108a0", "100a9e6c", DAT_100a9e6c)
            );
            mech_trace::wr!(
                "100108a0",
                end.add(2),
                write,
                mech_trace::gr!("100108a0", "100a9e6e", DAT_100a9e6e)
            );
            let mut m: usize = 0;
            while mech_trace::rd!("100108a0", buf.add(m), read) != 0 {
                m += 1;
            }
            let va = (&raw const format).add(1) as *const u8;
            mech_trace::cr!(
                "100108a0",
                "vsprintf",
                vsprintf(buf.add(m), format as *const i8, va)
            );
        }
        mech_trace::gw!("100108a0", "100a9e1c", DAT_100a9e1c, param_1);
        match param_1 {
            1 | 8 | 10 | 0x12 | 0x16 | 0x17 | 0x1d | 0x1e | 0x1f | 0x20 | 0x25 | 0x27 | 0x28
            | 0x2c | 0x2d | 0x2e | 0x2f | 0x30 | 0x31 | 0x32 | 0x33 | 0x34 | 0x36 | 0x37 | 0x38
            | 0x3d | 0x46 | 0x47 | 0x4b => {
                FUN_10010b8f(buf);
            }
            2 | 3 | 4 | 5 | 6 | 9 | 0xb | 0xd | 0xe | 0xf | 0x13 | 0x14 | 0x15 | 0x18 | 0x19
            | 0x1a | 0x1b | 0x1c | 0x21 | 0x22 | 0x23 | 0x26 | 0x29 | 0x2a | 0x2b | 0x35 | 0x39
            | 0x3a | 0x3b | 0x3c | 0x3e | 0x3f | 0x40 | 0x41 | 0x42 | 0x43 | 0x48 | 0x49 | 0x4a
            | 0x4d | 0x4e | 0x4f | 0x50 | 0x51 | 0x52 | 0x53 => {
                FUN_10010a8e();
                FUN_10010ac1(buf);
            }
            0x10 | 0x11 | 0x44 | 0x45 | 0x4c | 0x54 => {
                FUN_10010ac1(buf);
            }
            _ => {}
        }
    }
}
