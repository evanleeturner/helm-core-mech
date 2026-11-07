// SPDX-License-Identifier: MIT
//! The swap: the shell library's 580 ported functions in one library the
//! live game loads beside its own shell (HC5, `JOB.md` I1). Each port is
//! `mech-shell`'s module, copied by `build.rs` with its definition renamed
//! `HC5_<address>`; its globals, `DAT_<address>`, and its callees,
//! `FUN_<address>`, stay undefined here and are linked to the shell's own
//! addresses by the swap's build, so the library is valid only while the
//! shell sits at 0x10000000.
//!
//! The ports' DEBUG trace goes to a file: `DllMain` opens the path the
//! environment's `HC5_LOG` names and installs the sink, which writes each
//! record as one line of the workspace's grammar, `HH:MM:SS.mmm L event
//! k=v`, by one `WriteFile`; a line past [`LINE_BYTES`] is cut there, and a
//! line that cannot be written is dropped. No `HC5_LOG`, no trace.
//!
//! 32-bit x86 Windows only: on any other target the crate is [`body`]
//! alone, for the host test that holds it equal to `mech_logline::body`.
#![no_std]

#[cfg(test)]
extern crate std;

use core::fmt::{self, Write as _};

use log::Record;
use log::kv::{Error, Key, Value, VisitSource};

#[cfg(all(target_arch = "x86", target_os = "windows"))]
include!(concat!(env!("OUT_DIR"), "/ports.rs"));

pub mod bind;
pub mod listen;

#[cfg(all(target_arch = "x86", target_os = "windows", not(test)))]
mod listen_live;

#[cfg(all(target_arch = "x86", target_os = "windows", not(test)))]
mod live;

#[cfg(all(target_arch = "x86", target_os = "windows", not(test)))]
mod windows;

// The layer (HC6): linked in so the staticlib carries its exports.
#[cfg(all(target_arch = "x86", target_os = "windows"))]
use mech_layer as _;

/// The bytes one trace line may take, its newline included.
pub const LINE_BYTES: usize = 1024;

/// A fixed buffer that takes what fits and drops the rest.
pub struct Line {
    bytes: [u8; LINE_BYTES],
    len: usize,
}

impl Line {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            bytes: [0; LINE_BYTES],
            len: 0,
        }
    }

    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes[..self.len]
    }
}

impl Default for Line {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Write for Line {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        let take = text.len().min(LINE_BYTES - self.len);
        self.bytes[self.len..self.len + take].copy_from_slice(&text.as_bytes()[..take]);
        self.len += take;
        Ok(())
    }
}

/// Writes the line's text after its time and level into `line`: the event
/// id, then each field as `key=value`, a value with a space, a quote or `=`
/// double-quoted, as `mech_logline::body` renders it.
pub fn body(record: &Record<'_>, line: &mut Line) {
    struct Fields<'a>(&'a mut Line);
    impl<'kvs> VisitSource<'kvs> for Fields<'_> {
        fn visit_pair(&mut self, key: Key<'kvs>, value: Value<'kvs>) -> Result<(), Error> {
            let mut text = Line::new();
            let _ = write!(text, "{value}");
            // A value cut at LINE_BYTES may end inside a character: keep
            // the whole ones.
            let shown = match core::str::from_utf8(text.as_bytes()) {
                Ok(shown) => shown,
                Err(cut) => {
                    core::str::from_utf8(&text.as_bytes()[..cut.valid_up_to()]).unwrap_or_default()
                }
            };
            if shown.is_empty() || shown.contains([' ', '"', '=']) {
                let _ = write!(self.0, " {key}=\"");
                for c in shown.chars() {
                    let _ = match c {
                        '\\' => self.0.write_str("\\\\"),
                        '"' => self.0.write_str("\\\""),
                        _ => self.0.write_char(c),
                    };
                }
                let _ = self.0.write_char('"');
            } else {
                let _ = write!(self.0, " {key}={value}");
            }
            Ok(())
        }
    }
    let _ = write!(line, "{}", record.args());
    let _ = record.key_values().visit(&mut Fields(line));
}

#[cfg(test)]
mod tests {
    use log::{Level, Record};

    use super::{Line, body};

    /// The swap's line holds `mech_logline::body`'s for one record whose
    /// fields take every rule: plain, a space, a quote, `=`, and empty.
    #[test]
    fn body_equals_the_logline_adapters() {
        let fields: [(&str, &str); 5] = [
            ("f", "10004330"),
            ("v", "a b"),
            ("q", "say \"hi\""),
            ("e", "k=v"),
            ("z", ""),
        ];
        let record = Record::builder()
            .level(Level::Debug)
            .args(format_args!("fn.in"))
            .key_values(&fields)
            .build();
        let mut line = Line::new();
        body(&record, &mut line);
        assert_eq!(
            std::str::from_utf8(line.as_bytes()).expect("the line is UTF-8"),
            mech_logline::body(&record)
        );
    }
}
