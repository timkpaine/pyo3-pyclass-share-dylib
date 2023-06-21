pub use lib1_py::pyo3::prelude::*;
pub use lib2_py::MyOtherThing;
pub use lib1_py::MyThing;

#[pymodule]
#[pyo3(crate = "lib1_py::pyo3")]
fn lib2(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<MyOtherThing>()?;
    Ok(())
}
