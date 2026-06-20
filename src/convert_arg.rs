use std::marker::PhantomData;

use crate::abi::{CppMut, CppRef};

/// Trait for defining the conversion from a raw C++ `std::function` argument
/// type into a safe Rust argument type.
///
/// # Safety
///
/// The implementer must ensure that the raw C++ argument type matches what the
/// C++ STL passes as the argument, and that the conversion function returns a
/// Rust type that is valid until the end of the `std::function` invocation.
///
/// ## C++ STL parameter passing
///
/// For Clang and GCC compiled code, the C++ calling convention works as
/// follows:
///
/// * `const &T` is passed directly as `*const T`; the conversion function
///   should usually just perform the appropriate type transmutation.
//
/// * `&T` is passed directly as `*mut T`; the conversion function should
///   usually just perform the appropriate type transmutation.
///
/// * `const *T` is passed indirectly as `*const *const T` and must be read by
///   the conversion function.
///
/// * `*T` is passed indirectly as `*const *mut T` and must be read by the
///   conversion function.
///
/// * `T` by-value parameters are passed indirectly as `*mut T` and must call
///   either the copy or move constructor of `T`.
///
/// For MSVC compiled code, the C++ calling convention is not known by the
/// author.
pub unsafe trait ConvertArg: 'static {
    type Cxx: 'static;
    type Rust<'a>;

    /// Convert a raw C++ `std::function` argument type into a safe Rust type.
    unsafe fn convert(val: Self::Cxx) -> Self::Rust<'static>;
}

/// Copyable C++ parameter by value.
///
/// # Safety
///
/// The `Clone` method is called which means that a temporary reference to `T`
/// is created. No aliasing `&mut T` must exist, and no mutation of the `T`
/// (unless through internal mutability) must happen concurrently with the
/// `std::function` invocation.
///
/// # Example
///
/// ```cpp
/// std::function<void (uint32_t val)>
/// ```
///
/// ```rust,ignore
/// fn(val: u32)
/// ```
pub struct ByVal<T: Clone + 'static>(PhantomData<T>);

/// Movable C++ parameter by value.
///
/// # Safety
///
/// The `From<&mut T>` method is called which means that a temporary exclusive
/// reference to `T` is created. No aliasing `&T` or `&mut T` must exist, and no
/// mutation through aliasing pointers of `T` must happen.
///
/// This is generally an acceptable parameter passing vehicle for eg. raw
/// pointers but is very questionable for pass-by-value C++ classes. A non-POD
/// C++ class is always passed by mutable reference even when the function
/// declaration indicates pass-by-value, and the callee is just expected to call
/// the move constructor to take ownership of the value.
///
/// Using this passing vehicle means that Rust invokes the move constructor
/// (expected to be called by `From<&mut T>`) inside the `std::function` invoker
/// and then uses Rust calling convention to pass the `T` from the invoker to
/// the callee. The Rust calling convention does not include move constructors,
/// and therefore the second move from the invoker to the actual callee will not
/// call the C++ move constructor or any other constructor for that matter.
/// Depending on the C++ class this might either be entirely fine, or may
/// produce a corrupted instance in the callee.
///
/// # Example
///
/// ```cpp
/// std::function<void (Foo val)>
/// ```
///
/// ```rust,ignore
/// fn(val: Foo)
/// ```
pub struct ByValMove<T: From<&'static mut T> + 'static>(PhantomData<T>);

/// Const reference C++ parameter as `CppRef<T>`.
///
/// # Safety
///
/// This is a safe parameter passing vehicle.
///
/// # Example
///
/// ```cpp
/// std::function<void (const Foo &val)>
/// ```
///
/// ```rust,ignore
/// fn(val: CppRef<'_, Foo>)
/// ```
pub struct ByCppRef<T: 'static>(PhantomData<T>);

/// Mutable reference C++ parameter as `CppMut<T>`.
///
/// # Safety
///
/// This is a safe parameter passing vehicle.
///
/// # Example
///
/// ```cpp
/// std::function<void (Foo &val)>
/// ```
///
/// ```rust,ignore
/// fn(val: CppMut<'_, Foo>)
/// ```
pub struct ByCppMut<T: 'static>(PhantomData<T>);

/// Const reference C++ parameter by `&T`.
///
/// # Safety
///
/// No aliasing `&mut T` references must exist and the referenced value must not
/// be mutated (unless through internal mutability) during the `std::function`
/// invocation.
///
/// # Example
///
/// ```cpp
/// std::function<void (const Foo &val)>
/// ```
///
/// ```rust,ignore
/// fn(val: &Foo)
/// ```
pub struct ByRef<T: 'static>(PhantomData<T>);

/// Mutable reference C++ parameter as `&mut T`.
///
/// # Safety
///
/// No aliasing `&T` or `&mut T` references must exist and the referenced value
/// must not be mutated through aliasing raw pointers during the `std::function`
/// invocation.
///
/// # Example
///
/// ```cpp
/// std::function<void (bool &stop)>
/// ```
///
/// ```rust,ignore
/// fn (stop: &mut bool)
/// ```
pub struct ByMut<T: 'static>(PhantomData<T>);

unsafe impl<T: Clone + 'static> ConvertArg for ByVal<T> {
    type Cxx = *const T;

    type Rust<'a> = T;

    #[inline(always)]
    unsafe fn convert(val: Self::Cxx) -> Self::Rust<'static> {
        // SAFETY: T is Clone and C++ guarantees references are non-null
        // pointers.
        unsafe { (*val).clone() }
    }
}

unsafe impl<T: From<&'static mut T> + 'static> ConvertArg for ByValMove<T> {
    type Cxx = *mut T;

    type Rust<'a> = T;

    #[inline(always)]
    unsafe fn convert(val: Self::Cxx) -> Self::Rust<'static> {
        T::from(unsafe { &mut *val })
    }
}

unsafe impl<T: 'static> ConvertArg for ByCppRef<T> {
    type Cxx = *const T;

    type Rust<'a> = CppRef<'a, T>;

    #[inline(always)]
    unsafe fn convert(val: Self::Cxx) -> Self::Rust<'static> {
        unsafe { CppRef::from_ptr(val) }
    }
}

unsafe impl<T: 'static> ConvertArg for ByCppMut<T> {
    type Cxx = *mut T;

    type Rust<'a> = CppMut<'a, T>;

    #[inline(always)]
    unsafe fn convert(val: Self::Cxx) -> Self::Rust<'static> {
        unsafe { CppMut::from_ptr(val) }
    }
}

unsafe impl<T: 'static> ConvertArg for ByRef<T> {
    type Cxx = *const T;

    type Rust<'a> = &'a T;

    #[inline(always)]
    unsafe fn convert(val: Self::Cxx) -> Self::Rust<'static> {
        unsafe { &*val }
    }
}

unsafe impl<T: 'static> ConvertArg for ByMut<T> {
    type Cxx = *mut T;

    type Rust<'a> = &'a mut T;

    #[inline(always)]
    unsafe fn convert(val: Self::Cxx) -> Self::Rust<'static> {
        unsafe { &mut *val }
    }
}
