#![allow(non_snake_case)]

use pyo3::prelude::*;

pub use lib1;
pub use lib1_py;

use lib1::MyThing;
use lib1_py::MyThing as MyThingPy;
use lib2::MyOtherThing as BaseMyOtherThing;

#[repr(C)]
#[pyclass]
pub struct MyOtherThing {
    thing: BaseMyOtherThing,
}

// #[pyo3(crate = "lib1_py::pyo3")]
#[pymethods]
impl MyOtherThing {
    #[no_mangle]
    #[new]
    fn py_new(value: u32) -> PyResult<Self> {
        Ok(MyOtherThing {
            thing: BaseMyOtherThing::new(MyThing::new(value)),
        })
    }

    #[no_mangle]
    fn __str__(&self) -> PyResult<String> {
        Ok(format!("<{}>", self.thing.to_string()))
    }

    #[no_mangle]
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("<{}>", self.thing.to_string()))
    }

    #[no_mangle]
    #[getter]
    fn thing(&self) -> PyResult<MyThingPy> {
        Ok(MyThingPy {
            thing: MyThing::new(1),
        })
    }
}
