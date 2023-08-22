#![crate_type = "cdylib"]

pub use lib1;
pub use lib1_py;
pub use lib1_py::MyThing;
use pyo3::prelude::*;

#[no_mangle]
#[pymodule]
pub fn lib1_py_binding(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<MyThing>()?;
    Ok(())
}
