extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(16)]
pub struct Moveable {
    #[offset(0)]
    int_: (),
    #[offset(8)]
    double_: (),
}
impl Moveable {
    pub fn new_1() -> Self {
        Self {
            int_: 0_(),
            double_: 0_(),
        }
    }
    pub fn new_2(mut i: (), mut d: ()) -> Self {
        Self {
            int_: i,
            double_: d,
        }
    }
    pub fn move_from(x: Ptr<Moveable>) -> Self {
        let __this: Moveable = Self {
            int_: x.with(|__s| __s.int_),
            double_: x.with(|__s| __s.double_),
        };
        field!(x, int_).write(-1_());
        field!(x, double_).write(-1_());
        __this
    }
}
impl std::cmp::Ord for Moveable {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if MoveableImpl::operator_lt(
                &Rc::new(RefCell::new(Moveable {
                    int_: self.int_.clone(),
                    double_: self.double_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(Moveable {
                    int_: other.int_.clone(),
                    double_: other.double_.clone(),
                }))
                .as_pointer(),
            ) {
                std::cmp::Ordering::Less
            } else if MoveableImpl::operator_lt(
                &Rc::new(RefCell::new(Moveable {
                    int_: other.int_.clone(),
                    double_: other.double_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(Moveable {
                    int_: self.int_.clone(),
                    double_: self.double_.clone(),
                }))
                .as_pointer(),
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Moveable {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Moveable {
    fn eq(&self, other: &Self) -> bool {
        {
            MoveableImpl::operator_eq(
                &Rc::new(RefCell::new(Moveable {
                    int_: self.int_.clone(),
                    double_: self.double_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(Moveable {
                    int_: other.int_.clone(),
                    double_: other.double_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for Moveable {}
impl Default for Moveable {
    fn default() -> Self {
        { Moveable::new_1() }
    }
}
pub fn main() {
    let size: usize = ::std::env::args().map(|arg| arg.len() + 1).sum();
    let args: Value<Box<[i8]>> = Rc::new(RefCell::new(vec![0; size].into_boxed_slice()));
    let args_ptr: Ptr<i8> = args.as_pointer();
    let mut off = 0;
    let argv: Value<Box<[Ptr<i8>]>> = Rc::new(RefCell::new(
        ::std::env::args()
            .map(|arg| {
                let ptr = args_ptr.offset(off);
                ptr.write_c_bytes(arg.as_bytes());
                off += arg.len() + 1;
                ptr
            })
            .chain(::std::iter::once(Ptr::null()))
            .collect(),
    ));
    __cpp2rust_init_globals();
    ::std::process::exit(main_0(::std::env::args().len() as i32, argv.as_pointer()));
}
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut i: () = 0_();
        'loop_: while (i < 20_()) {
            unimplemented!();
            i += 2_();
        }
        unimplemented!();
        let mv1: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ 3_() }, { 3_f64 })));
        let mut i: () = 0_();
        'loop_: while (i < 20_()) {
            (*r.borrow_mut()) = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            i += 2_();
        }
        (*r.borrow_mut()) = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mv2: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ 5_() }, { 3_f64 })));
        (*r.borrow_mut()) = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut mv3: Moveable = Moveable::new_2({ -1_() }, { 3_f64 });
        (*r.borrow_mut()) = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut i: () = 0_();
        'loop_: while (i < 20_()) {
            unimplemented!();
            i += 2_();
        }
        unimplemented!();
        let mvkey1: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ 2_() }, { 2_f64 })));
        let mv1: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ 4_() }, { 4_f64 })));
        (*r.borrow_mut()) = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mvkey2: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ 3_() }, { 3_f64 })));
        (*r.borrow_mut()) = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut i: () = 0_();
        'loop_: while (i < 20_()) {
            unimplemented!();
            i += 2_();
        }
        unimplemented!();
        let it : Value<() > = Rc::new(RefCell::new(std___hash_map_const_iterator_std___hash_const_iterator_std___hash_node_std___hash_value_type_int__Moveable___void_ptr__ptr__ :: new_1 ( { unimplemented!()   } , )  )) ;
        let mv1: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ 3_() }, { 3_f64 })));
        let mut i: () = 0_();
        'loop_: while (i < 20_()) {
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            i += 2_();
        }
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut i: () = 0_();
        'loop_: while (i < 20_()) {
            unimplemented!();
            i += 2_();
        }
        unimplemented!();
        let it: Value<()> = Rc::new(RefCell::new(({ next_0(unimplemented!(), None) })));
        let mvkey1: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ 2_() }, { 2_f64 })));
        let mv1: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ 4_() }, { 4_f64 })));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mvkey2: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ 3_() }, { 3_f64 })));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    return 0_();
}
pub trait MoveableImpl {
    fn copy_assign(&self, _a0: Ptr<Moveable>) -> Ptr<Moveable> {
        unimplemented!()
    }
    fn move_assign(&self, x: Ptr<Moveable>) -> Ptr<Moveable>;
    fn operator_eq(&self, x: Ptr<Moveable>) -> ();
    fn operator_lt(&self, x: Ptr<Moveable>) -> ();
    fn hash(&self) -> ();
    fn get(&self) -> ();
    fn moved(&self) -> ();
}
impl MoveableImpl for Ptr<Moveable> {
    fn move_assign(&self, x: Ptr<Moveable>) -> Ptr<Moveable> {
        field!((*self), int_).write({ x.with(|__s| __s.int_) });
        field!(x, int_).write(-1_());
        field!((*self), double_).write({ x.with(|__s| __s.double_) });
        field!(x, double_).write(-1_());
        return (*self).clone();
    }
    fn operator_eq(&self, x: Ptr<Moveable>) -> () {
        return ({ (*self).with(|__s| __s.int_) } == { x.with(|__s| __s.int_) })
            && ({ (*self).with(|__s| __s.double_) } == { x.with(|__s| __s.double_) });
    }
    fn operator_lt(&self, x: Ptr<Moveable>) -> () {
        return ({ (*self).with(|__s| __s.int_) } < { x.with(|__s| __s.int_) })
            || (({ (*self).with(|__s| __s.int_) } == { x.with(|__s| __s.int_) })
                && ({ (*self).with(|__s| __s.double_) } < { x.with(|__s| __s.double_) }));
    }
    fn hash(&self) -> () {
        return ({
            (unimplemented!() as std___hash_impl_int__void_)((*self).with(|__s| __s.int_))
        })
        .wrapping_add(
            ({
                (unimplemented!() as std___hash_impl_double__void_)((*self).with(|__s| __s.double_))
            }),
        );
    }
    fn get(&self) -> () {
        return (*self).with(|__s| __s.int_);
    }
    fn moved(&self) -> () {
        return ((*self).with(|__s| __s.int_) == -1_());
    }
}
pub fn __cpp2rust_init_globals() {}
