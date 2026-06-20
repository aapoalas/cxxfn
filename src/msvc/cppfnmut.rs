use std::marker::PhantomData;

#[repr(C)]
pub(crate) struct MsvcCppFnMut<'a, F: 'static + Copy> {
    data: [u64; 8],
    _lt: PhantomData<&'a ()>,
    _marker: PhantomData<F>,
}
