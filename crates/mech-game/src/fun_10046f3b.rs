// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b04b0: u32;
    static mut DAT_100b04c4: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn mciSendCommandA(mci_id: u32, u_msg: u32, dw_param1: u32, dw_param2: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10046f3b(param_1: *mut u32) {
    mech_trace::enter!("10046f3b", param_1);
    let mut local_14: [u32; 4] = [0; 4];
    // SAFETY: the globals and the winmm import are the game's own; param_1
    // points at the caller's four u32 slots, written only on success.
    unsafe {
        if mech_trace::gr!("10046f3b", "100b04c4", DAT_100b04c4) != 0 {
            local_14[2] = 2;
            let m_var1 = mech_trace::cr!(
                "10046f3b",
                "mciSendCommandA",
                mciSendCommandA(
                    mech_trace::gr!("10046f3b", "100b04b0", DAT_100b04b0),
                    0x814,
                    0x100,
                    (&raw mut local_14) as *mut u32 as u32,
                )
            );
            if m_var1 == 0 {
                mech_trace::wr!("10046f3b", param_1, write_unaligned, local_14[1] & 0xff);
                mech_trace::wr!(
                    "10046f3b",
                    param_1.byte_add(4),
                    write_unaligned,
                    (local_14[1] >> 8) & 0xff
                );
                mech_trace::wr!(
                    "10046f3b",
                    param_1.byte_add(8),
                    write_unaligned,
                    (local_14[1] >> 0x10) & 0xff
                );
                mech_trace::wr!(
                    "10046f3b",
                    param_1.byte_add(12),
                    write_unaligned,
                    local_14[1] >> 0x18
                );
            }
        }
    }
}
