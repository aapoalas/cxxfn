# cxxfn

cxxfn (pronounced "cexx-fun") creates C++ `std::function` objects directly from
Rust for the purposes of interaction with C++ dynamic libraries.

## Why?

Are you getting hot and sweaty with a C++ dylib? Does the library expect you to
pass in `std::function` objects? Are you under a religious obligation to not use
any C++ glue code? In that case, cxxfn may be just what you're looking for.

## Why not?

Interacting with C++ is fraught with danger and unsafety, and `std::function`
especially is quite dangerous. This library tries to make the API mostly safe
for all users involved, but there are very little guarantees that really can be
made with FFI. It is therefore quite likely that you'll catch the C++ `std`
unsafety as well: sometimes that is the price of having a little cxxfn.

If you do not need this, do not look at this.

## Current status

The library implements the `std::function<T>` in three variants:

1. `CppFn<'a, F>`: this is your bread-and-butter `std::function<F>`. It is
   `Clone` and can be called re-entrantly, meaning that it cannot capture
   exclusive references or other non-`Clone` data.

1. `CppFnMut<'a, F>`: this is a spicy `std::function<F>` which cannot be cloned
   from Rust and will panic if C++ attempts to do so. It also requires `&mut
   self` to invoke from Rust, so it cannot be called re-entrantly. Note that it
   is undefined behaviour for C++ to call a `CppFnMut` re-entrantly, so when
   using this ensure that the C++ side will never do so. With that caveat, it
   can safely capture exclusive references and other non-`Clone` data.

1. `CppFnOnce<'a, F>`: this is an interesting `std::function<F>` which destructs
   itself after being called. It is also not clonable (though specialisation on
   captures could make this possible) and can therefore capture non-`Clone`
   data. In general there is little benefit for using this variant, except for
   runtime safety.

Note that the Rust variants have a lifetime on them: this is to allow capturing
on-stack data. When using these types in FFI declarations it is up to the
declaration to decide what the correct lifetime is. As an example, a
`Class::forEach` method would/could take a `CppFnMut<'a, fn(&D, &Entry, &mut
bool)>` to allow capturing on-stack data for the duration of the iteration,
whereas a `Class::sendRequest` would/could take a `CppFnOnce<'static, fn(&D,
Result)>` to only allow capturing heap data.

The only properly supported C++ compiler/STL is GCC/libstdc++. Work on
Clang/libc++ has started (and hit a bigger snag than was hoped for), and work on
MSVC has not been started.

Eventually the library should also provide `std::move_only_function<F>`,
`std::copyable_function<F>`, and `std::function_ref<F>` equivalents.

## Contributing

Contributions are very welcome - do whatever, be it adding tests (especially
interacting with compiled C++), furthering the Clang and MSVC support, adding
modern C++ `std::function<F>` replacement support, or even changing the public
API to support other kinds of Rust callables as well. The library is very much a
playground currently.

## YKINMKBYKIOK

Few years ago, I gave a talk about doing FFI crimes with a C++ dynamic library,
including creating `std::function` objects on the wrong side of the boundary.
After the talk, a listener came to give me their business card upon which they
had written the above abbreviation. They explained that it stands for the
following phrase:

> Your kink is not my kink, but your kink is okay.

If you're thinking of using this library, then your kink is probably the same as
my kink, and it is very much okay!
