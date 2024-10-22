use pyo3::{Bound, PyResult, PyTypeInfo, Python};
use pyo3::exceptions::PyRuntimeError;
use pyo3::types::PyCapsule;
use pyo3::impl_::pyclass::PyClassImpl;
use pyo3::impl_::pyclass::LazyTypeObject;
use pyo3::types::PyAnyMethods;
use pyo3::types::PyCapsuleMethods;
use pyo3::types::PyString;

pub trait ExternalPyClass: PyClassImpl + PyTypeInfo {}

/// Imports an externally-defined PyClass. Use this at module-init time.
///
/// Some linker settings cause lazy-statics to be duplicated when a native Python
/// extension is imported by another – that is, what we link against uses different static storage
/// than the corresponding native extension that Python has loaded. Pyo3 relies on these (via static
/// `LazyTypeObject` instances) to define the type of a PyClass. If they are duplicated, then they
/// no longer compare as equal and we can get "instance of" checks failing when an instance created
/// via Python is compared to an instance created from the linked library.
///
/// To get around this problem: at module load, before this dependent extension has had
/// a chance to create any instances, we overwrite our local instance of `LazyTypeObject` with the
/// instance that the Python interpreter is using (whose raw pointer is returned by
/// `__get_lazy_type_object__`).
///
/// Safety: this assumes that the native extension loaded for T is the same as the T that we've
/// linked against.
pub unsafe fn pyclass_import<T: ExternalPyClass>(py: Python, module: &str, name: &str) -> PyResult<()> {
    // let py_module = py.import(T::MODULE).unwrap();
    let py_module = py.import(PyString::new(py, module)).unwrap();
    let py_ty = py_module.getattr(name)?;
    let binding = py_ty
         .getattr("__get_lazy_type_object__")
         .map_err(|err| {
             PyRuntimeError::new_err(format!("{}; did you forget to use `pyclass_export!`?", err))
         })?
         .call0()?;
    let capsule: &Bound<'_, PyCapsule> = binding.downcast::<PyCapsule>().unwrap();

    // Safety: we assume that the native extension loaded in memory, i.e. our call to
    // `__get_lazy_type_object__`, is the same as the one we've linked against.
    let orig_lazy_ty_ref = unsafe { *capsule.reference::<&LazyTypeObject<T>>() };

    T::lazy_type_object().manually_init_as_copy_for_sharing(py, orig_lazy_ty_ref)
}

/// Allows a PyClass to be exported to another Python native extension.
///
/// Defines static method, `__get_lazy_type_object__`, (to be used internally) that returns a raw
/// pointer to its `LazyTypeObject`. This can be used by separately-built extensions to reuse the
/// same type objects (in case the linker has caused them to be duplicated).
///
/// Using `PyCapsule` makes this opaque to Python code. (So at least we can rule out a bad Python
/// program faking the implementation of `__get_lazy_type_object__`.)
#[macro_export]
macro_rules! pyclass_export {
    ( $py_class:ident ) => {
        use util::ExternalPyClass;
        impl util::ExternalPyClass for $py_class {}

        #[pymethods]
        impl $py_class {
            #[staticmethod]
            fn __get_lazy_type_object__(py: Python) -> PyResult<Bound<pyo3::types::PyCapsule>> {
                let lazy_ty = <Self as pyo3::impl_::pyclass::PyClassImpl>::lazy_type_object();
                pyo3::types::PyCapsule::new(py, lazy_ty, None)
            }
        }
    };
}
