// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut s_Tracker_1009611c: [i8; 0];
    static mut s_tracker_10096124: [i8; 0];
    static mut s_SideWinder_1009612c: [i8; 0];
    static mut s_sidewind_10096138: [i8; 0];
    static mut s_Flightstick_Pro_10096144: [i8; 0];
    static mut s_fltstick_10096154: [i8; 0];
    static mut s_Thrustmaster_10096160: [i8; 0];
    static mut s_tmaster_10096170: [i8; 0];
    static mut s_joystick_10096178: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    #[link_name = "strstr"]
    fn _strstr(param_1: *const i8, param_2: *const i8) -> *mut i8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003fd30(param_1: i32, param_2: i32) {
    // SAFETY: the globals, strstr and both pointers are the game's own memory;
    // reads and writes happen at the original's offsets and order.
    unsafe {
        let u_var1: u16 = ((param_2.wrapping_add(0x60)) as *const u16).read_unaligned();
        let name = param_1.wrapping_add(0xc) as *const i8;
        let d4 = param_1.wrapping_add(0x4c) as *mut u32;
        let d8 = param_1.wrapping_add(0x50) as *mut u32;
        let d12 = param_1.wrapping_add(0x54) as *mut i8;
        let m3c = (param_2.wrapping_add(0x3c)) as *const i32;

        let mut p = _strstr(name, &raw const s_Tracker_1009611c as *const i8);
        if !p.is_null() {
            let s = &raw const s_tracker_10096124 as *const u8;
            d4.write_unaligned((s as *const u32).read_unaligned());
            d8.write_unaligned((s.add(4) as *const u32).read_unaligned());
            return;
        }
        p = _strstr(name, &raw const s_SideWinder_1009612c as *const i8);
        if !p.is_null()
            || ((u_var1 & 1) != 0
                && (u_var1 & 2) != 0
                && (u_var1 & 0x10) != 0
                && (u_var1 & 0x20) != 0
                && m3c.read_unaligned() == 8)
        {
            let s = &raw const s_sidewind_10096138 as *const u8;
            d4.write_unaligned((s as *const u32).read_unaligned());
            d8.write_unaligned((s.add(4) as *const u32).read_unaligned());
            d12.write(s.add(8).read() as i8);
            return;
        }
        p = _strstr(name, &raw const s_Flightstick_Pro_10096144 as *const i8);
        if !p.is_null()
            || ((u_var1 & 1) != 0
                && (u_var1 & 0x10) != 0
                && (u_var1 & 0x20) != 0
                && m3c.read_unaligned() == 4)
        {
            let s = &raw const s_fltstick_10096154 as *const u8;
            d4.write_unaligned((s as *const u32).read_unaligned());
            d8.write_unaligned((s.add(4) as *const u32).read_unaligned());
            d12.write(s.add(8).read() as i8);
            return;
        }
        p = _strstr(name, &raw const s_Thrustmaster_10096160 as *const i8);
        if p.is_null()
            && ((u_var1 & 2) == 0
                || (u_var1 & 0x10) == 0
                || (u_var1 & 0x20) == 0
                || m3c.read_unaligned() != 4)
        {
            let s = &raw const s_joystick_10096178 as *const u8;
            d4.write_unaligned((s as *const u32).read_unaligned());
            d8.write_unaligned((s.add(4) as *const u32).read_unaligned());
            d12.write(s.add(8).read() as i8);
            return;
        }
        let s = &raw const s_tmaster_10096170 as *const u8;
        d4.write_unaligned((s as *const u32).read_unaligned());
        d8.write_unaligned((s.add(4) as *const u32).read_unaligned());
    }
}
