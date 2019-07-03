mod simple_logger;

pub use simple_logger::init;

pub mod prelude {
    pub use slog::{slog_crit, slog_debug, slog_error, slog_info, slog_trace, slog_warn};
    pub use slog_scope::{crit, debug, error, info, trace, warn};
}
