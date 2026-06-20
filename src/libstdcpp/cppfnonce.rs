use std::marker::PhantomData;

use super::{CapturedData, Invoker, Manager, ManagerOperation, cppfnmut::fn_mut_manager, fn_once};
use crate::{ConvertArg, PunFn};

#[repr(C)]
pub(crate) struct LibstdCppFnOnce<'a, F: 'static + Copy> {
    pub(crate) functor: CapturedData<'a>,
    manager: Option<Manager<Self>>,
    invoker: Option<Invoker<Self>>,
    _marker: PhantomData<F>,
}

impl<F: 'static + Copy> LibstdCppFnOnce<'_, F> {
    pub(crate) fn inhibit_drop(&mut self) {
        self.manager = None;
        self.invoker = None;
    }
}

impl<'a, R: 'static> LibstdCppFnOnce<'a, fn() -> R> {
    #[inline]
    pub(crate) fn new<D: 'a>(data: D, f: fn(D) -> R) -> Self {
        let functor = CapturedData::from_data_and_fn(data, unsafe {
            core::mem::transmute::<_, PunFn<'a>>(f)
        });
        let invoker =
            unsafe { core::mem::transmute::<_, Invoker<Self>>(fn_once::f0::<D, R> as *const ()) };
        let manager =
            unsafe { core::mem::transmute::<_, Manager<Self>>(fn_mut_manager::<D> as *const ()) };
        Self {
            functor,
            manager: Some(manager),
            invoker: Some(invoker),
            _marker: PhantomData,
        }
    }

    #[inline]
    pub(crate) fn invoke(mut self) -> R {
        let Some(invoker) = self.invoker else {
            panic!("bad function call");
        };
        let invoker = unsafe {
            core::mem::transmute::<_, unsafe extern "C" fn(*mut Self) -> R>(invoker as *const ())
        };
        unsafe { invoker(&mut self) }
    }
}

impl<'a, R: 'static, A0: ConvertArg> LibstdCppFnOnce<'a, fn(A0) -> R> {
    #[inline]
    pub(crate) fn new<D: 'a>(data: D, f: fn(D, A0::Rust<'_>) -> R) -> Self {
        let functor = CapturedData::from_data_and_fn(data, unsafe {
            core::mem::transmute::<_, PunFn<'a>>(f)
        });
        let invoker = unsafe {
            core::mem::transmute::<_, Invoker<Self>>(fn_once::f1::<D, R, A0> as *const ())
        };
        let manager =
            unsafe { core::mem::transmute::<_, Manager<Self>>(fn_mut_manager::<D> as *const ()) };
        Self {
            functor,
            manager: Some(manager),
            invoker: Some(invoker),
            _marker: PhantomData,
        }
    }

    #[inline]
    pub(crate) fn invoke(mut self, a0: A0::Cxx) -> R {
        let Some(invoker) = self.invoker else {
            panic!("bad function call");
        };
        let invoker = unsafe {
            core::mem::transmute::<_, unsafe extern "C" fn(*mut Self, A0::Cxx) -> R>(
                invoker as *const (),
            )
        };
        unsafe { invoker(&mut self, a0) }
    }
}

impl<'a, R: 'static, A0: ConvertArg, A1: ConvertArg> LibstdCppFnOnce<'a, fn(A0, A1) -> R> {
    #[inline]
    pub(crate) fn new<D: 'a>(data: D, f: fn(D, A0::Rust<'_>, A1::Rust<'_>) -> R) -> Self {
        let functor = CapturedData::from_data_and_fn(data, unsafe {
            core::mem::transmute::<*const (), PunFn<'a>>(f as *const ())
        });
        let invoker = unsafe {
            core::mem::transmute::<*const (), Invoker<Self>>(
                fn_once::f2::<D, R, A0, A1> as *const (),
            )
        };
        let manager =
            unsafe { core::mem::transmute::<_, Manager<Self>>(fn_mut_manager::<D> as *const ()) };
        Self {
            functor,
            manager: Some(manager),
            invoker: Some(invoker),
            _marker: PhantomData,
        }
    }

    #[inline]
    pub(crate) fn invoke(mut self, a0: A0::Cxx, a1: A1::Cxx) -> R {
        let Some(invoker) = self.invoker else {
            panic!("bad function call");
        };
        let invoker = unsafe {
            core::mem::transmute::<_, unsafe extern "C" fn(*mut Self, A0::Cxx, A1::Cxx) -> R>(
                invoker as *const (),
            )
        };
        unsafe { invoker(&mut self, a0, a1) }
    }
}

