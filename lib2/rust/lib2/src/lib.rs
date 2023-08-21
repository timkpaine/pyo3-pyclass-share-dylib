use std::cmp::Ordering;
use std::fmt;

struct MyThing;

#[link(name = "lib1")]
extern {
    impl MyThing {
        fn new(value: u32) -> MyThing;
    }
}

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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MyThing({})", self.thing)
    }
}

impl PartialOrd for MyOtherThing {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MyOtherThing {
    fn cmp(&self, other: &Self) -> Ordering {
        self.thing.cmp(&other.thing)
    }
}
