#[macro_use]
extern crate strum_macros;

use serde::{Deserialize, Serialize};

#[derive(
    Copy, Clone, Debug, Deserialize, Display, EnumIter, EnumString, Eq, PartialEq, Serialize,
)]
#[strum(serialize_all = "UPPERCASE")]
pub enum MyThing {
    ONE,
    TWO,
    THREE,
}

