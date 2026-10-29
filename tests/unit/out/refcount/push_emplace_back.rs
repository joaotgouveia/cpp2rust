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
    pub output: Ptr<std_vector_Chunk__std_allocator_Chunk__>,
    #[offset(8)]
    #[byte_size(4)]
    pub chunk: Chunk,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(48)]
pub struct JPEGData { #[offset(0)] #[byte_size(24)] pub com_data : std_vector_std_vector_unsigned_char__std_allocator_unsigned_char____std_allocator_std_vector_unsigned_char__std_allocator_unsigned_char____  , #[offset(24)] #[byte_size(24)] pub app_data : std_vector_std_vector_unsigned_char__std_allocator_unsigned_char____std_allocator_std_vector_unsigned_char__std_allocator_unsigned_char____  , }
impl JPEGData {
    pub fn move_from(_a0: Ptr<JPEGData>) -> Self {
        Self {
            com_data: field!(_a0, com_data).with_mut(std::mem::take),
            app_data: field!(_a0, app_data).with_mut(std::mem::take),
        }
    }
}
pub fn push_param_0(
    mut dest : Ptr< std_vector_std_vector_unsigned_char__std_allocator_unsigned_char____std_allocator_std_vector_unsigned_char__std_allocator_unsigned_char____  >,
) {
    ({
        let ___x: Value<std_vector_unsigned_char__std_allocator_unsigned_char__> = Rc::new(
            RefCell::new(<std_vector_unsigned_char__std_allocator_unsigned_char__>::default()),
        );
        (*dest.upgrade().deref()).push_back_58(___x.as_pointer())
    });
}
pub fn push_local_from_field_1(mut jpg: Ptr<JPEGData>, mut cond: ()) {
    let head: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_()])));
    let mut dest : Ptr< std_vector_std_vector_unsigned_char__std_allocator_unsigned_char____std_allocator_std_vector_unsigned_char__std_allocator_unsigned_char____  >  = Ptr::<std_vector_std_vector_unsigned_char__std_allocator_unsigned_char____std_allocator_std_vector_unsigned_char__std_allocator_unsigned_char____>::null() ;
    if cond {
        dest = (field_ptr!(jpg, com_data));
    } else {
        dest = (field_ptr!(jpg, app_data));
    }
    ({
        let ___x: Value<std_vector_unsigned_char__std_allocator_unsigned_char__> =
            Rc::new(RefCell::new(
                std_vector_unsigned_char__std_allocator_unsigned_char__::new_1(
                    { (head.as_pointer() as Ptr<()>) },
                    { (head.as_pointer() as Ptr<()>).offset((3_()) as isize) },
                    None,
                ),
            ));
        (*dest.upgrade().deref()).push_back_58(___x.as_pointer())
    });
}
pub fn shrink_through_ptr_2(mut comps: Ptr<std_vector_Chunk__std_allocator_Chunk__>) {
    ({ (*comps.upgrade().deref()).shrink_to_fit() });
}
pub fn nested_push_move_3(mut bw: Ptr<Writer>) {
    ({
        let ___x: Ptr<Chunk> = field_ptr!(bw, chunk);
        (*bw.with(|__s| __s.output.clone()).upgrade().deref()).push_back_58(___x)
    });
}
pub fn emplace_local_from_field_4(mut jpg: Ptr<JPEGData>, mut cond: ()) {
    let head: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_()])));
    let mut dest : Ptr< std_vector_std_vector_unsigned_char__std_allocator_unsigned_char____std_allocator_std_vector_unsigned_char__std_allocator_unsigned_char____  >  = Ptr::<std_vector_std_vector_unsigned_char__std_allocator_unsigned_char____std_allocator_std_vector_unsigned_char__std_allocator_unsigned_char____>::null() ;
    if cond {
        dest = (field_ptr!(jpg, com_data));
    } else {
        dest = (field_ptr!(jpg, app_data));
    }
    ({
        let ___args_0: Ptr<()> = head.as_pointer();
        let ___args_1: Value<Ptr<()>> = Rc::new(RefCell::new(
            (head.as_pointer() as Ptr<()>).offset((3_()) as isize),
        ));
        (*dest.upgrade().deref()).emplace_back(___args_0, ___args_1.as_pointer())
    });
}
pub fn nested_emplace_move_5(mut bw: Ptr<Writer>) {
    ({
        let ___args: Ptr<Chunk> = field_ptr!(bw, chunk);
        (*bw.with(|__s| __s.output.clone()).upgrade().deref()).emplace_back(___args)
    });
}
pub fn self_ref_push_6(mut comps: Ptr<std_vector_Chunk__std_allocator_Chunk__>) {
    ({
        let ___x: Ptr<Chunk> = ({ (*comps.upgrade().deref()).front_51() });
        (*comps.upgrade().deref()).push_back_57(___x)
    });
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
pub fn emplace_ctor_args_7(mut pairs: Ptr<std_vector_Pair__std_allocator_Pair__>) {
    ({ (*pairs.upgrade().deref()).emplace_back() });
    ({
        let ___args: Value<()> = Rc::new(RefCell::new(3_()));
        (*pairs.upgrade().deref()).emplace_back(___args.as_pointer())
    });
    ({
        let ___args_0: Value<()> = Rc::new(RefCell::new(4_()));
        let ___args_1: Value<()> = Rc::new(RefCell::new(5_()));
        (*pairs.upgrade().deref()).emplace_back(___args_0.as_pointer(), ___args_1.as_pointer())
    });
}
pub fn emplace_deque_8(mut queue: Ptr<std_deque_Pair__std_allocator_Pair__>) {
    ({
        let ___args_0: Value<()> = Rc::new(RefCell::new(6_()));
        let ___args_1: Value<()> = Rc::new(RefCell::new(7_()));
        (*queue.upgrade().deref()).emplace_back(___args_0.as_pointer(), ___args_1.as_pointer())
    });
    ({ (*queue.upgrade().deref()).emplace_back() });
}
pub fn emplace_scalar_9(mut values: Ptr<std_vector_long__std_allocator_long__>, x: ()) {
    let x: Value<()> = Rc::new(RefCell::new(x));
    ({ (*values.upgrade().deref()).emplace_back() });
    ({ (*values.upgrade().deref()).emplace_back(x.as_pointer()) });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let vecs : Value<std_vector_std_vector_unsigned_char__std_allocator_unsigned_char____std_allocator_std_vector_unsigned_char__std_allocator_unsigned_char____ > = Rc::new(RefCell::new(<std_vector_std_vector_unsigned_char__std_allocator_unsigned_char____std_allocator_std_vector_unsigned_char__std_allocator_unsigned_char____  >::default() )) ;
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
    let chunks: Value<std_vector_Chunk__std_allocator_Chunk__> = Rc::new(RefCell::new(
        <std_vector_Chunk__std_allocator_Chunk__>::default(),
    ));
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
    let pairs: Value<std_vector_Pair__std_allocator_Pair__> = Rc::new(RefCell::new(
        <std_vector_Pair__std_allocator_Pair__>::default(),
    ));
    ({ emplace_ctor_args_7((pairs.as_pointer())) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let queue: Value<std_deque_Pair__std_allocator_Pair__> = Rc::new(RefCell::new(
        <std_deque_Pair__std_allocator_Pair__>::default(),
    ));
    ({ emplace_deque_8((queue.as_pointer())) });
    unimplemented!();
    unimplemented!();
    let values: Value<std_vector_long__std_allocator_long__> = Rc::new(RefCell::new(
        <std_vector_long__std_allocator_long__>::default(),
    ));
    ({ emplace_scalar_9((values.as_pointer()), 7_()) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub trait JPEGDataImpl {
    fn move_assign(&self, _a0: Ptr<JPEGData>) -> Ptr<JPEGData>;
}
impl JPEGDataImpl for Ptr<JPEGData> {
    fn move_assign(&self, _a0: Ptr<JPEGData>) -> Ptr<JPEGData> {
        field!((*self), com_data).write({ { (*_a0.upgrade().deref()).com_data.clone() } });
        field!((*self), app_data).write({ { (*_a0.upgrade().deref()).app_data.clone() } });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
