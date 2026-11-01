// SPDX-License-Identifier: MIT
//! A test-only log sink: it keeps each DEBUG message on the thread that
//! logged it, so a test reads the lines its own call wrote while other
//! tests run beside it.
//!
//! Call: [`debug_lines`] `(call) -> Vec<String>`, the DEBUG messages `call`
//! logged on this thread, in order.

use std::cell::RefCell;
use std::sync::Once;

use log::{Level, LevelFilter, Log, Metadata, Record};

struct ThreadSink;

thread_local! {
    static LINES: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
}

impl Log for ThreadSink {
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        metadata.level() == Level::Debug
    }

    fn log(&self, record: &Record<'_>) {
        if self.enabled(record.metadata()) {
            LINES.with(|lines| lines.borrow_mut().push(record.args().to_string()));
        }
    }

    fn flush(&self) {}
}

static SINK: ThreadSink = ThreadSink;
static INSTALL: Once = Once::new();

/// Runs `call` with DEBUG on and returns the DEBUG messages it logged on
/// this thread. Installs the sink once for the whole test binary.
pub(crate) fn debug_lines(call: impl FnOnce()) -> Vec<String> {
    INSTALL.call_once(|| {
        log::set_logger(&SINK).expect("invariant: only this function sets the test logger");
        log::set_max_level(LevelFilter::Debug);
    });
    LINES.with(|lines| lines.borrow_mut().clear());
    call();
    LINES.with(|lines| lines.borrow_mut().drain(..).collect())
}
