// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b06b8: [*mut i32; 11];
    static mut DAT_101f5300: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10048402(param_1: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1004893c() {
    // SAFETY: the globals, the callee and every pointer reached through the
    // slot table are the game's own memory, read and written where the
    // original does; stored function addresses are cdecl callbacks.
    unsafe {
        let mut local_14: i32 = 0;
        while local_14 < 0xb {
            let slot = (&raw mut DAT_100b06b8)
                .cast::<*mut i32>()
                .add(local_14 as usize);
            let pi_var1 = slot.read();
            if !pi_var1.is_null() {
                let mut local_8: u32 = 0;
                if pi_var1.read_unaligned() != 0 && pi_var1.byte_add(24).read_unaligned() != 0 {
                    let p6 = pi_var1.byte_add(24).read_unaligned() as u32 as *mut u32;
                    p6.write_unaligned((&raw mut DAT_101f5300) as *mut u8 as u32);
                    let p0 = pi_var1.read_unaligned() as u32 as *mut u32;
                    let v = (pi_var1.byte_add(24).read_unaligned() as u32 as *const u32)
                        .read_unaligned();
                    p0.write_unaligned(v);
                    FUN_10048402(pi_var1);
                    let i_var2 = pi_var1.byte_add(0x68).read_unaligned();
                    if i_var2 != 0 {
                        local_8 = 1;
                        let f = ((i_var2 as u32).wrapping_add(0x14) as *const i32).read_unaligned();
                        if f != 0 {
                            let func: unsafe extern "C" fn(*mut i32, i32) -> i32 =
                                core::mem::transmute(f as u32 as usize);
                            local_8 = func(pi_var1, i_var2) as u32;
                            local_8 &= 1;
                        }
                        let i_var3 =
                            ((i_var2 as u32).wrapping_add(0xc) as *const i32).read_unaligned();
                        let mut local_1c: i32 = 0;
                        while local_1c < i_var3 {
                            let addr = (local_1c as u32)
                                .wrapping_mul(0x14)
                                .wrapping_add(0x28)
                                .wrapping_add(i_var2 as u32);
                            let i_var4 = (addr as *const i32).read_unaligned();
                            if i_var4 != 0 {
                                let g = ((i_var4 as u32).wrapping_add(0x14) as *const i32)
                                    .read_unaligned();
                                if g != 0 {
                                    let func: unsafe extern "C" fn(*mut i32, i32) -> i32 =
                                        core::mem::transmute(g as u32 as usize);
                                    let u_var5 = func(pi_var1, i_var4) as u32;
                                    local_8 &= u_var5;
                                }
                            }
                            local_1c = local_1c.wrapping_add(1);
                        }
                    }
                }
                if local_8 != 1 {
                    slot.write(core::ptr::null_mut());
                }
            }
            local_14 = local_14.wrapping_add(1);
        }
    }
}
