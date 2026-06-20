use std::marker::PhantomData;

#[repr(C)]
pub(crate) struct LibCppFn<'a, F: 'static + Copy> {
    data: [u64; 4],
    _lt: PhantomData<&'a ()>,
    _marker: PhantomData<F>,
}
