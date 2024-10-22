#![allow(non_snake_case)]
use pyo3::prelude::*;
use std::str::FromStr;

use lib1::MyThing;
use lib1_py::MyThing as MyThingPy;
use ::lib2::MyOtherThing as BaseMyOtherThing;
use util::pyclass_import;


#[pyclass]
pub struct MyOtherThing {
    thing: BaseMyOtherThing,
}

#[pymethods]
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

#[pymodule]
fn lib2(py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    unsafe { pyclass_import::<MyThingPy>(py, "lib1.lib1", "MyThing")?; }
    m.add_class::<MyOtherThing>().unwrap();
    Ok(())
}

