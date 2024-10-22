#![allow(non_snake_case)]
use pyo3::class::basic::CompareOp;
use pyo3::prelude::*;
use pyo3::types::PyType;
use std::str::FromStr;
use strum::IntoEnumIterator;

use ::lib1::MyThing as BaseMyThing;
use util::pyclass_export;

#[pyclass]
pub struct MyThing {
    pub thing: BaseMyThing,
}

#[pymethods]
impl MyThing {
    #[new]
    fn py_new(value: String) -> PyResult<Self> {
        Ok(MyThing {
            thing: BaseMyThing::from_str(value.as_str()).unwrap(),
        })
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(self.thing.to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("MyThing<{}>", self.thing))
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Lt => Ok(self.thing.to_string() < other.thing.to_string()),
            CompareOp::Le => Ok(self.thing.to_string() <= other.thing.to_string()),
            CompareOp::Eq => Ok(self.thing == other.thing),
            CompareOp::Ne => Ok(self.thing != other.thing),
            CompareOp::Gt => Ok(self.thing.to_string() > other.thing.to_string()),
            CompareOp::Ge => Ok(self.thing.to_string() >= other.thing.to_string()),
        }
    }

    // TODO figure out how to make a class iterator,
    // is way too complicated in pyo3 rn
    #[staticmethod]
    fn members() -> Vec<MyThing> {
        BaseMyThing::iter()
            .map(|item: BaseMyThing| MyThing { thing: item })
            .collect()
    }

    #[classattr]
    fn ONE() -> MyThing {
        MyThing {
            thing: BaseMyThing::ONE,
        }
    }
    #[classattr]
    fn TWO() -> MyThing {
        MyThing {
            thing: BaseMyThing::TWO,
        }
    }

    #[classmethod]
    fn len(_cls: Py<PyType>) -> PyResult<usize> {
        Ok(BaseMyThing::iter().count())
    }
}

#[pymodule]
fn lib1(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_class::<MyThing>().unwrap();
    Ok(())
}

pyclass_export!(MyThing);