impl<'a, R: 'static, A0: ConvertArg, A1: ConvertArg, A2: ConvertArg>
    LibstdCppFnOnce<'a, fn(A0, A1, A2) -> R>
{
    #[inline]
    pub(crate) fn new<D: 'a>(
        data: D,
        f: fn(D, A0::Rust<'_>, A1::Rust<'_>, A2::Rust<'_>) -> R,
    ) -> Self {
        let functor = CapturedData::from_data_and_fn(data, unsafe {
            core::mem::transmute::<_, PunFn<'a>>(f)
        });
        let invoker = unsafe {
            core::mem::transmute::<_, Invoker<Self>>(fn_once::f3::<D, R, A0, A1, A2> as *const ())
        };
        let manager =
            unsafe { core::mem::transmute::<_, Manager<Self>>(fn_mut_manager::<D> as *const ()) };
        Self {
            functor,
            manager: Some(manager),
            invoker: Some(invoker),
            _marker: PhantomData,
        }
    }

    #[inline]
    pub(crate) fn invoke(mut self, a0: A0::Cxx, a1: A1::Cxx, a2: A2::Cxx) -> R {
        let Some(invoker) = self.invoker else {
            panic!("bad function call");
        };
        let invoker = unsafe {
            core::mem::transmute::<_, unsafe extern "C" fn(*mut Self, A0::Cxx, A1::Cxx, A2::Cxx) -> R>(
                invoker as *const (),
            )
        };
        unsafe { invoker(&mut self, a0, a1, a2) }
    }
}

impl<'a, R: 'static, A0: ConvertArg, A1: ConvertArg, A2: ConvertArg, A3: ConvertArg>
    LibstdCppFnOnce<'a, fn(A0, A1, A2, A3) -> R>
{
    #[inline]
    pub(crate) fn new<D: 'a>(
        data: D,
        f: fn(D, A0::Rust<'_>, A1::Rust<'_>, A2::Rust<'_>, A3::Rust<'_>) -> R,
    ) -> Self {
        let functor = CapturedData::from_data_and_fn(data, unsafe {
            core::mem::transmute::<_, PunFn<'a>>(f)
        });
        let invoker = unsafe {
            core::mem::transmute::<_, Invoker<Self>>(
                fn_once::f4::<D, R, A0, A1, A2, A3> as *const (),
            )
        };
        let manager =
            unsafe { core::mem::transmute::<_, Manager<Self>>(fn_mut_manager::<D> as *const ()) };
        Self {
            functor,
            manager: Some(manager),
            invoker: Some(invoker),
            _marker: PhantomData,
        }
    }

    #[inline]
    pub(crate) fn invoke(mut self, a0: A0::Cxx, a1: A1::Cxx, a2: A2::Cxx, a3: A3::Cxx) -> R {
        let Some(invoker) = self.invoker else {
            panic!("bad function call");
        };
        let invoker = unsafe {
            core::mem::transmute::<
                _,
                unsafe extern "C" fn(*mut Self, A0::Cxx, A1::Cxx, A2::Cxx, A3::Cxx) -> R,
            >(invoker as *const ())
        };
        unsafe { invoker(&mut self, a0, a1, a2, a3) }
    }
}

impl<'a, F: 'static + Copy> From<&mut Self> for LibstdCppFnOnce<'a, F> {
    /// Move constructor.
    fn from(value: &mut Self) -> Self {
        Self {
            functor: unsafe { core::ptr::read(&mut value.functor) },
            manager: value.manager.take(),
            invoker: value.invoker.take(),
            _marker: PhantomData,
        }
    }
}

impl<'a, F: 'static + Copy> Drop for LibstdCppFnOnce<'a, F> {
    fn drop(&mut self) {
        let Some(manager) = self.manager else {
            return;
        };
        unsafe { manager(self, self, ManagerOperation::DestroyFunctor) };
    }
}
