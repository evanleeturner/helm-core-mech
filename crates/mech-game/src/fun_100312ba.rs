// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10115160: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100311c8(param_1: i32, param_2: u32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100312ba(param_1: i32) {
    let mut buf = [0u32; 44];
    let p = param_1 as *const u8;
    let local_8: u32 = 3;

    // SAFETY: the global and callee are the game's own; param_1 points at a
    // caller-owned structure read at the original's offsets; buf is a local
    // stack block laid out like the original's frame and passed to the callee.
    unsafe {
        DAT_10115160 = 0;

        // Section 1
        let a = p.byte_add(0x14).cast::<i32>().read_unaligned();
        let c = p.byte_add(0x1c).cast::<i32>().read_unaligned();
        let b = p.byte_add(0x18).cast::<i32>().read_unaligned();
        buf[3] = a as u32;
        buf[5] = c as u32;
        buf[4] = b as u32;
        buf[6] = 5;
        buf[7] = 0;
        buf[9] = 0;
        let a = p.byte_add(0x14).cast::<i32>().read_unaligned();
        let d = p.byte_add(0x2c).cast::<i32>().read_unaligned();
        buf[20] = a.wrapping_add(d) as u32;
        let d = p.byte_add(0x2c).cast::<i32>().read_unaligned();
        let c = p.byte_add(0x1c).cast::<i32>().read_unaligned();
        buf[22] = d.wrapping_add(c).wrapping_sub(0x32) as u32;
        buf[21] = p.byte_add(0x18).cast::<i32>().read_unaligned() as u32;
        buf[23] = 0;
        buf[24] = 0;
        buf[26] = 0;
        let a = p.byte_add(0x14).cast::<i32>().read_unaligned();
        let d = p.byte_add(0x2c).cast::<i32>().read_unaligned();
        buf[37] = a.wrapping_add(d) as u32;
        let c = p.byte_add(0x1c).cast::<i32>().read_unaligned();
        let d = p.byte_add(0x2c).cast::<i32>().read_unaligned();
        buf[39] = c.wrapping_sub(d).wrapping_add(0x32) as u32;
        buf[38] = p.byte_add(0x18).cast::<i32>().read_unaligned() as u32;
        buf[40] = 0;
        buf[41] = 0;
        buf[43] = 0;
        FUN_100311c8(buf.as_mut_ptr() as i32, 3);

        // Section 2
        buf[3] = p.byte_add(0x14).cast::<i32>().read_unaligned() as u32;
        buf[5] = p.byte_add(0x1c).cast::<i32>().read_unaligned() as u32;
        buf[4] = p.byte_add(0x18).cast::<i32>().read_unaligned() as u32;
        buf[6] = 5;
        buf[7] = 0;
        buf[9] = 0;
        let a = p.byte_add(0x14).cast::<i32>().read_unaligned();
        let d = p.byte_add(0x2c).cast::<i32>().read_unaligned();
        buf[20] = a.wrapping_add(d).wrapping_sub(0x32) as u32;
        let c = p.byte_add(0x1c).cast::<i32>().read_unaligned();
        let d = p.byte_add(0x2c).cast::<i32>().read_unaligned();
        buf[22] = c.wrapping_sub(d) as u32;
        buf[21] = p.byte_add(0x18).cast::<i32>().read_unaligned() as u32;
        buf[23] = 0;
        buf[24] = 0;
        buf[26] = 0;
        let a = p.byte_add(0x14).cast::<i32>().read_unaligned();
        let d = p.byte_add(0x2c).cast::<i32>().read_unaligned();
        buf[37] = a.wrapping_sub(d).wrapping_add(0x32) as u32;
        let c = p.byte_add(0x1c).cast::<i32>().read_unaligned();
        let d = p.byte_add(0x2c).cast::<i32>().read_unaligned();
        buf[39] = c.wrapping_sub(d) as u32;
        buf[38] = p.byte_add(0x18).cast::<i32>().read_unaligned() as u32;
        buf[40] = 0;
        buf[41] = 0;
        buf[43] = 0;
        FUN_100311c8(buf.as_mut_ptr() as i32, local_8);

        // Section 3
        buf[3] = p.byte_add(0x14).cast::<i32>().read_unaligned() as u32;
        buf[5] = p.byte_add(0x1c).cast::<i32>().read_unaligned() as u32;
        buf[4] = p.byte_add(0x18).cast::<i32>().read_unaligned() as u32;
        buf[6] = 5;
        buf[7] = 0;
        buf[9] = 0;
        let a = p.byte_add(0x14).cast::<i32>().read_unaligned();
        let d = p.byte_add(0x2c).cast::<i32>().read_unaligned();
        buf[20] = a.wrapping_sub(d) as u32;
        let c = p.byte_add(0x1c).cast::<i32>().read_unaligned();
        let d = p.byte_add(0x2c).cast::<i32>().read_unaligned();
        buf[22] = c.wrapping_sub(d).wrapping_add(0x32) as u32;
        buf[21] = p.byte_add(0x18).cast::<i32>().read_unaligned() as u32;
        buf[23] = 0;
        buf[24] = 0;
        buf[26] = 0;
        let a = p.byte_add(0x14).cast::<i32>().read_unaligned();
        let d = p.byte_add(0x2c).cast::<i32>().read_unaligned();
        buf[37] = a.wrapping_sub(d) as u32;
        let d = p.byte_add(0x2c).cast::<i32>().read_unaligned();
        let c = p.byte_add(0x1c).cast::<i32>().read_unaligned();
        buf[39] = d.wrapping_add(c).wrapping_sub(0x32) as u32;
        buf[38] = p.byte_add(0x18).cast::<i32>().read_unaligned() as u32;
        buf[40] = 0;
        buf[41] = 0;
        buf[43] = 0;
        FUN_100311c8(buf.as_mut_ptr() as i32, local_8);

        // Section 4
        buf[3] = p.byte_add(0x14).cast::<i32>().read_unaligned() as u32;
        buf[5] = p.byte_add(0x1c).cast::<i32>().read_unaligned() as u32;
        buf[4] = p.byte_add(0x18).cast::<i32>().read_unaligned() as u32;
        buf[6] = 5;
        buf[7] = 0;
        buf[9] = 0;
        let a = p.byte_add(0x14).cast::<i32>().read_unaligned();
        let d = p.byte_add(0x2c).cast::<i32>().read_unaligned();
        buf[20] = a.wrapping_sub(d).wrapping_add(0x32) as u32;
        let d = p.byte_add(0x2c).cast::<i32>().read_unaligned();
        let c = p.byte_add(0x1c).cast::<i32>().read_unaligned();
        buf[22] = d.wrapping_add(c) as u32;
        buf[21] = p.byte_add(0x18).cast::<i32>().read_unaligned() as u32;
        buf[23] = 0;
        buf[24] = 0;
        buf[26] = 0;
        let a = p.byte_add(0x14).cast::<i32>().read_unaligned();
        let d = p.byte_add(0x2c).cast::<i32>().read_unaligned();
        buf[37] = a.wrapping_add(d).wrapping_sub(0x32) as u32;
        let d = p.byte_add(0x2c).cast::<i32>().read_unaligned();
        let c = p.byte_add(0x1c).cast::<i32>().read_unaligned();
        buf[39] = d.wrapping_add(c) as u32;
        buf[38] = p.byte_add(0x18).cast::<i32>().read_unaligned() as u32;
        buf[40] = 0;
        buf[41] = 0;
        buf[43] = 0;
        FUN_100311c8(buf.as_mut_ptr() as i32, local_8);
    }
}
