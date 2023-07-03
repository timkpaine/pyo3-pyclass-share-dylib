pub use lib1_py::MyThing;
pub use lib1_py;
// use lib1_py::pyo3::prelude::*;
pub use pyo3::prelude::*;

// #[pyo3(crate = "lib1_py::pyo3")]
#[pymodule]
pub fn lib1_py_binding(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<MyThing>()?;
    Ok(())
}
