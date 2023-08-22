use std::cmp::Ordering;
use std::fmt;
use lib1::MyThing;


#[repr(C)]
#[derive(
    Copy, Clone, Debug, Eq, PartialEq
)]
pub struct MyOtherThing {
    thing: MyThing,
}

impl MyOtherThing {
    #[no_mangle]
    pub fn new(thing: MyThing) -> MyOtherThing {
        MyOtherThing { thing }
    }
}

impl fmt::Display for MyOtherThing {
    #[no_mangle]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MyThing({})", self.thing.value())
    }
}

impl PartialOrd for MyOtherThing {
    #[no_mangle]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MyOtherThing {
    #[no_mangle]
    fn cmp(&self, other: &Self) -> Ordering {
        self.thing.cmp(&other.thing)
    }
}
