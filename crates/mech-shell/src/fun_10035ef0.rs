// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10093900: [u8; 0x114];
    static mut DAT_10093a18: [u8; 0x114];
    static mut DAT_100b07fc: u32;
    static mut DAT_100b0800: [u8; 0x114];
    static mut DAT_100b0914: [u8; 0x114];
    static mut PTR_DAT_10093b2c: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10035ef0() {
    mech_trace::enter!("10035ef0");
    // SAFETY: all globals are the game's own memory; the copies move 0x45
    // dwords between the game's buffers, in the original's order.
    unsafe {
        mech_trace::gw!(
            "10035ef0",
            "PTR_DAT_10093b2c",
            PTR_DAT_10093b2c,
            (&raw mut DAT_10093900) as usize as u32
        );
        if mech_trace::gr!("10035ef0", "100b07fc", DAT_100b07fc) == 0 {
            mech_trace::gw!(
                "10035ef0",
                "PTR_DAT_10093b2c",
                PTR_DAT_10093b2c,
                (&raw mut DAT_10093a18) as usize as u32
            );
        }
        let mut src = (&raw mut DAT_100b0800) as usize as *const u32;
        let mut dst = (&raw mut DAT_10093900) as usize as *mut u32;
        let mut n: i32 = 0x45;
        while n != 0 {
            mech_trace::wr!(
                "10035ef0",
                dst,
                write_unaligned,
                mech_trace::rd!("10035ef0", src, read_unaligned)
            );
            src = src.add(1);
            dst = dst.add(1);
            n -= 1;
        }
        src = (&raw mut DAT_100b0914) as usize as *const u32;
        dst = (&raw mut DAT_10093a18) as usize as *mut u32;
        n = 0x45;
        while n != 0 {
            mech_trace::wr!(
                "10035ef0",
                dst,
                write_unaligned,
                mech_trace::rd!("10035ef0", src, read_unaligned)
            );
            src = src.add(1);
            dst = dst.add(1);
            n -= 1;
        }
    }
}
