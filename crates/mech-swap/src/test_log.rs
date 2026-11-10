// SPDX-License-Identifier: MIT
//! A test-only log sink, as `mech-rows`' and `mech-shell`'s, but keeping every level from
//! DEBUG up with its level: a swap's load line is INFO and its rows are DEBUG, and a test
//! reads both, in the order its own call wrote them on its own thread, while other tests
//! run beside it.
//!
//! Call: [`lines`] `(call) -> Vec<(Level, String)>`, each line `call` logged on this
//! thread, rendered `event k=v` by the logline adapter.

use core::cell::RefCell;
use std::string::String;
use std::sync::Once;
use std::vec::Vec;

use log::{Level, LevelFilter, Log, Metadata, Record};

struct ThreadSink;

std::thread_local! {
    static LINES: RefCell<Vec<(Level, String)>> = const { RefCell::new(Vec::new()) };
}

impl Log for ThreadSink {
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record<'_>) {
        if self.enabled(record.metadata()) {
            let line = (record.level(), mech_logline::body(record));
            LINES.with(|lines| lines.borrow_mut().push(line));
        }
    }

    fn flush(&self) {}
}

static SINK: ThreadSink = ThreadSink;
static INSTALL: Once = Once::new();

/// Runs `call` with DEBUG on and returns each line it logged on this thread, with its
/// level. Installs the sink once for the whole test binary.
pub(crate) fn lines(call: impl FnOnce()) -> Vec<(Level, String)> {
    INSTALL.call_once(|| {
        log::set_logger(&SINK).expect("invariant: only this function sets the test logger");
        log::set_max_level(LevelFilter::Debug);
    });
    LINES.with(|lines| lines.borrow_mut().clear());
    call();
    LINES.with(|lines| lines.borrow_mut().drain(..).collect())
}
