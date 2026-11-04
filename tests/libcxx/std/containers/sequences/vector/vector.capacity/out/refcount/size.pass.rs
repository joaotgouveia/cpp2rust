extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn DoNotOptimize_0(value: Ptr<AnyPtr>) -> Ptr<AnyPtr> {
    b"";
    return (value).clone();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> usize {
        assert!((alloc_count_2.with(|rc| *rc.borrow()) >= dealloc_count_3.with(|rc| *rc.borrow())));
        return ((alloc_count_2.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_3.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        assert!((({ malloc_allocator_base::outstanding_alloc() }) == 0_usize));
        disable_default_constructor_4.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_1.with(|rc| *rc.borrow_mut() = 0_usize);
        alloc_count_2.with(|rc| *rc.borrow_mut() = 0_usize);
        dealloc_count_3.with(|rc| *rc.borrow_mut() = 0_usize);
    }
}
impl CppMove for malloc_allocator_base {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static outstanding_bytes_1: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static alloc_count_2: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static dealloc_count_3: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static disable_default_constructor_4: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_int_ {}
impl CppMove for min_allocator_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct safe_allocator_int_ {}
impl safe_allocator_int_ {
    pub fn new() -> Self {
        Self {}
    }
}
impl CppMove for safe_allocator_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for safe_allocator_int_ {
    fn default() -> Self {
        { safe_allocator_int_::new() }
    }
}
pub fn tests_5() -> bool {
    {
        let c: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
        const _: () = assert!(true, "");;
        assert!(({ ((*c.borrow()).len() == 0_usize) }));
        {
            let __a1 = 2;
            (*c.borrow_mut()).push(__a1)
        };
        assert!(({ ((*c.borrow()).len() == 1_usize) }));
        {
            let __a1 = 1;
            (*c.borrow_mut()).push(__a1)
        };
        assert!(({ ((*c.borrow()).len() == 2_usize) }));
        {
            let __a1 = 3;
            (*c.borrow_mut()).push(__a1)
        };
        assert!(({ ((*c.borrow()).len() == 3_usize) }));
        match ({ <RandomAccessIterator<i32>>::from(CppRange::begin(&c.decay())) },) {
            (mut __tmp_2,) => {
                let __off = __tmp_2.index();
                c.as_pointer()
                    .with_mut(|__v: &mut Vec<i32>| __v.remove(__off));
                (__tmp_2).clone()
            }
        };
        assert!(({ ((*c.borrow()).len() == 2_usize) }));
        match ({ <RandomAccessIterator<i32>>::from(CppRange::begin(&c.decay())) },) {
            (mut __tmp_3,) => {
                let __off = __tmp_3.index();
                c.as_pointer()
                    .with_mut(|__v: &mut Vec<i32>| __v.remove(__off));
                (__tmp_3).clone()
            }
        };
        assert!(({ ((*c.borrow()).len() == 1_usize) }));
        match ({ <RandomAccessIterator<i32>>::from(CppRange::begin(&c.decay())) },) {
            (mut __tmp_4,) => {
                let __off = __tmp_4.index();
                c.as_pointer()
                    .with_mut(|__v: &mut Vec<i32>| __v.remove(__off));
                (__tmp_4).clone()
            }
        };
        assert!(({ ((*c.borrow()).len() == 0_usize) }));
    }
    {
        let c: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
        const _: () = assert!(true, "");;
        assert!(({ ((*c.borrow()).len() == 0_usize) }));
        {
            let __a1 = 2;
            (*c.borrow_mut()).push(__a1)
        };
        assert!(({ ((*c.borrow()).len() == 1_usize) }));
        {
            let __a1 = 1;
            (*c.borrow_mut()).push(__a1)
        };
        assert!(({ ((*c.borrow()).len() == 2_usize) }));
        {
            let __a1 = 3;
            (*c.borrow_mut()).push(__a1)
        };
        assert!(({ ((*c.borrow()).len() == 3_usize) }));
        match ({
            <__gnu_cxx___normal_iterator_min_pointer_const_int__std_integral_constant_unsigned_long__0____std_vector_int__min_allocator_int___  >::from(CppRange::begin(&c .decay() )  )
        },)
        {
            (mut __tmp_5,) => {
                let __off = __tmp_5.index();
                c.as_pointer()
                    .with_mut(|__v: &mut Vec<i32>| __v.remove(__off));
                (__tmp_5).clone()
            }
        };
        assert!(({ ((*c.borrow()).len() == 2_usize) }));
        match ({
            <__gnu_cxx___normal_iterator_min_pointer_const_int__std_integral_constant_unsigned_long__0____std_vector_int__min_allocator_int___  >::from(CppRange::begin(&c .decay() )  )
        },)
        {
            (mut __tmp_6,) => {
                let __off = __tmp_6.index();
                c.as_pointer()
                    .with_mut(|__v: &mut Vec<i32>| __v.remove(__off));
                (__tmp_6).clone()
            }
        };
        assert!(({ ((*c.borrow()).len() == 1_usize) }));
        match ({
            <__gnu_cxx___normal_iterator_min_pointer_const_int__std_integral_constant_unsigned_long__0____std_vector_int__min_allocator_int___  >::from(CppRange::begin(&c .decay() )  )
        },)
        {
            (mut __tmp_7,) => {
                let __off = __tmp_7.index();
                c.as_pointer()
                    .with_mut(|__v: &mut Vec<i32>| __v.remove(__off));
                (__tmp_7).clone()
            }
        };
        assert!(({ ((*c.borrow()).len() == 0_usize) }));
    }
    {
        let c: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
        const _: () = assert!(true, "");;
        assert!(({ ((*c.borrow()).len() == 0_usize) }));
        {
            let __a1 = 2;
            (*c.borrow_mut()).push(__a1)
        };
        assert!(({ ((*c.borrow()).len() == 1_usize) }));
        {
            let __a1 = 1;
            (*c.borrow_mut()).push(__a1)
        };
        assert!(({ ((*c.borrow()).len() == 2_usize) }));
        {
            let __a1 = 3;
            (*c.borrow_mut()).push(__a1)
        };
        assert!(({ ((*c.borrow()).len() == 3_usize) }));
        match ({ <RandomAccessIterator<i32>>::from(CppRange::begin(&c.decay())) },) {
            (mut __tmp_8,) => {
                let __off = __tmp_8.index();
                c.as_pointer()
                    .with_mut(|__v: &mut Vec<i32>| __v.remove(__off));
                (__tmp_8).clone()
            }
        };
        assert!(({ ((*c.borrow()).len() == 2_usize) }));
        match ({ <RandomAccessIterator<i32>>::from(CppRange::begin(&c.decay())) },) {
            (mut __tmp_9,) => {
                let __off = __tmp_9.index();
                c.as_pointer()
                    .with_mut(|__v: &mut Vec<i32>| __v.remove(__off));
                (__tmp_9).clone()
            }
        };
        assert!(({ ((*c.borrow()).len() == 1_usize) }));
        match ({ <RandomAccessIterator<i32>>::from(CppRange::begin(&c.decay())) },) {
            (mut __tmp_10,) => {
                let __off = __tmp_10.index();
                c.as_pointer()
                    .with_mut(|__v: &mut Vec<i32>| __v.remove(__off));
                (__tmp_10).clone()
            }
        };
        assert!(({ ((*c.borrow()).len() == 0_usize) }));
    }
    return true;
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
    ({ tests_5() });
    const _: () = assert!(true, "tests()");;
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__;
pub trait min_allocator_int_Impl {
    fn allocate(&self, n: usize) -> min_pointer_int__std_integral_constant_unsigned_long__0__;
    fn deallocate(&self, p: min_pointer_int__std_integral_constant_unsigned_long__0__, n: usize);
}
impl min_allocator_int_Impl for Ptr<min_allocator_int_> {
    fn allocate(&self, mut n: usize) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        return min_pointer_int__std_integral_constant_unsigned_long__0__::new_1({
            ({ <std_allocator_int_>::default().allocate(n) })
        });
    }
    fn deallocate(
        &self,
        mut p: min_pointer_int__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        ({ <std_allocator_int_>::default().deallocate((p.ptr_).clone(), n) });
    }
}
pub trait safe_allocator_int_Impl {
    fn allocate(&self, n: usize) -> Ptr<i32>;
    fn deallocate(&self, p: Ptr<i32>, n: usize);
}
impl safe_allocator_int_Impl for Ptr<safe_allocator_int_> {
    fn allocate(&self, mut n: usize) -> Ptr<i32> {
        let memory: Value<Ptr<i32>> = Rc::new(RefCell::new(
            ({ <std_allocator_int_>::default().allocate(n) }),
        ));
        {
            match ({ (*memory.borrow()).to_any() },) {
                (mut __tmp_0,) => {
                    __tmp_0.memset(
                        (0) as u8,
                        ((::std::mem::size_of::<i32>() as u64).wrapping_mul((n as u64)) as usize)
                            as usize,
                    );
                    (__tmp_0).clone()
                }
            };
        }
        return (*memory.borrow()).clone();
    }
    fn deallocate(&self, mut p: Ptr<i32>, mut n: usize) {
        {
            ({
                let _value: Value<AnyPtr> = Rc::new(RefCell::new(match ({ (p).to_any() },) {
                    (mut __tmp_1,) => {
                        __tmp_1.memset(
                            (0) as u8,
                            ((::std::mem::size_of::<i32>() as u64).wrapping_mul((n as u64))
                                as usize) as usize,
                        );
                        (__tmp_1).clone()
                    }
                }));
                DoNotOptimize_0(_value.as_pointer())
            });
        }
        ({ <std_allocator_int_>::default().deallocate((p).clone(), n) });
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_1.with(|_| ());
    let _ = alloc_count_2.with(|_| ());
    let _ = dealloc_count_3.with(|_| ());
    let _ = disable_default_constructor_4.with(|_| ());
}
