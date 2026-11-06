// SPDX-License-Identifier: MIT
//! This workspace's logbook against its own log calls. `fn.in` and `imp.call`
//! are logged partly by the crates this workspace takes from oxide95, so they are
//! named as shared: each must equal oxide95's row, whose own fence answers for it.

use std::path::Path;

#[test]
fn every_log_call_is_cataloged_and_every_entry_is_used() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    mech_logline::fence::check_sharing(&root, &["fn.in", "imp.call"]).enforce();
}
