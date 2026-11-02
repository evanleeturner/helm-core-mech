// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn auxGetNumDevs() -> u32;
    fn auxGetDevCapsA(uDeviceID: u32, pac: *mut u32, cbac: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10046060() -> u32 {
    mech_trace::enter!("10046060");
    let mut b_var1 = false;
    let mut local_44: u32 = 0xffff_ffff;
    let mut local_38: [u32; 12] = [0; 12];
    // SAFETY: the imported audio API is the system's own.
    let local_8: u32 = unsafe { mech_trace::cr!("10046060", "auxGetNumDevs", auxGetNumDevs()) };
    if 0 < local_8 as i32 {
        let mut local_3c: u32 = 0;
        while (local_3c as i32) < (local_8 as i32) && !b_var1 {
            // SAFETY: local_38 is a 0x30-byte buffer, as the call is told.
            let m_var2 = unsafe {
                mech_trace::cr!(
                    "10046060",
                    "auxGetDevCapsA",
                    auxGetDevCapsA(local_3c, local_38.as_mut_ptr(), 0x30)
                )
            };
            if m_var2 != 0 {
                return mech_trace::rt!("10046060", 0xffff_ffff);
            }
            if (local_38[10] & 0xffff) == 1 {
                b_var1 = true;
                local_44 = local_3c;
            }
            local_3c = local_3c.wrapping_add(1);
        }
        if !b_var1 {
            local_3c = 0;
            loop {
                if (local_8 as i32) <= (local_3c as i32) {
                    return mech_trace::rt!("10046060", local_44);
                }
                if b_var1 {
                    return mech_trace::rt!("10046060", local_44);
                }
                // SAFETY: local_38 is a 0x30-byte buffer, as the call is told.
                let m_var2 = unsafe {
                    mech_trace::cr!(
                        "10046060",
                        "auxGetDevCapsA",
                        auxGetDevCapsA(local_3c, local_38.as_mut_ptr(), 0x30)
                    )
                };
                if m_var2 != 0 {
                    break;
                }
                if (local_38[10] & 0xffff) == 2 {
                    b_var1 = true;
                    local_44 = local_3c;
                }
                local_3c = local_3c.wrapping_add(1);
            }
            local_44 = 0xffff_ffff;
        }
    }
    mech_trace::rt!("10046060", local_44)
}
