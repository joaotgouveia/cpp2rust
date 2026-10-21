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
pub struct Guard__lambda_at_lambda_as_field_cpp_34_13__ {
    pub f: FnPtr<fn()>,
}
impl Guard__lambda_at_lambda_as_field_cpp_34_13__ {
    pub unsafe fn destructor(&mut self) {
        (unsafe { self.f.call() });
    }
}
impl Default for Guard__lambda_at_lambda_as_field_cpp_34_13__ {
    fn default() -> Self {
        unsafe {
            Guard__lambda_at_lambda_as_field_cpp_34_13__ {
                f: FnPtr::<fn()>::new(|| unsafe {
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_0)).postfix_inc();
                }),
            }
        }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct Guard__lambda_at_lambda_as_field_cpp_45_13__ {
    pub f: FnPtr<fn()>,
}
impl Guard__lambda_at_lambda_as_field_cpp_45_13__ {
    pub unsafe fn destructor(&mut self) {
        (unsafe { self.f.call() });
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg)]
pub struct Holder__lambda_at_lambda_as_field_cpp_39_12__ {
    pub f: FnPtr<fn(()) -> ()>,
    pub calls: (),
}
impl Holder__lambda_at_lambda_as_field_cpp_39_12__ {
    pub unsafe fn call(&mut self, mut x: ()) -> () {
        self.calls.postfix_inc();
        return (unsafe { self.f.call(x) });
    }
}
impl Default for Holder__lambda_at_lambda_as_field_cpp_39_12__ {
    fn default() -> Self {
        unsafe {
            Holder__lambda_at_lambda_as_field_cpp_39_12__ {
                f: FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
                    unsafe {
                        return ((x) + (1_()));
                    }
                }),
                calls: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct Holder__lambda_at_lambda_as_field_cpp_51_12__ {
    pub f: FnPtr<fn(()) -> ()>,
    pub calls: (),
}
impl Holder__lambda_at_lambda_as_field_cpp_51_12__ {
    pub unsafe fn call(&mut self, mut x: ()) -> () {
        self.calls.postfix_inc();
        return (unsafe { self.f.call(x) });
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe)]
pub struct Moving__lambda_at_lambda_as_field_cpp_64_12__ {
    pub f: FnPtr<fn(()) -> ()>,
    pub moves: (),
}
impl Moving__lambda_at_lambda_as_field_cpp_64_12__ {
    pub unsafe fn new(mut f: FnPtr<fn(()) -> ()>) -> Self {
        let mut this = Self {
            f: f.copy_from(),
            moves: 0_(),
        };
        this
    }
    pub unsafe fn move_from(o: *mut Moving__lambda_at_lambda_as_field_cpp_64_12__) -> Self {
        let mut this = Self {
            f: (*unimplemented!()).move_from(),
            moves: (((*o).moves) + (1_())),
        };
        this
    }
}
impl Default for Moving__lambda_at_lambda_as_field_cpp_64_12__ {
    fn default() -> Self {
        unsafe {
            Moving__lambda_at_lambda_as_field_cpp_64_12__ {
                f: FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
                    unsafe {
                        return ((x) * (2_()));
                    }
                }),
                moves: 0_(),
            }
        }
    }
}
pub unsafe fn wrap_1(mut fn_: FnPtr<fn(()) -> ()>) -> FnPtr<fn(()) -> ()> {
    return lambda_unsafe!(
        {
            let fn_: FnPtr<fn(()) -> ()> = fn_.copy_from();
        },
        |mut x: ()| -> () {
            return ((unsafe { fn_.call(x) }) + (1_()));
        }
    );
}
pub unsafe fn wrap_2(mut fn_: FnPtr<fn(()) -> ()>) -> FnPtr<fn(()) -> ()> {
    return lambda_unsafe!(
        {
            let fn_: FnPtr<fn(()) -> ()> = fn_.copy_from();
        },
        |mut x: ()| -> () {
            return ((unsafe { fn_.call(x) }) + (1_()));
        }
    );
}
pub static mut count_0: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    {
        let mut g: Guard__lambda_at_lambda_as_field_cpp_34_13__ =
            Guard__lambda_at_lambda_as_field_cpp_34_13__ {
                f: FnPtr::<fn()>::new(|| unsafe {
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_0)).postfix_inc();
                }),
            };
        let _dtor_g = ScopedDestructorUnsafe::new(
            &raw mut g,
            Guard__lambda_at_lambda_as_field_cpp_34_13__::destructor,
        );
        unimplemented!();
    }
    unimplemented!();
    let mut n: Holder__lambda_at_lambda_as_field_cpp_39_12__ =
        Holder__lambda_at_lambda_as_field_cpp_39_12__ {
            f: FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
                unsafe {
                    return ((x) + (1_()));
                }
            }),
            calls: 0_(),
        };
    unimplemented!();
    unimplemented!();
    let mut cleaned: () = 0_();
    {
        let mut g: Guard__lambda_at_lambda_as_field_cpp_45_13__ =
            Guard__lambda_at_lambda_as_field_cpp_45_13__ {
                f: lambda_unsafe!(
                    {
                        let cleaned: *mut () = &mut cleaned;
                    },
                    || {
                        (*cleaned).postfix_inc();
                    }
                ),
            };
        let _dtor_g = ScopedDestructorUnsafe::new(
            &raw mut g,
            Guard__lambda_at_lambda_as_field_cpp_45_13__::destructor,
        );
        unimplemented!();
    }
    unimplemented!();
    let mut factor: () = 3_();
    let mut h: Holder__lambda_at_lambda_as_field_cpp_51_12__ =
        Holder__lambda_at_lambda_as_field_cpp_51_12__ {
            f: lambda_unsafe!(
                {
                    let factor: () = factor;
                },
                |mut x: ()| -> () {
                    return ((x) * (factor));
                }
            ),
            calls: 0_(),
        };
    factor = 100_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut w: FnPtr<fn(()) -> ()> = (unsafe {
        wrap_1(lambda_unsafe!(
            {
                let factor: () = factor;
            },
            |mut x: ()| -> () {
                return ((x) * (factor));
            }
        ))
    });
    factor = 7_();
    unimplemented!();
    let mut ww: FnPtr<fn(()) -> ()> = (unsafe { wrap_2(w.copy_from()) });
    unimplemented!();
    let mut m: Moving__lambda_at_lambda_as_field_cpp_64_12__ =
        Moving__lambda_at_lambda_as_field_cpp_64_12__::new({
            FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
                unsafe {
                    return ((x) * (2_()));
                }
            })
        });
    let mut m2: Moving__lambda_at_lambda_as_field_cpp_64_12__ =
        Moving__lambda_at_lambda_as_field_cpp_64_12__::move_from({ &mut (*unimplemented!()) });
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const count_0);
}
