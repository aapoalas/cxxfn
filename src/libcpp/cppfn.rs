#[cfg(feature = "libcpp_optimized_function")]
use crate::libcpp::policy_func::__policy_func;
#[cfg(not(feature = "libcpp_optimized_function"))]
use crate::libcpp::value_func::__value_func;

#[repr(C)]
pub(crate) struct LibCppFn<'a, F: 'static + Copy> {
    #[cfg(not(feature = "libcpp_optimized_function"))]
    __f_: __value_func<'a, F>,
    #[cfg(feature = "libcpp_optimized_function")]
    __f_: __policy_func<'a, F>,
}

impl<'a, R: 'static> LibstdCppFn<'a, fn() -> R> {
    #[inline]
    pub fn new<D: 'a + Clone>(data: D, f: fn(&D) -> R) -> Self {
        Self {
            __f_: __value_func::empty(),
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
        let functor =
            Functor::from_data_and_fn(data, unsafe { core::mem::transmute::<_, PunFn<'a>>(f) });
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
        let functor = Functor::from_data_and_fn(data, unsafe {
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
        let functor =
            Functor::from_data_and_fn(data, unsafe { core::mem::transmute::<_, PunFn<'a>>(f) });
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
        let functor =
            Functor::from_data_and_fn(data, unsafe { core::mem::transmute::<_, PunFn<'a>>(f) });
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
