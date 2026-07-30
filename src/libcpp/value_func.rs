//! When `_LIBCPP_ABI_OPTIMIZED_FUNCTION` is turned off, libcxx uses its
//! "legacy" C++ inheritance based `__value_func` construction. The construction
//! has a peculiar mix of outline and inline storage, where the first pointer is
//! potentially self-referential, pointing to the actual C++ class instance, and
//! the three pointers worth of inline storage either contain the actual C++
//! class instance with its vtable pointer and captured data, or alternatively
//! contains something else for heap-allocated captures.

use std::{
    alloc::{Layout, dealloc},
    any::TypeId,
    ffi::c_void as void,
    marker::PhantomData,
    mem::{MaybeUninit, needs_drop},
    ptr::drop_in_place,
};

#[allow(non_camel_case_types)]
#[repr(C, align(8))]
struct type_info;

#[repr(C)]
struct VtableHeader {
    offset_to_top: usize,
    type_info: *const type_info,
}

impl VtableHeader {
    const fn default() -> Self {
        Self {
            offset_to_top: 0,
            type_info: std::ptr::null(),
        }
    }
}

#[repr(C)]
struct LibcppFunctionTable {
    dtor_complete: unsafe extern "C" fn(this: *mut void),
    dtor_deleting: unsafe extern "C" fn(this: *mut void),
    clone: unsafe extern "C" fn(this: *const void) -> *mut void,
    clone_in: unsafe extern "C" fn(this: *const void, p: *mut void),
    destroy: unsafe extern "C" fn(p: *mut void),
    destroy_deallocate: unsafe extern "C" fn(p: *mut void),
    /// # `unsafe extern "C" fn(this: *const void, ...args) -> R`
    ///
    /// This cannot be named a function type as casting between function
    /// pointers does not work in const contexts.
    invoke: *const void,
    #[cfg(feature = "rtti")]
    target: unsafe extern "C" fn(&self, ti: &type_info) -> *const (),
    #[cfg(feature = "rtti")]
    target_type: unsafe extern "C" fn(this: &This) -> &type_info,
}

#[repr(C)]
struct LibcppFnVtable {
    header: VtableHeader,
    table: LibcppFunctionTable,
}

/// __base provides an abstract interface for copyable functors.
#[repr(C)]
#[derive(Clone, Copy)]
struct __base<F: 'static + Copy> {
    vptr: *const LibcppFunctionTable,
    _marker: PhantomData<F>,
}

unsafe impl<F: 'static + Copy> Send for __base<F> {}
unsafe impl<F: 'static + Copy> Sync for __base<F> {}

impl<F: 'static + Copy> __base<F> {
    unsafe fn clone(this: *const Self) -> *mut Self {
        let f = unsafe { &*(*this).vptr }.clone;
        unsafe { f(this.cast()).cast() }
    }

    unsafe fn clone_in(this: *const Self, p: *mut Self) {
        let f = unsafe { &*(*this).vptr }.clone_in;
        unsafe { f(this.cast(), p.cast()) };
    }

    unsafe fn destroy(this: *mut Self) {
        let f = unsafe { &*(*this).vptr }.destroy;
        unsafe { f(this.cast()) };
    }

    unsafe fn destroy_deallocate(this: *mut Self) {
        let f = unsafe { &*(*this).vptr }.destroy_deallocate;
        unsafe { f(this.cast()) };
    }
}

/// __func implements __base for a given functor type.
#[repr(C)]
#[derive(Clone, Copy)]
struct __func<'a, _FD: 'a + Clone, _FB: 'static + Copy> {
    base: __base<_FB>,
    __func_: _FD,
    _marker: PhantomData<&'a ()>,
}

impl<'a, _Fp: 'a + Clone, R: 'static> From<_Fp> for __func<'a, _Fp, fn() -> R> {
    fn from(__f: _Fp) -> Self {
        Self {
            base: __base {
                vptr: std::ptr::null(),
                _marker: PhantomData,
            },
            __func_: __f,
            _marker: PhantomData,
        }
    }
}

impl<'a, _Fp: 'a + Clone, R: 'static> From<&_Fp> for __func<'a, _Fp, fn() -> R> {
    fn from(__f: &_Fp) -> Self {
        Self {
            base: __base {
                vptr: std::ptr::null(),
                _marker: PhantomData,
            },
            __func_: __f.clone(),
            _marker: PhantomData,
        }
    }
}

