// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1008da60: u8;
    static mut DAT_1008da7c: u32;
    static mut DAT_1008da80: u32;
    static mut DAT_1008da84: u32;
    static mut DAT_1008da88: u32;
    static mut DAT_1008da8c: u32;
    static mut DAT_1008da90: u32;
    static mut DAT_1008da94: u32;
    static mut DAT_1008daa4: u8;
    static mut DAT_1008e3e0: u32;
    static mut DAT_100a672c: *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10012f10();
    fn FUN_1000f290(slot_index: i32);
    fn FUN_1000e790(slot_index: u32) -> u32;
    fn FUN_1000f3f0(slot_index: *mut i32, param_2: u32) -> u32;
    fn FUN_1000f640(slot_index: i32, param_2: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000f6d0(
    slot_index: i32,
    param_2: u32,
    param_3: u32,
    param_4: u32,
    extra_flags: u32,
    rate_per_second: u32,
) -> i32 {
    mech_trace::enter!(
        "1000f6d0",
        slot_index,
        param_2,
        param_3,
        param_4,
        extra_flags,
        rate_per_second
    );
    let mut rate = rate_per_second;
    // SAFETY: the globals and callees are the game's own; the slot tables are
    // indexed only after the range check, exactly as the original does.
    unsafe {
        let slot_record = (&raw mut DAT_1008da60)
            .wrapping_offset(slot_index.wrapping_mul(0x13) as isize)
            as *mut i32;
        if -1 < slot_index && slot_index < 0x20 {
            let off = slot_index.wrapping_mul(0x13) as isize;
            let p7c = (&raw mut DAT_1008da7c).offset(off);
            let p80 = (&raw mut DAT_1008da80).offset(off);
            let p84 = (&raw mut DAT_1008da84).offset(off);
            let p88 = (&raw mut DAT_1008da88).offset(off);
            let p8c = (&raw mut DAT_1008da8c).offset(off);
            let p90 = (&raw mut DAT_1008da90).offset(off);
            let p94 = (&raw mut DAT_1008da94).offset(off);

            if (mech_trace::rd!("1000f6d0", p7c, read_unaligned) & 0x8000_0000) != 0 {
                if (mech_trace::rd!("1000f6d0", p7c, read_unaligned) & 0x10) != 0 {
                    let this = mech_trace::gr!("1000f6d0", "100a672c", DAT_100a672c);
                    let a1 = mech_trace::rd!("1000f6d0", p90, read_unaligned);
                    let a2 = mech_trace::rd!("1000f6d0", p94, read_unaligned);
                    let a3 = mech_trace::rd!("1000f6d0", p88, read_unaligned);
                    let a4 = mech_trace::rd!("1000f6d0", p8c, read_unaligned);
                    mech_trace::log::debug!(
                        f = "1000f6d0", b = "10012f10", a:% = mech_trace::Args([
                            mech_trace::__shown!(a1),
                            mech_trace::__shown!(a2),
                            mech_trace::__shown!(a3),
                            mech_trace::__shown!(a4),
                            mech_trace::__shown!(this),
                        ]); "asm.in"
                    );
                    core::arch::asm!(
                        "push {a4}",
                        "push {a3}",
                        "push {a2}",
                        "push {a1}",
                        "call {f}",
                        a1 = in(reg) a1,
                        a2 = in(reg) a2,
                        a3 = in(reg) a3,
                        a4 = in(reg) a4,
                        f = sym FUN_10012f10,
                        inout("ecx") this => _,
                        clobber_abi("C"),
                    );
                }
                FUN_1000f290(slot_index);
            }
            mech_trace::wr!("1000f6d0", p80, write_unaligned, param_3);
            mech_trace::wr!("1000f6d0", p84, write_unaligned, param_4);
            mech_trace::wr!("1000f6d0", p7c, write_unaligned, extra_flags | 0x8000_0100);
            if rate == 0 {
                rate = 10;
            }
            mech_trace::wr!(
                "1000f6d0",
                (&raw mut DAT_1008daa4)
                    .offset(slot_index.wrapping_mul(0x4c) as isize)
                    .cast::<i32>(),
                write_unaligned,
                (1000u32 / rate) as i32
            );
            let mut call_result = mech_trace::cr!("1000f6d0", "1000e790", FUN_1000e790(param_2));
            if call_result == 0 {
                call_result = mech_trace::cr!(
                    "1000f6d0",
                    "1000f640",
                    FUN_1000f640(slot_record as i32, param_2)
                );
                if call_result == 0 {
                    mech_trace::gw!("1000f6d0", "1008e3e0", DAT_1008e3e0, 1);
                    call_result = mech_trace::cr!(
                        "1000f6d0",
                        "1000f640",
                        FUN_1000f640(slot_record as i32, param_2)
                    );
                    if call_result == 0 {
                        mech_trace::wr!("1000f6d0", p7c, write_unaligned, 0);
                        return mech_trace::rt!("1000f6d0", -1);
                    }
                }
            } else {
                call_result =
                    mech_trace::cr!("1000f6d0", "1000f3f0", FUN_1000f3f0(slot_record, param_2));
                if call_result == 0 {
                    mech_trace::wr!("1000f6d0", p7c, write_unaligned, 0);
                    return mech_trace::rt!("1000f6d0", -1);
                }
            }
            mech_trace::wr!(
                "1000f6d0",
                p90,
                write_unaligned,
                mech_trace::rd!("1000f6d0", p80, read_unaligned)
            );
            mech_trace::wr!(
                "1000f6d0",
                p94,
                write_unaligned,
                mech_trace::rd!("1000f6d0", p84, read_unaligned)
            );
            return mech_trace::rt!("1000f6d0", slot_index);
        }
    }
    mech_trace::rt!("1000f6d0", -1)
}
