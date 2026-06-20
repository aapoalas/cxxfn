mod abi;
mod convert_arg;
mod cppfn;
mod cppfnmut;
mod cppfnonce;
#[cfg(all(not(unix), not(windows)))]
mod libcpp;
#[cfg(unix)]
pub(crate) mod libstdcpp;
#[cfg(windows)]
mod msvc;

use std::marker::PhantomData;

pub use abi::*;
pub use convert_arg::*;
pub use cppfn::*;
pub use cppfnmut::*;
pub use cppfnonce::*;

#[repr(transparent)]
#[derive(Clone, Copy)]
struct PunFn<'a>(fn(*mut PhantomData<&'a ()>));

impl<'a> PunFn<'a> {
    unsafe fn get<F: Copy + 'a>(&self) -> F {
        unsafe { *(&raw const self.0).cast::<F>() }
    }
}
