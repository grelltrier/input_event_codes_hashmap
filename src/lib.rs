//! Linux input event codes form `linux/input_event_codes.h`.
//!
#[macro_use]
extern crate lazy_static;

pub use self::abs::*;
pub use self::btn::*;
pub use self::ev::*;
pub use self::input_prop::*;
pub use self::key::*;
pub use self::led::*;
pub use self::msc::*;
pub use self::rel::*;
pub use self::rep::*;
pub use self::snd::*;
pub use self::sw::*;
pub use self::syn::*;

mod abs;
mod btn;
mod ev;
mod input_prop;
mod key;
mod led;
mod msc;
mod rel;
mod rep;
mod snd;
mod sw;
mod syn;
