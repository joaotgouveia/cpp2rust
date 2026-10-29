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
pub struct nasty_vector_int_ {
    pub v_: (),
}
impl nasty_vector_int_ {
    pub unsafe fn new_1() -> Self {
        let mut this = Self {
            v_: unimplemented!(),
        };
        this
    }
    pub unsafe fn destructor(&mut self) {}
    pub unsafe fn begin_12(&mut self) -> () {
        return unimplemented!();
    }
}
impl Default for nasty_vector_int_ {
    fn default() -> Self {
        unsafe { nasty_vector_int_::new_1() }
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
        unimplemented!();
        return std::ptr::null_mut();
    }
    unsafe fn copy_from(_a0: *const nasty_mutex) -> Self {
        let mut this = Self {};
        unimplemented!();
        this
    }
    unsafe fn copy_assign(&mut self, _a0: *const nasty_mutex) -> *mut nasty_mutex {
        unimplemented!();
        return &mut (*(self as *mut nasty_mutex));
    }
    pub unsafe fn lock(&mut self) {}
    pub unsafe fn try_lock(&mut self) -> () {
        return true;
    }
    pub unsafe fn unlock(&mut self) {}
    pub unsafe fn lock_shared(&mut self) {}
    pub unsafe fn try_lock_shared(&mut self) -> () {
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
    data_: [(); 10],
    size_: (),
}
impl ConstexprFixedCapacityDeque_int__10_ {
    pub unsafe fn begin_2(&mut self) -> *mut () {
        return self.data_.as_mut_ptr();
    }
}
impl Default for ConstexprFixedCapacityDeque_int__10_ {
    fn default() -> Self {
        unsafe {
            ConstexprFixedCapacityDeque_int__10_ {
                data_: [unimplemented!(); 10],
                size_: 0_(),
            }
        }
    }
}
pub unsafe fn test_0(mut c: ()) -> () {
    let mut i: () = std_insert_iterator_std_vector_int__std_allocator_int___::new({ &mut c }, {
        unimplemented!()
    });
    return true;
}
pub unsafe fn test_1(mut c: nasty_vector_int_) -> () {
    let mut i: () = std_insert_iterator_nasty_vector_int__::new({ &mut c }, {
        (unsafe { nasty_vector_int_::begin_12(&mut c) })
    });
    return true;
}
pub unsafe fn test_2(mut c: ConstexprFixedCapacityDeque_int__10_) -> () {
    let mut i: () = std_insert_iterator_ConstexprFixedCapacityDeque_int__10__::new({ &mut c }, {
        (unsafe { ConstexprFixedCapacityDeque_int__10_::begin_2(&mut c) })
    });
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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { test_0(unimplemented!()) });
    (unsafe { test_1(nasty_vector_int_::new_1()) });
    (unsafe { test_2(<ConstexprFixedCapacityDeque_int__10_>::default()) });
    const _: () = assert!(true, "test(ConstexprFixedCapacityDeque<int, 10>())");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
