extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg)]
pub struct Sorted__lambda_at_lambda_decltype_type_cpp_22_18__ {
    pub items: [(); 4],
    pub size: (),
    pub cmp: FnPtr<fn((), ()) -> ()>,
}
impl Sorted__lambda_at_lambda_decltype_type_cpp_22_18__ {
    pub unsafe fn insert(&mut self, mut v: ()) {
        let mut i: () = self.size;
        'loop_: while ((i) > (0_()))
            && (unsafe {
                let _b: () = self.items[((i) - (1_())) as usize];
                self.cmp.call(v, _b)
            })
        {
            self.items[(i) as usize] = self.items[((i) - (1_())) as usize];
            i.postfix_dec();
        }
        self.items[(i) as usize] = v;
        self.size.postfix_inc();
    }
}
impl Default for Sorted__lambda_at_lambda_decltype_type_cpp_22_18__ {
    fn default() -> Self {
        unsafe {
            Sorted__lambda_at_lambda_decltype_type_cpp_22_18__ {
                items: [unimplemented!(); 4],
                size: 0_(),
                cmp: FnPtr::<fn((), ()) -> ()>::new(|mut a: (), mut b: ()| -> () {
                    unsafe {
                        return ((a) > (b));
                    }
                }),
            }
        }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg)]
pub struct Sorted__lambda_at_lambda_decltype_type_cpp_19_23__ {
    pub items: [(); 4],
    pub size: (),
    pub cmp: FnPtr<fn((), ()) -> ()>,
}
impl Sorted__lambda_at_lambda_decltype_type_cpp_19_23__ {
    pub unsafe fn insert(&mut self, mut v: ()) {
        let mut i: () = self.size;
        'loop_: while ((i) > (0_()))
            && (unsafe {
                let _b: () = self.items[((i) - (1_())) as usize];
                self.cmp.call(v, _b)
            })
        {
            self.items[(i) as usize] = self.items[((i) - (1_())) as usize];
            i.postfix_dec();
        }
        self.items[(i) as usize] = v;
        self.size.postfix_inc();
    }
}
impl Default for Sorted__lambda_at_lambda_decltype_type_cpp_19_23__ {
    fn default() -> Self {
        unsafe {
            Sorted__lambda_at_lambda_decltype_type_cpp_19_23__ {
                items: [unimplemented!(); 4],
                size: 0_(),
                cmp: FnPtr::<fn((), ()) -> ()>::new(|mut a: (), mut b: ()| -> () {
                    unsafe {
                        return ((a) < (b));
                    }
                }),
            }
        }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut greater: FnPtr<fn((), ()) -> ()> =
        FnPtr::<fn((), ()) -> ()>::new(|mut a: (), mut b: ()| -> () {
            unsafe {
                return ((a) > (b));
            }
        });
    let mut desc: Sorted__lambda_at_lambda_decltype_type_cpp_22_18__ =
        <Sorted__lambda_at_lambda_decltype_type_cpp_22_18__>::default();
    (unsafe { Sorted__lambda_at_lambda_decltype_type_cpp_22_18__::insert(&mut desc, 2_()) });
    (unsafe { Sorted__lambda_at_lambda_decltype_type_cpp_22_18__::insert(&mut desc, 7_()) });
    (unsafe { Sorted__lambda_at_lambda_decltype_type_cpp_22_18__::insert(&mut desc, 4_()) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut asc: Sorted__lambda_at_lambda_decltype_type_cpp_19_23__ =
        <Sorted__lambda_at_lambda_decltype_type_cpp_19_23__>::default();
    (unsafe { Sorted__lambda_at_lambda_decltype_type_cpp_19_23__::insert(&mut asc, 2_()) });
    (unsafe { Sorted__lambda_at_lambda_decltype_type_cpp_19_23__::insert(&mut asc, 7_()) });
    (unsafe { Sorted__lambda_at_lambda_decltype_type_cpp_19_23__::insert(&mut asc, 4_()) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut fresh: FnPtr<fn((), ()) -> ()> =
        FnPtr::<fn((), ()) -> ()>::new(|mut a: (), mut b: ()| -> () {
            unsafe {
                return ((a) > (b));
            }
        });
    unimplemented!();
    unimplemented!();
    let mut assigned: FnPtr<fn((), ()) -> ()> =
        FnPtr::<fn((), ()) -> ()>::new(|mut a: (), mut b: ()| -> () {
            unsafe {
                return ((a) > (b));
            }
        });
    assigned = (greater).clone();
    unimplemented!();
    let mut less: FnPtr<fn((), ()) -> ()> =
        FnPtr::<fn((), ()) -> ()>::new(|mut a: (), mut b: ()| -> () {
            unsafe {
                return ((a) < (b));
            }
        });
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
