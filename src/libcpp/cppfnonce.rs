use std::marker::PhantomData;

use crate::libcpp::LibcppFunctionVtable;

#[repr(C)]
pub(crate) struct LibCppFnOnce<'a, F: 'static + Copy> {
    vtable: *const LibcppFunctionVtable<Self>,
    data: [u64; 3],
    _lt: PhantomData<&'a ()>,
    _marker: PhantomData<F>,
}
