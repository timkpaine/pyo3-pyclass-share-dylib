use lib1::MyThing;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MyOtherThing {
  thing: MyThing,
}

impl MyOtherThing {
    pub fn new(thing: MyThing) -> MyOtherThing {
        MyOtherThing {
           thing,
        }
    }
    pub fn to_string(&self) -> String {
        self.thing.to_string()
    }
}

impl PartialEq for MyOtherThing {
    fn eq(&self, other: &Self) -> bool {
        self.thing == other.thing
    }
}

