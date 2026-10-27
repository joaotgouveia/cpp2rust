extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct gnu_obj {
    #[offset(0)]
    i: (),
}
impl gnu_obj {
    pub fn new(arg: Option<()>) -> Self {
        let mut arg: () = arg.unwrap_or_else(|| 0_());
        Self { i: arg }
    }
}
impl std::cmp::Ord for gnu_obj {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if gnu_objImpl::operator_lt(
                &Rc::new(RefCell::new(gnu_obj { i: self.i.clone() })).as_pointer(),
                Rc::new(RefCell::new(gnu_obj { i: other.i.clone() })).as_pointer(),
            ) {
                std::cmp::Ordering::Less
            } else if gnu_objImpl::operator_lt(
                &Rc::new(RefCell::new(gnu_obj { i: other.i.clone() })).as_pointer(),
                Rc::new(RefCell::new(gnu_obj { i: self.i.clone() })).as_pointer(),
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for gnu_obj {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for gnu_obj {
    fn eq(&self, other: &Self) -> bool {
        {
            gnu_objImpl::operator_eq(
                &Rc::new(RefCell::new(gnu_obj { i: self.i.clone() })).as_pointer(),
                Rc::new(RefCell::new(gnu_obj { i: other.i.clone() })).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for gnu_obj {}
impl Default for gnu_obj {
    fn default() -> Self {
        { gnu_obj::new(None) }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct gnu_t_long_ {
    #[offset(0)]
    pub b: (),
}
impl gnu_t_long_ {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    return 0_();
}
pub trait gnu_objImpl {
    fn operator_eq(&self, rhs: Ptr<gnu_obj>) -> ();
    fn operator_ne(&self, rhs: Ptr<gnu_obj>) -> ();
    fn operator_lt(&self, rhs: Ptr<gnu_obj>) -> ();
}
impl gnu_objImpl for Ptr<gnu_obj> {
    fn operator_eq(&self, rhs: Ptr<gnu_obj>) -> () {
        return ({ (*self).with(|__s| __s.i) } == { rhs.with(|__s| __s.i) });
    }
    fn operator_ne(&self, rhs: Ptr<gnu_obj>) -> () {
        return ({ (*self).with(|__s| __s.i) } != { rhs.with(|__s| __s.i) });
    }
    fn operator_lt(&self, rhs: Ptr<gnu_obj>) -> () {
        return ({ (*self).with(|__s| __s.i) } < { rhs.with(|__s| __s.i) });
    }
}
pub trait gnu_t_long_Impl {
    fn operator_eq(&self, rhs: Ptr<gnu_t_long_>) -> () {
        unimplemented!()
    }
    fn operator_ne(&self, rhs: Ptr<gnu_t_long_>) -> ();
    fn operator_lt(&self, rhs: Ptr<gnu_t_long_>) -> () {
        unimplemented!()
    }
}
impl gnu_t_long_Impl for Ptr<gnu_t_long_> {
    fn operator_ne(&self, rhs: Ptr<gnu_t_long_>) -> () {
        return ({ (*self).with(|__s| __s.b) } != { rhs.with(|__s| __s.b) });
    }
}
pub fn __cpp2rust_init_globals() {}
