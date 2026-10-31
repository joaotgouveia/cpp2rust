extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub unsafe fn outstanding_alloc() -> usize {
        assert!(
            ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_1))
                >= (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_2)))
        );
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_1))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_2))));
    }
    pub unsafe fn reset() {
        assert!(((unsafe { malloc_allocator_base::outstanding_alloc() }) == (0_usize)));
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_3)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_0)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_1)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_2)) = 0_usize;
    }
}
pub static mut outstanding_bytes_0: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut alloc_count_1: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut dealloc_count_2: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut disable_default_constructor_3: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut i32,
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut ::libc::c_void,
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_int_ {}
pub unsafe fn verify_insert_return_type_4(t: *mut ()) {
    const _: () = assert!(true, "std::is_aggregate_v<verified_type>");;
    const _: () = assert!(
        true,
        "std::is_same_v<verified_type, typename Container::insert_return_type>"
    );;
    let __decomp_5: *mut () = t;
    const _: () = assert!(
        true,
        "std::is_same_v<decltype(pos), typename Container::iterator>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<decltype(t.position), typename Container::iterator>"
    );;
    assert!(((unimplemented!()) == (unimplemented!())));
    const _: () = assert!(true, "std::is_same_v<decltype(ins), bool>");;
    const _: () = assert!(true, "std::is_same_v<decltype(t.inserted), bool>");;
    assert!(((&mut unimplemented!() as *mut bool) == (&mut unimplemented!() as *mut bool)));
    const _: () = assert!(
        true,
        "std::is_same_v<decltype(nod), typename Container::node_type>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<decltype(t.node), typename Container::node_type>"
    );;
    assert!(((unimplemented!()) == (unimplemented!())));
}
pub unsafe fn verify_insert_return_type_6(t: *mut ()) {
    const _: () = assert!(true, "std::is_aggregate_v<verified_type>");;
    const _: () = assert!(
        true,
        "std::is_same_v<verified_type, typename Container::insert_return_type>"
    );;
    let __decomp_7: *mut () = t;
    const _: () = assert!(
        true,
        "std::is_same_v<decltype(pos), typename Container::iterator>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<decltype(t.position), typename Container::iterator>"
    );;
    assert!(((unimplemented!()) == (unimplemented!())));
    const _: () = assert!(true, "std::is_same_v<decltype(ins), bool>");;
    const _: () = assert!(true, "std::is_same_v<decltype(t.inserted), bool>");;
    assert!(((&mut unimplemented!() as *mut bool) == (&mut unimplemented!() as *mut bool)));
    const _: () = assert!(
        true,
        "std::is_same_v<decltype(nod), typename Container::node_type>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<decltype(t.node), typename Container::node_type>"
    );;
    assert!(((unimplemented!()) == (unimplemented!())));
}
pub unsafe fn node_factory_8(c: *mut (), key: *const ()) -> () {
    let mut p: (UnsafeBidirectionalIterator<i32>, bool) = unimplemented!();
    assert!((p.1 as bool));
    return unimplemented!();
}
pub unsafe fn node_factory_9(c: *mut (), key: *const ()) -> () {
    let mut p: (UnsafeBidirectionalIterator<i32>, bool) = unimplemented!();
    assert!((p.1 as bool));
    return unimplemented!();
}
pub unsafe fn test_10(c: *mut ()) {
    let mut nf: Option<unsafe fn(*mut (), *const ()) -> ()> = (Some(node_factory_8));
    let mut c2: () = unimplemented!();
    let mut i: i32 = 0;
    'loop_: while ((i) != (10)) {
        let mut node: () = (unsafe { (nf).unwrap()(&mut c2, &*(&raw const i).cast::<()>()) });
        assert!(!(unimplemented!()));
        let mut irt: () = unimplemented!();
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(((*unimplemented!().iter_deref()) == (i)));
        (unsafe { verify_insert_return_type_4(&mut irt) });
        i.prefix_inc();
    }
    assert!(((unimplemented!()) == (10_())));
    {
        let mut def: () = unimplemented!();
        let mut irt: () = unimplemented!();
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!() == unimplemented!());
        (unsafe { verify_insert_return_type_4(&mut irt) });
    }
    {
        let mut dupl: () = (unsafe {
            let mut _arg1: () = (0 as ());
            (nf).unwrap()(&mut c2, &mut _arg1)
        });
        let mut irt: () = unimplemented!();
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(!(unimplemented!()));
        assert!(unimplemented!() == unimplemented!());
        assert!(((*unimplemented!()) == (0)));
        (unsafe { verify_insert_return_type_4(&mut irt) });
    }
    assert!(((unimplemented!()) == (10_())));
    let mut i: i32 = 0;
    'loop_: while ((i) != (10)) {
        assert!(((unimplemented!()) == (1_())));
        i.prefix_inc();
    }
}
pub unsafe fn test_11(c: *mut ()) {
    let mut nf: Option<unsafe fn(*mut (), *const ()) -> ()> = (Some(node_factory_9));
    let mut c2: () = unimplemented!();
    let mut i: i32 = 0;
    'loop_: while ((i) != (10)) {
        let mut node: () = (unsafe { (nf).unwrap()(&mut c2, &*(&raw const i).cast::<()>()) });
        assert!(!(unimplemented!()));
        let mut irt: () = unimplemented!();
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(((*unimplemented!().iter_deref()) == (i)));
        (unsafe { verify_insert_return_type_6(&mut irt) });
        i.prefix_inc();
    }
    assert!(((unimplemented!()) == (10_())));
    {
        let mut def: () = unimplemented!();
        let mut irt: () = unimplemented!();
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!() == unimplemented!());
        (unsafe { verify_insert_return_type_6(&mut irt) });
    }
    {
        let mut dupl: () = (unsafe {
            let mut _arg1: () = (0 as ());
            (nf).unwrap()(&mut c2, &mut _arg1)
        });
        let mut irt: () = unimplemented!();
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(!(unimplemented!()));
        assert!(unimplemented!() == unimplemented!());
        assert!(((*unimplemented!()) == (0)));
        (unsafe { verify_insert_return_type_6(&mut irt) });
    }
    assert!(((unimplemented!()) == (10_())));
    let mut i: i32 = 0;
    'loop_: while ((i) != (10)) {
        assert!(((unimplemented!()) == (1_())));
        i.prefix_inc();
    }
}
pub unsafe fn test_12() -> bool {
    let mut m: () = unimplemented!();
    (unsafe { test_10(&mut m) });
    let mut m2: () = unimplemented!();
    (unsafe { test_11(&mut m2) });
    return true;
}
pub fn main() {
    let mut args: Vec<Vec<u8>> = std::env::args()
        .map(|arg| arg.as_bytes().to_vec())
        .collect();
    args.iter_mut().for_each(|v| v.push(0));
    let mut argv: Vec<*mut libc::c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut libc::c_char)
        .collect();
    argv.push(::std::ptr::null_mut());
    unsafe {
        __cpp2rust_init_globals();
        ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32)
    }
}
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { test_12() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
}
