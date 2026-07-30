//! # `std::function` as told by libstdc++
//!
//! This is the default one on Linux. The size of `std::function` is 32 bytes
//! and generally has only 8 bytes of inline data.

mod cppfn;
mod cppfnmut;
mod cppfnonce;
mod functor;
mod invokers;

use invokers::*;

pub(crate) use cppfn::LibstdCppFn;
pub(crate) use cppfnmut::LibstdCppFnMut;
pub(crate) use cppfnonce::LibstdCppFnOnce;
use functor::Functor;

type Invoker<T> = unsafe extern "C" fn(*const T);
type Manager<T> = unsafe extern "C" fn(*mut T, *const T, ManagerOperation) -> bool;

#[allow(dead_code)]
#[repr(u32)]
pub(crate) enum ManagerOperation {
    GetTypeInfo,
    GetFunctorPtr,
    CloneFunctor,
    DestroyFunctor,
}
