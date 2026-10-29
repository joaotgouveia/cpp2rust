extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestAllocator_int_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestAllocator_double_ {}
pub fn copy_0(mut copy_vector: std_vector_int__TestAllocator_int__) {}
pub fn fn_1(v: Ptr<std_vector_int__TestAllocator_int__>, v3: std_vector_int__TestAllocator_int__) {
    let v3: Value<std_vector_int__TestAllocator_int__> = Rc::new(RefCell::new(v3));
    ({
        let ___x: Value<()> = Rc::new(RefCell::new(20_()));
        (*v.upgrade().deref()).push_back_58(___x.as_pointer())
    });
    let mut x: () = unimplemented!();
    let mut v4: Ptr<std_vector_int__TestAllocator_int__> = (v3.as_pointer());
    let v2: Value<std_vector_int__TestAllocator_int__> = Rc::new(RefCell::new(
        <std_vector_int__TestAllocator_int__>::default(),
    ));
    ({
        let ___x: Value<()> = Rc::new(RefCell::new(0_()));
        (*v2.borrow()).push_back_58(___x.as_pointer())
    });
    ({
        let ___x: Value<()> = Rc::new(RefCell::new(1_()));
        (*v2.borrow()).push_back_58(___x.as_pointer())
    });
    ({
        let ___x: Value<()> = Rc::new(RefCell::new(3_()));
        (*v2.borrow()).push_back_58(___x.as_pointer())
    });
    x = (elem!(v, 2_()).read());
    (*v2.borrow_mut())[0_()] = 1_();
    elem!((if true { (v3.as_pointer()) } else { (v).clone() }), 0_()).write(7_());
    elem!((v4), 1_()).write(13_());
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    ({
        let ___x: Value<()> = Rc::new(RefCell::new(20_()));
        (*v.upgrade().deref()).push_back_58(___x.as_pointer())
    });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let v1: Value<std_vector_int__TestAllocator_int__> = Rc::new(RefCell::new(
        <std_vector_int__TestAllocator_int__>::default(),
    ));
    unimplemented!();
    unimplemented!();
    ({
        let ___x: Value<()> = Rc::new(RefCell::new(1_()));
        (*v1.borrow()).push_back_58(___x.as_pointer())
    });
    unimplemented!();
    ({ (*v1.borrow()).pop_back() });
    unimplemented!();
    let mut s1: () = ({ (*v1.borrow()).size() });
    ({ (*v1.borrow()).resize_40(100_()) });
    unimplemented!();
    unimplemented!();
    (*v1.borrow_mut())[0_()] = 40_();
    (*v1.borrow_mut())[99_()] = 50_();
    unimplemented!();
    unimplemented!();
    let v2: Value<std_vector_int__TestAllocator_int__> = Rc::new(RefCell::new(
        <std_vector_int__TestAllocator_int__>::default(),
    ));
    unimplemented!();
    ({
        let ___x: Value<()> = Rc::new(RefCell::new(1_()));
        (*v2.borrow()).push_back_58(___x.as_pointer())
    });
    ({
        let ___x: Value<()> = Rc::new(RefCell::new(2_()));
        (*v2.borrow()).push_back_58(___x.as_pointer())
    });
    ({
        let ___x: Value<()> = Rc::new(RefCell::new(3_()));
        (*v2.borrow()).push_back_58(___x.as_pointer())
    });
    unimplemented!();
    ({
        let ___position: __gnu_cxx___normal_iterator_const_int_ptr__std_vector_int__TestAllocator_int___   = { let __tmp_0 : Value<__gnu_cxx___normal_iterator_int_ptr__std_vector_int__TestAllocator_int___ > = Rc::new(RefCell::new(( { (*v2.borrow()) . begin_26 ( ) } )  )); __gnu_cxx___normal_iterator_const_int_ptr__std_vector_int__TestAllocator_int___ :: new_1 ( { __tmp_0.as_pointer() } , )  }  ;
        (*v2.borrow()).erase_67(___position)
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    ({
        let ___position: __gnu_cxx___normal_iterator_const_int_ptr__std_vector_int__TestAllocator_int___   = { let __tmp_1 : Value<__gnu_cxx___normal_iterator_int_ptr__std_vector_int__TestAllocator_int___ > = Rc::new(RefCell::new(( { (*v2.borrow()) . begin_26 ( ) } )  )); __gnu_cxx___normal_iterator_const_int_ptr__std_vector_int__TestAllocator_int___ :: new_1 ( { __tmp_1.as_pointer() } , )  }  ;
        let ___x: Value<()> = Rc::new(RefCell::new(100_()));
        (*v2.borrow()).insert_63(___position, ___x.as_pointer())
    });
    ({ copy_0((*v2.borrow()).clone()) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut s2: () = ({ (*v2.borrow()).size() });
    let v3: Value<std_vector_int__TestAllocator_int__> = Rc::new(RefCell::new({
        let __tmp_2: Value<()> = Rc::new(RefCell::new(1_()));
        std_vector_int__TestAllocator_int__::new_1({ 100_() }, { __tmp_2.as_pointer() }, None)
    }));
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < 100_()) {
        unimplemented!();
        i.prefix_inc();
    }
    let v6: Value<std_vector_double__TestAllocator_double__> = Rc::new(RefCell::new({
        let __tmp_3: Value<()> = Rc::new(RefCell::new(2_f64));
        std_vector_double__TestAllocator_double__::new_1({ s2 }, { __tmp_3.as_pointer() }, None)
    }));
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < s2) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut p1: Ptr<()> = ({ (*v6.borrow()).data_55() });
    unimplemented!();
    let mut p2: Ptr<()> = ({ (*v3.borrow()).data_55() });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    p2.write(99_f64);
    unimplemented!();
    unimplemented!();
    unimplemented!();
    p2.prefix_inc();
    p2.write(98_());
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    ({ (*v3.borrow()).reserve(200_()) });
    unimplemented!();
    unimplemented!();
    ({ (*v3.borrow()).reserve(50_()) });
    unimplemented!();
    unimplemented!();
    ({ (*v3.borrow()).reserve(200_()) });
    unimplemented!();
    unimplemented!();
    ({ (*v3.borrow()).reserve(201_()) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let ref0: Ptr<()> = ({ (*v6.borrow()).back_53() });
    ref0.write(5_f64);
    unimplemented!();
    let mut x0: () = (({ (*v6.borrow()).back_53() }).read());
    unimplemented!();
    x0 = 6_f64;
    unimplemented!();
    let mut idx: () = 0_();
    unimplemented!();
    unimplemented!();
    let ref1: Ptr<()> = ({ (*v6.borrow()).at_49((s2).wrapping_sub(1_())) });
    {
        ref1.with_mut(|__v| *__v = *__v + 1.5_f64)
    };
    unimplemented!();
    let mut x1: () = (({ (*v6.borrow()).at_49((s2).wrapping_sub(1_())) }).read());
    unimplemented!();
    x1 -= 1.5_f64;
    unimplemented!();
    let v7: Value<std_vector_int__TestAllocator_int__> = Rc::new(RefCell::new(
        <std_vector_int__TestAllocator_int__>::default(),
    ));
    let v8: Value<std_vector_int__TestAllocator_int__> = Rc::new(RefCell::new(
        <std_vector_int__TestAllocator_int__>::default(),
    ));
    ({
        let ___x: Value<()> = Rc::new(RefCell::new(4_()));
        (*v7.borrow()).push_back_58(___x.as_pointer())
    });
    ({
        let ___x: Value<()> = Rc::new(RefCell::new(5_()));
        (*v7.borrow()).push_back_58(___x.as_pointer())
    });
    ({
        let ___x: Value<()> = Rc::new(RefCell::new(6_()));
        (*v7.borrow()).push_back_58(___x.as_pointer())
    });
    ({
        let ___x: Value<()> = Rc::new(RefCell::new(8_()));
        (*v8.borrow()).push_back_58(___x.as_pointer())
    });
    ({
        let ___x: Value<()> = Rc::new(RefCell::new(9_()));
        (*v8.borrow()).push_back_58(___x.as_pointer())
    });
    ({ fn_1(v7.as_pointer(), (*v8.borrow()).clone()) });
    let src: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_()])));
    let v9: Value<std_vector_unsigned_int__std_allocator_unsigned_int__> = Rc::new(RefCell::new(
        std_vector_unsigned_int__std_allocator_unsigned_int__::new_1(
            { (src.as_pointer() as Ptr<()>) },
            { (src.as_pointer() as Ptr<()>).offset((3_()) as isize) },
            None,
        ),
    ));
    unimplemented!();
    unimplemented!();
    let v10: Value<std_vector_unsigned_long__std_allocator_unsigned_long__> =
        Rc::new(RefCell::new(
            std_vector_unsigned_long__std_allocator_unsigned_long__::new_1(
                { (src.as_pointer() as Ptr<()>) },
                { (src.as_pointer() as Ptr<()>).offset((3_()) as isize) },
                None,
            ),
        ));
    unimplemented!();
    unimplemented!();
    let v11: Value<std_vector_int__std_allocator_int__> =
        Rc::new(RefCell::new(std_vector_int__std_allocator_int__::new_1(
            { (src.as_pointer() as Ptr<()>) },
            { (src.as_pointer() as Ptr<()>).offset((3_()) as isize) },
            None,
        )));
    unimplemented!();
    unimplemented!();
    let src1: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_()])));
    let v12: Value<std_vector_unsigned_int__std_allocator_unsigned_int__> = Rc::new(RefCell::new(
        std_vector_unsigned_int__std_allocator_unsigned_int__::new_2(
            { (src1.as_pointer() as Ptr<()>) },
            { ({ end_2(src1.as_pointer()) }) },
            None,
        ),
    ));
    unimplemented!();
    unimplemented!();
    let buf: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([10_(), 20_(), 30_(), 40_(), 50_()])));
    let mut start: Ptr<()> = (buf.as_pointer() as Ptr<()>);
    let mut len: () = 5_();
    let v13: Value<std_vector_unsigned_char__std_allocator_unsigned_char__> =
        Rc::new(RefCell::new(
            std_vector_unsigned_char__std_allocator_unsigned_char__::new_1(
                { (start).clone() },
                { start.offset((len) as isize) },
                None,
            ),
        ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub trait TestAllocator_double_Impl {
    fn allocate(&self, n: ()) -> Ptr<()>;
    fn deallocate(&self, p: Ptr<()>, _a1: ());
}
impl TestAllocator_double_Impl for Ptr<TestAllocator_double_> {
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        return Ptr::alloc_array((0..n).map(|_| unimplemented!()).collect::<Box<[()]>>());
    }
    fn deallocate(&self, mut p: Ptr<()>, mut _a1: ()) {
        p.delete();
    }
}
pub trait TestAllocator_int_Impl {
    fn allocate(&self, n: ()) -> Ptr<()>;
    fn deallocate(&self, p: Ptr<()>, _a1: ());
}
impl TestAllocator_int_Impl for Ptr<TestAllocator_int_> {
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        return Ptr::alloc_array((0..n).map(|_| unimplemented!()).collect::<Box<[()]>>());
    }
    fn deallocate(&self, mut p: Ptr<()>, mut _a1: ()) {
        p.delete();
    }
}
pub fn __cpp2rust_init_globals() {}
