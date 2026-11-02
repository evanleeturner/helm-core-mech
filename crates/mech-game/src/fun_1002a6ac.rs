// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100eef40: u32;
    static mut DAT_100ef034: u8;
    static mut DAT_10215688: u8;
    static mut DAT_1021568c: u8;
    static mut DAT_10215690: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002954c(param_1: u16) -> i32;
    fn FUN_1000dde1(param_1: i32, param_2: *mut u32, param_3: *mut u32, param_4: *mut u32) -> u8;
    fn FUN_10082490(param_1: u32, param_2: u32, param_3: u32, param_4: u32) -> i32;
    fn FUN_1002a89b(param_1: i32, param_2: i16) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002a6ac(param_1: i32) {
    let mut b_var2 = true;
    let mut local_1c: i32 = 0;
    let mut local_18: u32 = 0;
    let mut local_10: i32 = 0;
    // SAFETY: param_1 is the caller's object pointer; the globals and callees
    // are the game's own memory and code, accessed as the original does.
    unsafe {
        let base = param_1 as usize as *const u8;
        let flags = base.byte_add(0x178).cast::<u16>().read_unaligned();
        let mut local_c = FUN_1002954c(flags);
        let local_14 = base.byte_add(4).cast::<u32>().read_unaligned();
        let local_8 = (flags & 0xff) as u32;
        let u_var3 = flags & 0xf00;
        if u_var3 == 0x100 {
            let i_var1 = local_8.wrapping_mul(0x54) as usize;
            local_10 = (&raw mut DAT_10215688)
                .byte_add(i_var1)
                .cast::<i32>()
                .read_unaligned();
            local_18 = (&raw mut DAT_1021568c)
                .byte_add(i_var1)
                .cast::<u32>()
                .read_unaligned();
            local_1c = (&raw mut DAT_10215690)
                .byte_add(i_var1)
                .cast::<i32>()
                .read_unaligned();
        } else if u_var3 == 0x200 {
            let i_var1 = (&raw mut DAT_100eef40)
                .add(local_8 as usize)
                .read_unaligned() as usize as *const u8;
            local_10 = i_var1.byte_add(0x50).cast::<i32>().read_unaligned();
            local_18 = i_var1.byte_add(0x54).cast::<u32>().read_unaligned();
            local_1c = i_var1.byte_add(0x58).cast::<i32>().read_unaligned();
        } else if u_var3 == 0x400 {
            let p = (&raw mut DAT_100ef034)
                .byte_add(local_8.wrapping_mul(0x40) as usize)
                .cast::<i32>()
                .read_unaligned();
            FUN_1000dde1(
                p,
                (&raw mut local_10).cast::<u32>(),
                &raw mut local_18,
                (&raw mut local_1c).cast::<u32>(),
            );
            local_c = 25000;
        } else {
            b_var2 = false;
        }
        if b_var2 {
            FUN_10082490(
                local_14,
                local_10.wrapping_sub(local_c) as u32,
                local_18,
                local_1c as u32,
            );
            FUN_10082490(
                local_14,
                local_10 as u32,
                local_18,
                local_1c.wrapping_sub(local_c) as u32,
            );
            FUN_10082490(
                local_14,
                local_c.wrapping_add(local_10) as u32,
                local_18,
                local_1c as u32,
            );
            FUN_10082490(
                local_14,
                local_10 as u32,
                local_18,
                local_c.wrapping_add(local_1c) as u32,
            );
            FUN_1002a89b(param_1, 0x100);
        }
    }
}
