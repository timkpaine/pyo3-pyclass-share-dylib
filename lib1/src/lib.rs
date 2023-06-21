use lib1_py::MyThing;
use lib1_py::pyo3::prelude::*;

#[pymodule]
#[pyo3(crate = "lib1_py::pyo3")]
pub fn lib1(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<MyThing>()?;
    Ok(())
}
