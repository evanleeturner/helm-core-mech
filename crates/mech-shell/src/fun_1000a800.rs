// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000a690(record: i32) -> u8;
    fn FUN_1000a180(record: i32) -> *mut u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000a800(record: i32) {
    // SAFETY: record is an address of the game's record; the callees are the
    // game's own, and fields are read and written where the original does.
    unsafe {
        let p = |off: i32| record.wrapping_add(off) as *mut i32;
        FUN_1000a690(record);
        if p(0x280).read_unaligned() < p(0x288).read_unaligned() {
            loop {
                FUN_1000a180(p(0x288).read_unaligned().wrapping_add(0x2134));
                let lowered_count = p(0x288).read_unaligned().wrapping_sub(1);
                p(0x288).write_unaligned(lowered_count);
                if !(p(0x280).read_unaligned() < lowered_count) {
                    break;
                }
            }
        }
        if p(0x200).read_unaligned() < 0x157d {
            p(0x234).write_unaligned(0x32);
        } else if p(0x200).read_unaligned() < 0x2135 {
            p(0x234).write_unaligned(100);
        } else {
            p(0x234).write_unaligned(200);
        }
        p(0x238).write_unaligned(
            p(0x288)
                .read_unaligned()
                .wrapping_mul(p(0x234).read_unaligned()),
        );
    }
}
