//! When `_LIBCPP_ABI_OPTIMIZED_FUNCTION` is turned on, libcxx uses a
//! `__policy_func` construction that avoids normal C++ inheritance in favour of
//! manual vtable command structures and call functions.
//!
//! It is intended to become the future standard implementation, but is not yet
//! ABI stable.

use std::{
    ffi::c_void as void,
    marker::PhantomData,
    mem::{MaybeUninit, needs_drop},
};

use crate::PunFn;

#[allow(non_camel_case_types)]
#[repr(C, align(8))]
struct type_info;

/// Storage for a functor object, to be used with __policy to manage copy and
/// destruction.
#[repr(C)]
union __policy_storage {
    __small: [MaybeUninit<usize>; 2],
    __large: *const void,
}

/// Policy contains information about how to copy, destroy, and move the
/// underlying functor. You can think of it as a vtable of sorts.
#[repr(C)]
struct __policy {
    /// Used to copy or destroy __large values. null for trivial objects.
    __clone: Option<fn(*const void) -> *const void>,
    __destroy: Option<fn(*const void)>,
    /// True if this is the null policy (no value).
    __is_null: bool,
    /// The target type. May be null if RTTI is disabled.
    __type_info: *const type_info,
}

unsafe impl Sync for __policy {}

const fn __use_small_storage<_Fun: Clone>() -> bool {
    size_of::<_Fun>() <= size_of::<[MaybeUninit<u64>; 2]>()
        && align_of::<_Fun>() <= align_of::<[MaybeUninit<u64>; 2]>()
        && !needs_drop::<_Fun>()
}

impl __policy {
    /// Returns a pointer to a static policy object suitable for the functor
    /// type.
    const fn __create<_Fun: Clone>() -> &'static Self {
        if const { __use_small_storage::<_Fun>() } {
            &__policy {
                __clone: None,
                __destroy: None,
                __is_null: false,
                __type_info: std::ptr::null(),
            }
        } else {
            &__policy {
                __clone: Some(__policy::__large_clone::<_Fun>),
                __destroy: Some(__policy::__large_destroy::<_Fun>),
                __is_null: false,
                __type_info: std::ptr::null(),
            }
        }
    }

    const fn __create_empty() -> &'static Self {
        static __POLICY: __policy = __policy {
            __clone: None,
            __destroy: None,
            __is_null: true,
            __type_info: std::ptr::null(),
        };
        return &__POLICY;
    }

    fn __large_clone<_Fun: Clone>(__s: *const void) -> *const void {
        let __f: *const _Fun = __s.cast::<_Fun>();
        return Box::into_raw(Box::new(unsafe { &*__f }.clone()))
            .cast_const()
            .cast::<void>();
    }

    fn __large_destroy<_Fun>(__s: *const void) {
        unsafe { drop(Box::from_raw(__s.cast_mut().cast::<_Fun>())) };
    }
}

#[repr(C)]
pub(super) struct __policy_func<'a, F: 'static + Copy> {
    /// Inline storage for small objects.
    __buf_: __policy_storage,
    __func_: Option<PunFn<'a>>,
    /// The policy that describes how to move / copy / destroy __buf_. Never
    /// null, even if the function is empty.
    __policy_: &'static __policy,
    _marker: PhantomData<F>,
}

