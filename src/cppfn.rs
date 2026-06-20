use crate::ConvertArg;
#[cfg(all(not(unix), not(windows)))]
use crate::libcpp::LibCppFn as Inner;
#[cfg(unix)]
use crate::libstdcpp::LibstdCppFn as Inner;
#[cfg(windows)]
use crate::msvc::MsvcCppFn as Inner;

#[repr(transparent)]
#[derive(Clone)]
pub struct CppFn<'a, F: 'static + Copy> {
    inner: Inner<'a, F>,
}

impl<'a, R: 'static> CppFn<'a, fn() -> R> {
    #[inline]
    pub fn new<D: 'a + Clone>(data: D, f: fn(&D) -> R) -> Self {
        Self {
            inner: Inner::<fn() -> R>::new(data, f),
        }
    }

    #[inline]
    pub fn invoke(&self) -> R {
        self.inner.invoke()
    }
}

impl<'a, R: 'static, A0: ConvertArg> CppFn<'a, fn(A0) -> R> {
    #[inline]
    pub fn new<D: 'a + Clone>(data: D, f: fn(&D, A0::Rust<'_>) -> R) -> Self {
        Self {
            inner: Inner::<fn(A0) -> R>::new(data, f),
        }
    }

    #[inline]
    pub fn invoke(&self, a0: A0::Cxx) -> R {
        self.inner.invoke(a0)
    }
}

impl<'a, R: 'static, A0: ConvertArg, A1: ConvertArg> CppFn<'a, fn(A0, A1) -> R> {
    #[inline]
    pub fn new<D: 'a + Clone>(data: D, f: fn(&D, A0::Rust<'_>, A1::Rust<'_>) -> R) -> Self {
        Self {
            inner: Inner::<fn(A0, A1) -> R>::new(data, f),
        }
    }

    #[inline]
    pub fn invoke(&self, a0: A0::Cxx, a1: A1::Cxx) -> R {
        self.inner.invoke(a0, a1)
    }
}

impl<'a, R: 'static, A0: ConvertArg, A1: ConvertArg, A2: ConvertArg>
    CppFn<'a, fn(A0, A1, A2) -> R>
{
    #[inline]
    pub fn new<D: 'a + Clone>(
        data: D,
        f: fn(&D, A0::Rust<'_>, A1::Rust<'_>, A2::Rust<'_>) -> R,
    ) -> Self {
        Self {
            inner: Inner::<fn(A0, A1, A2) -> R>::new(data, f),
        }
    }

    #[inline]
    pub fn invoke(&self, a0: A0::Cxx, a1: A1::Cxx, a2: A2::Cxx) -> R {
        self.inner.invoke(a0, a1, a2)
    }
}

