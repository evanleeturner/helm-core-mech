// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002fcce(param_1: *mut i32);
    fn FUN_100896cd(param_1: *mut u32, param_2: *mut i32);
    fn FUN_100895ca(param_1: *mut i32, param_2: *mut i32, param_3: *mut u32);
    fn FUN_1002fbc0(param_1: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002fd94(param_1: *mut i32, param_2: *mut i32) {
    let mut local_34: [i32; 12] = [0; 12];
    // SAFETY: param_1 and param_2 point at the caller's game objects; the
    // callees are the game's own and are called in the original's order.
    unsafe {
        if param_1.read_unaligned() != 0 {
            FUN_1002fcce(param_1);
        }
        param_1.write_unaligned(param_2 as i32);
        param_1
            .add(2)
            .write_unaligned(param_2.add(1).read_unaligned());
        param_2.add(1).write_unaligned(param_1 as i32);
        FUN_100896cd(param_2.add(0xf).cast::<u32>(), local_34.as_mut_ptr());
        FUN_100895ca(
            local_34.as_mut_ptr(),
            param_1.add(0xf),
            param_1.add(3).cast::<u32>(),
        );
        param_1
            .add(0x1d)
            .write_unaligned(param_1.add(0x1d).read_unaligned() & 0xfffffffeu32 as i32);
        param_2
            .add(0x1d)
            .write_unaligned(param_2.add(0x1d).read_unaligned() | 1);
        FUN_1002fbc0(param_2);
    }
}
