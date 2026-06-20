use std::{marker::PhantomData, ptr::NonNull};

/// # C++ const reference type
///
/// C++ const references are non-null const pointers with an implied lifetime
/// which this type makes explicit. C++ const references do not guarantee that
/// the target data is immutable, as a const reference can be freely cast into a
/// mutable reference.
///
/// ## Example
///
/// ```cpp
/// template <T>
/// using CppRef = const &T;
/// ```
#[derive(Clone, Copy)]
pub struct CppRef<'a, T>(NonNull<T>, PhantomData<&'a T>);

impl<'a, T> CppRef<'a, T> {
    #[inline]
    pub unsafe fn from_ptr(ptr: *const T) -> Self {
        Self(
            unsafe { NonNull::new_unchecked(ptr.cast_mut()) },
            PhantomData,
        )
    }

    #[inline]
    pub fn from_ref(ptr: &'a T) -> Self {
        unsafe { Self::from_ptr(ptr) }
    }

    #[inline]
    pub fn cast_mut(self) -> CppMut<'a, T> {
        unsafe { CppMut::from_ptr(self.0.as_ptr().cast()) }
    }
}

/// # C++ mutable reference type
///
/// C++ mutable references are non-null mutable pointers with an implied
/// lifetime which this type makes explicit. C++ mutable references do not
/// guarantee that the target data is held exclusively, or in other words the
/// references can alias.
///
/// ## Example
///
/// ```cpp
/// template <T>
/// using CppMut = &T;
/// ```
#[derive(Clone, Copy)]
pub struct CppMut<'a, T>(NonNull<T>, PhantomData<&'a T>);

impl<'a, T> CppMut<'a, T> {
    #[inline]
    pub unsafe fn from_ptr(ptr: *mut T) -> Self {
        Self(unsafe { NonNull::new_unchecked(ptr) }, PhantomData)
    }

    #[inline]
    pub unsafe fn from_mut(ptr: &'a mut T) -> Self {
        unsafe { Self::from_ptr(ptr) }
    }

    #[inline]
    pub fn cast_const(self) -> CppRef<'a, T> {
        unsafe { CppRef::from_ptr(self.0.as_ptr().cast_const()) }
    }
}
