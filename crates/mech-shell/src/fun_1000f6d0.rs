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

            if (p7c.read_unaligned() & 0x8000_0000) != 0 {
                if (p7c.read_unaligned() & 0x10) != 0 {
                    let this = DAT_100a672c;
                    let a1 = p90.read_unaligned();
                    let a2 = p94.read_unaligned();
                    let a3 = p88.read_unaligned();
                    let a4 = p8c.read_unaligned();
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
            p80.write_unaligned(param_3);
            p84.write_unaligned(param_4);
            p7c.write_unaligned(extra_flags | 0x8000_0100);
            if rate == 0 {
                rate = 10;
            }
            (&raw mut DAT_1008daa4)
                .offset(slot_index.wrapping_mul(0x4c) as isize)
                .cast::<i32>()
                .write_unaligned((1000u32 / rate) as i32);
            let mut call_result = FUN_1000e790(param_2);
            if call_result == 0 {
                call_result = FUN_1000f640(slot_record as i32, param_2);
                if call_result == 0 {
                    DAT_1008e3e0 = 1;
                    call_result = FUN_1000f640(slot_record as i32, param_2);
                    if call_result == 0 {
                        p7c.write_unaligned(0);
                        return -1;
                    }
                }
            } else {
                call_result = FUN_1000f3f0(slot_record, param_2);
                if call_result == 0 {
                    p7c.write_unaligned(0);
                    return -1;
                }
            }
            p90.write_unaligned(p80.read_unaligned());
            p94.write_unaligned(p84.read_unaligned());
            return slot_index;
        }
    }
    -1
}
