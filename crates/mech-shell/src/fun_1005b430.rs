// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c61fc: *mut u8;
    static mut DAT_100c6270: u8;
    static mut DAT_100c6220: u32;
    static mut DAT_100c61e0: u32;
    static mut DAT_100c6160: u32;
    static mut DAT_100c6200: u32;
    static mut DAT_100c6110: u32;
    static mut DAT_100c6130: u32;
    static mut DAT_100c61c0: u32;
    static mut DAT_100c6190: u32;
    static mut DAT_100c6398: *mut u32;
    static mut DAT_100c61b4: *mut u32;
    static mut s_DTBLPLNTPALGTERRCLIMLITEVWSPVWST_10095a38: u8;
    static mut s_Error___Unable_to_allocate_memor_100a5110: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1006fab0(param_1: u32) -> *mut u32;
    fn _wprintf(format: *const i8, ...) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1005b430() -> u32 {
    mech_trace::enter!("1005b430");
    // SAFETY: all globals and callees are the game's own memory and code; the
    // table pointer addresses a buffer large enough for the offsets written,
    // and the allocation is zeroed only over its 0xfa00 bytes.
    unsafe {
        let mut result: u32 = 1;
        mech_trace::gw!("1005b430", "100c61fc", DAT_100c61fc, &raw mut DAT_100c6270);
        let mut i: i32 = 0;
        loop {
            i = i.wrapping_add(0x20);
            let base = mech_trace::gr!("1005b430", "100c61fc", DAT_100c61fc);
            mech_trace::wr!(
                "1005b430",
                base.byte_offset(i as isize - 0x18).cast::<u32>(),
                write_unaligned,
                0xffff_ffff
            );
            if !(i < 0x100) {
                break;
            }
        }
        let s = (&raw mut s_DTBLPLNTPALGTERRCLIMLITEVWSPVWST_10095a38).cast::<u8>();
        let rd =
            |off: usize| mech_trace::rd!("1005b430", s.byte_add(off).cast::<u32>(), read_unaligned);
        mech_trace::gw!("1005b430", "100c6220", DAT_100c6220, rd(4));
        mech_trace::gw!("1005b430", "100c61e0", DAT_100c61e0, rd(0));
        mech_trace::gw!("1005b430", "100c6160", DAT_100c6160, rd(8));
        mech_trace::gw!("1005b430", "100c6200", DAT_100c6200, rd(0xc));
        mech_trace::gw!("1005b430", "100c6110", DAT_100c6110, rd(0x10));
        mech_trace::gw!("1005b430", "100c6130", DAT_100c6130, rd(0x14));
        let v = rd(0x18);
        mech_trace::wr!(
            "1005b430",
            mech_trace::gr!("1005b430", "100c61fc", DAT_100c61fc).cast::<u32>(),
            write_unaligned,
            v
        );
        mech_trace::gw!("1005b430", "100c61c0", DAT_100c61c0, rd(0x20));
        mech_trace::gw!("1005b430", "100c6190", DAT_100c6190, rd(0x1c));
        mech_trace::gw!(
            "1005b430",
            "100c6398",
            DAT_100c6398,
            mech_trace::cr!("1005b430", "1006fab0", FUN_1006fab0(0xfa00))
        );
        if mech_trace::gr!("1005b430", "100c6398", DAT_100c6398).is_null() {
            result = 0;
            mech_trace::cr!(
                "1005b430",
                "_wprintf",
                _wprintf(&raw const s_Error___Unable_to_allocate_memor_100a5110)
            );
        } else {
            let mut p = mech_trace::gr!("1005b430", "100c6398", DAT_100c6398);
            let mut n: i32 = 16000;
            while n != 0 {
                mech_trace::wr!("1005b430", p, write_unaligned, 0);
                p = p.add(1);
                n -= 1;
            }
        }
        mech_trace::gw!(
            "1005b430",
            "100c61b4",
            DAT_100c61b4,
            mech_trace::gr!("1005b430", "100c6398", DAT_100c6398)
        );
        mech_trace::rt!("1005b430", result)
    }
}