impl<'a, _Fp: 'a + Clone, R: 'static> __func<'a, _Fp, fn() -> R> {
    const fn make_vtable() -> &'static LibcppFnVtable {
        // let invoke = unsafe { std::mem::transmute(Self::invoke as *const ()) };
        &LibcppFnVtable {
            header: VtableHeader {
                offset_to_top: 0,
                type_info: std::ptr::null(),
            },
            table: LibcppFunctionTable {
                dtor_complete: Self::destroy,
                dtor_deleting: Self::destroy_deallocate,
                clone: Self::__clone,
                clone_in: Self::__clone_in,
                destroy: Self::destroy,
                destroy_deallocate: Self::destroy_deallocate,
                invoke: Self::invoke as *const void,
                #[cfg(feature = "rtti")]
                target: Self::target,
                #[cfg(feature = "rtti")]
                target_type: Self::target_type,
            },
        }
    }

    // __base<_Rp(_ArgTypes...)>* __clone() const override
    unsafe extern "C" fn __clone(this: *const void) -> *mut void {
        Box::into_raw(Box::new(this.cast::<Self>().clone())).cast::<void>()
    }

    // void __clone(__base<_Rp(_ArgTypes...)>* __p) const override
    unsafe extern "C" fn __clone_in(this: *const void, __p: *mut void) {
        unsafe {
            // ::new ((void*)__p) __func(__func_);
            __p.cast::<Self>().write((&*this.cast::<Self>()).clone());
        }
    }

    // override
    unsafe extern "C" fn destroy(this: *mut void) {
        unsafe { drop_in_place(&raw mut (*this.cast::<Self>()).__func_) };
    }

    // override
    unsafe extern "C" fn destroy_deallocate(this: *mut void) {
        unsafe {
            Self::destroy(this);
            dealloc(this.cast(), Layout::new::<Self>());
        }
    }

    // _Rp operator()(_ArgTypes&&... __arg) override
    unsafe extern "C" fn invoke(this: *const void) -> R {
        // return std::__invoke_r<_Rp>(__func_, std::forward<_ArgTypes>(__arg)...);
        let this = this.cast::<__func<_Fp, fn() -> R>>();
        unsafe { (&*this).__func_(&raw const (*this).__func_) }
    }

    // override
    #[cfg(feature = "rtti")]
    unsafe extern "C" fn target(this: *const void, __ti: &'static type_info) -> *const void {
        if false
        /* __ti == typeid(_Fp) */
        {
            unsafe { (&raw const (*this.cast::<Self>()).__func_).cast::<void>() }
        } else {
            std::ptr::null()
        }
    }

    // override
    #[cfg(feature = "rtti")]
    unsafe extern "C" fn target_type(_: *const void) -> &'static type_info {
        use std::any::TypeId;

        unsafe { &*std::ptr::from_ref(&TypeId::of::<Self>()).cast::<type_info>() }
    }
}

/// __value_func creates a value-type from a __func.
///
/// Note: this is a self-referential class when small enough and requires
/// Pinning for safe operations in Rust. That is quite terrible.
#[repr(C)]
pub(crate) struct __value_func<'a, F: 'static + Copy> {
    __buf_: [MaybeUninit<usize>; 3],
    __f_: *mut __base<F>,
    _marker: PhantomData<&'a ()>,
}

