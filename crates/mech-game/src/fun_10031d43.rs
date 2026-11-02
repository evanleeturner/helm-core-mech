// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7b60: u32;
    static mut DAT_100aafd8: u32;
    static mut DAT_100adca0: [u8; 0];
    static mut DAT_100f3040: [u8; 0];
    static mut DAT_101f5300: [u8; 0];
    static mut DAT_100a71b0: [u8; 0];
    static mut DAT_100a730c: u32;
    static mut DAT_100aaf68: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10066209(param_1: i32, param_2: i32) -> i32;
    fn FUN_10065e40(param_1: u32, param_2: i32, param_3: i32) -> i32;
    fn FUN_100662fd(param_1: *mut u32, param_2: *mut u32) -> *mut u32;
    fn FUN_10066004(param_1: u32, param_2: *mut i32, param_3: *mut i32) -> *mut i32;
    fn FUN_100603ff(param_1: i32);
    fn FUN_1003b59a(param_1: i32);
    fn FUN_1003b954(param_1: *mut i32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1005c952();
    fn FUN_1005bcb0();
    fn FUN_1006a46b();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10031d43() {
    mech_trace::enter!("10031d43");
    let mut local_10: [u32; 3] = [0; 3];
    // SAFETY: the globals, arrays and callees are the game's own memory and
    // code; every address passed is computed as the original does.
    unsafe {
        let mut i: i32 = 0;
        while i < 8 {
            let p = (&raw mut DAT_100f3040 as usize as i32)
                .wrapping_add((i as u32).wrapping_mul(0x14) as i32);
            mech_trace::cr!("10031d43", "10066209", FUN_10066209(p, p));
            mech_trace::cr!(
                "10031d43",
                "10065e40",
                FUN_10065e40(&raw mut DAT_101f5300 as usize as u32, p, p)
            );
            i = i.wrapping_add(1);
        }
        i = 0;
        while i < 5 {
            let p = (&raw mut DAT_100a71b0 as usize as i32)
                .wrapping_add((i as u32).wrapping_mul(5) as i32);
            mech_trace::cr!("10031d43", "10066209", FUN_10066209(p, p));
            mech_trace::cr!(
                "10031d43",
                "10065e40",
                FUN_10065e40(&raw mut DAT_101f5300 as usize as u32, p, p)
            );
            i = i.wrapping_add(1);
        }
        let q = mech_trace::gr!("10031d43", "100a730c", DAT_100a730c);
        mech_trace::cr!(
            "10031d43",
            "100662fd",
            FUN_100662fd(q as usize as *mut u32, q as usize as *mut u32)
        );
        mech_trace::cr!(
            "10031d43",
            "10066004",
            FUN_10066004(
                &raw mut DAT_101f5300 as usize as u32,
                q as usize as *mut i32,
                q as usize as *mut i32,
            )
        );
        i = 0;
        while i < 6 {
            let p =
                (&raw mut DAT_100adca0 as usize as u32).wrapping_add((i as u32).wrapping_mul(2));
            mech_trace::cr!(
                "10031d43",
                "100662fd",
                FUN_100662fd(p as usize as *mut u32, p as usize as *mut u32)
            );
            mech_trace::cr!(
                "10031d43",
                "10066004",
                FUN_10066004(
                    &raw mut DAT_101f5300 as usize as u32,
                    p as usize as *mut i32,
                    p as usize as *mut i32,
                )
            );
            i = i.wrapping_add(1);
        }
        FUN_100603ff(0);
        FUN_1003b59a(mech_trace::gr!("10031d43", "100aaf68", DAT_100aaf68) as i32);
        FUN_1003b954(mech_trace::gr!("10031d43", "100aaf68", DAT_100aaf68) as usize as *mut i32);
        mech_trace::gw!("10031d43", "100a7b60", DAT_100a7b60, 0);
        local_10[0] = mech_trace::gr!("10031d43", "100aafd8", DAT_100aafd8);
        local_10[1] = 0;
        let lp = local_10.as_mut_ptr();
        mech_trace::cr!("10031d43", "100662fd", FUN_100662fd(lp, lp));
        mech_trace::cr!(
            "10031d43",
            "10066004",
            FUN_10066004(
                &raw mut DAT_101f5300 as usize as u32,
                lp.cast::<i32>(),
                lp.cast::<i32>(),
            )
        );
        mech_trace::gw!("10031d43", "100aafd8", DAT_100aafd8, local_10[0]);
        FUN_1005c952();
        FUN_1005bcb0();
        FUN_1006a46b();
    }
}