impl<'a, F: 'static + Copy> __policy_func<'a, F> {
    fn __empty_func(_: *const __policy_storage /*, __fast_forward<_ArgTypes>...*/) {
        panic!("bad function call")
        // std::__throw_bad_function_call();
    }

    fn __call_func<_Fun: 'static + Clone>(
        __buf: *const __policy_storage, /*, __fast_forward<_ArgTypes>... __args*/
    ) {
        let __func: *const _Fun = if __use_small_storage::<_Fun>() {
            (unsafe { &raw const (*__buf).__small }).cast::<_Fun>()
        } else {
            unsafe { (*__buf).__large }.cast::<_Fun>()
        };
        // return std::__invoke_r<_Rp>(*__func, std::forward<_ArgTypes>(__args)...);
    }

    #[inline(always)]
    fn new() -> Self {
        Self {
            __buf_: __policy_storage {
                __small: [MaybeUninit::uninit(); 2],
            },
            __func_: Some(unsafe {
                std::mem::transmute::<_, PunFn<'a>>(
                    Self::__empty_func as fn(*const __policy_storage),
                )
            }),
            __policy_: __policy::__create_empty(),
            _marker: PhantomData,
        }
    }

    #[inline(always)]
    fn new_from_functor<_Fp: Clone>(__f: &mut _Fp) -> Self {
        if __function::__is_null(__f) {
            return Self {
                __buf_: __policy_storage {
                    __small: [MaybeUninit::uninit(); 2],
                },
                __func_: Some(unsafe {
                    std::mem::transmute::<_, PunFn<'a>>(
                        Self::__empty_func as fn(*const __policy_storage),
                    )
                }),
                __policy_: __policy::__create_empty(),
                _marker: PhantomData,
            };
        }
        Self {
            __func_: Some(Self::__call_func::<_Fp>),
            __policy_: __policy::__create::<_Fp>(),
            __buf_: if __use_small_storage::<_Fp>() {
                __policy_storage {
                    // ::new ((void*)&__buf_.__small) _Fp(std::move(__f));
                    __small: todo!(),
                }
            } else {
                __policy_storage {
                    // __buf_.__large = ::new _Fp(std::move(__f));
                    __large: todo!(),
                }
            },
            _marker: PhantomData,
        }
    }

    #[inline(always)]
    fn r#move(__f: &mut Self) -> Self {
        // _LIBCPP_HIDE_FROM_ABI __policy_func(__policy_func&& __f)
        //     : __buf_(__f.__buf_), __func_(__f.__func_), __policy_(__f.__policy_) {
        //     if (__policy_->__destroy) {
        //     __f.__policy_ = __policy::__create_empty();
        //     __f.__func_   = {};
        //     }
        // }
        todo!()
    }

    #[inline(always)]
    fn assign_from(&mut self, __f: &mut Self) -> &mut Self {
        unsafe {
            std::ptr::drop_in_place(self);
            std::ptr::copy(&__f.__buf_, &mut self.__buf_, 1);
        }
        self.__func_ = __f.__func_;
        self.__policy_ = __f.__policy_;
        __f.__policy_ = __policy::__create_empty();
        __f.__func_ = None;
        self
    }

    fn set_null(&mut self) -> &mut Self {
        let __p = self.__policy_;
        self.__policy_ = __policy::__create_empty();
        self.__func_ = None;
        if let Some(destroy) = __p.__destroy {
            destroy(unsafe { self.__buf_.__large });
        }
        self
    }

    unsafe fn invoke(&self) {
        unsafe { self.__func_.unwrap_unchecked() }.0((&raw const self.__buf_).cast_mut().cast());
    }

    fn swap(&mut self, __f: &mut Self) {
        unsafe {
            std::ptr::swap(&mut self.__func_, &mut __f.__func_);
            std::ptr::swap(&mut self.__policy_, &mut __f.__policy_);
            std::ptr::swap(&mut self.__buf_, &mut __f.__buf_);
        };
    }

    #[inline(always)]
    fn ok(&self) -> bool {
        self.__policy_.__is_null
    }

    #[cfg(feature = "rtti")]
    const fn target_type(&self) -> *const type_info {
        self.__policy_.__type_info
    }

    #[cfg(feature = "rtti")]
    const fn target<_Tp>(&self) -> *const _Tp {
        if self.__policy_.__is_null || /*TypeId::of<_Tp>() != *self.__policy_.__type_info*/ false {
            std::ptr::null::<_Tp>()
        } else if self.__policy_.__clone.is_some() {
            // Out of line storage.
            unsafe { self.__buf_.__large }.cast::<_Tp>()
        } else {
            (&raw const self.__buf_.__small).cast::<_Tp>()
        }
    }
}

impl<'a, F: 'static + Copy> Clone for __policy_func<'a, F> {
    fn clone(&self) -> Self {
        let __buf_ = if let Some(clone) = self.__policy_.__clone {
            __policy_storage {
                __large: clone(unsafe { self.__buf_.__large }),
            }
        } else {
            __policy_storage {
                __small: unsafe { self.__buf_.__small }.clone(),
            }
        };

        Self {
            __buf_,
            __func_: self.__func_,
            __policy_: self.__policy_,
            _marker: PhantomData,
        }
    }
}

impl<'a, F: 'static + Copy> Drop for __policy_func<'a, F> {
    fn drop(&mut self) {
        if let Some(dtor) = self.__policy_.__destroy {
            dtor(unsafe { self.__buf_.__large });
        }
    }
}
