// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100bf7d0: u32;
    static mut DAT_100c8908: [u8; 0];
    static mut DAT_100c8920: [u8; 0];
    static mut DAT_100c8dc0: [u8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10088e1a() {
    // SAFETY: the globals are the game's own tables; each entry is read at the
    // original's offsets and the vtable slot is called as the original does.
    unsafe {
        let mut local_8: i32 = DAT_100bf7d0 as i32;
        loop {
            let i_var1 = local_8.wrapping_sub(1);
            if local_8 == 0 {
                break;
            }
            local_8 = i_var1;
            let flag = (&raw mut DAT_100c8908)
                .cast::<u8>()
                .byte_add(i_var1.wrapping_mul(4) as isize as usize)
                .cast::<i32>()
                .read_unaligned();
            if flag != 0 {
                let obj = (&raw mut DAT_100c8dc0)
                    .cast::<u8>()
                    .byte_add(i_var1.wrapping_mul(4) as isize as usize)
                    .cast::<i32>()
                    .read_unaligned();
                let fp = ((obj as u32).wrapping_add(0) as usize as *const u8)
                    .cast::<i32>()
                    .read_unaligned();
                let slot = ((fp as u32).wrapping_add(0xc) as usize as *const u32).read_unaligned();
                let f: unsafe extern "C" fn(*mut u8) = core::mem::transmute(slot as usize);
                let arg = (&raw mut DAT_100c8920)
                    .cast::<u8>()
                    .byte_add(i_var1.wrapping_mul(0x74) as isize as usize);
                f(arg);
            }
        }
    }
}
