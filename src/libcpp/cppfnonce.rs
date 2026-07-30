use std::marker::PhantomData;

#[repr(C)]
pub(crate) struct LibCppFnOnce<'a, F: 'static + Copy> {
    vtable: *const LibcppFunctionVtable<Self>,
    data: [u64; 3],
    _marker: PhantomData<(&'a (), F)>,
}
