//! # `std::function` as told by libstdc++
//!
//! This is the default one on Linux. The size of `std::function` is 32 bytes
//! and generally has only 8 bytes of inline data.

use std::{
    alloc::{Layout, dealloc},
    marker::PhantomData,
    mem::MaybeUninit,
};

use crate::PunFn;

mod cppfn;
mod cppfnmut;
mod cppfnonce;
mod invokers;

use invokers::*;

pub(crate) use cppfn::LibstdCppFn;
pub(crate) use cppfnmut::LibstdCppFnMut;
pub(crate) use cppfnonce::LibstdCppFnOnce;

#[derive(Default, Clone, Copy)]
#[repr(transparent)]
struct HeapData<'a>(*mut PhantomData<&'a ()>);

impl<'a> HeapData<'a> {
    #[inline]
    fn from<D: 'a>(value: D) -> Self {
        assert!(!fits_inline::<D>());
        let mut this = Self::default();
        this.write::<D>(Box::new(value));
        this
    }

    fn write<D: 'a>(&mut self, value: Box<D>) {
        assert!(!fits_inline::<D>());
        self.0 = Box::into_raw(value).cast::<PhantomData<&'a ()>>();
    }

    unsafe fn drop<D: 'a>(self) {
        let _ = unsafe { Box::from_raw(self.0.cast::<D>()) };
    }

    unsafe fn as_ref<D: 'a>(&self) -> &D {
        unsafe { &*self.0.cast::<D>() }
    }

    unsafe fn as_mut<D: 'a>(&mut self) -> &mut D {
        unsafe { &mut *self.0.cast::<D>() }
    }
}

#[derive(Clone, Copy)]
#[repr(transparent)]
struct StackData<'a>(#[allow(dead_code)] MaybeUninit<u64>, PhantomData<&'a ()>);

impl Default for StackData<'_> {
    fn default() -> Self {
        Self(MaybeUninit::zeroed(), Default::default())
    }
}

impl<'a> StackData<'a> {
    #[inline]
    fn from<D: 'a>(value: D) -> Self {
        let mut this = Self::default();
        unsafe { this.write(value) };
        this
    }

    unsafe fn write<D: 'a>(&mut self, value: D) {
        assert!(fits_inline::<D>());
        unsafe { self.0.as_mut_ptr().cast::<D>().write(value) };
    }

    unsafe fn drop<D: 'a>(&mut self) {
        unsafe { core::ptr::drop_in_place(self.0.as_mut_ptr().cast::<D>()) };
    }

    unsafe fn as_ref<D: 'a>(&self) -> &D {
        unsafe { &*(self as *const Self).cast::<D>() }
    }

    unsafe fn as_mut<D: 'a>(&mut self) -> &mut D {
        unsafe { &mut *(self as *mut Self).cast::<D>() }
    }
}

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

pub(crate) union CapturedData<'a> {
    heap: (HeapData<'a>, PunFn<'a>),
    stack: (StackData<'a>, PunFn<'a>),
    #[allow(dead_code)]
    closure: HeapData<'a>,
}

