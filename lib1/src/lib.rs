
pub use lib1_py::MyThing;
pub use lib1_py;
use pyo3::prelude::*;

#[pymodule]
pub fn lib1_py_binding(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<MyThing>()?;
    Ok(())
}
