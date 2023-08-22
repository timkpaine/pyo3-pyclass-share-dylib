#![crate_type = "cdylib"]
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
    pub extern "C" fn new(value: u32) -> MyThing {
        MyThing {
            value,
        }
    }

    #[no_mangle]
    pub extern "C" fn value(&self) -> u32 {
        self.value
    }
}

#[no_mangle]
pub extern "C" fn newMyThing(value: u32) -> MyThing {
    MyThing::new(value)
}

impl fmt::Display for MyThing {
    #[no_mangle]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl PartialOrd for MyThing {
    #[no_mangle]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MyThing {
    #[no_mangle]
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}
