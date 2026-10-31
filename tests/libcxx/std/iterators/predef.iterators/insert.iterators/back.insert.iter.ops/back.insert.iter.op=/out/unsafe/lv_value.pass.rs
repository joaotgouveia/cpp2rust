extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ConstexprFixedCapacityDeque_int__10_ {
    data_: [i32; 10],
    size_: i32,
}
impl ConstexprFixedCapacityDeque_int__10_ {
    pub unsafe fn end_3(&mut self) -> *mut i32 {
        return self.data_.as_mut_ptr().offset((self.size_) as isize);
    }
    pub unsafe fn back(&self) -> *const i32 {
        assert!(((self.size_) >= (1)));
        return &self.data_[((self.size_) - (1)) as usize];
    }
    pub unsafe fn insert(&mut self, mut pos: *const i32, mut t: i32) -> *mut i32 {
        let mut i: i32 = ((((pos as usize - self.data_.as_mut_ptr() as usize)
            / ::std::mem::size_of::<i32>()) as i64) as i32);
        if ((i) != (self.size_)) {
            unimplemented!();
        }
        self.data_[(i) as usize] = t;
        self.size_ += 1;
        return self.data_.as_mut_ptr().offset((i) as isize);
    }
    pub unsafe fn push_back(&mut self, mut t: i32) {
        (unsafe {
            let _pos: *const i32 =
                (unsafe { ConstexprFixedCapacityDeque_int__10_::end_3(self) }).cast_const();
            ConstexprFixedCapacityDeque_int__10_::insert(self, _pos, t)
        });
    }
}
impl Default for ConstexprFixedCapacityDeque_int__10_ {
    fn default() -> Self {
        unsafe {
            ConstexprFixedCapacityDeque_int__10_ {
                data_: [0_i32; 10],
                size_: 0,
            }
        }
    }
}
pub unsafe fn test_0(mut c: Vec<Copyable>) -> bool {
    let mut v: Copyable = Copyable::new();
    let _dtor_v = ScopedDestructorUnsafe::new(&raw mut v, Copyable::destructor);
    let mut i: () = unimplemented!();
    unimplemented!();
    assert!(
        (unsafe {
            let _x: *const Copyable = ((&mut c).last_mut().unwrap());
            operator_eq_1(_x, &v)
        })
    );
    return true;
}
pub unsafe fn test_2(mut c: ConstexprFixedCapacityDeque_int__10_) -> bool {
    let v: i32 = 0_i32;
    let mut i: () = unimplemented!();
    unimplemented!();
    assert!(((*(unsafe { ConstexprFixedCapacityDeque_int__10_::back(&c,) })) == (v)));
    return true;
}
pub unsafe fn operator_eq_1(x: *const Copyable, y: *const Copyable) -> bool {
    return (((*x).data_) == ((*y).data_));
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe)]
pub struct Copyable {
    data_: i32,
}
impl Copyable {
    pub unsafe fn new() -> Self {
        let mut this = Self { data_: 0 };
        this
    }
    pub unsafe fn destructor(&mut self) {
        self.data_ = -1_i32;
    }
}
impl std::cmp::PartialEq for Copyable {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_1(self as *const Copyable, other as *const Copyable) }
    }
}
impl std::cmp::Eq for Copyable {}
impl Default for Copyable {
    fn default() -> Self {
        unsafe { Copyable::new() }
    }
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
    (unsafe { test_0(Vec::new()) });
    (unsafe { test_2(<ConstexprFixedCapacityDeque_int__10_>::default()) });
    const _: () = assert!(true, "test(ConstexprFixedCapacityDeque<int, 10>())");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
