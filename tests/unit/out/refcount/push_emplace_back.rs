extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Chunk {
    #[offset(0)]
    pub data: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Writer {
    #[offset(0)]
    #[byte_size(8)]
    pub output: Ptr<()>,
    #[offset(8)]
    #[byte_size(4)]
    pub chunk: Chunk,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(48)]
pub struct JPEGData {
    #[offset(0)]
    #[byte_size(24)]
    pub com_data: (),
    #[offset(24)]
    #[byte_size(24)]
    pub app_data: (),
}
pub fn push_param_0(mut dest: Ptr<()>) {
    unimplemented!();
}
pub fn push_local_from_field_1(mut jpg: Ptr<JPEGData>, mut cond: ()) {
    let head: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([1_u8, 2_u8, 3_u8])));
    let mut dest: Ptr<()> = Ptr::<()>::null();
    if cond {
        dest = (field_ptr!(jpg, com_data));
    } else {
        dest = (field_ptr!(jpg, app_data));
    }
    unimplemented!();
}
pub fn shrink_through_ptr_2(mut comps: Ptr<()>) {
    unimplemented!();
}
pub fn nested_push_move_3(mut bw: Ptr<Writer>) {
    unimplemented!();
}
pub fn emplace_local_from_field_4(mut jpg: Ptr<JPEGData>, mut cond: ()) {
    let head: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([1_u8, 2_u8, 3_u8])));
    let mut dest: Ptr<()> = Ptr::<()>::null();
    if cond {
        dest = (field_ptr!(jpg, com_data));
    } else {
        dest = (field_ptr!(jpg, app_data));
    }
    unimplemented!();
}
pub fn nested_emplace_move_5(mut bw: Ptr<Writer>) {
    unimplemented!();
}
pub fn self_ref_push_6(mut comps: Ptr<()>) {
    unimplemented!();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct Pair {
    #[offset(0)]
    pub first: (),
    #[offset(4)]
    pub second: (),
}
impl Pair {
    pub fn new_1() -> Self {
        Self {
            first: -1_(),
            second: -1_(),
        }
    }
    pub fn new_2(mut a: ()) -> Self {
        Self {
            first: a,
            second: 0_(),
        }
    }
    pub fn new_3(mut a: (), mut b: ()) -> Self {
        Self {
            first: a,
            second: (b * 2_()),
        }
    }
}
impl Default for Pair {
    fn default() -> Self {
        { Pair::new_1() }
    }
}
pub fn emplace_ctor_args_7(mut pairs: Ptr<()>) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn emplace_deque_8(mut queue: Ptr<()>) {
    unimplemented!();
    unimplemented!();
}
pub fn emplace_scalar_9(mut values: Ptr<()>, mut x: ()) {
    unimplemented!();
    unimplemented!();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let vecs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ push_param_0((vecs.as_pointer())) });
    unimplemented!();
    unimplemented!();
    let jpg: Value<JPEGData> = Rc::new(RefCell::new(<JPEGData>::default()));
    ({ push_local_from_field_1((jpg.as_pointer()), true) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let chunks: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ shrink_through_ptr_2((chunks.as_pointer())) });
    unimplemented!();
    let w: Value<Writer> = Rc::new(RefCell::new(<Writer>::default()));
    (*w.borrow_mut()).chunk.data = 42_();
    (*w.borrow_mut()).output = (chunks.as_pointer());
    ({ nested_push_move_3((w.as_pointer())) });
    unimplemented!();
    unimplemented!();
    ({ emplace_local_from_field_4((jpg.as_pointer()), false) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*w.borrow_mut()).chunk.data = 99_();
    (*w.borrow_mut()).output = (chunks.as_pointer());
    ({ nested_emplace_move_5((w.as_pointer())) });
    unimplemented!();
    unimplemented!();
    ({ self_ref_push_6((chunks.as_pointer())) });
    unimplemented!();
    unimplemented!();
    let pairs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ emplace_ctor_args_7((pairs.as_pointer())) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let queue: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ emplace_deque_8((queue.as_pointer())) });
    unimplemented!();
    unimplemented!();
    let values: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ emplace_scalar_9((values.as_pointer()), 7_()) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
