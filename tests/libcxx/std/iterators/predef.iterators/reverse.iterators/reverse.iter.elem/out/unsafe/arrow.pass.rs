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
pub struct cpp20_bidirectional_iterator_with_arrow_B_ptr_ {
    it_: *mut B,
}
impl cpp20_bidirectional_iterator_with_arrow_B_ptr_ {
    pub unsafe fn new_1(mut it: *mut B) -> Self {
        let mut this = Self { it_: it };
        this
    }
    pub unsafe fn operator_deref(&self) -> *mut B {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_arrow(&self) -> *mut B {
        {
            return self.it_;
        }
        panic!("ub: non-void function does not return a value")
    }
    pub unsafe fn operator_dec_6(&mut self) -> *mut cpp20_bidirectional_iterator_with_arrow_B_ptr_ {
        self.it_.prefix_dec();
        return &mut (*(self as *mut cpp20_bidirectional_iterator_with_arrow_B_ptr_));
    }
}
pub unsafe fn operator_eq_0(x: *const A, y: *const A) -> bool {
    return (((*x).data_) == ((*y).data_));
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe)]
pub struct A {
    data_: i32,
}
impl A {
    pub unsafe fn new() -> Self {
        let mut this = Self { data_: 1 };
        this
    }
    pub unsafe fn destructor(&mut self) {
        self.data_ = -1_i32;
    }
    pub unsafe fn get(&self) -> i32 {
        return self.data_;
    }
}
impl std::cmp::PartialEq for A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_0(self as *const A, other as *const A) }
    }
}
impl std::cmp::Eq for A {}
impl Default for A {
    fn default() -> Self {
        unsafe { A::new() }
    }
}
pub unsafe fn test_1(mut i: *mut A, mut x: A) {
    let mut r: () = unimplemented!();
    assert!(
        ((unsafe { A::get(&(*(unimplemented!()).cast_const()),) }) == (unsafe { A::get(&x,) }))
    );
}
pub unsafe fn operator_eq_2(x: *const B, y: *const B) -> bool {
    return (((*x).data_) == ((*y).data_));
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, CppFromUnsafe, DestructorUnsafe)]
#[cpp_from(i32, new, some_value)]
pub struct B {
    data_: i32,
}
impl B {
    pub unsafe fn new(mut d: Option<i32>) -> Self {
        let mut d: i32 = d.unwrap_or_else(|| unsafe { 1 });
        let mut this = Self { data_: d };
        this
    }
    pub unsafe fn destructor(&mut self) {
        self.data_ = -1_i32;
    }
    pub unsafe fn get(&self) -> i32 {
        return self.data_;
    }
    pub unsafe fn operator_addr_6(&self) -> *const B {
        return std::ptr::null();
    }
    pub unsafe fn operator_addr_7(&mut self) -> *mut B {
        return std::ptr::null_mut();
    }
}
impl std::cmp::PartialEq for B {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_2(self as *const B, other as *const B) }
    }
}
impl std::cmp::Eq for B {}
impl Default for B {
    fn default() -> Self {
        unsafe { B::new(None) }
    }
}
pub unsafe fn operator_eq_3(x: *const C, y: *const C) -> bool {
    return (((*x).data_) == ((*y).data_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct C {
    data_: i32,
}
impl C {
    pub unsafe fn new() -> Self {
        let mut this = Self { data_: 1 };
        this
    }
    pub unsafe fn get(&self) -> i32 {
        return self.data_;
    }
}
impl std::cmp::PartialEq for C {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_3(self as *const C, other as *const C) }
    }
}
impl std::cmp::Eq for C {}
impl Default for C {
    fn default() -> Self {
        unsafe { C::new() }
    }
}
pub static mut gC_4: std::cell::LazyCell<C> = std::cell::LazyCell::new(|| unsafe { C::new() });

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
    let mut a: A = A::new();
    let _dtor_a = ScopedDestructorUnsafe::new(&raw mut a, A::destructor);
    (unsafe { test_1((&mut a as *mut A).offset((1) as isize), A::new()) });
    {
        let mut l: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        {
            let mut i: () = <UnsafeBidirectionalIterator<B>>::from(unimplemented!());
            assert!(((unsafe { B::get(&(*i.iter_deref()),) }) == (0)));
            i.iter_inc();
            assert!(((unsafe { B::get(&(*i.iter_deref()),) }) == (1)));
            i.iter_inc();
            assert!(((unsafe { B::get(&(*i.iter_deref()),) }) == (2)));
            i.iter_inc();
            assert!(i == <UnsafeBidirectionalIterator<B>>::from(unimplemented!()));
        }
        {
            let mut ri: () = unimplemented!();
            assert!(((unsafe { B::get(&(*unimplemented!()),) }) == (2)));
            unimplemented!();
            assert!(((unsafe { B::get(&(*unimplemented!()),) }) == (1)));
            unimplemented!();
            assert!(((unsafe { B::get(&(*unimplemented!()),) }) == (0)));
            unimplemented!();
            assert!(unimplemented!());
        }
    }
    {
        let it1: () = unimplemented!();
        const _: () = assert!(true, "it1->get() == gC.get()");;
    }
    {
        let mut data: [B; 3] = [
            B::new({ Some(1) }),
            B::new({ Some(2) }),
            B::new({ Some(3) }),
        ];
        let mut iter: cpp20_bidirectional_iterator_with_arrow_B_ptr_ =
            cpp20_bidirectional_iterator_with_arrow_B_ptr_::new_1({
                data.as_mut_ptr().offset((3) as isize)
            });
        let mut ri: () = unimplemented!();
        assert!(((unsafe { B::get(&(*(unimplemented!()).cast_const()),) }) == (3)));
    }
    {
        (&(*std::cell::LazyCell::force_mut(&mut *&raw mut gC_4)));
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const gC_4);
}
