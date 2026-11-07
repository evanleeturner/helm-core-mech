// SPDX-License-Identifier: MIT
use core::arch::asm;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aefa8: u32;
    static mut DAT_10165950: [u8; 0];
    static mut DAT_101665d0: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1003c170(param_1: i32, param_2: i32, param_3: i32) -> u32;
    fn FUN_10003d60(param_1: i32, param_2: i32) -> i32;
    fn FUN_10011e60(param_1: i32, param_2: i32, param_3: i32) -> u32;
    fn FUN_10015b30(param_1: i32, param_2: i32) -> i32;
    fn FUN_1003c1f0(
        param_1: *mut u32,
        param_2: *mut u32,
        param_3: *mut i16,
        param_4: u32,
        param_5: u32,
    ) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003b59a(param_1: i32) {
    mech_trace::enter!("1003b59a", param_1);
    let p = param_1 as usize as *mut u8;
    let mut local_28: u32 = 0;
    let mut local_24: u32 = 0;
    let mut local_5c: u32 = 0;
    let mut local_58: u32 = 0;
    let mut local_30: [i16; 2] = [0; 2];
    let mut local_34: [i16; 2] = [0; 2];
    // SAFETY: param_1 points at the game's object; globals and callees are
    // the game's own; offsets are read and written as the original does.
    unsafe {
        let mut local_60 =
            mech_trace::rd!("1003b59a", p.byte_add(0x18).cast::<u32>(), read_unaligned);
        let local_1c = mech_trace::rd!("1003b59a", p.byte_add(0x44).cast::<i32>(), read_unaligned);
        let local_44 =
            mech_trace::rd!("1003b59a", p.byte_add(0x4c).cast::<i16>(), read_unaligned) as i32;
        let local_2c =
            mech_trace::rd!("1003b59a", p.byte_add(0x50).cast::<i16>(), read_unaligned) as i32;
        let local_50 = mech_trace::rd!("1003b59a", p.byte_add(0x34).cast::<i32>(), read_unaligned);
        let local_c = mech_trace::rd!("1003b59a", p.byte_add(0x38).cast::<i32>(), read_unaligned);
        let local_20 = mech_trace::rd!("1003b59a", p.byte_add(0x2c).cast::<i32>(), read_unaligned);
        let local_18 = mech_trace::rd!("1003b59a", p.byte_add(0x30).cast::<i32>(), read_unaligned);

        let local_8 = (local_18.wrapping_add(local_20).wrapping_add(1) >> 1).wrapping_add(local_44);
        mech_trace::wr!(
            "1003b59a",
            p.byte_add(0x8c).cast::<i32>(),
            write_unaligned,
            local_8
        );
        let local_14 = (local_c.wrapping_add(local_50).wrapping_add(1) >> 1).wrapping_add(local_2c);
        mech_trace::wr!(
            "1003b59a",
            p.byte_add(0x90).cast::<i32>(),
            write_unaligned,
            local_14
        );
        let _ = (local_8, local_14);

        let mut local_4c: u32 = (local_18.wrapping_sub(local_20).wrapping_add(1) >> 1) as u32;
        if (local_4c as i32) < 2 {
            local_4c = 1;
        }
        mech_trace::wr!(
            "1003b59a",
            p.byte_add(0x84).cast::<u32>(),
            write_unaligned,
            local_4c
        );
        let mut local_54: i32 = local_c.wrapping_sub(local_50).wrapping_add(1) >> 1;
        if local_54 < 2 {
            local_54 = 1;
        }
        mech_trace::wr!(
            "1003b59a",
            p.byte_add(0x88).cast::<i32>(),
            write_unaligned,
            local_54
        );

        if 0x100000 < (local_60 as i32) {
            local_60 = 0x100000;
        }
        if (local_60 as i32) < 0x8000 {
            local_60 = 0x8000;
        }
        let mut local_64 = mech_trace::cr!(
            "1003b59a",
            "1003c170",
            FUN_1003c170(local_60 as i32, local_4c as i32, local_44)
        );
        let mut local_48 = mech_trace::cr!(
            "1003b59a",
            "1003c170",
            FUN_1003c170(local_60 as i32, local_4c as i32, local_44.wrapping_neg())
        );
        let u_var3 = local_4c;
        let i_var2 = local_54;
        let i_var1 = mech_trace::cr!(
            "1003b59a",
            "10003d60",
            FUN_10003d60(local_60 as i32, local_1c)
        );
        let local_3c = mech_trace::cr!(
            "1003b59a",
            "10011e60",
            FUN_10011e60(i_var1, u_var3 as i32, i_var2)
        ) as i32;
        mech_trace::wr!(
            "1003b59a",
            p.byte_add(0xb0).cast::<i32>(),
            write_unaligned,
            local_3c
        );
        let mut local_10 = mech_trace::cr!(
            "1003b59a",
            "1003c170",
            FUN_1003c170(local_3c, local_54, local_2c)
        );
        let mut local_40 = mech_trace::cr!(
            "1003b59a",
            "1003c170",
            FUN_1003c170(local_3c, local_54, local_2c.wrapping_neg())
        );
        if 799 < (local_64 as i32) {
            local_64 = 799;
        }
        if 799 < (local_48 as i32) {
            local_48 = 799;
        }
        if 799 < (local_10 as i32) {
            local_10 = 799;
        }
        if 799 < (local_40 as i32) {
            local_40 = 799;
        }
        let _ = (local_48, local_40);

        let a = (&raw mut DAT_10165950).cast::<u8>();
        let b = (&raw mut DAT_101665d0).cast::<u8>();

        let i_var2 = mech_trace::cr!(
            "1003b59a",
            "10015b30",
            FUN_10015b30(
                local_60 as i32,
                mech_trace::rd!(
                    "1003b59a",
                    a.byte_add(local_64.wrapping_mul(4) as usize).cast::<i32>(),
                    read_unaligned
                ),
            )
        );
        let t = mech_trace::rd!(
            "1003b59a",
            b.byte_add(local_64.wrapping_mul(4) as usize).cast::<i32>(),
            read_unaligned
        ) >> 0xd;
        mech_trace::wr!(
            "1003b59a",
            p.byte_add(0xa8).cast::<i32>(),
            write_unaligned,
            i_var2.wrapping_add(t)
        );
        let i_var2 = mech_trace::cr!(
            "1003b59a",
            "10015b30",
            FUN_10015b30(
                local_3c,
                mech_trace::rd!(
                    "1003b59a",
                    a.byte_add(local_10.wrapping_mul(4) as usize).cast::<i32>(),
                    read_unaligned
                ),
            )
        );
        let t = mech_trace::rd!(
            "1003b59a",
            b.byte_add(local_10.wrapping_mul(4) as usize).cast::<i32>(),
            read_unaligned
        ) >> 0xd;
        mech_trace::wr!(
            "1003b59a",
            p.byte_add(0xac).cast::<i32>(),
            write_unaligned,
            i_var2.wrapping_add(t)
        );

        local_30[0] = 2;
        local_34[0] = 2;
        let u_var3 = mech_trace::cr!(
            "1003b59a",
            "10003d60",
            FUN_10003d60(local_60 as i32, local_1c)
        ) as u32;
        mech_trace::cr!(
            "1003b59a",
            "1003c1f0",
            FUN_1003c1f0(
                &raw mut local_28,
                &raw mut local_5c,
                local_30.as_mut_ptr(),
                local_4c,
                u_var3,
            )
        );
        mech_trace::cr!(
            "1003b59a",
            "1003c1f0",
            FUN_1003c1f0(
                &raw mut local_24,
                &raw mut local_58,
                local_34.as_mut_ptr(),
                local_4c,
                local_60,
            )
        );

        mech_trace::wr!(
            "1003b59a",
            p.byte_add(0x3c).cast::<i32>(),
            write_unaligned,
            ((local_24 as i32) >> 0x11).wrapping_add(1)
        );
        let v = mech_trace::rd!("1003b59a", p.byte_add(0x40).cast::<u32>(), read_unaligned);
        mech_trace::wr!(
            "1003b59a",
            p.byte_add(0xb4).cast::<u32>(),
            write_unaligned,
            v
        );
        mech_trace::wr!(
            "1003b59a",
            p.byte_add(0xa4).cast::<i16>(),
            write_unaligned,
            local_34[0]
        );
        mech_trace::wr!(
            "1003b59a",
            p.byte_add(0xa6).cast::<i16>(),
            write_unaligned,
            local_30[0]
        );
        mech_trace::wr!(
            "1003b59a",
            p.byte_add(0x9c).cast::<u32>(),
            write_unaligned,
            local_58
        );
        mech_trace::wr!(
            "1003b59a",
            p.byte_add(0xa0).cast::<u32>(),
            write_unaligned,
            local_5c
        );
        mech_trace::wr!(
            "1003b59a",
            p.byte_add(0x94).cast::<u32>(),
            write_unaligned,
            local_24
        );
        mech_trace::wr!(
            "1003b59a",
            p.byte_add(0x98).cast::<u32>(),
            write_unaligned,
            local_28
        );

        if mech_trace::gr!("1003b59a", "100aefa8", DAT_100aefa8) < 1 {
            mech_trace::gw!("1003b59a", "100aefa8", DAT_100aefa8, 1);
        }
        let divisor: u32 = mech_trace::gr!("1003b59a", "100aefa8", DAT_100aefa8).wrapping_mul(0xa0);
        let q: u32;
        mech_trace::log::debug!(
            f = "1003b59a", b = "div", a:% = mech_trace::Args([
                mech_trace::__shown!(divisor),
                mech_trace::__shown!(local_24),
                mech_trace::__shown!(0u32),
            ]); "asm.in"
        );
        asm!(
            "div {d}",
            d = in(reg) divisor,
            inout("eax") local_24 => q,
            inout("edx") 0u32 => _,
        );
        mech_trace::log::debug!(
            f = "1003b59a", b = "div", r = "eax", v:% = mech_trace::__shown!(q); "asm.out"
        );
        mech_trace::wr!(
            "1003b59a",
            p.byte_add(0xb8).cast::<i32>(),
            write_unaligned,
            q as i32
        );
    }
}
