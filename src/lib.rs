//! Linux input event codes form `linux/input_event_codes.h`.
//!
#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

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

/// Returns true if the keycode is in the HashMap and thus a valid input code
pub fn is_valid_input_code<'a>(map: &'a HashMap<&'static str, u32>, code: u32) -> bool {
    map.iter()
        .find_map(|(key, &val)| if val == code { Some(key) } else { None })
        .is_some()
}
