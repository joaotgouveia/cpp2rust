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
#[byte_size(8)]
pub struct min_pointer_bool__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<bool>,
}
impl min_pointer_bool__std_integral_constant_unsigned_long__0__ {}
impl CppMove for min_pointer_bool__std_integral_constant_unsigned_long__0__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: AnyPtr,
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
impl CppMove for min_pointer_void__std_integral_constant_unsigned_long__0__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<u64>,
}
impl min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
    fn new_1(mut p: Ptr<u64>) -> Self {
        Self { ptr_: (p).clone() }
    }
}
impl CppMove for min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_bool_ {}
impl CppMove for min_allocator_bool_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl min_allocator_unsigned_long_ {
    pub fn new_1(mut _a0: min_allocator_bool_) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(min_allocator_bool_, new_1, value)]
#[byte_size(1)]
pub struct min_allocator_unsigned_long_ {}
impl CppMove for min_allocator_unsigned_long_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl min_allocator_unsigned_long_ {}
pub fn tests_4() -> bool {
    let a1: Value<Box<[bool]>> = Rc::new(RefCell::new(Box::new([(1 != 0), (0 != 0), (1 != 0)])));
    {
        let l1: Value<Vec<bool>> = Rc::new(RefCell::new(
            match ({ (a1.as_pointer() as Ptr<bool>).offset((3) as isize) },) {
                (mut __tmp_0,) => {
                    let mut __v = Vec::new();
                    let mut __it = (a1.as_pointer() as Ptr<bool>);
                    while __it != __tmp_0 {
                        __v.push(__it.iter_deref().read() != false);
                        __it.iter_inc();
                    }
                    __v
                }
            },
        ));
        let i: Value<RandomAccessIterator<bool>> = Rc::new(RefCell::new(
            ({
                let ___first: RandomAccessIterator<bool> = l1.decay().begin();
                let ___last: RandomAccessIterator<bool> = l1.decay().begin();
                (*l1.borrow_mut()).erase_61(___first, ___last)
            }),
        ));
        assert!(({ ((&(*l1.borrow())).len() == 3_usize) }));
        assert!((unimplemented!() == 3_()));
        assert!((*i.borrow()) == l1.decay().begin());
    }
    {
        let l1: Value<Vec<bool>> = Rc::new(RefCell::new(
            match ({ (a1.as_pointer() as Ptr<bool>).offset((3) as isize) },) {
                (mut __tmp_1,) => {
                    let mut __v = Vec::new();
                    let mut __it = (a1.as_pointer() as Ptr<bool>);
                    while __it != __tmp_1 {
                        __v.push(__it.iter_deref().read() != false);
                        __it.iter_inc();
                    }
                    __v
                }
            },
        ));
        let i: Value<RandomAccessIterator<bool>> = Rc::new(RefCell::new(
            ({
                let ___first: RandomAccessIterator<bool> = l1.decay().begin();
                let ___last: RandomAccessIterator<bool> =
                    libcc2rs::next_refcount(l1.decay().begin(), 1_());
                (*l1.borrow_mut()).erase_61(___first, ___last)
            }),
        ));
        assert!(({ ((&(*l1.borrow())).len() == 2_usize) }));
        assert!((unimplemented!() == 2_()));
        assert!((*i.borrow()) == l1.decay().begin());
        assert!(unimplemented!());
    }
    {
        let l1: Value<Vec<bool>> = Rc::new(RefCell::new(
            match ({ (a1.as_pointer() as Ptr<bool>).offset((3) as isize) },) {
                (mut __tmp_2,) => {
                    let mut __v = Vec::new();
                    let mut __it = (a1.as_pointer() as Ptr<bool>);
                    while __it != __tmp_2 {
                        __v.push(__it.iter_deref().read() != false);
                        __it.iter_inc();
                    }
                    __v
                }
            },
        ));
        let i: Value<RandomAccessIterator<bool>> = Rc::new(RefCell::new(
            ({
                let ___first: RandomAccessIterator<bool> = l1.decay().begin();
                let ___last: RandomAccessIterator<bool> =
                    libcc2rs::next_refcount(l1.decay().begin(), 2_());
                (*l1.borrow_mut()).erase_61(___first, ___last)
            }),
        ));
        assert!(({ ((&(*l1.borrow())).len() == 1_usize) }));
        assert!((unimplemented!() == 1_()));
        assert!((*i.borrow()) == l1.decay().begin());
        assert!(unimplemented!());
    }
    {
        let l1: Value<Vec<bool>> = Rc::new(RefCell::new(
            match ({ (a1.as_pointer() as Ptr<bool>).offset((3) as isize) },) {
                (mut __tmp_3,) => {
                    let mut __v = Vec::new();
                    let mut __it = (a1.as_pointer() as Ptr<bool>);
                    while __it != __tmp_3 {
                        __v.push(__it.iter_deref().read() != false);
                        __it.iter_inc();
                    }
                    __v
                }
            },
        ));
        let i: Value<RandomAccessIterator<bool>> = Rc::new(RefCell::new(
            ({
                let ___first: RandomAccessIterator<bool> = l1.decay().begin();
                let ___last: RandomAccessIterator<bool> =
                    libcc2rs::next_refcount(l1.decay().begin(), 3_());
                (*l1.borrow_mut()).erase_61(___first, ___last)
            }),
        ));
        assert!(({ ((&(*l1.borrow())).len() == 0_usize) }));
        assert!((unimplemented!() == 0_()));
        assert!((*i.borrow()) == l1.decay().begin());
    }
    {
        let l1: Value<Vec<bool>> = Rc::new(RefCell::new(
            match ({ (a1.as_pointer() as Ptr<bool>).offset((3) as isize) },) {
                (mut __tmp_4,) => {
                    let mut __v = Vec::new();
                    let mut __it = (a1.as_pointer() as Ptr<bool>);
                    while __it != __tmp_4 {
                        __v.push(__it.iter_deref().read() != false);
                        __it.iter_inc();
                    }
                    __v
                }
            },
        ));
        let i: Value<RandomAccessIterator<bool>> = Rc::new(RefCell::new(
            ({
                let ___first: RandomAccessIterator<bool> = l1.decay().begin();
                let ___last: RandomAccessIterator<bool> = l1.decay().begin();
                (*l1.borrow_mut()).erase_61(___first, ___last)
            }),
        ));
        assert!(({ ((&(*l1.borrow())).len() == 3_usize) }));
        assert!((unimplemented!() == 3_()));
        assert!((*i.borrow()) == l1.decay().begin());
    }
    {
        let l1: Value<Vec<bool>> = Rc::new(RefCell::new(
            match ({ (a1.as_pointer() as Ptr<bool>).offset((3) as isize) },) {
                (mut __tmp_5,) => {
                    let mut __v = Vec::new();
                    let mut __it = (a1.as_pointer() as Ptr<bool>);
                    while __it != __tmp_5 {
                        __v.push(__it.iter_deref().read() != false);
                        __it.iter_inc();
                    }
                    __v
                }
            },
        ));
        let i: Value<RandomAccessIterator<bool>> = Rc::new(RefCell::new(
            ({
                let ___first: RandomAccessIterator<bool> = l1.decay().begin();
                let ___last: RandomAccessIterator<bool> =
                    libcc2rs::next_refcount(l1.decay().begin(), 1_());
                (*l1.borrow_mut()).erase_61(___first, ___last)
            }),
        ));
        assert!(({ ((&(*l1.borrow())).len() == 2_usize) }));
        assert!((unimplemented!() == 2_()));
        assert!((*i.borrow()) == l1.decay().begin());
        assert!((unimplemented!()));
    }
    {
        let l1: Value<Vec<bool>> = Rc::new(RefCell::new(
            match ({ (a1.as_pointer() as Ptr<bool>).offset((3) as isize) },) {
                (mut __tmp_6,) => {
                    let mut __v = Vec::new();
                    let mut __it = (a1.as_pointer() as Ptr<bool>);
                    while __it != __tmp_6 {
                        __v.push(__it.iter_deref().read() != false);
                        __it.iter_inc();
                    }
                    __v
                }
            },
        ));
        let i: Value<RandomAccessIterator<bool>> = Rc::new(RefCell::new(
            ({
                let ___first: RandomAccessIterator<bool> = l1.decay().begin();
                let ___last: RandomAccessIterator<bool> =
                    libcc2rs::next_refcount(l1.decay().begin(), 2_());
                (*l1.borrow_mut()).erase_61(___first, ___last)
            }),
        ));
        assert!(({ ((&(*l1.borrow())).len() == 1_usize) }));
        assert!((unimplemented!() == 1_()));
        assert!((*i.borrow()) == l1.decay().begin());
        assert!((unimplemented!()));
    }
    {
        let l1: Value<Vec<bool>> = Rc::new(RefCell::new(
            match ({ (a1.as_pointer() as Ptr<bool>).offset((3) as isize) },) {
                (mut __tmp_7,) => {
                    let mut __v = Vec::new();
                    let mut __it = (a1.as_pointer() as Ptr<bool>);
                    while __it != __tmp_7 {
                        __v.push(__it.iter_deref().read() != false);
                        __it.iter_inc();
                    }
                    __v
                }
            },
        ));
        let i: Value<RandomAccessIterator<bool>> = Rc::new(RefCell::new(
            ({
                let ___first: RandomAccessIterator<bool> = l1.decay().begin();
                let ___last: RandomAccessIterator<bool> =
                    libcc2rs::next_refcount(l1.decay().begin(), 3_());
                (*l1.borrow_mut()).erase_61(___first, ___last)
            }),
        ));
        assert!(({ ((&(*l1.borrow())).len() == 0_usize) }));
        assert!((unimplemented!() == 0_()));
        assert!((*i.borrow()) == l1.decay().begin());
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
    ({ tests_4() });
    const _: () = assert!(true, "tests()");;
    return 0;
}
pub trait min_allocator_bool_Impl {
    fn allocate(&self, mut n: usize) -> min_pointer_bool__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p: min_pointer_bool__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!()
    }
}
impl min_allocator_bool_Impl for Ptr<min_allocator_bool_> {}
pub trait min_allocator_unsigned_long_Impl {
    fn allocate(
        &self,
        n: usize,
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__;
    fn deallocate(
        &self,
        p: min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__,
        n: usize,
    );
}
impl min_allocator_unsigned_long_Impl for Ptr<min_allocator_unsigned_long_> {
    fn allocate(
        &self,
        mut n: usize,
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
        return min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    fn deallocate(
        &self,
        mut p: min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!();
    }
}
pub trait min_pointer_bool__std_integral_constant_unsigned_long__0__Impl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<bool> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<bool> {
        unimplemented!()
    }
    fn operator_inc_8(&self) -> Ptr<min_pointer_bool__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_9(
        &self,
        mut _a0: i32,
    ) -> min_pointer_bool__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(&self) -> Ptr<min_pointer_bool__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_11(
        &self,
        mut _a0: i32,
    ) -> min_pointer_bool__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_bool__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_bool__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: isize,
    ) -> min_pointer_bool__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: isize,
    ) -> min_pointer_bool__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: isize) -> Ptr<bool> {
        unimplemented!()
    }
}
impl min_pointer_bool__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_bool__std_integral_constant_unsigned_long__0__>
{
}
pub trait min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__Impl {
    fn to_bool(&self) -> bool;
    fn operator_deref(&self) -> Ptr<u64>;
    fn operator_arrow(&self) -> Ptr<u64>;
    fn operator_inc_8(
        &self,
    ) -> Ptr<min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_9(
        &self,
        mut _a0: i32,
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(
        &self,
    ) -> Ptr<min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_11(
        &self,
        mut _a0: i32,
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        n: isize,
    ) -> Ptr<min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__>;
    fn operator_sub_assign(
        &self,
        n: isize,
    ) -> Ptr<min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__>;
    fn operator_add(
        &self,
        n: isize,
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__;
    fn operator_sub(
        &self,
        n: isize,
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__;
    fn operator_index(&self, n: isize) -> Ptr<u64>;
}
impl min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__>
{
    fn to_bool(&self) -> bool {
        return !(((*self).with(|__s| __s.ptr_.clone())).is_null());
    }
    fn operator_deref(&self) -> Ptr<u64> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_arrow(&self) -> Ptr<u64> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_add_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_sub_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() - n)
        };
        return (*self).clone();
    }
    fn operator_add(
        &self,
        mut n: isize,
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
        let tmp: Value<min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__> =
            Rc::new(RefCell::new((*(*self).upgrade().deref()).clone()));
        ({
            min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__Impl :: operator_add_assign ( &tmp .as_pointer()  , n  , )
        });
        return (*tmp.borrow()).clone();
    }
    fn operator_sub(
        &self,
        mut n: isize,
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
        let tmp: Value<min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__> =
            Rc::new(RefCell::new((*(*self).upgrade().deref()).clone()));
        ({
            min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__Impl :: operator_sub_assign ( &tmp .as_pointer()  , n  , )
        });
        return (*tmp.borrow()).clone();
    }
    fn operator_index(&self, mut n: isize) -> Ptr<u64> {
        return (*self).with(|__s| __s.ptr_.clone()).offset((n) as isize);
    }
}
pub trait min_pointer_void__std_integral_constant_unsigned_long__0__Impl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_void__std_integral_constant_unsigned_long__0__>
{
}
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_0.with(|_| ());
    let _ = alloc_count_1.with(|_| ());
    let _ = dealloc_count_2.with(|_| ());
    let _ = disable_default_constructor_3.with(|_| ());
}
