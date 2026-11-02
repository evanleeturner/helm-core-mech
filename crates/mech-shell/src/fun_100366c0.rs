// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b5298: [u32; 0x200];
    static mut DAT_10093c60: u32;
    static mut DAT_10093c64: u32;
    static mut DAT_10093b40: u32;
    static mut DAT_10093b44: u32;
    static mut DAT_10093b48: [u8; 4];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10036660(param_1: *mut u32, param_2: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100366c0() {
    mech_trace::enter!("100366c0");
    let mut local_34: [u32; 3] = [0; 3];
    let mut local_28: [u32; 3] = [0; 3];
    let mut local_1c: [u32; 7] = [0; 7];

    // SAFETY: the globals and the callee are the game's own; the locals are
    // valid buffers of at least the sizes passed to the callee.
    unsafe {
        let mut p = (&raw mut DAT_100b5298).cast::<u32>();
        let mut i: i32 = 0x200;
        while i != 0 {
            mech_trace::wr!("100366c0", p, write, 0);
            p = p.add(1);
            i -= 1;
        }
        mech_trace::gw!("100366c0", "10093c60", DAT_10093c60, 0);
        mech_trace::gw!("100366c0", "10093c64", DAT_10093c64, 0);

        local_28[0] = mech_trace::gr!("100366c0", "10093b40", DAT_10093b40);
        local_28[1] = 0;
        local_28[2] = 0;
        FUN_10036660(local_28.as_mut_ptr(), 0xc);

        local_34[0] = mech_trace::gr!("100366c0", "10093b44", DAT_10093b44);
        local_34[1] = 0xc;
        local_34[2] = u32::from_le_bytes([0x31, 0x2e, 0x32, 0x32]);
        FUN_10036660(local_34.as_mut_ptr(), 0xc);

        local_1c[0] = u32::from_le_bytes(mech_trace::gr!("100366c0", "10093b48", DAT_10093b48));
        local_1c[1] = 0x1c;
        local_1c[2] = 0;
        local_1c[3] = 0;
        local_1c[4] = 0;
        local_1c[5] = 0;
        local_1c[6] = 0;
        FUN_10036660(local_1c.as_mut_ptr(), 0x1c);
    }
}
