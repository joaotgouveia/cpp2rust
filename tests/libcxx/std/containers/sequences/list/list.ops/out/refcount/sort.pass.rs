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
    pub fn outstanding_alloc() -> () {
        unimplemented!();
        return ((alloc_count_1.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_2.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        unimplemented!();
        disable_default_constructor_3.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_0.with(|rc| *rc.borrow_mut() = 0_());
        alloc_count_1.with(|rc| *rc.borrow_mut() = 0_());
        dealloc_count_2.with(|rc| *rc.borrow_mut() = 0_());
    }
}
thread_local!(
    pub static outstanding_bytes_0: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static alloc_count_1: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static dealloc_count_2: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static disable_default_constructor_3: Value<()> = Rc::new(RefCell::new(false));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_const_int__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_int_ {}
thread_local!(
    pub static randomness_4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Payload {
    #[offset(0)]
    pub val: (),
    #[offset(4)]
    pub side: (),
}
impl Payload {
    pub fn new_1(mut v: ()) -> Self {
        Self { val: v, side: 0_() }
    }
    pub fn new_2(mut v: (), mut s: ()) -> Self {
        Self { val: v, side: s }
    }
}
impl std::cmp::Ord for Payload {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if PayloadImpl::operator_lt(
                &Rc::new(RefCell::new(Payload {
                    val: self.val.clone(),
                    side: self.side.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(Payload {
                    val: other.val.clone(),
                    side: other.side.clone(),
                }))
                .as_pointer(),
            ) {
                std::cmp::Ordering::Less
            } else if PayloadImpl::operator_lt(
                &Rc::new(RefCell::new(Payload {
                    val: other.val.clone(),
                    side: other.side.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(Payload {
                    val: self.val.clone(),
                    side: self.side.clone(),
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
impl std::cmp::PartialOrd for Payload {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Payload {
    fn eq(&self, other: &Self) -> bool {
        {
            !(PayloadImpl::operator_lt(
                &Rc::new(RefCell::new(Payload {
                    val: self.val.clone(),
                    side: self.side.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(Payload {
                    val: other.val.clone(),
                    side: other.side.clone(),
                }))
                .as_pointer(),
            )) && !(PayloadImpl::operator_lt(
                &Rc::new(RefCell::new(Payload {
                    val: other.val.clone(),
                    side: other.side.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(Payload {
                    val: self.val.clone(),
                    side: self.side.clone(),
                }))
                .as_pointer(),
            ))
        }
    }
}
impl std::cmp::Eq for Payload {}
pub fn test_stable_5(mut N: ()) {
    let mut v: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < N) {
        unimplemented!();
        i.prefix_inc();
    }
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < N) {
        let __rhs = i;
        field!(unimplemented!(), side).write(__rhs);
        i.prefix_inc();
    }
    let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let j: Value<()> = Rc::new(RefCell::new({
        let __tmp_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        std___list_const_iterator_Payload__void_ptr_::new_1({ __tmp_0.as_pointer() })
    }));
    let mut i: () = 0_();
    'loop_: while (i < N) {
        unimplemented!();
        {
            i.prefix_inc();
            unimplemented!()
        };
    }
    let it: Value<()> = Rc::new(RefCell::new({
        let __tmp_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        std___list_const_iterator_Payload__void_ptr_::new_1({ __tmp_1.as_pointer() })
    }));
    'loop_: while unimplemented!() {
        let next: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        if (unimplemented!())
            && ({ ((*it.borrow()).read()).val } == { ((*next.borrow()).read()).val })
        {
            unimplemented!();
        }
        unimplemented!();
    }
}
pub fn test_6() -> () {
    {
        let a1: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            4_(),
            8_(),
            1_(),
            0_(),
            5_(),
            7_(),
            2_(),
            3_(),
            6_(),
            11_(),
            10_(),
            9_(),
        ])));
        let a2: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            0_(),
            1_(),
            2_(),
            3_(),
            4_(),
            5_(),
            6_(),
            7_(),
            8_(),
            9_(),
            10_(),
            11_(),
        ])));
        let c1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let a1: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            4_(),
            8_(),
            1_(),
            0_(),
            5_(),
            7_(),
            2_(),
            3_(),
            6_(),
            11_(),
            10_(),
            9_(),
        ])));
        let a2: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            0_(),
            1_(),
            2_(),
            3_(),
            4_(),
            5_(),
            6_(),
            7_(),
            8_(),
            9_(),
            10_(),
            11_(),
        ])));
        let c1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let mut i: () = 0_();
        'loop_: while (i < 40_()) {
            ({ test_stable_5(i) });
            i.prefix_inc();
        }
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    unimplemented!();
    return 0_();
}
pub trait PayloadImpl {
    fn operator_lt(&self, rhs: Ptr<Payload>) -> ();
}
impl PayloadImpl for Ptr<Payload> {
    fn operator_lt(&self, rhs: Ptr<Payload>) -> () {
        return ({ (*self).with(|__s| __s.val) } < { rhs.with(|__s| __s.val) });
    }
}
pub trait min_allocator_int_Impl {
    fn allocate(&self, mut n: ()) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p: min_pointer_int__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!()
    }
}
impl min_allocator_int_Impl for Ptr<min_allocator_int_> {}
pub trait min_pointer_const_int__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_inc_9(
        &self,
    ) -> Ptr<min_pointer_const_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_10(
        &self,
        mut _a0: (),
    ) -> min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_11(
        &self,
    ) -> Ptr<min_pointer_const_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_12(
        &self,
        mut _a0: (),
    ) -> min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
}
impl min_pointer_const_int__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_const_int__std_integral_constant_unsigned_long__0__>
{
}
pub trait min_pointer_int__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_inc_8(&self) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_9(
        &self,
        mut _a0: (),
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(&self) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_11(
        &self,
        mut _a0: (),
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(&self, mut n: ()) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(&self, mut n: ()) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__>
{
}
pub trait min_pointer_void__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
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
    let _ = randomness_4.with(|_| ());
}
