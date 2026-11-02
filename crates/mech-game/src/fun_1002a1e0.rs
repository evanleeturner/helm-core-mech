// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100eef40: u32;
    static mut DAT_100ef030: u8;
    static mut DAT_10215694: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002a1e0(param_1: u32, param_2: i32) -> u32 {
    let mut local_18: *const u16 = core::ptr::null();
    let mut local_10: u32 = 1;
    let mut local_c: u16 = 0;
    let u_var1 = param_1 & 0xff00;
    let u_var2 = param_1 & 0xff;
    if (param_1 & 0xf000) != 0 {
        return 1;
    }
    // SAFETY: the globals are the game's own tables; indices come from the
    // masked parameter exactly as in the original.
    unsafe {
        if u_var1 == 0x100 {
            local_18 = (&raw mut DAT_10215694)
                .byte_add((u_var2 as usize).wrapping_mul(0x54))
                .cast::<u16>();
        } else if u_var1 == 0x200 {
            let v = (&raw mut DAT_100eef40)
                .add(u_var2 as usize)
                .read_unaligned();
            local_18 = v.wrapping_add(0x14) as *const u16;
        } else if u_var1 == 0x400 {
            local_18 = (&raw mut DAT_100ef030)
                .byte_add((u_var2 as usize).wrapping_mul(0x40))
                .cast::<u16>();
        }
    }
    if local_18.is_null() {
        return 1;
    }
    let mut apply = false;
    if param_2 < 5 {
        if !(param_2 != 4 && (param_2 < 1 || 2 < param_2)) {
            apply = true;
        }
    } else if param_2 == 8 || param_2 == 0x100 {
        if u_var1 == 0x200 {
            local_c = 0x20;
        } else if u_var1 == 0x400 {
            local_c = 0x20;
        } else if u_var1 == 0x100 {
            local_c = 0x20;
        }
    } else if param_2 == 0x1000 {
        apply = true;
    }
    if apply {
        if u_var1 == 0x200 {
            local_c = 4;
        } else if u_var1 == 0x400 {
            local_c = 4;
        } else if u_var1 == 0x100 {
            local_18 = core::ptr::null();
            local_10 = 0;
        }
    }
    if !local_18.is_null() {
        // SAFETY: local_18 points into the game's table entry.
        let w = unsafe { local_18.read_unaligned() };
        if (local_c & w) == 0 {
            local_10 = 0;
        }
    }
    local_10
}
