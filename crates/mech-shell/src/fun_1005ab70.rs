// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static s_MW2DIF_CFG_100a50c0: [i8; 0];
    static DAT_100a50bc: [i8; 0];
    static mut DAT_100a6848: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn __wfopen(filename: *const u8, mode: *const u8) -> *mut u8;
    fn FUN_1006ece0(ptr: *mut u8, size: i32, count: i32, stream: *mut u8) -> i32;
    fn _fclose(stream: *mut u8) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1005ab70() {
    mech_trace::enter!("1005ab70");
    // SAFETY: the globals and the C runtime functions are the game's own;
    // the file handle is checked for null before use.
    unsafe {
        let file = mech_trace::cr!(
            "1005ab70",
            "__wfopen",
            __wfopen(
                (&raw const s_MW2DIF_CFG_100a50c0).cast::<u8>(),
                (&raw const DAT_100a50bc).cast::<u8>(),
            )
        );
        if !file.is_null() {
            mech_trace::cr!(
                "1005ab70",
                "1006ece0",
                FUN_1006ece0((&raw mut DAT_100a6848).cast::<u8>(), 0x17, 1, file)
            );
            mech_trace::cr!("1005ab70", "_fclose", _fclose(file));
        }
    }
}
