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
    pub data: i32,
}
impl CppMove for Chunk {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
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
impl CppMove for Writer {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
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
impl CppMove for JPEGData {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
pub fn push_param_0(
    mut dest : Ptr< std_vector_std_vector_unsigned_char__std_allocator_unsigned_char____std_allocator_std_vector_unsigned_char__std_allocator_unsigned_char____  >,
) {
    ({
        let ___x: Value<std_vector_unsigned_char__std_allocator_unsigned_char__> = Rc::new(
            RefCell::new(<std_vector_unsigned_char__std_allocator_unsigned_char__>::default()),
        );
        (*dest.upgrade().deref()).push_back_60(___x.as_pointer())
    });
}
pub fn push_local_from_field_1(mut jpg: Ptr<JPEGData>, mut cond: bool) {
    let head: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([1_u8, 2_u8, 3_u8])));
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
                    { (head.as_pointer() as Ptr<u8>) },
                    { (head.as_pointer() as Ptr<u8>).offset((3) as isize) },
                    None,
                ),
            ));
        (*dest.upgrade().deref()).push_back_60(___x.as_pointer())
    });
}
pub fn shrink_through_ptr_2(mut comps: Ptr<std_vector_Chunk__std_allocator_Chunk__>) {
    ({ (*comps.upgrade().deref()).shrink_to_fit() });
}
pub fn nested_push_move_3(mut bw: Ptr<Writer>) {
    ({
        let ___x: Ptr<Chunk> = field_ptr!(bw, chunk);
        (*bw.with(|__s| __s.output.clone()).upgrade().deref()).push_back_60(___x)
    });
}
pub fn emplace_local_from_field_4(mut jpg: Ptr<JPEGData>, mut cond: bool) {
    let head: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([1_u8, 2_u8, 3_u8])));
    let mut dest : Ptr< std_vector_std_vector_unsigned_char__std_allocator_unsigned_char____std_allocator_std_vector_unsigned_char__std_allocator_unsigned_char____  >  = Ptr::<std_vector_std_vector_unsigned_char__std_allocator_unsigned_char____std_allocator_std_vector_unsigned_char__std_allocator_unsigned_char____>::null() ;
    if cond {
        dest = (field_ptr!(jpg, com_data));
    } else {
        dest = (field_ptr!(jpg, app_data));
    }
    ({
        let ___args_0: Ptr<u8> = head.as_pointer();
        let ___args_1: Value<Ptr<u8>> = Rc::new(RefCell::new(
            (head.as_pointer() as Ptr<u8>).offset((3) as isize),
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
        let ___x: Ptr<Chunk> = ({ (*comps.upgrade().deref()).front_53() });
        (*comps.upgrade().deref()).push_back_59(___x)
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom)]
#[cpp_from(i32, new_1, value)]
#[byte_size(8)]
pub struct Pair {
    #[offset(0)]
    pub first: i32,
    #[offset(4)]
    pub second: i32,
}
impl Pair {
    pub fn new_2() -> Self {
        Self {
            first: -1_i32,
            second: -1_i32,
        }
    }
    pub fn new_1(mut a: i32) -> Self {
        Self {
            first: a,
            second: 0,
        }
    }
    pub fn new_3(mut a: i32, mut b: i32) -> Self {
        Self {
            first: a,
            second: (b * 2),
        }
    }
}
impl CppMove for Pair {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for Pair {
    fn default() -> Self {
        { Pair::new_2() }
    }
}
pub fn emplace_ctor_args_7(mut pairs: Ptr<std_vector_Pair__std_allocator_Pair__>) {
    ({ (*pairs.upgrade().deref()).emplace_back() });
    ({
        let ___args: Value<i32> = Rc::new(RefCell::new(3));
        (*pairs.upgrade().deref()).emplace_back(___args.as_pointer())
    });
    ({
        let ___args_0: Value<i32> = Rc::new(RefCell::new(4));
        let ___args_1: Value<i32> = Rc::new(RefCell::new(5));
        (*pairs.upgrade().deref()).emplace_back(___args_0.as_pointer(), ___args_1.as_pointer())
    });
}
pub fn emplace_deque_8(mut queue: Ptr<std_deque_Pair__std_allocator_Pair__>) {
    ({
        let ___args_0: Value<i32> = Rc::new(RefCell::new(6));
        let ___args_1: Value<i32> = Rc::new(RefCell::new(7));
        (*queue.upgrade().deref()).emplace_back(___args_0.as_pointer(), ___args_1.as_pointer())
    });
    ({ (*queue.upgrade().deref()).emplace_back() });
}
pub fn emplace_scalar_9(mut values: Ptr<std_vector_long__std_allocator_long__>, x: i32) {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    ({ (*values.upgrade().deref()).emplace_back() });
    ({ (*values.upgrade().deref()).emplace_back(x.as_pointer()) });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let vecs : Value<std_vector_std_vector_unsigned_char__std_allocator_unsigned_char____std_allocator_std_vector_unsigned_char__std_allocator_unsigned_char____ > = Rc::new(RefCell::new(<std_vector_std_vector_unsigned_char__std_allocator_unsigned_char____std_allocator_std_vector_unsigned_char__std_allocator_unsigned_char____  >::default() )) ;
    ({ push_param_0((vecs.as_pointer())) });
    ({ cpp2rust_assert_fail_10((({ (*vecs.borrow()).size() }) == 1_u64)) });
    ({ cpp2rust_assert_fail_10(({ (*vecs.borrow())[0_u64].empty() })) });
    let jpg: Value<JPEGData> = Rc::new(RefCell::new(<JPEGData>::default()));
    ({ push_local_from_field_1((jpg.as_pointer()), true) });
    ({ cpp2rust_assert_fail_10((({ (*jpg.borrow()).com_data.size() }) == 1_u64)) });
    ({
        cpp2rust_assert_fail_10(
            (({
                (*elem!(field_ptr!(jpg.as_pointer(), com_data), 0_u64)
                    .upgrade()
                    .deref())
                .size()
            }) == 3_u64),
        )
    });
    ({
        cpp2rust_assert_fail_10(
            (((elem!(field_ptr!(jpg.as_pointer(), com_data).offset(0_u64), 0_u64).read()) as i32)
                == 1),
        )
    });
    ({
        cpp2rust_assert_fail_10(
            (((elem!(field_ptr!(jpg.as_pointer(), com_data).offset(0_u64), 1_u64).read()) as i32)
                == 2),
        )
    });
    ({
        cpp2rust_assert_fail_10(
            (((elem!(field_ptr!(jpg.as_pointer(), com_data).offset(0_u64), 2_u64).read()) as i32)
                == 3),
        )
    });
    ({ cpp2rust_assert_fail_10(({ (*jpg.borrow()).app_data.empty() })) });
    let chunks: Value<std_vector_Chunk__std_allocator_Chunk__> = Rc::new(RefCell::new(
        <std_vector_Chunk__std_allocator_Chunk__>::default(),
    ));
    ({ shrink_through_ptr_2((chunks.as_pointer())) });
    ({ cpp2rust_assert_fail_10(({ (*chunks.borrow()).empty() })) });
    let w: Value<Writer> = Rc::new(RefCell::new(<Writer>::default()));
    (*w.borrow_mut()).chunk.data = 42;
    (*w.borrow_mut()).output = (chunks.as_pointer());
    ({ nested_push_move_3((w.as_pointer())) });
    ({ cpp2rust_assert_fail_10((({ (*chunks.borrow()).size() }) == 1_u64)) });
    ({ cpp2rust_assert_fail_10(({ (*chunks.borrow())[0_u64].data } == 42)) });
    ({ emplace_local_from_field_4((jpg.as_pointer()), false) });
    ({ cpp2rust_assert_fail_10((({ (*jpg.borrow()).app_data.size() }) == 1_u64)) });
    ({
        cpp2rust_assert_fail_10(
            (({
                (*elem!(field_ptr!(jpg.as_pointer(), app_data), 0_u64)
                    .upgrade()
                    .deref())
                .size()
            }) == 3_u64),
        )
    });
    ({
        cpp2rust_assert_fail_10(
            (((elem!(field_ptr!(jpg.as_pointer(), app_data).offset(0_u64), 0_u64).read()) as i32)
                == 1),
        )
    });
    ({
        cpp2rust_assert_fail_10(
            (((elem!(field_ptr!(jpg.as_pointer(), app_data).offset(0_u64), 2_u64).read()) as i32)
                == 3),
        )
    });
    ({ cpp2rust_assert_fail_10((({ (*jpg.borrow()).com_data.size() }) == 1_u64)) });
    (*w.borrow_mut()).chunk.data = 99;
    (*w.borrow_mut()).output = (chunks.as_pointer());
    ({ nested_emplace_move_5((w.as_pointer())) });
    ({ cpp2rust_assert_fail_10((({ (*chunks.borrow()).size() }) == 2_u64)) });
    ({ cpp2rust_assert_fail_10(({ (*chunks.borrow())[1_u64].data } == 99)) });
    ({ self_ref_push_6((chunks.as_pointer())) });
    ({ cpp2rust_assert_fail_10((({ (*chunks.borrow()).size() }) == 3_u64)) });
    ({ cpp2rust_assert_fail_10(({ (*chunks.borrow())[2_u64].data } == 42)) });
    let pairs: Value<std_vector_Pair__std_allocator_Pair__> = Rc::new(RefCell::new(
        <std_vector_Pair__std_allocator_Pair__>::default(),
    ));
    ({ emplace_ctor_args_7((pairs.as_pointer())) });
    ({ cpp2rust_assert_fail_10((({ (*pairs.borrow()).size() }) == 3_u64)) });
    ({
        cpp2rust_assert_fail_10(
            ({ (*pairs.borrow())[0_u64].first } == -1_i32)
                && ({ (*pairs.borrow())[0_u64].second } == -1_i32),
        )
    });
    ({
        cpp2rust_assert_fail_10(
            ({ (*pairs.borrow())[1_u64].first } == 3) && ({ (*pairs.borrow())[1_u64].second } == 0),
        )
    });
    ({
        cpp2rust_assert_fail_10(
            ({ (*pairs.borrow())[2_u64].first } == 4)
                && ({ (*pairs.borrow())[2_u64].second } == 10),
        )
    });
    let queue: Value<std_deque_Pair__std_allocator_Pair__> = Rc::new(RefCell::new(
        <std_deque_Pair__std_allocator_Pair__>::default(),
    ));
    ({ emplace_deque_8((queue.as_pointer())) });
    ({
        cpp2rust_assert_fail_10(
            (({ (*queue.borrow()).front_47() }).with(|__s| __s.first) == 6)
                && (({ (*queue.borrow()).front_47() }).with(|__s| __s.second) == 14),
        )
    });
    ({
        cpp2rust_assert_fail_10(
            (({ (*queue.borrow()).back_49() }).with(|__s| __s.first) == -1_i32)
                && (({ (*queue.borrow()).back_49() }).with(|__s| __s.second) == -1_i32),
        )
    });
    let values: Value<std_vector_long__std_allocator_long__> = Rc::new(RefCell::new(
        <std_vector_long__std_allocator_long__>::default(),
    ));
    ({ emplace_scalar_9((values.as_pointer()), 7) });
    ({ cpp2rust_assert_fail_10((({ (*values.borrow()).size() }) == 2_u64)) });
    ({ cpp2rust_assert_fail_10(({ (*values.borrow())[0_u64] } == 0_i64)) });
    ({ cpp2rust_assert_fail_10(({ (*values.borrow())[1_u64] } == 7_i64)) });
    return 0;
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