impl<'a> CapturedData<'a> {
    #[inline]
    unsafe fn write_data<D: 'a>(&mut self, data: D, f: PunFn<'a>) {
        if const { fits_inline::<D>() } {
            unsafe { self.write_stack_data::<D>(data, f) };
        } else {
            unsafe { self.write_heap_data::<D>(data, f) };
        }
    }

    #[inline]
    unsafe fn write_stack_data<D: 'a>(&mut self, data: D, f: PunFn<'a>) {
        Self::assert_stack_size::<D>();
        unsafe {
            self.stack.0.write(data);
            self.stack.1 = f;
        }
    }

    #[inline]
    unsafe fn write_heap_data<D: 'a>(&mut self, data: D, f: PunFn<'a>) {
        Self::assert_heap_size::<D>();
        unsafe {
            self.heap.0.write::<D>(Box::new(data));
            self.stack.1 = f;
        };
    }

    fn from_data_and_fn<D: 'a>(data: D, f: PunFn<'a>) -> Self {
        if fits_inline::<D>() {
            CapturedData {
                stack: (StackData::from(data), f),
            }
        } else {
            CapturedData {
                heap: (HeapData::from(data), f),
            }
        }
    }

    fn assert_stack_size<D>() {
        if const { !fits_inline::<D>() } {
            unreachable!("Reading heap-allocated data from stack data");
        }
    }

    fn assert_heap_size<D>() {
        if const { fits_inline::<D>() } {
            unreachable!("Reading stack-allocated data from heap data");
        }
    }

    unsafe fn get_stack_data<D: 'a, F: Copy>(&self) -> (&D, F) {
        Self::assert_stack_size::<D>();
        let data: &D = unsafe { self.stack.0.as_ref::<D>() };
        let f: F = unsafe { *(&raw const self.stack.1).cast::<F>() };
        (data, f)
    }

    unsafe fn get_stack_data_mut<D: 'a, F: Copy>(&mut self) -> (&mut D, F) {
        Self::assert_stack_size::<D>();
        assert!(fits_inline::<D>());
        let data: &mut D = unsafe { self.stack.0.as_mut::<D>() };
        let f: F = unsafe { *(&raw const self.stack.1).cast::<F>() };
        (data, f)
    }

    unsafe fn take_stack_data<D: 'a, F: Copy>(this: *mut Self) -> (D, F) {
        Self::assert_stack_size::<D>();
        let data: D = unsafe { (&raw mut (*this).stack.0).cast::<D>().read() };
        let f = unsafe { (*this).stack.1 };
        let f: F = unsafe { *(&raw const f).cast::<F>() };

        // Null out the functor contents in debug builds.
        #[cfg(debug_assertions)]
        unsafe {
            this.cast::<(*mut (), *const ())>()
                .write((core::ptr::null_mut(), core::ptr::null()));
        };

        (data, f)
    }

    unsafe fn get_data<D: 'a, F: Copy + 'a>(&self) -> (&D, F) {
        if const { fits_inline::<D>() } {
            unsafe { self.get_stack_data::<D, F>() }
        } else {
            unsafe { self.get_heap_data::<D, F>() }
        }
    }

    unsafe fn get_data_mut<D: 'a, F: Copy>(&mut self) -> (&mut D, F) {
        if const { fits_inline::<D>() } {
            unsafe { self.get_stack_data_mut::<D, F>() }
        } else {
            unsafe { self.get_heap_data_mut::<D, F>() }
        }
    }

    unsafe fn take_data<D: 'a, F: Copy>(this: *mut Self) -> (D, F) {
        if const { fits_inline::<D>() } {
            unsafe { Self::take_stack_data::<D, F>(this) }
        } else {
            unsafe { Self::take_heap_data::<D, F>(this) }
        }
    }

    unsafe fn get_heap_data<D: 'a, F: Copy + 'a>(&self) -> (&D, F) {
        Self::assert_heap_size::<D>();
        let data: &D = unsafe { self.heap.0.as_ref() };
        let f: F = unsafe { self.heap.1.get::<F>() };
        (data, f)
    }

    unsafe fn get_heap_data_mut<D: 'a, F: Copy>(&mut self) -> (&mut D, F) {
        Self::assert_heap_size::<D>();
        let data: &mut D = unsafe { self.heap.0.as_mut() };
        let f: F = unsafe { *(&raw const self.heap.1).cast::<F>() };
        (data, f)
    }

    unsafe fn take_heap_data<D: 'a, F: Copy>(this: *mut Self) -> (D, F) {
        Self::assert_heap_size::<D>();
        let data: *mut Box<D> = unsafe { &raw mut (*this).heap.0 }.cast::<Box<D>>();
        let data: D = unsafe {
            let value: D = data.cast::<*const D>().read().read();
            dealloc(*data.cast::<*mut u8>(), Layout::new::<D>());
            value
        };
        let f = unsafe { (*this).heap.1 };
        let f: F = unsafe { *(&raw const f).cast::<F>() };
        // Null out the functor contents in debug builds.
        #[cfg(debug_assertions)]
        unsafe {
            this.cast::<(*mut (), *const ())>()
                .write((core::ptr::null_mut(), core::ptr::null()));
        };
        (data, f)
    }

    unsafe fn drop_data<D: 'a>(&mut self) {
        if const { fits_inline::<D>() } {
            Self::assert_stack_size::<D>();
            unsafe { self.stack.0.drop::<D>() };
        } else {
            Self::assert_heap_size::<D>();
            unsafe { self.heap.0.drop::<D>() };
        }
    }
}

const fn fits_inline<T>() -> bool {
    size_of::<T>() <= 8
}
