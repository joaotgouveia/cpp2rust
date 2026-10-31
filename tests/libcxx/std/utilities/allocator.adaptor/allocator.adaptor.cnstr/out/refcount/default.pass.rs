extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static copy_called_0: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static move_called_1: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(4)]
pub struct A1_int_ {
    #[offset(0)]
    id_: i32,
}
impl A1_int_ {
    pub fn new(id: Option<i32>) -> Self {
        let mut id: i32 = id.unwrap_or_else(|| 0);
        Self { id_: id }
    }
}
impl CppEq for A1_int_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_4(__lhs.clone(), __rhs.clone())
    }
}
impl Default for A1_int_ {
    fn default() -> Self {
        { A1_int_::new(None) }
    }
}
pub fn operator_eq_4(x: Ptr<A1_int_>, y: Ptr<A1_int_>) -> bool {
    return ({ ({ A1_int_Impl::id(&x) }) } == { ({ A1_int_Impl::id(&y) }) });
}
thread_local!(
    pub static copy_called_5: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static move_called_6: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(4)]
pub struct A2_int_ {
    #[offset(0)]
    id_: i32,
}
impl A2_int_ {
    pub fn new(id: Option<i32>) -> Self {
        let mut id: i32 = id.unwrap_or_else(|| 0);
        Self { id_: id }
    }
}
impl CppEq for A2_int_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_8(__lhs.clone(), __rhs.clone())
    }
}
impl Default for A2_int_ {
    fn default() -> Self {
        { A2_int_::new(None) }
    }
}
pub fn operator_eq_8(x: Ptr<A2_int_>, y: Ptr<A2_int_>) -> bool {
    return ({ ({ A2_int_Impl::id(&x) }) } == { ({ A2_int_Impl::id(&y) }) });
}
thread_local!(
    pub static copy_called_9: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static move_called_10: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(4)]
pub struct A3_int_ {
    #[offset(0)]
    id_: i32,
}
impl A3_int_ {
    pub fn new(id: Option<i32>) -> Self {
        let mut id: i32 = id.unwrap_or_else(|| 0);
        Self { id_: id }
    }
}
impl CppEq for A3_int_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_13(__lhs.clone(), __rhs.clone())
    }
}
impl Default for A3_int_ {
    fn default() -> Self {
        { A3_int_::new(None) }
    }
}
pub fn operator_eq_13(x: Ptr<A3_int_>, y: Ptr<A3_int_>) -> bool {
    return ({ ({ A3_int_Impl::id(&x) }) } == { ({ A3_int_Impl::id(&y) }) });
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                let _x: Ptr<A1_int_> = unimplemented!();
                let _y: Value<A1_int_> = Rc::new(RefCell::new(A1_int_::new(None)));
                operator_eq_4(_x, _y.as_pointer())
            })
        );
        assert!(unimplemented!());
        assert!(((copy_called_0.with(|rc| *rc.borrow()) as i32) == (false as i32)));
        assert!(((move_called_1.with(|rc| *rc.borrow()) as i32) == (false as i32)));
    }
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                let _x: Ptr<A1_int_> = unimplemented!();
                let _y: Value<A1_int_> = Rc::new(RefCell::new(A1_int_::new(None)));
                operator_eq_4(_x, _y.as_pointer())
            })
        );
        assert!(unimplemented!());
        assert!(((copy_called_0.with(|rc| *rc.borrow()) as i32) == (false as i32)));
        assert!(((move_called_1.with(|rc| *rc.borrow()) as i32) == (false as i32)));
        assert!(((copy_called_5.with(|rc| *rc.borrow()) as i32) == (false as i32)));
        assert!(((move_called_6.with(|rc| *rc.borrow()) as i32) == (false as i32)));
    }
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                let _x: Ptr<A1_int_> = unimplemented!();
                let _y: Value<A1_int_> = Rc::new(RefCell::new(A1_int_::new(None)));
                operator_eq_4(_x, _y.as_pointer())
            })
        );
        assert!((unimplemented!()));
        assert!(((copy_called_0.with(|rc| *rc.borrow()) as i32) == (false as i32)));
        assert!(((move_called_1.with(|rc| *rc.borrow()) as i32) == (false as i32)));
        assert!(((copy_called_5.with(|rc| *rc.borrow()) as i32) == (false as i32)));
        assert!(((move_called_6.with(|rc| *rc.borrow()) as i32) == (false as i32)));
        assert!(((copy_called_9.with(|rc| *rc.borrow()) as i32) == (false as i32)));
        assert!(((move_called_10.with(|rc| *rc.borrow()) as i32) == (false as i32)));
    }
    return 0;
}
pub trait A1_int_Impl {
    fn id(&self) -> i32;
    fn copy_assign(&self, a: Ptr<A1_int_>) -> Ptr<A1_int_> {
        unimplemented!()
    }
    fn move_assign(&self, a: Ptr<A1_int_>) -> Ptr<A1_int_> {
        unimplemented!()
    }
    fn allocate(&self, mut n: usize) -> Ptr<i32> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<i32>, mut n: usize) {
        unimplemented!()
    }
    fn max_size(&self) -> usize {
        unimplemented!()
    }
}
impl A1_int_Impl for Ptr<A1_int_> {
    fn id(&self) -> i32 {
        return (*self).with(|__s| __s.id_);
    }
}
pub trait A2_int_Impl {
    fn id(&self) -> i32;
    fn copy_assign(&self, a: Ptr<A2_int_>) -> Ptr<A2_int_> {
        unimplemented!()
    }
    fn move_assign(&self, a: Ptr<A2_int_>) -> Ptr<A2_int_> {
        unimplemented!()
    }
    fn allocate(&self, mut _a0: usize, mut hint: AnyPtr) -> Ptr<i32> {
        unimplemented!()
    }
}
impl A2_int_Impl for Ptr<A2_int_> {
    fn id(&self) -> i32 {
        return (*self).with(|__s| __s.id_);
    }
}
pub trait A3_int_Impl {
    fn id(&self) -> i32;
    fn copy_assign(&self, a: Ptr<A3_int_>) -> Ptr<A3_int_> {
        unimplemented!()
    }
    fn move_assign(&self, a: Ptr<A3_int_>) -> Ptr<A3_int_> {
        unimplemented!()
    }
    fn select_on_container_copy_construction(&self) -> A3_int_ {
        unimplemented!()
    }
}
impl A3_int_Impl for Ptr<A3_int_> {
    fn id(&self) -> i32 {
        return (*self).with(|__s| __s.id_);
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = copy_called_0.with(|_| ());
    let _ = move_called_1.with(|_| ());
    let _ = copy_called_5.with(|_| ());
    let _ = move_called_6.with(|_| ());
    let _ = copy_called_9.with(|_| ());
    let _ = move_called_10.with(|_| ());
}
