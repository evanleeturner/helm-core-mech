// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b0a2c: [u32; 45];
    static mut DAT_100b0a30: [u32; 45];
}

unsafe fn xor_str(p: *mut u8) {
    if p.is_null() {
        return;
    }
    let mut len: i32 = 0;
    // SAFETY: p points at a NUL-terminated string in the game's memory.
    unsafe {
        while mech_trace::rd!("10052143", p.byte_add(len as usize), read) != 0 {
            len = len.wrapping_add(1);
        }
        let mut i: i32 = 0;
        while i < len {
            let q = p.byte_add(i as usize);
            mech_trace::wr!(
                "10052143",
                q,
                write,
                mech_trace::rd!("10052143", q, read) ^ 10
            );
            i = i.wrapping_add(1);
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10052143() {
    mech_trace::enter!("10052143");
    let mut i: i32 = 0;
    while i <= 0xe {
        let idx = (i * 3) as usize;
        // SAFETY: the tables are the game's own; entries are string pointers or null.
        unsafe {
            let a = mech_trace::rd!(
                "10052143",
                (&raw const DAT_100b0a2c).cast::<u32>().add(idx),
                read
            );
            xor_str(a as *mut u8);
            let b = mech_trace::rd!(
                "10052143",
                (&raw const DAT_100b0a30).cast::<u32>().add(idx),
                read
            );
            xor_str(b as *mut u8);
        }
        i += 1;
    }
}
