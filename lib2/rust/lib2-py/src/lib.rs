#![allow(non_snake_case)]

use lib1_py::pyo3::prelude::*;
// use pyo3::class::basic::CompareOp;
// use pyo3::types::PyType;

use lib1::MyThing;
use lib1_py::MyThing as MyThingPy;
use lib2::MyOtherThing as BaseMyOtherThing;

use std::str::FromStr;

#[pyclass]
#[pyo3(crate = "lib1_py::pyo3")]
pub struct MyOtherThing {
    thing: BaseMyOtherThing,
}

#[pymethods]
#[pyo3(crate = "lib1_py::pyo3")]
impl MyOtherThing {
    #[new]
    fn py_new(value: String) -> PyResult<Self> {
        Ok(MyOtherThing {
            thing: BaseMyOtherThing::new(
                MyThing::from_str(value.as_str()).unwrap()
            )
        })
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(self.thing.to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("<{}>", self.thing.to_string()))
    }

    #[getter]
    fn thing(&self) -> PyResult<MyThingPy> {
        Ok(MyThingPy { thing: MyThing::ONE })
    }
}
