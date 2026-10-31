extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn operator_eq_1(x: Ptr<DefaultOnly>, y: Ptr<DefaultOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_lt_2(x: Ptr<DefaultOnly>, y: Ptr<DefaultOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } < { y.with(|__s| __s.data_) });
}
#[derive(Record, ByteRepr, FnPtrArg, Destructor)]
#[byte_size(4)]
pub struct DefaultOnly {
    #[offset(0)]
    data_: i32,
}
impl DefaultOnly {
    pub fn new() -> Self {
        let __this: DefaultOnly = Self { data_: -1_i32 };
        (*count_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl CppLess for DefaultOnly {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_2(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for DefaultOnly {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_1(__lhs.clone(), __rhs.clone())
    }
}
impl Default for DefaultOnly {
    fn default() -> Self {
        { DefaultOnly::new() }
    }
}
thread_local!(
    pub static count_0: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> usize {
        assert!((alloc_count_4.with(|rc| *rc.borrow()) >= dealloc_count_5.with(|rc| *rc.borrow())));
        return ((alloc_count_4.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_5.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        assert!((({ malloc_allocator_base::outstanding_alloc() }) == 0_usize));
        disable_default_constructor_6.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_3.with(|rc| *rc.borrow_mut() = 0_usize);
        alloc_count_4.with(|rc| *rc.borrow_mut() = 0_usize);
        dealloc_count_5.with(|rc| *rc.borrow_mut() = 0_usize);
    }
}
thread_local!(
    pub static outstanding_bytes_3: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static alloc_count_4: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static dealloc_count_5: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static disable_default_constructor_6: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<DefaultOnly>,
}
impl min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: AnyPtr,
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
pub fn operator_eq_7(
    mut _a0: min_allocator_DefaultOnly_,
    mut _a1: min_allocator_DefaultOnly_,
) -> bool {
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_DefaultOnly_ {}
impl CppEq for min_allocator_DefaultOnly_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_7(__lhs.read(), __rhs.read())
    }
}
pub fn check_allocator_8(mut n: u32, alloc: Option<Ptr<()>>) {
    let mut __alloc_default: Option<Value<()>> = None;
    let alloc: Ptr<()> = alloc.unwrap_or_else(|| {
        __alloc_default
            .insert(Rc::new(RefCell::new(unimplemented!())))
            .as_pointer()
    });
    let d: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    assert!(unimplemented!());
    assert!(((unimplemented!() as usize) == (n as usize)));
}
pub fn check_allocator_9(mut n: u32, alloc: Option<Ptr<min_allocator_DefaultOnly_>>) {
    let mut __alloc_default: Option<Value<min_allocator_DefaultOnly_>> = None;
    let alloc: Ptr<min_allocator_DefaultOnly_> = alloc.unwrap_or_else(|| {
        __alloc_default
            .insert(Rc::new(RefCell::new(
                <min_allocator_DefaultOnly_>::default(),
            )))
            .as_pointer()
    });
    let d: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    assert!(
        ({
            let _arg0: min_allocator_DefaultOnly_ = unimplemented!();
            let _arg1: min_allocator_DefaultOnly_ = (*alloc.upgrade().deref()).clone();
            operator_eq_7(_arg0, _arg1)
        })
    );
    assert!(((unimplemented!() as usize) == (n as usize)));
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
        const _: () = assert!(true, "(std::is_constructible<C, std::size_t>::value)");;
        const _: () = assert!(true, "(!std::is_convertible<std::size_t, C>::value)");;
    }
    {
        let mut N: u32 = 10_u32;
        let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut n: u32 = 0_u32;
        let i: Value<()> = Rc::new(RefCell::new(<()>::from(unimplemented!())));
        let e: Value<()> = Rc::new(RefCell::new(<()>::from(unimplemented!())));
        'loop_: while (*i.borrow()) != (*e.borrow()) {
            assert!(
                ({
                    let _x: Ptr<DefaultOnly> = (*i.borrow()).iter_deref();
                    let _y: Value<DefaultOnly> = Rc::new(RefCell::new(DefaultOnly::new()));
                    operator_eq_1(_x, _y.as_pointer())
                })
            );
            {
                (*i.borrow_mut()).iter_inc();
                n.prefix_inc()
            };
        }
        assert!((n == N));
    }
    {
        let mut N: u32 = 10_u32;
        let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut n: u32 = 0_u32;
        let i: Value<()> = Rc::new(RefCell::new(<()>::from(unimplemented!())));
        let e: Value<()> = Rc::new(RefCell::new(<()>::from(unimplemented!())));
        'loop_: while (*i.borrow()) != (*e.borrow()) {
            assert!(
                ({
                    let _x: Ptr<DefaultOnly> = (*i.borrow()).iter_deref();
                    let _y: Value<DefaultOnly> = Rc::new(RefCell::new(DefaultOnly::new()));
                    operator_eq_1(_x, _y.as_pointer())
                })
            );
            {
                (*i.borrow_mut()).iter_inc();
                n.prefix_inc()
            };
        }
        assert!((n == N));
        ({ check_allocator_8(0_u32, None) });
        ({ check_allocator_8(3_u32, None) });
    }
    {
        let mut N: u32 = 10_u32;
        let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut n: u32 = 0_u32;
        let i: Value<()> = Rc::new(RefCell::new(<()>::from(unimplemented!())));
        let e: Value<()> = Rc::new(RefCell::new(<()>::from(unimplemented!())));
        'loop_: while (*i.borrow()) != (*e.borrow()) {
            assert!(
                ({
                    let _x: Ptr<DefaultOnly> = (*i.borrow()).iter_deref();
                    let _y: Value<DefaultOnly> = Rc::new(RefCell::new(DefaultOnly::new()));
                    operator_eq_1(_x, _y.as_pointer())
                })
            );
            {
                (*i.borrow_mut()).iter_inc();
                n.prefix_inc()
            };
        }
        assert!((n == N));
        ({ check_allocator_9(0_u32, None) });
        ({ check_allocator_9(3_u32, None) });
    }
    return 0;
}
pub trait DefaultOnlyImpl {
    fn copy_assign(&self, _a0: Ptr<DefaultOnly>) -> Ptr<DefaultOnly> {
        unimplemented!()
    }
    fn destructor(&self);
}
impl DefaultOnlyImpl for Ptr<DefaultOnly> {
    fn destructor(&self) {
        field!((*self), data_).write(0);
        (*count_0.with(Value::clone).borrow_mut()).prefix_dec();
    }
}
pub trait min_allocator_DefaultOnly_Impl {
    fn allocate(
        &self,
        mut n: usize,
    ) -> min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p: min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!()
    }
}
impl min_allocator_DefaultOnly_Impl for Ptr<min_allocator_DefaultOnly_> {}
pub trait min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__Impl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<DefaultOnly> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<DefaultOnly> {
        unimplemented!()
    }
    fn operator_inc_8(
        &self,
    ) -> Ptr<min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_9(
        &self,
        mut _a0: i32,
    ) -> min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(
        &self,
    ) -> Ptr<min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_11(
        &self,
        mut _a0: i32,
    ) -> min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: isize,
    ) -> min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: isize,
    ) -> min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: isize) -> Ptr<DefaultOnly> {
        unimplemented!()
    }
}
impl min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__>
{
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
    let _ = count_0.with(|_| ());
    let _ = outstanding_bytes_3.with(|_| ());
    let _ = alloc_count_4.with(|_| ());
    let _ = dealloc_count_5.with(|_| ());
    let _ = disable_default_constructor_6.with(|_| ());
}