impl<'a, R: 'static, A0: ConvertArg, A1: ConvertArg, A2: ConvertArg, A3: ConvertArg>
    CppFn<'a, fn(A0, A1, A2, A3) -> R>
{
    #[inline]
    pub fn new<D: 'a + Clone>(
        data: D,
        f: fn(&D, A0::Rust<'_>, A1::Rust<'_>, A2::Rust<'_>, A3::Rust<'_>) -> R,
    ) -> Self {
        Self {
            inner: Inner::<fn(A0, A1, A2, A3) -> R>::new(data, f),
        }
    }

    #[inline]
    pub fn invoke(&self, a0: A0::Cxx, a1: A1::Cxx, a2: A2::Cxx, a3: A3::Cxx) -> R {
        self.inner.invoke(a0, a1, a2, a3)
    }
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use crate::ByVal;

    use super::*;

    #[test]
    fn nullary_cxxfn() {
        let cxxfn = CppFn::<fn() -> u64>::new(66u64, |data| -> u64 {
            assert_eq!(*data, 66u64);
            128
        });
        assert_eq!(cxxfn.invoke(), 128);
    }

    #[test]
    fn nullary_cxxfn_with_capture() {
        let val = Cell::new(0u64);
        let cxxfn = CppFn::<fn() -> u64>::new(&val, |data| -> u64 {
            let v = data.get();
            data.set(v + 1);
            v
        });
        for i in 0..10 {
            assert_eq!(cxxfn.invoke(), i);
        }
        let _ = cxxfn.clone();
    }

    #[test]
    fn unary_cxxfn() {
        let cxxfn = CppFn::<fn(ByVal<u64>) -> u64>::new(66u64, |data, val| -> u64 {
            assert_eq!(*data, 66u64);
            assert_eq!(val, 67u64);
            *data + val
        });
        assert_eq!(cxxfn.invoke(&67), 66 + 67);
        let _ = cxxfn.clone();
    }

    #[test]
    fn unary_cxxfn_with_capture() {
        let val = Cell::new(0u64);
        let cxxfn = CppFn::<fn(ByVal<u64>) -> u64>::new(&val, |data, val| -> u64 {
            let v = data.get();
            data.set(v + val);
            v
        });
        assert_eq!(cxxfn.invoke(&66), 0);
        assert_eq!(cxxfn.invoke(&67), 66);
        assert_eq!(cxxfn.invoke(&0), 66 + 67);
        let _ = cxxfn.clone();
    }

    #[test]
    fn binary_cxxfn() {
        let cxxfn = CppFn::<fn(ByVal<u64>, ByVal<u64>) -> u64>::new((), |_, val, val2| -> u64 {
            val + val2
        });
        assert_eq!(cxxfn.invoke(&0, &0), 0);
        assert_eq!(cxxfn.invoke(&66, &0), 66);
        assert_eq!(cxxfn.invoke(&66, &67), 66 + 67);
        let _ = cxxfn.clone();
    }

    #[test]
    fn trinary_cxxfn() {
        let cxxfn = CppFn::<fn(ByVal<u64>, ByVal<u64>, ByVal<u64>) -> u64>::new(
            (),
            |_, val, val2, val3| -> u64 { val + val2 + val3 },
        );
        assert_eq!(cxxfn.invoke(&1, &2, &3), 1 + 2 + 3);
        assert_eq!(cxxfn.invoke(&56, &32, &76), 56 + 32 + 76);
        let _ = cxxfn.clone();
    }

    #[test]
    fn quaternary_cxxfn() {
        let cxxfn = CppFn::<fn(ByVal<u64>, ByVal<u64>, ByVal<u64>, ByVal<u64>) -> u64>::new(
            (),
            |_, val, val2, val3, val4| -> u64 { val + val2 + val3 + val4 },
        );
        assert_eq!(cxxfn.invoke(&1, &2, &3, &4), 1 + 2 + 3 + 4);
        assert_eq!(cxxfn.invoke(&56, &32, &76, &234), 56 + 32 + 76 + 234);
        let _ = cxxfn.clone();
    }

    #[test]
    fn nullary_heap_cxxfn() {
        let cxxfn = CppFn::<fn() -> u64>::new((0u64, 0u64), |data| -> u64 {
            assert_eq!(*data, (0u64, 0u64));
            128
        });
        assert_eq!(cxxfn.invoke(), 128);
        let _ = cxxfn.clone();
    }

    #[test]
    fn unary_heap_cxxfn() {
        let cxxfn = CppFn::<fn(ByVal<u64>) -> u64>::new((0u64, 0u64), |data, val| -> u64 {
            assert_eq!(*data, (0u64, 0u64));
            assert_eq!(val, 67u64);
            val
        });
        assert_eq!(cxxfn.invoke(&67), 67);
        let _ = cxxfn.clone();
    }

    #[test]
    fn binary_heap_cxxfn() {
        let cxxfn =
            CppFn::<fn(ByVal<u64>, ByVal<u64>) -> u64>::new((0u64, 0u64), |_, val, val2| -> u64 {
                val + val2
            });
        assert_eq!(cxxfn.invoke(&0, &0), 0);
        assert_eq!(cxxfn.invoke(&66, &0), 66);
        assert_eq!(cxxfn.invoke(&66, &67), 66 + 67);
        let _ = cxxfn.clone();
    }

    #[test]
    fn trinary_heap_cxxfn() {
        let cxxfn = CppFn::<fn(ByVal<u64>, ByVal<u64>, ByVal<u64>) -> u64>::new(
            (0u64, 0u64),
            |_, val, val2, val3| -> u64 { val + val2 + val3 },
        );
        assert_eq!(cxxfn.invoke(&1, &2, &3), 1 + 2 + 3);
        assert_eq!(cxxfn.invoke(&56, &32, &76), 56 + 32 + 76);
        let _ = cxxfn.clone();
    }

    #[test]
    fn quaternary_heap_cxxfn() {
        let cxxfn = CppFn::<fn(ByVal<u64>, ByVal<u64>, ByVal<u64>, ByVal<u64>) -> u64>::new(
            (0u64, 0u64),
            |_, val, val2, val3, val4| -> u64 { val + val2 + val3 + val4 },
        );
        assert_eq!(cxxfn.invoke(&1, &2, &3, &4), 1 + 2 + 3 + 4);
        assert_eq!(cxxfn.invoke(&56, &32, &76, &234), 56 + 32 + 76 + 234);
        let _ = cxxfn.clone();
    }
}
