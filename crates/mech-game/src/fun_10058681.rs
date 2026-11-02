// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10058681(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: u32,
    param_6: *mut i32,
) -> u32 {
    mech_trace::enter!(
        "10058681", param_1, param_2, param_3, param_4, param_5, param_6
    );
    let mut local_1c: i32 = 0;
    let mut local_20: i32 = 0;
    let mut local_24: i32 = 0;
    let mut local_28: i32 = 0;

    // SAFETY: param_1 addresses the game's sprite data and param_6 points at
    // four caller-owned i32 slots; reads and writes follow the original.
    unsafe {
        let i_var7 = mech_trace::rd!(
            "10058681",
            ((param_1
                .wrapping_add(param_2.wrapping_mul(8))
                .wrapping_add(8)) as *const i32),
            read_unaligned
        )
        .wrapping_add(param_1);
        let i_var3 = mech_trace::rd!(
            "10058681",
            ((i_var7.wrapping_add(8)) as *const i32),
            read_unaligned
        )
        .wrapping_add(param_3);
        let mut i_var10 = mech_trace::rd!(
            "10058681",
            ((i_var7.wrapping_add(0xc)) as *const i32),
            read_unaligned
        );
        let mut i_var4 = i_var10.wrapping_add(param_4);
        let i_var5 = mech_trace::rd!(
            "10058681",
            ((i_var7.wrapping_add(0x14)) as *const i32),
            read_unaligned
        )
        .wrapping_add(1);
        let mut i_var6 = i_var5.wrapping_sub(i_var10);
        if i_var6 != 0 && i_var10 <= i_var5 {
            let mut p = i_var7.wrapping_add(0x18) as *const u8;
            local_1c = 0x7fffffff;
            local_20 = 0x7fffffff;
            local_24 = i32::MIN;
            local_28 = i32::MIN;
            loop {
                let h = mech_trace::rd!("10058681", p, read);
                p = p.add(1);
                let mut flag = (h & 1) != 0;
                let mut count = h >> 1;
                i_var10 = i_var3;
                loop {
                    if count == 0 {
                        if !flag {
                            break;
                        }
                        let nxt = p.add(1);
                        i_var10 = i_var10.wrapping_add(mech_trace::rd!("10058681", p, read) as i32);
                        p = p.add(2);
                        let b = mech_trace::rd!("10058681", nxt, read);
                        flag = (b & 1) != 0;
                        count = b >> 1;
                    } else if !flag {
                        let nxt = p.add(1);
                        if i_var10 <= local_1c {
                            local_1c = i_var10;
                        }
                        i_var10 = i_var10.wrapping_add(count as i32);
                        if local_24 <= i_var10 {
                            local_24 = i_var10;
                        }
                        if i_var4 <= local_20 {
                            local_20 = i_var4;
                        }
                        if local_28 <= i_var4 {
                            local_28 = i_var4;
                        }
                        p = p.add(2);
                        let b = mech_trace::rd!("10058681", nxt, read);
                        flag = (b & 1) != 0;
                        count = b >> 1;
                    } else {
                        if i_var10 <= local_1c {
                            local_1c = i_var10;
                        }
                        i_var10 = i_var10.wrapping_add(count as i32);
                        if local_24 <= i_var10 {
                            local_24 = i_var10;
                        }
                        if i_var4 <= local_20 {
                            local_20 = i_var4;
                        }
                        if local_28 <= i_var4 {
                            local_28 = i_var4;
                        }
                        let b = mech_trace::rd!("10058681", p.add(count as usize), read);
                        p = p.add(count as usize + 1);
                        flag = (b & 1) != 0;
                        count = b >> 1;
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
        mech_trace::wr!("10058681", param_6, write_unaligned, local_1c);
        mech_trace::wr!("10058681", param_6.add(1), write_unaligned, local_20);
        mech_trace::wr!("10058681", param_6.add(2), write_unaligned, local_24);
        mech_trace::wr!("10058681", param_6.add(3), write_unaligned, local_28);
    }
    mech_trace::rt!("10058681", 0)
}