impl<'a, F: 'static + Copy> __value_func<'a, F> {
    #[inline(always)]
    unsafe fn __as_base(__p: *mut void) -> *mut __base<F> {
        __p.cast()
    }
}
impl<'a, R: 'static> __value_func<'a, fn() -> R> {
    #[inline(always)]
    pub(crate) fn empty() -> Self {
        Self {
            __buf_: [MaybeUninit::uninit(); 3],
            __f_: std::ptr::null_mut(),
            _marker: PhantomData,
        }
    }

    // FIXME: _Fp should impl MoveInto<__function::__func<_Fp, _Rp(_ArgTypes...)>>
    unsafe fn new_from_other<_Fp: Copy>(ret: &mut MaybeUninit<Self>, __f: _Fp) -> &mut Self {
        const {
            if TypeId::of::<_Fp>() == TypeId::of::<__value_func<_Fp>>() {
                panic!("Invalid new_from_other invocation");
            }
        }

        if __function::__is_null(__f) {
            unsafe {
                ret.as_mut_ptr().write(Self {
                    __buf_: [MaybeUninit::uninit(); 3],
                    __f_: std::ptr::null_mut(),
                    _marker: PhantomData,
                });
                return ret.assume_init_mut();
            }
        }
        if size_of::<__func<_Fp, fn() -> R>>() <= size_of::<[MaybeUninit<usize>; 3]>()
            && !needs_drop::<_Fp>()
        {
            unsafe {
                std::ptr::write(
                    (&raw mut (*ret.as_mut_ptr()).__buf_).cast(),
                    __func::<_Fp, fn() -> R>::from(__f),
                );
                std::ptr::write(&raw mut (*ret.as_mut_ptr()).__f_, ret.as_mut_ptr().cast());
            }
        } else {
            // __f_ = new _Fun(std::move(__f));
            unsafe {
                std::ptr::write(
                    &raw mut (*ret.as_mut_ptr()).__f_,
                    Box::into_raw(Box::new(__func::<_Fp, fn() -> R>::from(__f))).cast(),
                );
            }
        }
        unsafe { ret.assume_init_mut() }
    }

    fn r#move(__f: &mut Self) {
        // if (__f.__f_ == nullptr)
        //   __f_ = nullptr;
        // else if ((void*)__f.__f_ == &__f.__buf_) {
        //   __f_ = __as_base(&__buf_);
        //   __f.__f_->__clone(__f_);
        // } else {
        //   __f_     = __f.__f_;
        //   __f.__f_ = nullptr;
        // }
    }

    // __value_func& operator=(__value_func&& __f) {
    fn assign_operator(__f: &mut Self) {
        // *this = nullptr;
        // if (__f.__f_ == nullptr)
        // __f_ = nullptr;
        // else if ((void*)__f.__f_ == &__f.__buf_) {
        // __f_ = __as_base(&__buf_);
        // __f.__f_->__clone(__f_);
        // } else {
        // __f_     = __f.__f_;
        // __f.__f_ = nullptr;
        // }
        // return *this;
    }

    fn assign_null_operator(&mut self) -> &mut Self {
        // __value_func& operator=(nullptr_t) {
        // __func* __f = __f_;
        // __f_        = nullptr;
        // if ((void*)__f == &__buf_)
        // __f->destroy();
        // else if (__f)
        // __f->destroy_deallocate();
        // return *this;
        todo!()
    }

    fn invoke(&self) -> R {
        // if (__f_ == nullptr)
        // std::__throw_bad_function_call();
        // return (*__f_)(std::forward<_ArgTypes>(__args)...);
        todo!()
    }

    fn swap(&mut self, __f: &mut Self) {
        // if (std::addressof(__f) == this)
        //   return;
        // if ((void*)__f_ == &__buf_ && (void*)__f.__f_ == &__f.__buf_) {
        //   _LIBCPP_SUPPRESS_DEPRECATED_PUSH
        //   typename aligned_storage<sizeof(__buf_)>::type __tempbuf;
        //   _LIBCPP_SUPPRESS_DEPRECATED_POP
        //   __func* __t = __as_base(&__tempbuf);
        //   __f_->__clone(__t);
        //   __f_->destroy();
        //   __f_ = nullptr;
        //   __f.__f_->__clone(__as_base(&__buf_));
        //   __f.__f_->destroy();
        //   __f.__f_ = nullptr;
        //   __f_     = __as_base(&__buf_);
        //   __t->__clone(__as_base(&__f.__buf_));
        //   __t->destroy();
        //   __f.__f_ = __as_base(&__f.__buf_);
        // } else if ((void*)__f_ == &__buf_) {
        //   __f_->__clone(__as_base(&__f.__buf_));
        //   __f_->destroy();
        //   __f_     = __f.__f_;
        //   __f.__f_ = __as_base(&__f.__buf_);
        // } else if ((void*)__f.__f_ == &__f.__buf_) {
        //   __f.__f_->__clone(__as_base(&__buf_));
        //   __f.__f_->destroy();
        //   __f.__f_ = __f_;
        //   __f_     = __as_base(&__buf_);
        // } else
        //   std::swap(__f_, __f.__f_);
    }

    #[inline(always)]
    const fn ok(&self) -> bool {
        !self.__f_.is_null()
    }

    #[cfg(feature = "rtti")]
    const fn target_type(&self) -> &'static type_info {
        if self.__f_.is_null() {
            return &TypeId::of::<void>();
        }
        self.__f_.target_type()
    }

    #[cfg(feature = "rtti")]
    const fn target<_Tp>() -> *const _Tp {
        if self.__f_.is_null() {
            return std::ptr::null();
        }
        self.__f_.target(TypeId::of::<_Tp>())
    }
}

impl<'a, F: 'static + Copy> Clone for __value_func<'a, F> {
    #[inline(always)]
    fn clone(&self) -> Self {
        let mut res = MaybeUninit::<Self>::uninit();
        let __f_ = unsafe {
            res.as_mut_ptr()
                .byte_add(size_of::<[MaybeUninit<usize>; 3]>())
        }
        .cast::<*mut __base<F>>();
        if self.__f_.is_null() {
            unsafe { __f_.write(std::ptr::null_mut()) };
        } else if std::ptr::eq(self.__f_, (&raw const self.__buf_).cast()) {
            let value =
                unsafe { Self::__as_base((&raw const self.__buf_).cast_mut().cast::<void>()) };
            unsafe {
                __f_.write(value);
                __base::clone_in(self.__f_, value);
            }
        } else {
            unsafe {
                __f_.write(__base::clone(self.__f_));
            }
        }
        unsafe { res.assume_init() }
    }
}

impl<'a, F: 'static + Copy> Drop for __value_func<'a, F> {
    fn drop(&mut self) {
        if std::ptr::eq(
            self.__f_.cast::<void>(),
            (&raw const self.__buf_).cast::<void>(),
        ) {
            unsafe { __base::destroy(self.__f_) }
        } else {
            unsafe {
                __base::destroy_deallocate(self.__f_);
            }
        }
    }
}
