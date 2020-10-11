use super::iter;
use super::pybool;
use super::pytype::PyTypeRef;
use crate::pyobject::{PyClassImpl, PyContext, PyObjectRef, PyRef, PyResult, PyValue};
use crate::vm::VirtualMachine;

pub type PyFilterRef = PyRef<PyFilter>;

/// filter(function or None, iterable) --> filter object
///
/// Return an iterator yielding those items of iterable for which function(item)
/// is true. If function is None, return the items that are true.
#[pyclass(module = false, name = "filter")]
#[derive(Debug)]
pub struct PyFilter {
    predicate: PyObjectRef,
    iterator: PyObjectRef,
}

impl PyValue for PyFilter {
    fn class(vm: &VirtualMachine) -> PyTypeRef {
        vm.ctx.types.filter_type.clone()
    }
}

#[pyimpl(flags(BASETYPE))]
impl PyFilter {
    #[pyslot]
    fn tp_new(
        cls: PyTypeRef,
        function: PyObjectRef,
        iterable: PyObjectRef,
        vm: &VirtualMachine,
    ) -> PyResult<PyFilterRef> {
        let iterator = iter::get_iter(vm, &iterable)?;

        PyFilter {
            predicate: function,
            iterator,
        }
        .into_ref_with_type(vm, cls)
    }

    #[pymethod(name = "__next__")]
    fn next(&self, vm: &VirtualMachine) -> PyResult {
        let predicate = &self.predicate;
        let iterator = &self.iterator;
        loop {
            let next_obj = iter::call_next(vm, iterator)?;
            let predicate_value = if vm.is_none(predicate) {
                next_obj.clone()
            } else {
                // the predicate itself can raise StopIteration which does stop the filter
                // iteration
                vm.invoke(&predicate, vec![next_obj.clone()])?
            };
            if pybool::boolval(vm, predicate_value)? {
                return Ok(next_obj);
            }
        }
    }

    #[pymethod(name = "__iter__")]
    fn iter(zelf: PyRef<Self>) -> PyRef<Self> {
        zelf
    }
}

pub fn init(context: &PyContext) {
    PyFilter::extend_class(context, &context.types.filter_type);
}
