use std::marker::PhantomData;

use crate::libcpp::LibcppFunctionVtable;

#[repr(C)]
pub(crate) struct LibCppFn<'a, F: 'static + Copy> {
    vtable: *const LibcppFunctionVtable<Self>,
    data: [u64; 3],
    _marker: PhantomData<(&'a (), F)>,
}
