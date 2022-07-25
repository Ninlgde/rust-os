use core::{fmt, mem};
use core::sync::atomic::{AtomicUsize, Ordering};
use log::{self, Level, LevelFilter};

static MAX_LOG_LEVEL_FILTER: AtomicUsize = AtomicUsize::new(0);

#[inline]
pub fn set_max_level(level: LevelFilter) {
    MAX_LOG_LEVEL_FILTER.store(level as usize, Ordering::SeqCst)
}

#[inline(always)]
pub fn max_level() -> LevelFilter {
    unsafe { mem::transmute(MAX_LOG_LEVEL_FILTER.load(Ordering::Relaxed)) }
}

pub fn init() {
    set_max_level(match option_env!("LOG") {
        Some("error") => LevelFilter::Error,
        Some("ERROR") => LevelFilter::Error,
        Some("warn") => LevelFilter::Warn,
        Some("WARN") => LevelFilter::Warn,
        Some("info") => LevelFilter::Info,
        Some("INFO") => LevelFilter::Info,
        Some("debug") => LevelFilter::Debug,
        Some("DEBUG") => LevelFilter::Debug,
        Some("trace") => LevelFilter::Trace,
        Some("TRACE") => LevelFilter::Trace,
        _ => LevelFilter::Info,
    });
}

fn level_to_color_code(level: Level) -> u8 {
    match level {
        Level::Error => 31, // Red
        Level::Warn => 93,  // BrightYellow
        Level::Info => 34,  // Blue
        Level::Debug => 32, // Green
        Level::Trace => 90, // BrightBlack
    }
}

pub fn println_with_level(args: fmt::Arguments, lvl: Level) {
    if lvl <= max_level() {
        print!(
            "\u{1B}[{}m[{:>5}][{}-{}] [kernel] {}\u{1B}[0m",
            level_to_color_code(lvl),
            lvl.as_str(),
            0, // cpu id
            "main", // thread id
            args
        );
    }
}

#[macro_export]
macro_rules! log {
    ($lvl:expr, $fmt: literal $(, $($arg: tt)+)?) => {
        $crate::logging::println_with_level(format_args!(concat!($fmt, "\n") $(, $($arg)+)?), $lvl);
    }
}

#[macro_export]
macro_rules! error {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        log!(log::Level::Error, $fmt $(, $($arg)+)?);
    }
}


#[macro_export]
macro_rules! warn {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        log!(log::Level::Warn, $fmt $(, $($arg)+)?);
    }
}


#[macro_export]
macro_rules! info {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        log!(log::Level::Info, $fmt $(, $($arg)+)?);
    }
}


#[macro_export]
macro_rules! debug {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        log!(log::Level::Debug, $fmt $(, $($arg)+)?);
    }
}


#[macro_export]
macro_rules! trace {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        log!(log::Level::Trace, $fmt $(, $($arg)+)?);
    }
}
