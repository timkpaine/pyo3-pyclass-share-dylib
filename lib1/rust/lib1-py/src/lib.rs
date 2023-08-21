#![allow(non_snake_case)]

use pyo3::class::basic::CompareOp;
use pyo3::prelude::*;
pub use pyo3::types::PyType;

use lib1::MyThing as BaseMyThing;

#[repr(C)]
#[pyclass]
pub struct MyThing {
    pub thing: BaseMyThing,
}

#[pymethods]
impl MyThing {
    #[no_mangle]
    #[new]
    fn py_new(value: u32) -> PyResult<Self> {
        Ok(MyThing {
            thing: BaseMyThing::new(value),
        })
    }

    #[no_mangle]
    fn __str__(&self) -> PyResult<String> {
        Ok(format!("MyThing<{}>", self.thing))
    }

    #[no_mangle]
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("MyThing<{}>", self.thing))
    }

    #[no_mangle]
    fn __richcmp__(&self, other_py: &PyAny, op: CompareOp) -> PyResult<bool> {
        let other: &Self = unsafe { std::mem::transmute::<&PyAny, &Self>(other_py) };
        match op {
            CompareOp::Lt => Ok(self.thing < other.thing),
            CompareOp::Le => Ok(self.thing <= other.thing),
            CompareOp::Eq => Ok(self.thing == other.thing),
            CompareOp::Ne => Ok(self.thing != other.thing),
            CompareOp::Gt => Ok(self.thing > other.thing),
            CompareOp::Ge => Ok(self.thing >= other.thing),
        }
    }

    #[no_mangle]
    #[classattr]
    fn ONE() -> MyThing {
        MyThing {
            thing: BaseMyThing::new(1),
        }
    }
    #[no_mangle]
    #[classattr]
    fn TWO() -> MyThing {
        MyThing {
            thing: BaseMyThing::new(2),
        }
    }
}
