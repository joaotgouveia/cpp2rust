extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe)]
pub struct nasty_list_int_ {
    pub l_: (),
}
impl nasty_list_int_ {
    pub unsafe fn new_1() -> Self {
        let mut this = Self {
            l_: unimplemented!(),
        };
        this
    }
    pub unsafe fn destructor(&mut self) {}
    pub unsafe fn front_25(&mut self) -> () {
        return unimplemented!();
    }
    pub unsafe fn size(&self) -> () {
        return unimplemented!();
    }
    pub unsafe fn push_front_35(&mut self, x: *mut ()) {
        unimplemented!();
    }
}
impl Default for nasty_list_int_ {
    fn default() -> Self {
        unsafe { nasty_list_int_::new_1() }
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe)]
pub struct nasty_mutex {}
impl nasty_mutex {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn destructor(&mut self) {}
    pub unsafe fn operator_addr(&mut self) -> *mut nasty_mutex {
        assert!(false);
        return std::ptr::null_mut();
    }
    unsafe fn copy_from(_a0: *const nasty_mutex) -> Self {
        let mut this = Self {};
        assert!(false);
        this
    }
    unsafe fn copy_assign(&mut self, _a0: *const nasty_mutex) -> *mut nasty_mutex {
        assert!(false);
        return &mut (*(self as *mut nasty_mutex));
    }
    pub unsafe fn lock(&mut self) {}
    pub unsafe fn try_lock(&mut self) -> bool {
        return true;
    }
    pub unsafe fn unlock(&mut self) {}
    pub unsafe fn lock_shared(&mut self) {}
    pub unsafe fn try_lock_shared(&mut self) -> bool {
        return true;
    }
    pub unsafe fn unlock_shared(&mut self) {}
}
impl Clone for nasty_mutex {
    fn clone(&self) -> Self {
        unsafe { nasty_mutex::copy_from(self as *const nasty_mutex) }
    }
}
impl Default for nasty_mutex {
    fn default() -> Self {
        unsafe { nasty_mutex::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ConstexprFixedCapacityDeque_int__10_ {
    data_: [i32; 10],
    size_: i32,
}
impl ConstexprFixedCapacityDeque_int__10_ {
    pub unsafe fn begin_2(&mut self) -> *mut i32 {
        return self.data_.as_mut_ptr();
    }
    pub unsafe fn size(&self) -> usize {
        return (self.size_ as usize);
    }
    pub unsafe fn front(&self) -> *const i32 {
        assert!(((self.size_) >= (1)));
        return &self.data_[(0) as usize];
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
    pub unsafe fn push_front(&mut self, mut t: i32) {
        (unsafe {
            let _pos: *const i32 =
                (unsafe { ConstexprFixedCapacityDeque_int__10_::begin_2(self) }).cast_const();
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
pub unsafe fn test_0(mut c: ()) -> bool {
    let mut i: () = unimplemented!();
    unimplemented!();
    assert!(((unimplemented!()) == (1_())));
    assert!(((*unimplemented!()) == (3)));
    unimplemented!();
    assert!(((unimplemented!()) == (2_())));
    assert!(((*unimplemented!()) == (4)));
    return true;
}
pub unsafe fn test_1(mut c: nasty_list_int_) -> bool {
    let mut i: () = unimplemented!();
    unimplemented!();
    assert!(((unsafe { nasty_list_int_::size(&c,) }) == (1_())));
    assert!(((*(unsafe { nasty_list_int_::front_25(&mut c,) })) == (3)));
    unimplemented!();
    assert!(((unsafe { nasty_list_int_::size(&c,) }) == (2_())));
    assert!(((*(unsafe { nasty_list_int_::front_25(&mut c,) })) == (4)));
    return true;
}
pub unsafe fn test_2(mut c: ConstexprFixedCapacityDeque_int__10_) -> bool {
    let mut i: () = unimplemented!();
    unimplemented!();
    assert!(((unsafe { ConstexprFixedCapacityDeque_int__10_::size(&c,) }) == (1_usize)));
    assert!(((*(unsafe { ConstexprFixedCapacityDeque_int__10_::front(&c,) })) == (3)));
    unimplemented!();
    assert!(((unsafe { ConstexprFixedCapacityDeque_int__10_::size(&c,) }) == (2_usize)));
    assert!(((*(unsafe { ConstexprFixedCapacityDeque_int__10_::front(&c,) })) == (4)));
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
    (unsafe { test_0(unimplemented!()) });
    (unsafe { test_1(nasty_list_int_::new_1()) });
    (unsafe { test_2(<ConstexprFixedCapacityDeque_int__10_>::default()) });
    const _: () = assert!(true, "test(ConstexprFixedCapacityDeque<int, 10>())");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
