// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf38: u32;
    static mut DAT_100b3788: i32;
    static mut DAT_100b9f68: u32;
    static mut DAT_100eef40: [u32; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10082d2f(param_1: i32, param_2: i32, param_3: u32) -> u32;
    fn FUN_1008304a(param_1: i32, param_2: i32, param_3: u32) -> u32;
    fn FUN_100832d4(param_1: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100838c2() {
    // SAFETY: the globals and callees are the game's own; all pointers
    // are read and written at the offsets the original uses.
    unsafe {
        let i_var5 = DAT_100b3788;
        let mut local_10: u32 = 0xffffffff;
        let idx = DAT_100adf38;
        let i_var1 = (&raw mut DAT_100eef40)
            .cast::<u32>()
            .add(idx as usize)
            .read_unaligned() as i32;
        let u_var2 = ((i_var1 as u32).wrapping_add(0xdc) as *const u32).read_unaligned();
        let p20 = ((i_var1 as u32).wrapping_add(0x20) as *const i32).read_unaligned();
        let i_var3 = ((p20 as u32).wrapping_add(0x3f0) as *const i32).read_unaligned();
        if DAT_100b3788 != 0 {
            let flags = ((DAT_100b3788 as u32).wrapping_add(2) as *const u16).read_unaligned();
            if flags & 0x100 == 0 {
                let flags2 = ((DAT_100b3788 as u32).wrapping_add(2) as *const u16).read_unaligned();
                if flags2 & 0x200 != 0 {
                    let w =
                        ((DAT_100b3788 as u32).wrapping_add(0x14) as *const u16).read_unaligned();
                    let i_var4 = FUN_1008304a(DAT_100adf38 as i32, w as i32, 0) as i32;
                    if -1 < i_var4 {
                        let w2 =
                            ((i_var5 as u32).wrapping_add(0x14) as *const u16).read_unaligned();
                        local_10 = (w2 as u32) | 0x400;
                    }
                }
            } else {
                let w = ((DAT_100b3788 as u32).wrapping_add(0x14) as *const u16).read_unaligned();
                let i_var4 = FUN_10082d2f(DAT_100adf38 as i32, w as i32, 0) as i32;
                if -1 < i_var4 {
                    let w2 = ((i_var5 as u32).wrapping_add(0x14) as *const u16).read_unaligned();
                    local_10 = (w2 as u32) | 0x200;
                }
            }
            let cur = ((i_var1 as u32).wrapping_add(0xdc) as *const u32).read_unaligned();
            if local_10 != 0xffffffff && cur != local_10 {
                ((i_var1 as u32).wrapping_add(0xdc) as *mut u32).write_unaligned(local_10);
                let b = ((i_var1 as u32).wrapping_add(0x105) as *const u8).read();
                let p20b = ((i_var1 as u32).wrapping_add(0x20) as *const i32).read_unaligned();
                if b & 1 == 0
                    && ((p20b as u32).wrapping_add(0x3f0) as *const i32).read_unaligned() != 0
                {
                    let p4c = ((i_var1 as u32).wrapping_add(0x4c) as *const i32).read_unaligned();
                    ((p4c as u32).wrapping_add(0x43) as *mut u8).write(1);
                }
                let i_var5b = FUN_100832d4(i_var1);
                if i_var5b == 0 {
                    ((i_var1 as u32).wrapping_add(0xdc) as *mut u32).write_unaligned(u_var2);
                    if i_var3 == 1 {
                        let p4c =
                            ((i_var1 as u32).wrapping_add(0x4c) as *const i32).read_unaligned();
                        ((p4c as u32).wrapping_add(0x43) as *mut u8).write(0);
                        let p20c =
                            ((i_var1 as u32).wrapping_add(0x20) as *const i32).read_unaligned();
                        ((p20c as u32).wrapping_add(0x3f0) as *mut u32).write_unaligned(1);
                    }
                }
            }
        }
        DAT_100b9f68 = 1;
    }
}
