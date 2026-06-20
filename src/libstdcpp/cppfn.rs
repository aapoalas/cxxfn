use std::{hint::unreachable_unchecked, marker::PhantomData, mem::MaybeUninit};

use super::{CapturedData, Invoker, Manager, fn_ref};
use crate::{ConvertArg, PunFn, libstdcpp::ManagerOperation};

#[repr(C)]
pub(crate) struct LibstdCppFn<'a, F: 'static + Copy> {
    pub(crate) functor: CapturedData<'a>,
    manager: Option<Manager<Self>>,
    invoker: Option<Invoker<Self>>,
    _marker: PhantomData<F>,
}

impl<'a, R: 'static> LibstdCppFn<'a, fn() -> R> {
    #[inline]
    pub fn new<D: 'a + Clone>(data: D, f: fn(&D) -> R) -> Self {
        let functor = CapturedData::from_data_and_fn(data, unsafe {
            core::mem::transmute::<_, PunFn<'a>>(f)
        });
        let invoker =
            unsafe { core::mem::transmute::<_, Invoker<Self>>(fn_ref::f0::<D, R> as *const ()) };
        let manager =
            unsafe { core::mem::transmute::<_, Manager<Self>>(fn_ref_manager::<D> as *const ()) };
        Self {
            functor,
            manager: Some(manager),
            invoker: Some(invoker),
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn invoke(&self) -> R {
        let Some(invoker) = self.invoker else {
            panic!("bad function call");
        };
        let invoker = unsafe {
            core::mem::transmute::<_, unsafe extern "C" fn(*const Self) -> R>(invoker as *const ())
        };
        unsafe { invoker(self) }
    }
}

