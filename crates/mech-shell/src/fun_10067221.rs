// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10067221(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: u32,
    param_6: *mut i32,
) -> u32 {
    let mut local_1c: i32 = 0;
    let mut local_20: i32 = 0;
    let mut local_24: i32 = 0;
    let mut local_28: i32 = 0;

    // SAFETY: param_1 is the address of the caller's sprite data; every
    // read follows the offsets and run-length stream of the original, and
    // param_6 points at four writable i32 values.
    unsafe {
        let off = param_2
            .wrapping_mul(8)
            .wrapping_add(8)
            .wrapping_add(param_1);
        let i_var7 = (off as usize as *const i32)
            .read_unaligned()
            .wrapping_add(param_1);
        let i_var3 = ((i_var7.wrapping_add(8)) as usize as *const i32)
            .read_unaligned()
            .wrapping_add(param_3);
        let i_var10 = ((i_var7.wrapping_add(0xc)) as usize as *const i32).read_unaligned();
        let mut i_var4 = i_var10.wrapping_add(param_4);
        let i_var5 = ((i_var7.wrapping_add(0x14)) as usize as *const i32)
            .read_unaligned()
            .wrapping_add(1);
        let mut i_var6 = i_var5.wrapping_sub(i_var10);
        if i_var6 != 0 && i_var10 <= i_var5 {
            let mut p = i_var7.wrapping_add(0x18) as usize as *const u8;
            local_1c = 0x7fffffff;
            local_20 = 0x7fffffff;
            local_24 = i32::MIN;
            local_28 = i32::MIN;
            loop {
                let mut x = i_var3;
                let mut h = p.read();
                p = p.add(1);
                loop {
                    let flag = (h & 1) != 0;
                    let cnt = h >> 1;
                    if !flag {
                        if cnt == 0 {
                            break;
                        }
                        if x <= local_1c {
                            local_1c = x;
                        }
                        x = x.wrapping_add(cnt as i32);
                        if local_24 <= x {
                            local_24 = x;
                        }
                        if i_var4 <= local_20 {
                            local_20 = i_var4;
                        }
                        if local_28 <= i_var4 {
                            local_28 = i_var4;
                        }
                        h = p.add(1).read();
                        p = p.add(2);
                    } else if cnt != 0 {
                        if x <= local_1c {
                            local_1c = x;
                        }
                        x = x.wrapping_add(cnt as i32);
                        if local_24 <= x {
                            local_24 = x;
                        }
                        if i_var4 <= local_20 {
                            local_20 = i_var4;
                        }
                        if local_28 <= i_var4 {
                            local_28 = i_var4;
                        }
                        h = p.add(cnt as usize).read();
                        p = p.add(cnt as usize + 1);
                    } else {
                        x = x.wrapping_add(p.read() as i32);
                        h = p.add(1).read();
                        p = p.add(2);
                    }
                }
                i_var4 = i_var4.wrapping_add(1);
                i_var6 = i_var6.wrapping_sub(1);
                if i_var6 == 0 {
                    break;
                }
            }
        }
        if (param_5 & 1) != 0 {
            let t = param_3.wrapping_mul(2).wrapping_sub(local_1c);
            local_1c = param_3.wrapping_mul(2).wrapping_sub(local_24);
            local_24 = t;
        }
        if (param_5 & 2) != 0 {
            let t = param_4.wrapping_mul(2).wrapping_sub(local_20);
            local_20 = param_4.wrapping_mul(2).wrapping_sub(local_28);
            local_28 = t;
        }
        param_6.write_unaligned(local_1c);
        param_6.add(1).write_unaligned(local_20);
        param_6.add(2).write_unaligned(local_24);
        param_6.add(3).write_unaligned(local_28);
    }
    0
}
