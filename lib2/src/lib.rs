// pub use lib1_py::pyo3::prelude::*;
pub use pyo3::prelude::*;
pub use lib2_py::MyOtherThing;
pub use lib2_py::lib1;
pub use lib2_py::lib1_py;

pub use lib2;
pub use lib2_py;

#[pymodule]
pub fn lib2_py_binding(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<MyOtherThing>()?;
    Ok(())
}