impl<'a, R: 'static, A0: ConvertArg> LibstdCppFn<'a, fn(A0) -> R> {
    #[inline]
    pub fn new<D: 'a + Clone>(data: D, f: fn(&D, A0::Rust<'_>) -> R) -> Self {
        let functor = CapturedData::from_data_and_fn(data, unsafe {
            core::mem::transmute::<_, PunFn<'a>>(f)
        });
        let invoker = unsafe {
            core::mem::transmute::<_, Invoker<Self>>(fn_ref::f1::<D, R, A0> as *const ())
        };
        let manager =
            unsafe { core::mem::transmute::<_, Manager<Self>>(fn_ref_manager::<D> as *const ()) };
        Self {
            functor,
            manager: Some(manager),
            invoker: Some(invoker),
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn invoke(&self, a0: A0::Cxx) -> R {
        let Some(invoker) = self.invoker else {
            panic!("bad function call");
        };
        let invoker = unsafe {
            core::mem::transmute::<_, unsafe extern "C" fn(*const Self, A0::Cxx) -> R>(
                invoker as *const (),
            )
        };
        unsafe { invoker(self, a0) }
    }
}

impl<'a, R: 'static, A0: ConvertArg, A1: ConvertArg> LibstdCppFn<'a, fn(A0, A1) -> R> {
    #[inline]
    pub fn new<D: 'a + Clone>(data: D, f: fn(&D, A0::Rust<'_>, A1::Rust<'_>) -> R) -> Self {
        let functor = CapturedData::from_data_and_fn(data, unsafe {
            core::mem::transmute::<*const (), PunFn<'a>>(f as *const ())
        });
        let invoker = unsafe {
            core::mem::transmute::<*const (), Invoker<Self>>(
                fn_ref::f2::<D, R, A0, A1> as *const (),
            )
        };
        let manager =
            unsafe { core::mem::transmute::<_, Manager<Self>>(fn_ref_manager::<D> as *const ()) };
        Self {
            functor,
            manager: Some(manager),
            invoker: Some(invoker),
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn invoke(&self, a0: A0::Cxx, a1: A1::Cxx) -> R {
        let Some(invoker) = self.invoker else {
            panic!("bad function call");
        };
        let invoker = unsafe {
            core::mem::transmute::<_, unsafe extern "C" fn(*const Self, A0::Cxx, A1::Cxx) -> R>(
                invoker as *const (),
            )
        };
        unsafe { invoker(self, a0, a1) }
    }
}

impl<'a, R: 'static, A0: ConvertArg, A1: ConvertArg, A2: ConvertArg>
    LibstdCppFn<'a, fn(A0, A1, A2) -> R>
{
    #[inline]
    pub fn new<D: 'a + Clone>(
        data: D,
        f: fn(&D, A0::Rust<'_>, A1::Rust<'_>, A2::Rust<'_>) -> R,
    ) -> Self {
        let functor = CapturedData::from_data_and_fn(data, unsafe {
            core::mem::transmute::<_, PunFn<'a>>(f)
        });
        let invoker = unsafe {
            core::mem::transmute::<_, Invoker<Self>>(fn_ref::f3::<D, R, A0, A1, A2> as *const ())
        };
        let manager =
            unsafe { core::mem::transmute::<_, Manager<Self>>(fn_ref_manager::<D> as *const ()) };
        Self {
            functor,
            manager: Some(manager),
            invoker: Some(invoker),
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn invoke(&self, a0: A0::Cxx, a1: A1::Cxx, a2: A2::Cxx) -> R {
        let Some(invoker) = self.invoker else {
            panic!("bad function call");
        };
        let invoker = unsafe {
            core::mem::transmute::<
                _,
                unsafe extern "C" fn(*const Self, A0::Cxx, A1::Cxx, A2::Cxx) -> R,
            >(invoker as *const ())
        };
        unsafe { invoker(self, a0, a1, a2) }
    }
}

impl<'a, R: 'static, A0: ConvertArg, A1: ConvertArg, A2: ConvertArg, A3: ConvertArg>
    LibstdCppFn<'a, fn(A0, A1, A2, A3) -> R>
{
    #[inline]
    pub fn new<D: 'a + Clone>(
        data: D,
        f: fn(&D, A0::Rust<'_>, A1::Rust<'_>, A2::Rust<'_>, A3::Rust<'_>) -> R,
    ) -> Self {
        let functor = CapturedData::from_data_and_fn(data, unsafe {
            core::mem::transmute::<_, PunFn<'a>>(f)
        });
        let invoker = unsafe {
            core::mem::transmute::<_, Invoker<Self>>(
                fn_ref::f4::<D, R, A0, A1, A2, A3> as *const (),
            )
        };
        let manager =
            unsafe { core::mem::transmute::<_, Manager<Self>>(fn_ref_manager::<D> as *const ()) };
        Self {
            functor,
            manager: Some(manager),
            invoker: Some(invoker),
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn invoke(&self, a0: A0::Cxx, a1: A1::Cxx, a2: A2::Cxx, a3: A3::Cxx) -> R {
        let Some(invoker) = self.invoker else {
            panic!("bad function call");
        };
        let invoker = unsafe {
            core::mem::transmute::<
                _,
                unsafe extern "C" fn(*const Self, A0::Cxx, A1::Cxx, A2::Cxx, A3::Cxx) -> R,
            >(invoker as *const ())
        };
        unsafe { invoker(self, a0, a1, a2, a3) }
    }
}

unsafe extern "C" fn fn_ref_manager<'a, D: 'a + Clone>(
    dest: *mut LibstdCppFn<'a, fn()>,
    src: *const LibstdCppFn<'a, fn()>,
    op: ManagerOperation,
) -> bool {
    match op {
        ManagerOperation::GetTypeInfo => unsafe {
            dest.cast::<*const ()>().write(core::ptr::null());
        },
        // SAFETY: only called from constexpr.
        ManagerOperation::GetFunctorPtr => unsafe { unreachable_unchecked() },
        ManagerOperation::CloneFunctor => unsafe {
            let (data, f) = (*src).functor.get_data::<D, PunFn<'a>>();
            (*dest).functor.write_data::<D>(data.clone(), f);
        },
        ManagerOperation::DestroyFunctor => unsafe {
            (*dest).functor.drop_data::<D>();
        },
    }
    false
}

impl<'a, F: 'a + Copy> Clone for LibstdCppFn<'a, F> {
    fn clone(&self) -> Self {
        let Some(manager) = self.manager else {
            return Self {
                functor: unsafe { MaybeUninit::<CapturedData<'a>>::zeroed().assume_init() },
                manager: None,
                invoker: None,
                _marker: PhantomData,
            };
        };
        let mut functor = MaybeUninit::<CapturedData<'a>>::uninit();
        let _ = unsafe {
            manager(
                functor.as_mut_ptr().cast(),
                self,
                ManagerOperation::CloneFunctor,
            )
        };
        Self {
            functor: unsafe { functor.assume_init() },
            manager: self.manager.clone(),
            invoker: self.invoker.clone(),
            _marker: self._marker.clone(),
        }
    }
}

impl<'a, F: 'a + Copy> From<&mut Self> for LibstdCppFn<'a, F> {
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

impl<'a, F: 'a + Copy> Drop for LibstdCppFn<'a, F> {
    fn drop(&mut self) {
        let Some(manager) = self.manager else {
            return;
        };
        unsafe { manager(self, self, ManagerOperation::DestroyFunctor) };
    }
}
