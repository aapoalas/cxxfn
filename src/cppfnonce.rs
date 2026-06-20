use crate::ConvertArg;
#[cfg(all(not(unix), not(windows)))]
use crate::libcpp::LibCppFnOnce as Inner;
#[cfg(unix)]
use crate::libstdcpp::LibstdCppFnOnce as Inner;
#[cfg(windows)]
use crate::msvc::MsvcCppFnOnce as Inner;

#[repr(transparent)]
pub struct CppFnOnce<'a, F: 'static + Copy> {
    inner: Inner<'a, F>,
}

impl<'a, R: 'static> CppFnOnce<'a, fn() -> R> {
    #[inline]
    pub fn new<D: 'a>(data: D, f: fn(D) -> R) -> Self {
        Self {
            inner: Inner::<fn() -> R>::new(data, f),
        }
    }

    #[inline]
    pub fn invoke(self) -> R {
        self.inner.invoke()
    }
}

impl<'a, R: 'static, A0: ConvertArg> CppFnOnce<'a, fn(A0) -> R> {
    #[inline]
    pub fn new<D: 'a>(data: D, f: fn(D, A0::Rust<'_>) -> R) -> Self {
        Self {
            inner: Inner::<fn(A0) -> R>::new(data, f),
        }
    }

    #[inline]
    pub fn invoke(self, a0: A0::Cxx) -> R {
        self.inner.invoke(a0)
    }
}

impl<'a, R: 'static, A0: ConvertArg, A1: ConvertArg> CppFnOnce<'a, fn(A0, A1) -> R> {
    #[inline]
    pub fn new<D: 'a>(data: D, f: fn(D, A0::Rust<'_>, A1::Rust<'_>) -> R) -> Self {
        Self {
            inner: Inner::<fn(A0, A1) -> R>::new(data, f),
        }
    }

    #[inline]
    pub fn invoke(self, a0: A0::Cxx, a1: A1::Cxx) -> R {
        self.inner.invoke(a0, a1)
    }
}

impl<'a, R: 'static, A0: ConvertArg, A1: ConvertArg, A2: ConvertArg>
    CppFnOnce<'a, fn(A0, A1, A2) -> R>
{
    #[inline]
    pub fn new<D: 'a>(data: D, f: fn(D, A0::Rust<'_>, A1::Rust<'_>, A2::Rust<'_>) -> R) -> Self {
        Self {
            inner: Inner::<fn(A0, A1, A2) -> R>::new(data, f),
        }
    }

    #[inline]
    pub fn invoke(self, a0: A0::Cxx, a1: A1::Cxx, a2: A2::Cxx) -> R {
        self.inner.invoke(a0, a1, a2)
    }
}

impl<'a, R: 'static, A0: ConvertArg, A1: ConvertArg, A2: ConvertArg, A3: ConvertArg>
    CppFnOnce<'a, fn(A0, A1, A2, A3) -> R>
{
    #[inline]
    pub fn new<D: 'a>(
        data: D,
        f: fn(D, A0::Rust<'_>, A1::Rust<'_>, A2::Rust<'_>, A3::Rust<'_>) -> R,
    ) -> Self {
        Self {
            inner: Inner::<fn(A0, A1, A2, A3) -> R>::new(data, f),
        }
    }

    #[inline]
    pub fn invoke(self, a0: A0::Cxx, a1: A1::Cxx, a2: A2::Cxx, a3: A3::Cxx) -> R {
        self.inner.invoke(a0, a1, a2, a3)
    }
}

#[cfg(test)]
mod tests {
    use crate::ByVal;

    use super::*;

    #[test]
    fn nullary_cxxfn() {
        let cxxfn = CppFnOnce::<fn() -> u64>::new(66u64, |data| -> u64 {
            assert_eq!(data, 66);
            data
        });
        assert_eq!(cxxfn.invoke(), 66);
    }

    #[test]
    fn nullary_cxxfn_with_capture() {
        let mut val = 0u64;
        let cxxfn = CppFnOnce::<fn() -> u64>::new(&mut val, |data| -> u64 {
            let v = *data;
            *data = v + 1;
            v
        });
        assert_eq!(cxxfn.invoke(), 0);
        assert_eq!(val, 1);
    }

