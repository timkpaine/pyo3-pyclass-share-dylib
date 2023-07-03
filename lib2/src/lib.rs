// pub use lib1_py::pyo3::prelude::*;
pub use pyo3::prelude::*;
pub use lib2_py::MyOtherThing;
pub use lib1_py::MyThing;

// #[pyo3(crate = "lib1_py::pyo3")]
#[pymodule]
pub fn lib2_py_binding(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<MyOtherThing>()?;
    Ok(())
}
