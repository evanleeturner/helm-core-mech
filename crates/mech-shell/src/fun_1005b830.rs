// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a55d8: [u8; 0];
    static mut DAT_100c40c0: u32;
    static mut DAT_100c40c4: u32;
    static mut DAT_100c6080: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn MapVirtualKeyA(code: u32, map_type: u32) -> u32;
    fn GetKeyState(key: i32) -> i16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _tolower(c: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005b830(code: u32) {
    // SAFETY: the globals and callees are the game's own memory and the
    // system's API; the table reads and ring-buffer write are at the
    // offsets the original uses.
    unsafe {
        let mut u_var4: u16 = (&raw mut DAT_100a55d8)
            .cast::<u8>()
            .byte_add(code.wrapping_mul(2) as usize)
            .cast::<u16>()
            .read_unaligned();
        if u_var4 == 0 {
            let u_var2 = MapVirtualKeyA(code, 2);
            let i_var3 = _tolower((u_var2 as i8) as i32);
            u_var4 = i_var3 as u16;
            if u_var4 == 0 {
                return;
            }
        }
        let mut u_var1 = GetKeyState(0x11) as u16;
        if (u_var1 & 0x1000) != 0 {
            u_var4 |= 0x100;
        }
        u_var1 = GetKeyState(0x12) as u16;
        if (u_var1 & 0x1000) != 0 {
            u_var4 |= 0x400;
        }
        u_var1 = GetKeyState(0x10) as u16;
        if (u_var1 & 0x1000) != 0 {
            u_var4 |= 0x200;
        }
        (&raw mut DAT_100c6080)
            .cast::<u8>()
            .byte_add(DAT_100c40c4.wrapping_mul(2) as usize)
            .cast::<u16>()
            .write_unaligned(u_var4);
        DAT_100c40c4 = DAT_100c40c4.wrapping_add(1);
        if DAT_100c40c4 != 0x40 {
            if DAT_100c40c0 == DAT_100c40c4 {
                DAT_100c40c0 = DAT_100c40c0.wrapping_add(1);
                if DAT_100c40c0 == 0x40 {
                    DAT_100c40c0 = 0;
                }
            }
            return;
        }
        DAT_100c40c4 = 0;
    }
}
