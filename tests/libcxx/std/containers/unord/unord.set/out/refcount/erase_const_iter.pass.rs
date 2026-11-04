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
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> usize {
        assert!((alloc_count_1.with(|rc| *rc.borrow()) >= dealloc_count_2.with(|rc| *rc.borrow())));
        return ((alloc_count_1.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_2.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        assert!((({ malloc_allocator_base::outstanding_alloc() }) == 0_usize));
        disable_default_constructor_3.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_0.with(|rc| *rc.borrow_mut() = 0_usize);
        alloc_count_1.with(|rc| *rc.borrow_mut() = 0_usize);
        dealloc_count_2.with(|rc| *rc.borrow_mut() = 0_usize);
    }
}
impl CppMove for malloc_allocator_base {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static outstanding_bytes_0: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static alloc_count_1: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static dealloc_count_2: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static disable_default_constructor_3: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_int_ {}
impl CppMove for min_allocator_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl TemplateConstructor {
    pub fn new(_a0: Ptr<i32>) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i32, new, ptr)]
#[byte_size(1)]
pub struct TemplateConstructor {}
impl CppEq for TemplateConstructor {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_4(__lhs.clone(), __rhs.clone())
    }
}
impl CppMove for TemplateConstructor {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn operator_eq_4(_a0: Ptr<TemplateConstructor>, _a1: Ptr<TemplateConstructor>) -> bool {
    return false;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Hash {}
impl Callable1<Ptr<TemplateConstructor>, usize> for Hash {
    fn call(&self, a1: Ptr<TemplateConstructor>) -> usize {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| HashImpl::operator_call(__it, a1))
    }
}
impl CppMove for Hash {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    {
        let a: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 1, 2])));
        let c: Value<std_unordered_set_int__std_hash_int___std_equal_to_int___std_allocator_int__> =
            Rc::new(RefCell::new(
                std_unordered_set_int__std_hash_int___std_equal_to_int___std_allocator_int__::new_1(
                    { (a.as_pointer() as Ptr<i32>) },
                    {
                        (a.as_pointer() as Ptr<i32>).offset(
                            ((::std::mem::size_of::<[i32; 6]>() as usize)
                                .wrapping_div((::std::mem::size_of::<i32>() as usize)))
                                as isize,
                        )
                    },
                    None,
                    None,
                    None,
                    None,
                ),
            ));
        let i: Value<std___detail__Node_const_iterator_int__true__false_> = Rc::new(RefCell::new(
            <std___detail__Node_const_iterator_int__true__false_>::from(
                ({
                    let ___x: Value<i32> = Rc::new(RefCell::new(2));
                    (*c.borrow()).find_58(___x.as_pointer())
                }),
            ),
        ));
        let i_next: Value<std___detail__Node_const_iterator_int__true__false_> =
            Rc::new(RefCell::new((*i.borrow()).clone()));
        {
            i_next.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i_next.as_pointer()
        };
        let j: Value<std___detail__Node_iterator_int__true__false_> = Rc::new(RefCell::new(
            ({ (*c.borrow()).erase_46((*i.borrow()).clone()) }),
        ));
        assert!((*j.borrow()) == (*i_next.borrow()));
        assert!(({ (({ (*c.borrow()).size() }) == 3_u64) }));
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(1));
                (*c.borrow()).count(___x.as_pointer())
            }) == 1_u64)
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(3));
                (*c.borrow()).count(___x.as_pointer())
            }) == 1_u64)
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(4));
                (*c.borrow()).count(___x.as_pointer())
            }) == 1_u64)
        );
    }
    {
        let a: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 1, 2])));
        let c: Value<std_unordered_set_int__std_hash_int___std_equal_to_int___min_allocator_int__> =
            Rc::new(RefCell::new(
                std_unordered_set_int__std_hash_int___std_equal_to_int___min_allocator_int__::new_1(
                    { (a.as_pointer() as Ptr<i32>) },
                    {
                        (a.as_pointer() as Ptr<i32>).offset(
                            ((::std::mem::size_of::<[i32; 6]>() as usize)
                                .wrapping_div((::std::mem::size_of::<i32>() as usize)))
                                as isize,
                        )
                    },
                    None,
                    None,
                    None,
                    None,
                ),
            ));
        let i: Value<std___detail__Node_const_iterator_int__true__false_> = Rc::new(RefCell::new(
            <std___detail__Node_const_iterator_int__true__false_>::from(
                ({
                    let ___x: Value<i32> = Rc::new(RefCell::new(2));
                    (*c.borrow()).find_58(___x.as_pointer())
                }),
            ),
        ));
        let i_next: Value<std___detail__Node_const_iterator_int__true__false_> =
            Rc::new(RefCell::new((*i.borrow()).clone()));
        {
            i_next.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i_next.as_pointer()
        };
        let j: Value<std___detail__Node_iterator_int__true__false_> = Rc::new(RefCell::new(
            ({ (*c.borrow()).erase_46((*i.borrow()).clone()) }),
        ));
        assert!((*j.borrow()) == (*i_next.borrow()));
        assert!(({ (({ (*c.borrow()).size() }) == 3_u64) }));
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(1));
                (*c.borrow()).count(___x.as_pointer())
            }) == 1_u64)
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(3));
                (*c.borrow()).count(___x.as_pointer())
            }) == 1_u64)
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(4));
                (*c.borrow()).count(___x.as_pointer())
            }) == 1_u64)
        );
    }
    {
        let m : Value<std_unordered_set_TemplateConstructor__Hash__std_equal_to_TemplateConstructor___std_allocator_TemplateConstructor__ > = Rc::new(RefCell::new(<std_unordered_set_TemplateConstructor__Hash__std_equal_to_TemplateConstructor___std_allocator_TemplateConstructor__  >::default() )) ;
        let a: Value<TemplateConstructor> = Rc::new(RefCell::new({
            let __tmp_0: Value<i32> = Rc::new(RefCell::new(0));
            TemplateConstructor::new({ __tmp_0.as_pointer() })
        }));
        let it: Value<std___detail__Node_iterator_TemplateConstructor__true__true_> =
            Rc::new(RefCell::new(({ (*m.borrow()).find_58(a.as_pointer()) })));
        if (*it.borrow()) != ({ (*m.borrow()).end_29() }) {
            ({ (*m.borrow()).erase_47((*it.borrow()).clone()) });
        }
    }
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__;
pub trait HashImpl {
    fn operator_call(&self, _a0: Ptr<TemplateConstructor>) -> usize;
}
impl HashImpl for Ptr<Hash> {
    fn operator_call(&self, _a0: Ptr<TemplateConstructor>) -> usize {
        return 0_usize;
    }
}
pub trait min_allocator_int_Impl {
    fn allocate(&self, mut n: usize) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p: min_pointer_int__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!()
    }
}
impl min_allocator_int_Impl for Ptr<min_allocator_int_> {}
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_0.with(|_| ());
    let _ = alloc_count_1.with(|_| ());
    let _ = dealloc_count_2.with(|_| ());
    let _ = disable_default_constructor_3.with(|_| ());
}
