// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c6c30: *mut u8;
    static mut s_Ran_out_of_memory_during_copyfil_100a3548: u8;
    static mut DAT_100a356c: u8;
    static mut s_______Can_t_open_the_input_file___100a3570: u8;
    static mut DAT_100a3594: u8;
    static mut s_______Can_t_open_the_output_file_100a3598: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1006fab0(param_1: u32) -> *mut u8;
    fn _fwprintf(file: *mut u8, format: *const u16, ...) -> i32;
    fn _fflush(file: *mut u8) -> i32;
    fn __wfopen(filename: *const i8, mode: *const i8) -> *mut u8;
    fn _fseek(file: *mut u8, offset: i32, origin: i32) -> i32;
    fn FUN_1006f790(dst: *mut u8, size: u32, count: u32, file: *mut u8) -> u32;
    fn FUN_1006ece0(dst: *mut u8, size: u32, count: u32, file: *mut u8) -> u32;
    fn FUN_1006e790(param_1: *mut u8);
    fn _fclose(file: *mut u8) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10056c80(param_1: *const i8, param_2: *const i8) -> u32 {
    mech_trace::enter!("10056c80", param_1, param_2);
    // SAFETY: the callees and globals are the game's own; the pointers come
    // from the caller and are passed through as the original does.
    unsafe {
        let buf = mech_trace::cr!("10056c80", "1006fab0", FUN_1006fab0(0x8000));
        if buf.is_null() {
            mech_trace::cr!(
                "10056c80",
                "_fwprintf",
                _fwprintf(
                    mech_trace::gr!("10056c80", "100c6c30", DAT_100c6c30),
                    (&raw mut s_Ran_out_of_memory_during_copyfil_100a3548) as *const u16,
                )
            );
            let r = mech_trace::cr!(
                "10056c80",
                "_fflush",
                _fflush(mech_trace::gr!("10056c80", "100c6c30", DAT_100c6c30))
            );
            return mech_trace::rt!("10056c80", (r as u32 & 0xffff0000) | 4);
        }
        let f_in = mech_trace::cr!(
            "10056c80",
            "__wfopen",
            __wfopen(param_1, (&raw mut DAT_100a356c) as *const i8)
        );
        if f_in.is_null() {
            mech_trace::cr!(
                "10056c80",
                "_fwprintf",
                _fwprintf(
                    mech_trace::gr!("10056c80", "100c6c30", DAT_100c6c30),
                    (&raw mut s_______Can_t_open_the_input_file___100a3570) as *const u16,
                    param_1,
                )
            );
            let r = mech_trace::cr!(
                "10056c80",
                "_fflush",
                _fflush(mech_trace::gr!("10056c80", "100c6c30", DAT_100c6c30))
            );
            return mech_trace::rt!("10056c80", (r as u32 & 0xffff0000) | 3);
        }
        let f_out = mech_trace::cr!(
            "10056c80",
            "__wfopen",
            __wfopen(param_2, (&raw mut DAT_100a3594) as *const i8)
        );
        if f_out.is_null() {
            mech_trace::cr!(
                "10056c80",
                "_fwprintf",
                _fwprintf(
                    mech_trace::gr!("10056c80", "100c6c30", DAT_100c6c30),
                    (&raw mut s_______Can_t_open_the_output_file_100a3598) as *const u16,
                    param_1,
                )
            );
            let r = mech_trace::cr!(
                "10056c80",
                "_fflush",
                _fflush(mech_trace::gr!("10056c80", "100c6c30", DAT_100c6c30))
            );
            return mech_trace::rt!("10056c80", (r as u32 & 0xffff0000) | 1);
        }
        mech_trace::cr!("10056c80", "_fseek", _fseek(f_in, 0, 0));
        mech_trace::cr!("10056c80", "_fseek", _fseek(f_out, 0, 0));
        loop {
            let count = mech_trace::cr!("10056c80", "1006f790", FUN_1006f790(buf, 1, 0x8000, f_in));
            mech_trace::cr!("10056c80", "1006ece0", FUN_1006ece0(buf, 1, count, f_out));
            if count != 0x8000 {
                break;
            }
        }
        FUN_1006e790(buf);
        mech_trace::cr!("10056c80", "_fclose", _fclose(f_in));
        let r = mech_trace::cr!("10056c80", "_fclose", _fclose(f_out));
        mech_trace::rt!("10056c80", r as u32 & 0xffff0000)
    }
}
