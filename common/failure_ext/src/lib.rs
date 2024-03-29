pub use failure::{
    _core, bail, ensure, err_msg, format_err, AsFail, Backtrace, Causes, Compat, Context, Error,
    Fail, ResultExt, SyncFailure,
};

// Custom error handling macros are placed in the failure_macros crate. Due to
// the way intra-crate macro exports currently work, macros can't be exported
// from anywhere but the top level when they are defined in the same crate.
pub use failure_macros::bail_err;

pub type Result<T> = std::result::Result<T, Error>;

/// Prelude module containing most commonly used types/macros this crate exports.
pub mod prelude {
    pub use crate::Result;
    pub use failure::{bail, ensure, err_msg, format_err, Error, Fail, ResultExt};
    pub use failure_macros::bail_err;
}
