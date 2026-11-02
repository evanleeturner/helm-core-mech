// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b6820: i32;
    static mut DAT_100b681c: i32;
    static mut DAT_100c96b4: i32;
    static mut DAT_100c96b8: i32;
    static mut DAT_100c9ac0: u8;
    static mut DAT_100c9ed0: u8;
    static mut DAT_100c9ed4: u8;
    static mut DAT_100c9ed8: u8;
    static mut DAT_100c9edc: u8;
    static mut DAT_100c9ee0: u8;
    static mut DAT_100c9ee4: u8;
    static mut DAT_100c9ee8: u8;
    static mut DAT_100c9eec: u8;
    static mut DAT_100c9ef0: u8;
    static mut DAT_100c9ef4: u8;
    static mut DAT_100c9ef8: u8;
    static mut DAT_100c9efc: u8;
    static mut DAT_100c9f00: u8;
    static mut DAT_100c9f04: u8;
    static mut DAT_100c9f08: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1007ce33() {
    // SAFETY: the globals are the game's own memory; the tables hold 0x100
    // entries, written at the same offsets and in the same order as the original.
    unsafe {
        DAT_100b6820 = 0;
        DAT_100b681c = 0;
        DAT_100c96b4 = 0x100;
        DAT_100c96b8 = 0xff;
        let mut local_8: i32 = 0;
        while local_8 < 0x100 {
            let i_var1 = (local_8 as u32).wrapping_mul(0x40) as usize;
            (&raw mut DAT_100c9ac0)
                .byte_add((local_8 as u32).wrapping_mul(4) as usize)
                .cast::<i32>()
                .write_unaligned(local_8);
            (&raw mut DAT_100c9ed8)
                .byte_add(i_var1)
                .cast::<u32>()
                .write_unaligned(0);
            (&raw mut DAT_100c9ed0)
                .byte_add(i_var1)
                .cast::<u32>()
                .write_unaligned(0);
            (&raw mut DAT_100c9ed4)
                .byte_add(i_var1)
                .cast::<u32>()
                .write_unaligned(0);
            (&raw mut DAT_100c9edc)
                .byte_add(i_var1)
                .cast::<i32>()
                .write_unaligned(local_8);
            (&raw mut DAT_100c9ee0)
                .byte_add(i_var1)
                .cast::<u32>()
                .write_unaligned(0xffffffff);
            (&raw mut DAT_100c9ee4)
                .byte_add(i_var1)
                .cast::<u32>()
                .write_unaligned(0);
            (&raw mut DAT_100c9ee8)
                .byte_add(i_var1)
                .cast::<u32>()
                .write_unaligned(0);
            (&raw mut DAT_100c9eec)
                .byte_add(i_var1)
                .cast::<u32>()
                .write_unaligned(0);
            (&raw mut DAT_100c9ef0)
                .byte_add(i_var1)
                .cast::<u32>()
                .write_unaligned(0);
            (&raw mut DAT_100c9ef4)
                .byte_add(i_var1)
                .cast::<u32>()
                .write_unaligned(0);
            (&raw mut DAT_100c9ef8)
                .byte_add(i_var1)
                .cast::<u32>()
                .write_unaligned(0);
            (&raw mut DAT_100c9efc)
                .byte_add(i_var1)
                .cast::<u32>()
                .write_unaligned(0);
            (&raw mut DAT_100c9f00)
                .byte_add(i_var1)
                .cast::<u32>()
                .write_unaligned(0);
            (&raw mut DAT_100c9f04)
                .byte_add(i_var1)
                .cast::<u32>()
                .write_unaligned(0);
            (&raw mut DAT_100c9f08)
                .byte_add(i_var1)
                .cast::<u32>()
                .write_unaligned(0xffffffff);
            local_8 = ((local_8 as u32).wrapping_add(1)) as i32;
        }
    }
}
