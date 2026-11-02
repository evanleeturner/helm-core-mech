// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000fc30(segment: i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_10002570(segment: u32, param_2: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002637d(segment: *mut i32) {
    // SAFETY: segment points at the caller's segment record of at least 14
    // ints; the callees are the game's own.
    unsafe {
        if segment.add(0xd).read_unaligned() == 0 {
            let length = FUN_1000fc30(
                segment.add(6).read_unaligned(),
                segment.add(7).read_unaligned(),
                segment.add(8).read_unaligned(),
            );
            segment.add(0xc).write_unaligned(length);
            let length = segment.add(0xc).read_unaligned();
            if 0 < length {
                let v = FUN_10002570(segment.add(6).read_unaligned() as u32, length);
                segment.add(9).write_unaligned(v);
                let v = FUN_10002570(segment.add(7).read_unaligned() as u32, length);
                segment.add(10).write_unaligned(v);
                let v = FUN_10002570(segment.add(8).read_unaligned() as u32, length);
                segment.add(0xb).write_unaligned(v);
                segment.add(0xd).write_unaligned(1);
            }
        }
    }
}
