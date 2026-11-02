// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1008d150: *mut core::ffi::c_void;
    static mut DAT_1008d55c: *mut core::ffi::c_void;
    static mut DAT_100d1540: i32;
    static mut DAT_100c6060: i32;
    static mut DAT_1008da40: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn GetSystemMetrics(n_index: i32) -> i32;
    fn BitBlt(
        hdc: *mut core::ffi::c_void,
        x: i32,
        y: i32,
        cx: i32,
        cy: i32,
        hdc_src: *mut core::ffi::c_void,
        x1: i32,
        y1: i32,
        rop: u32,
    ) -> i32;
    fn GetLastError() -> u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100640a0(param_1: *const i8, ...);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000e070(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
) -> i32 {
    // SAFETY: the globals and callees are the game's own; the format string
    // address is passed as-is to the game's printf-like function.
    unsafe {
        let rop: u32 = 0xcc0020;
        let cy = param_4.wrapping_sub(param_2).wrapping_add(1);
        let cx = param_3.wrapping_sub(param_1).wrapping_add(1);
        let hdc_src = DAT_1008d55c;
        let x1 = param_1;
        let y1 = param_2;
        let i_var1 = GetSystemMetrics(0xf);
        DAT_100d1540 = BitBlt(
            DAT_1008d150,
            param_1,
            param_2.wrapping_sub(i_var1),
            cx,
            cy,
            hdc_src,
            x1,
            y1,
            rop,
        );
        if DAT_100d1540 == 0 {
            FUN_100640a0(
                (&raw mut DAT_1008da40).cast::<i8>() as *const i8,
                GetLastError(),
            );
        }
        (DAT_100c6060 == DAT_100d1540) as i32 - 1
    }
}
