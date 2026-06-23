mod cppfn;
mod cppfnmut;
mod cppfnonce;

#[allow(non_camel_case_types)]
struct type_info;

#[repr(C)]
struct LibcppFunctionVtable<This> {
    clone: fn(this: &This) -> *mut This,
    clone_in: fn(p: *mut (), this: &This) -> *mut This,
    destroy: fn(p: *mut This),
    destroy_deallocate: fn(p: *mut This),
    invoke: fn(p: *const This),
    #[cfg(rtti)]
    target: fn(&self, ti: &type_info) -> *const (),
    #[cfg(rtti)]
    target_type: fn(this: &This) -> &type_info,
}
