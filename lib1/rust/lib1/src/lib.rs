use std::cmp::Ordering;
use std::fmt;

#[repr(C)]
#[derive(
    Copy, Clone, Debug, Eq, PartialEq
)]
pub struct MyThing {
    value: u32,
}

impl MyThing {
    #[no_mangle]
    pub fn new(value: u32) -> MyThing {
        MyThing {
            value,
        }
    }
}

impl fmt::Display for MyThing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl PartialOrd for MyThing {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MyThing {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}
