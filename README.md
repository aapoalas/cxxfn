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

## YKINMKBYKIOK

Few years ago, I gave a talk about doing FFI crimes with a C++ dynamic library,
including creating `std::function` objects on the wrong side of the boundary.
After the talk, a listener came to give me their business card upon which they
had written the above abbreviation. They explained that it stands for the
following phrase:

> Your kink is not my kink, but your kink is okay.

If you're thinking of using this library, then your kink is probably the same as
my kink, and it is very much okay!
