// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000a690(record_base: i32) {
    let base = record_base as usize;
    // SAFETY: record_base is the address of the caller's record; the fields
    // at the given offsets are read and written where the original does.
    unsafe {
        let p200 = (base + 0x200) as *mut i32;
        if p200.read_unaligned() == 0 {
            p200.write_unaligned(1);
        }
        let num = ((base + 0x210) as *const u32)
            .read_unaligned()
            .wrapping_mul(100) as i32;
        let divisor = p200.read_unaligned();
        let percent: i32;
        core::arch::asm!(
            "cdq",
            "idiv {d:e}",
            d = in(reg) divisor,
            inout("eax") num => percent,
            out("edx") _,
            options(nomem, nostack)
        );
        ((base + 0x280) as *mut i32).write_unaligned(percent);
        let v = (percent as u32).wrapping_mul(3).wrapping_add(2) as i32;
        ((base + 0x284) as *mut i32).write_unaligned(v / 2);
    }
}
