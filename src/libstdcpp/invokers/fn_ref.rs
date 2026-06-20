use super::super::LibstdCppFn;
use crate::ConvertArg;

pub(crate) unsafe extern "C" fn f0<'a, D: 'a + Clone, R: 'static>(
    f: *const LibstdCppFn<'a, fn() -> R>,
) -> R {
    let (data, f) = unsafe { (*f).functor.get_data::<D, fn(&D) -> R>() };
    f(data)
}
pub(crate) unsafe extern "C" fn f1<'a, D: 'a + Clone, R: 'static, A0: ConvertArg>(
    f: *const LibstdCppFn<'a, fn(A0) -> R>,
    a0: A0::Cxx,
) -> R {
    let (data, f) = unsafe { (*f).functor.get_data::<D, fn(&D, A0::Rust<'_>) -> R>() };
    f(data, unsafe { A0::convert(a0) })
}
pub(crate) unsafe extern "C" fn f2<
    'a,
    D: 'a + Clone,
    R: 'static,
    A0: ConvertArg,
    A1: ConvertArg,
>(
    f: *const LibstdCppFn<'a, fn(A0, A1) -> R>,
    a0: A0::Cxx,
    a1: A1::Cxx,
) -> R {
    let (data, f) = unsafe {
        (*f).functor
            .get_data::<D, fn(&D, A0::Rust<'_>, A1::Rust<'_>) -> R>()
    };
    f(data, unsafe { A0::convert(a0) }, unsafe { A1::convert(a1) })
}
pub(crate) unsafe extern "C" fn f3<
    'a,
    D: 'a + Clone,
    R: 'static,
    A0: ConvertArg,
    A1: ConvertArg,
    A2: ConvertArg,
>(
    f: *const LibstdCppFn<'a, fn(A0) -> R>,
    a0: A0::Cxx,
    a1: A1::Cxx,
    a2: A2::Cxx,
) -> R {
    let (data, f) = unsafe {
        (*f).functor
            .get_data::<D, fn(&D, A0::Rust<'_>, A1::Rust<'_>, A2::Rust<'_>) -> R>()
    };
    f(
        data,
        unsafe { A0::convert(a0) },
        unsafe { A1::convert(a1) },
        unsafe { A2::convert(a2) },
    )
}
pub(crate) unsafe extern "C" fn f4<
    'a,
    D: 'a + Clone,
    R: 'static,
    A0: ConvertArg,
    A1: ConvertArg,
    A2: ConvertArg,
    A3: ConvertArg,
>(
    f: *const LibstdCppFn<'a, fn(A0) -> R>,
    a0: A0::Cxx,
    a1: A1::Cxx,
    a2: A2::Cxx,
    a3: A3::Cxx,
) -> R {
    let (data, f) = unsafe {
        (*f).functor
            .get_data::<D, fn(&D, A0::Rust<'_>, A1::Rust<'_>, A2::Rust<'_>, A3::Rust<'_>) -> R>()
    };
    f(
        data,
        unsafe { A0::convert(a0) },
        unsafe { A1::convert(a1) },
        unsafe { A2::convert(a2) },
        unsafe { A3::convert(a3) },
    )
}
