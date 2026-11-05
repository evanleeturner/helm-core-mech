// SPDX-License-Identifier: MIT
//! mercs's input rows (WI1, ruling 1417): the game's facts `window_input`
//! serves. The first, the stick: in play alone the game file's
//! `fun_1005ed99` reads the pointer as a joystick, each axis from
//! -`0x10000` at the client's left or top edge to just under `0x10000` at
//! its right or bottom. Its formula is the machine code's, `MERCSW.DLL`
//! `1005ef3a` to `1005ef6c`; no run has reached play (wall 1), so no
//! number here is harvested. The tests live in `tests`.

#[cfg(test)]
mod tests;

/// The stick the game file reads at the client point `point` on a client
/// `size` wide and high: `[y, x]`, the order `fun_1005ed99` writes them,
/// each axis `((2p - n) << 16) / n` divided signed and truncated toward 0,
/// as its `cdq; idiv`. `None` for a point outside the client, where the
/// game writes nothing.
#[must_use]
pub const fn stick(point: (i32, i32), size: (i32, i32)) -> Option<[i32; 2]> {
    let ((x, y), (width, height)) = (point, size);
    if x < 0 || x >= width || y < 0 || y >= height {
        return None;
    }
    Some([axis(y, height), axis(x, width)])
}

/// One axis: `p` on a side `n` long, `n` above `p` and so above 0.
const fn axis(p: i32, n: i32) -> i32 {
    p.wrapping_mul(2).wrapping_sub(n).wrapping_shl(16) / n
}
