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
pub struct Counter_base {}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CppFromUnsafe, DestructorUnsafe, Default)]
#[cpp_from(i32, new_1, ptr)]
pub struct Counter_int_ {
    data_: i32,
}
impl Counter_int_ {
    pub unsafe fn new_1(data: *const i32) -> Self {
        let mut this = Self { data_: (*data) };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut gConstructed_0)).prefix_inc();
        this
    }
    pub unsafe fn copy_from(rhs: *const Counter_int_) -> Self {
        let mut this = Self {
            data_: (*rhs).data_,
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut gConstructed_0)).prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut gConstructed_0)).prefix_dec();
    }
    pub unsafe fn operator_eq(&self, x: *const Counter_int_) -> bool {
        return ((self.data_) == ((*x).data_));
    }
    pub unsafe fn operator_lt(&self, x: *const Counter_int_) -> bool {
        return ((self.data_) < ((*x).data_));
    }
}
impl std::cmp::Ord for Counter_int_ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if Counter_int_::operator_lt(self, other as *const Counter_int_) {
                std::cmp::Ordering::Less
            } else if Counter_int_::operator_lt(other, self as *const Counter_int_) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Counter_int_ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Counter_int_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { Counter_int_::operator_eq(self, other as *const Counter_int_) }
    }
}
impl std::cmp::Eq for Counter_int_ {}
impl Clone for Counter_int_ {
    fn clone(&self) -> Self {
        unsafe { Counter_int_::copy_from(self as *const Counter_int_) }
    }
}
pub static mut gConstructed_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub unsafe fn map_equal_1(
    map: *const UnsafeMap<i32, (i32, i32)>,
    mut other: UnsafeMap<i32, (i32, i32)>,
) -> bool {
    return unimplemented!();
}
pub unsafe fn map_equal_2(
    map: *const UnsafeMap<Counter_int_, (Counter_int_, i32)>,
    mut other: UnsafeMap<Counter_int_, (Counter_int_, i32)>,
) -> bool {
    return unimplemented!();
}
pub unsafe fn map_equal_3(
    map: *const UnsafeMap<Counter_int_, (Counter_int_, i32)>,
    mut other: UnsafeMap<Counter_int_, (Counter_int_, i32)>,
) -> bool {
    return unimplemented!();
}
pub unsafe fn map_equal_4(map: *const (), mut other: ()) -> bool {
    return unimplemented!();
}
pub unsafe fn test_5() -> bool {
    {
        let mut src: UnsafeMap<i32, (i32, i32)> = unimplemented!();
        let mut dst: UnsafeMap<i32, (i32, i32)> = unimplemented!();
        unimplemented!();
        assert!((unsafe { map_equal_1(&src, unimplemented!(),) }));
        assert!((unsafe { map_equal_1(&dst, unimplemented!(),) }));
    }
    {
        assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut gConstructed_0)) == (0)));
    };
    {
        {
            let mut first: UnsafeMap<Counter_int_, (Counter_int_, i32)> = unimplemented!();
            let mut second: UnsafeMap<Counter_int_, (Counter_int_, i32)> = unimplemented!();
            let mut third: () = unimplemented!();
            assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut gConstructed_0)) == (8)));
            unimplemented!();
            unimplemented!();
            assert!((unsafe { map_equal_2(&first, unimplemented!(),) }));
            assert!((unsafe { map_equal_3(&second, unimplemented!(),) }));
            assert!((unsafe { map_equal_4(&third, unimplemented!(),) }));
            assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut gConstructed_0)) == (8)));
        }
        assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut gConstructed_0)) == (0)));
        {
            let mut first: UnsafeMap<Counter_int_, (Counter_int_, i32)> = unimplemented!();
            let mut second: UnsafeMap<Counter_int_, (Counter_int_, i32)> = unimplemented!();
            let mut third: () = unimplemented!();
            assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut gConstructed_0)) == (8)));
            unimplemented!();
            unimplemented!();
            assert!((unsafe { map_equal_2(&first, unimplemented!(),) }));
            assert!((unsafe { map_equal_3(&second, unimplemented!(),) }));
            assert!((unsafe { map_equal_4(&third, unimplemented!(),) }));
            assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut gConstructed_0)) == (8)));
        }
        assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut gConstructed_0)) == (0)));
    }
    {
        assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut gConstructed_0)) == (0)));
    }
    {
        let mut first: UnsafeMap<i32, (i32, i32)> = UnsafeMap::new();
        {
            let mut second: UnsafeMap<i32, (i32, i32)> = UnsafeMap::new();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut second: () = unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct comparator_6 {}
impl comparator_6 {
    pub unsafe fn operator_call(&self, lhs: *const Counter_int_, rhs: *const Counter_int_) -> bool {
        return (unsafe {
            let _x: *const Counter_int_ = rhs;
            Counter_int_::operator_lt(&(*lhs), _x)
        });
    }
}
impl Callable2<*const Counter_int_, *const Counter_int_, bool> for comparator_6 {
    fn call(&self, a1: *const Counter_int_, a2: *const Counter_int_) -> bool {
        unsafe { comparator_6::operator_call(self, a1, a2) }
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
    (unsafe { test_5() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const gConstructed_0);
}
