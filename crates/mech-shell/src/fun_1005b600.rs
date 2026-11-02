// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a5208: u8;
    static mut DAT_100a53f0: u8;
    static mut DAT_100a596c: i8;
    static mut DAT_100a5960: i8;
    static mut DAT_100a5978: i8;
}

unsafe extern "C" {
    fn sprintf(dest: *mut i8, format: *const i8, ...) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005b600(_param_1: u32, keyboard_entry: *mut i8) -> u32 {
    mech_trace::enter!("1005b600", _param_1, keyboard_entry);
    // SAFETY: keyboard_entry points at the caller's entry record, large enough
    // for the offsets written; the globals and sprintf are the game's own.
    unsafe {
        mech_trace::wr!(
            "1005b600",
            keyboard_entry.byte_add(0x5c).cast::<u32>(),
            write_unaligned,
            0x79
        );
        mech_trace::wr!(
            "1005b600",
            keyboard_entry.byte_add(0x58).cast::<u32>(),
            write_unaligned,
            0
        );
        mech_trace::cr!(
            "1005b600",
            "sprintf",
            sprintf(keyboard_entry, &raw const DAT_100a5960)
        );
        mech_trace::cr!(
            "1005b600",
            "sprintf",
            sprintf(keyboard_entry.byte_add(0xc), &raw const DAT_100a596c)
        );
        mech_trace::cr!(
            "1005b600",
            "sprintf",
            sprintf(keyboard_entry.byte_add(0x4c), &raw const DAT_100a5978)
        );
        mech_trace::wr!(
            "1005b600",
            keyboard_entry.byte_add(0x60).cast::<u32>(),
            write_unaligned,
            0
        );
        mech_trace::wr!(
            "1005b600",
            keyboard_entry.byte_add(0x64).cast::<u32>(),
            write_unaligned,
            0
        );
        mech_trace::wr!(
            "1005b600",
            keyboard_entry.byte_add(0x68).cast::<*mut u8>(),
            write_unaligned,
            &raw mut DAT_100a53f0
        );
        mech_trace::wr!(
            "1005b600",
            keyboard_entry.byte_add(0x6c).cast::<*mut u8>(),
            write_unaligned,
            &raw mut DAT_100a5208
        );
        mech_trace::wr!(
            "1005b600",
            keyboard_entry.byte_add(0x70).cast::<u32>(),
            write_unaligned,
            0
        );
    }
    mech_trace::rt!("1005b600", 0)
}