    #[test]
    fn unary_cxxfn() {
        let cxxfn = CppFnOnce::<fn(ByVal<u64>) -> u64>::new(66u64, |data, val| -> u64 {
            assert_eq!(data, 66u64);
            assert_eq!(val, 67u64);
            data + val
        });
        assert_eq!(cxxfn.invoke(&67), 66 + 67);
    }

    #[test]
    fn unary_cxxfn_with_capture() {
        let mut val = 0u64;
        let cxxfn = CppFnOnce::<fn(ByVal<u64>) -> u64>::new(&mut val, |data, val| -> u64 {
            let v = *data;
            *data = v + val;
            v
        });
        assert_eq!(cxxfn.invoke(&66), 0);
        assert_eq!(val, 66);
    }

    #[test]
    fn binary_cxxfn() {
        let cxxfn =
            CppFnOnce::<fn(ByVal<u64>, ByVal<u64>) -> u64>::new((), |_, val, val2| -> u64 {
                val + val2
            });
        assert_eq!(cxxfn.invoke(&1, &2), 3);
    }

    #[test]
    fn trinary_cxxfn() {
        let cxxfn = CppFnOnce::<fn(ByVal<u64>, ByVal<u64>, ByVal<u64>) -> u64>::new(
            (),
            |_, val, val2, val3| -> u64 { val + val2 + val3 },
        );
        assert_eq!(cxxfn.invoke(&1, &2, &3), 1 + 2 + 3);
    }

    #[test]
    fn quaternary_cxxfn() {
        let cxxfn = CppFnOnce::<fn(ByVal<u64>, ByVal<u64>, ByVal<u64>, ByVal<u64>) -> u64>::new(
            (),
            |_, val, val2, val3, val4| -> u64 { val + val2 + val3 + val4 },
        );
        assert_eq!(cxxfn.invoke(&1, &2, &3, &4), 1 + 2 + 3 + 4);
    }

    #[test]
    fn nullary_heap_cxxfn() {
        let cxxfn = CppFnOnce::<fn() -> u64>::new((535u64, 657u64), |data| -> u64 {
            assert_eq!(data, (535u64, 657u64));
            128
        });
        assert_eq!(cxxfn.invoke(), 128);
    }

    #[test]
    fn unary_heap_cxxfn() {
        let cxxfn =
            CppFnOnce::<fn(ByVal<u64>) -> u64>::new((1644u64, 57468u64), |data, val| -> u64 {
                assert_eq!(data, (1644u64, 57468u64));
                assert_eq!(val, 67u64);
                val
            });
        assert_eq!(cxxfn.invoke(&67), 67);
    }

    #[test]
    fn binary_heap_cxxfn() {
        let cxxfn = CppFnOnce::<fn(ByVal<u64>, ByVal<u64>) -> u64>::new(
            (34745u64, 136357u64),
            |(data1, data2), val, val2| -> u64 {
                assert_eq!(data1, 34745u64);
                assert_eq!(data2, 136357u64);
                val + val2
            },
        );
        assert_eq!(cxxfn.invoke(&584, &136), 584 + 136);
    }

    #[test]
    fn trinary_heap_cxxfn() {
        let cxxfn = CppFnOnce::<fn(ByVal<u64>, ByVal<u64>, ByVal<u64>) -> u64>::new(
            (123u64, 456u64),
            |(data1, data2), val, val2, val3| -> u64 {
                assert_eq!(data1, 123u64);
                assert_eq!(data2, 456u64);
                val + val2 + val3
            },
        );
        assert_eq!(cxxfn.invoke(&1, &2, &3), 1 + 2 + 3);
    }

    #[test]
    fn quaternary_heap_cxxfn() {
        let cxxfn = CppFnOnce::<fn(ByVal<u64>, ByVal<u64>, ByVal<u64>, ByVal<u64>) -> u64>::new(
            vec![0u32, 1u32, 2u32],
            |v, val, val2, val3, val4| -> u64 {
                assert_eq!(v[0], 0u32);
                assert_eq!(v[1], 1u32);
                assert_eq!(v[2], 2u32);
                val + val2 + val3 + val4
            },
        );
        assert_eq!(cxxfn.invoke(&1, &2, &3, &4), 1 + 2 + 3 + 4);
    }
}
