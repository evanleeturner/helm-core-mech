// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b34b0: u8;
    static mut DAT_100b34c8: u8;
    static mut DAT_100b34b8: u8;
    static mut DAT_100b34a8: u8;
    static mut DAT_100b3508: i8;
    static mut DAT_100b3500: i8;
    static mut DAT_100b3510: i8;
}

unsafe extern "C" {
    fn sprintf(dest: *mut i8, format: *const i8, ...) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005ec45(param_1: u32, param_2: *mut i8) -> u32 {
    mech_trace::enter!("1005ec45", param_1, param_2);
    let _ = param_1;
    // SAFETY: param_2 points at the caller's buffer, written where the
    // original does; the globals and sprintf are the game's own.
    unsafe {
        mech_trace::wr!("1005ec45", param_2.byte_add(0x58).cast::<u8>(), write, 2);
        mech_trace::wr!("1005ec45", param_2.byte_add(0x59).cast::<u8>(), write, 0);
        mech_trace::wr!("1005ec45", param_2.byte_add(0x5a).cast::<u8>(), write, 0);
        mech_trace::wr!("1005ec45", param_2.byte_add(0x5b).cast::<u8>(), write, 0);
        mech_trace::wr!("1005ec45", param_2.byte_add(0x5c).cast::<u8>(), write, 3);
        mech_trace::wr!("1005ec45", param_2.byte_add(0x5d).cast::<u8>(), write, 0);
        mech_trace::wr!("1005ec45", param_2.byte_add(0x5e).cast::<u8>(), write, 0);
        mech_trace::wr!("1005ec45", param_2.byte_add(0x5f).cast::<u8>(), write, 0);
        mech_trace::cr!(
            "1005ec45",
            "sprintf",
            sprintf(param_2, &raw const DAT_100b3500)
        );
        mech_trace::cr!(
            "1005ec45",
            "sprintf",
            sprintf(param_2.byte_add(0xc), &raw const DAT_100b3508)
        );
        mech_trace::cr!(
            "1005ec45",
            "sprintf",
            sprintf(param_2.byte_add(0x4c), &raw const DAT_100b3510)
        );
        mech_trace::wr!(
            "1005ec45",
            param_2.byte_add(0x60).cast::<*mut u8>(),
            write_unaligned,
            &raw mut DAT_100b34a8
        );
        mech_trace::wr!(
            "1005ec45",
            param_2.byte_add(100).cast::<*mut u8>(),
            write_unaligned,
            &raw mut DAT_100b34b0
        );
        mech_trace::wr!(
            "1005ec45",
            param_2.byte_add(0x68).cast::<*mut u8>(),
            write_unaligned,
            &raw mut DAT_100b34b8
        );
        mech_trace::wr!(
            "1005ec45",
            param_2.byte_add(0x6c).cast::<*mut u8>(),
            write_unaligned,
            &raw mut DAT_100b34c8
        );
        mech_trace::wr!("1005ec45", param_2.byte_add(0x70).cast::<u8>(), write, 0);
        mech_trace::wr!("1005ec45", param_2.byte_add(0x71).cast::<u8>(), write, 0);
        mech_trace::wr!("1005ec45", param_2.byte_add(0x72).cast::<u8>(), write, 0);
        mech_trace::wr!("1005ec45", param_2.byte_add(0x73).cast::<u8>(), write, 0);
    }
    mech_trace::rt!("1005ec45", 0)
}
