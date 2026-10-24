extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn DoNotOptimize_0(value: Ptr<()>) -> Ptr<()> {
    b"";
    return (value).clone();
}
pub fn is_double_ended_contiguous_container_asan_correct_1(_a0: Ptr<()>) -> () {
    return true;
}
pub fn is_double_ended_contiguous_container_asan_correct_2(_a0: Ptr<()>) -> () {
    return true;
}
pub fn is_double_ended_contiguous_container_asan_correct_3(_a0: Ptr<()>) -> () {
    return true;
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(16)]
pub struct Emplaceable {
    #[offset(0)]
    int_: (),
    #[offset(8)]
    double_: (),
}
impl Emplaceable {
    pub fn new_1() -> Self {
        Self {
            int_: 0_(),
            double_: 0_(),
        }
    }
    pub fn new_2(mut i: (), mut d: ()) -> Self {
        Self {
            int_: i,
            double_: d,
        }
    }
    pub fn move_from(x: Ptr<Emplaceable>) -> Self {
        let __this: Emplaceable = Self {
            int_: x.with(|__s| __s.int_),
            double_: x.with(|__s| __s.double_),
        };
        field!(x, int_).write(0_());
        field!(x, double_).write(0_());
        __this
    }
}
impl std::cmp::Ord for Emplaceable {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if EmplaceableImpl::operator_lt(
                &Rc::new(RefCell::new(Emplaceable {
                    int_: self.int_.clone(),
                    double_: self.double_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(Emplaceable {
                    int_: other.int_.clone(),
                    double_: other.double_.clone(),
                }))
                .as_pointer(),
            ) {
                std::cmp::Ordering::Less
            } else if EmplaceableImpl::operator_lt(
                &Rc::new(RefCell::new(Emplaceable {
                    int_: other.int_.clone(),
                    double_: other.double_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(Emplaceable {
                    int_: self.int_.clone(),
                    double_: self.double_.clone(),
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
impl std::cmp::PartialOrd for Emplaceable {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Emplaceable {
    fn eq(&self, other: &Self) -> bool {
        {
            EmplaceableImpl::operator_eq(
                &Rc::new(RefCell::new(Emplaceable {
                    int_: self.int_.clone(),
                    double_: self.double_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(Emplaceable {
                    int_: other.int_.clone(),
                    double_: other.double_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for Emplaceable {}
impl Default for Emplaceable {
    fn default() -> Self {
        { Emplaceable::new_1() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> () {
        unimplemented!();
        return ((alloc_count_5.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_6.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        unimplemented!();
        disable_default_constructor_7.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_4.with(|rc| *rc.borrow_mut() = 0_());
        alloc_count_5.with(|rc| *rc.borrow_mut() = 0_());
        dealloc_count_6.with(|rc| *rc.borrow_mut() = 0_());
    }
}
thread_local!(
    pub static outstanding_bytes_4: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static alloc_count_5: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static dealloc_count_6: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static disable_default_constructor_7: Value<()> = Rc::new(RefCell::new(false));
);
pub fn operator_sub_8(
    mut x: min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as () as ());
}
pub fn operator_eq_9(
    mut x: min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__,
) -> () {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
pub fn operator_ne_10(
    mut x: min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__,
) -> () {
    return !({
        let _x: min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_9(_x, y)
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<Emplaceable>,
}
impl min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__ {
    fn new_1(mut p: Ptr<Emplaceable>) -> Self {
        Self { ptr_: (p).clone() }
    }
    pub fn new_2(mut _a0: ()) -> Self {
        Self {
            ptr_: Ptr::<Emplaceable>::null(),
        }
    }
}
impl std::cmp::PartialEq for min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        { operator_eq_9(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
pub fn operator_sub_11(
    mut x: min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as () as ());
}
pub fn operator_eq_12(
    mut x: min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0__,
) -> () {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<Emplaceable>,
}
impl min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0__ {
    pub fn new_1(mut p: min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__) -> Self {
        Self {
            ptr_: (p.ptr_).clone(),
        }
    }
}
impl std::cmp::PartialEq
    for min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0__
{
    fn eq(&self, other: &Self) -> bool {
        { operator_eq_12(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_const_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: (),
}
impl min_pointer_const_void__std_integral_constant_unsigned_long__0__ {}
pub fn operator_sub_13(
    mut x : min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as () as ());
}
pub fn operator_eq_14(
    mut x : min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> () {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
pub fn operator_ne_15(
    mut x : min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> () {
    return !({
        let _x: min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__   = x   ;
        operator_eq_14(_x, y)
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__
{
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__>,
}
impl min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { fn new_1 ( mut p : Ptr< min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__  >  , ) -> Self { Self { ptr_ : (p ).clone() , }  } pub fn new_2 ( mut _a0 : ()  , ) -> Self { Self { ptr_ : Ptr::<min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__>::null()  , }  } }
impl std::cmp::PartialEq for min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { fn eq(&self, other: &Self) -> bool {  { operator_eq_14(self.clone(), other.clone()) }}}
impl std::cmp::Eq for min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ {}
pub fn operator_sub_16(
    mut x : min_pointer_const_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_const_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as () as ());
}
pub fn operator_eq_17(
    mut x : min_pointer_const_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_const_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> () {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
pub fn operator_ne_18(
    mut x : min_pointer_const_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_const_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> () {
    return ({ (x.ptr_).clone() } != { (y.ptr_).clone() });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_const_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__
{
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__>,
}
impl min_pointer_const_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { pub fn new_1 ( mut p : min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  , ) -> Self { Self { ptr_ : (p . ptr_  ).clone() , }  } }
impl std::cmp::PartialEq for min_pointer_const_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { fn eq(&self, other: &Self) -> bool {  { operator_eq_17(self.clone(), other.clone()) }}}
impl std::cmp::Eq for min_pointer_const_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__
{
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0__>,
}
impl min_pointer_min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_const_min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__
{
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0__>,
}
impl min_pointer_const_min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_Emplaceable_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0___ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0___ {
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct safe_allocator_Emplaceable_ {}
impl safe_allocator_Emplaceable_ {
    pub fn new() -> Self {
        Self {}
    }
}
impl Default for safe_allocator_Emplaceable_ {
    fn default() -> Self {
        { safe_allocator_Emplaceable_::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct safe_allocator_Emplaceable_ptr_ {}
impl safe_allocator_Emplaceable_ptr_ {
    pub fn new() -> Self {
        Self {}
    }
}
impl Default for safe_allocator_Emplaceable_ptr_ {
    fn default() -> Self {
        { safe_allocator_Emplaceable_ptr_::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct safe_allocator_const_Emplaceable_ptr_ {}
impl safe_allocator_const_Emplaceable_ptr_ {}
pub fn make_20(mut size: (), mut start: ()) -> () {
    let mut b: () = (4096_()).wrapping_div((::std::mem::size_of::<()>() as ()));
    let mut init: () = 0_();
    if (start > 0_()) {
        init = (((start + 1_()) / b) + (((start + 1_()) % b) != 0_()));
        init *= b;
        init.prefix_dec();
    }
    let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut i: () = 0_();
    'loop_: while (i < (init - start)) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while (i < size) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while (i < start) {
        unimplemented!();
        i.prefix_inc();
    }
    return unimplemented!();
}
pub fn make_21(mut size: (), mut start: ()) -> () {
    let mut b: () = (4096_()).wrapping_div((::std::mem::size_of::<()>() as ()));
    let mut init: () = 0_();
    if (start > 0_()) {
        init = (((start + 1_()) / b) + (((start + 1_()) % b) != 0_()));
        init *= b;
        init.prefix_dec();
    }
    let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut i: () = 0_();
    'loop_: while (i < (init - start)) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while (i < size) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while (i < start) {
        unimplemented!();
        i.prefix_inc();
    }
    return unimplemented!();
}
pub fn make_22(mut size: (), mut start: ()) -> () {
    let mut b: () = (4096_()).wrapping_div((::std::mem::size_of::<()>() as ()));
    let mut init: () = 0_();
    if (start > 0_()) {
        init = (((start + 1_()) / b) + (((start + 1_()) % b) != 0_()));
        init *= b;
        init.prefix_dec();
    }
    let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut i: () = 0_();
    'loop_: while (i < (init - start)) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while (i < size) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while (i < start) {
        unimplemented!();
        i.prefix_inc();
    }
    return unimplemented!();
}
pub fn test_23(mut P: (), c1: Ptr<()>) {
    let mut c1_osize: () = unimplemented!();
    let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_24(mut P: (), c1: Ptr<()>) {
    let mut c1_osize: () = unimplemented!();
    let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_25(mut P: (), c1: Ptr<()>) {
    let mut c1_osize: () = unimplemented!();
    let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn testN_26(mut start: (), mut N: ()) {
    let mut i: () = 0_();
    'loop_: while (i <= 3_()) {
        if (0_() <= i) && (i <= N) {
            let c1: Value<()> = Rc::new(RefCell::new(({ make_20(N, start) })));
            ({ test_23(i, c1.as_pointer()) });
        }
        i.prefix_inc();
    }
    let mut i: () = ((N / 2_()) - 1_());
    'loop_: while (i <= ((N / 2_()) + 1_())) {
        if (0_() <= i) && (i <= N) {
            let c1: Value<()> = Rc::new(RefCell::new(({ make_20(N, start) })));
            ({ test_23(i, c1.as_pointer()) });
        }
        i.prefix_inc();
    }
    let mut i: () = (N - 3_());
    'loop_: while (i <= N) {
        if (0_() <= i) && (i <= N) {
            let c1: Value<()> = Rc::new(RefCell::new(({ make_20(N, start) })));
            ({ test_23(i, c1.as_pointer()) });
        }
        i.prefix_inc();
    }
}
pub fn testN_27(mut start: (), mut N: ()) {
    let mut i: () = 0_();
    'loop_: while (i <= 3_()) {
        if (0_() <= i) && (i <= N) {
            let c1: Value<()> = Rc::new(RefCell::new(({ make_21(N, start) })));
            ({ test_24(i, c1.as_pointer()) });
        }
        i.prefix_inc();
    }
    let mut i: () = ((N / 2_()) - 1_());
    'loop_: while (i <= ((N / 2_()) + 1_())) {
        if (0_() <= i) && (i <= N) {
            let c1: Value<()> = Rc::new(RefCell::new(({ make_21(N, start) })));
            ({ test_24(i, c1.as_pointer()) });
        }
        i.prefix_inc();
    }
    let mut i: () = (N - 3_());
    'loop_: while (i <= N) {
        if (0_() <= i) && (i <= N) {
            let c1: Value<()> = Rc::new(RefCell::new(({ make_21(N, start) })));
            ({ test_24(i, c1.as_pointer()) });
        }
        i.prefix_inc();
    }
}
pub fn testN_28(mut start: (), mut N: ()) {
    let mut i: () = 0_();
    'loop_: while (i <= 3_()) {
        if (0_() <= i) && (i <= N) {
            let c1: Value<()> = Rc::new(RefCell::new(({ make_22(N, start) })));
            ({ test_25(i, c1.as_pointer()) });
        }
        i.prefix_inc();
    }
    let mut i: () = ((N / 2_()) - 1_());
    'loop_: while (i <= ((N / 2_()) + 1_())) {
        if (0_() <= i) && (i <= N) {
            let c1: Value<()> = Rc::new(RefCell::new(({ make_22(N, start) })));
            ({ test_25(i, c1.as_pointer()) });
        }
        i.prefix_inc();
    }
    let mut i: () = (N - 3_());
    'loop_: while (i <= N) {
        if (0_() <= i) && (i <= N) {
            let c1: Value<()> = Rc::new(RefCell::new(({ make_22(N, start) })));
            ({ test_25(i, c1.as_pointer()) });
        }
        i.prefix_inc();
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    {
        let mut rng: [(); 10] = [
            0_(),
            1_(),
            2_(),
            3_(),
            1023_(),
            1024_(),
            1025_(),
            2047_(),
            2048_(),
            2049_(),
        ];
        let mut N: () = (::std::mem::size_of::<[(); 10]>() as ())
            .wrapping_div((::std::mem::size_of::<()>() as ()));
        let mut i: () = 0_();
        'loop_: while (i < N) {
            let mut j: () = 0_();
            'loop_: while (j < N) {
                ({
                    let _start: () = rng[(i) as usize];
                    let _N: () = rng[(j) as usize];
                    testN_26(_start, _N)
                });
                j.prefix_inc();
            }
            i.prefix_inc();
        }
    }
    {
        let mut rng: [(); 10] = [
            0_(),
            1_(),
            2_(),
            3_(),
            1023_(),
            1024_(),
            1025_(),
            2047_(),
            2048_(),
            2049_(),
        ];
        let mut N: () = (::std::mem::size_of::<[(); 10]>() as ())
            .wrapping_div((::std::mem::size_of::<()>() as ()));
        let mut i: () = 0_();
        'loop_: while (i < N) {
            let mut j: () = 0_();
            'loop_: while (j < N) {
                ({
                    let _start: () = rng[(i) as usize];
                    let _N: () = rng[(j) as usize];
                    testN_27(_start, _N)
                });
                j.prefix_inc();
            }
            i.prefix_inc();
        }
    }
    {
        let mut rng: [(); 10] = [
            0_(),
            1_(),
            2_(),
            3_(),
            1023_(),
            1024_(),
            1025_(),
            2047_(),
            2048_(),
            2049_(),
        ];
        let mut N: () = (::std::mem::size_of::<[(); 10]>() as ())
            .wrapping_div((::std::mem::size_of::<()>() as ()));
        let mut i: () = 0_();
        'loop_: while (i < N) {
            let mut j: () = 0_();
            'loop_: while (j < N) {
                ({
                    let _start: () = rng[(i) as usize];
                    let _N: () = rng[(j) as usize];
                    testN_28(_start, _N)
                });
                j.prefix_inc();
            }
            i.prefix_inc();
        }
    }
    return 0_();
}
pub trait EmplaceableImpl {
    fn copy_assign(&self, _a0: Ptr<Emplaceable>) -> Ptr<Emplaceable> {
        unimplemented!()
    }
    fn move_assign(&self, x: Ptr<Emplaceable>) -> Ptr<Emplaceable>;
    fn operator_eq(&self, x: Ptr<Emplaceable>) -> ();
    fn operator_lt(&self, x: Ptr<Emplaceable>) -> ();
    fn get(&self) -> ();
}
impl EmplaceableImpl for Ptr<Emplaceable> {
    fn move_assign(&self, x: Ptr<Emplaceable>) -> Ptr<Emplaceable> {
        field!((*self), int_).write({ x.with(|__s| __s.int_) });
        field!(x, int_).write(0_());
        field!((*self), double_).write({ x.with(|__s| __s.double_) });
        field!(x, double_).write(0_());
        return (*self).clone();
    }
    fn operator_eq(&self, x: Ptr<Emplaceable>) -> () {
        return ({ (*self).with(|__s| __s.int_) } == { x.with(|__s| __s.int_) })
            && ({ (*self).with(|__s| __s.double_) } == { x.with(|__s| __s.double_) });
    }
    fn operator_lt(&self, x: Ptr<Emplaceable>) -> () {
        return ({ (*self).with(|__s| __s.int_) } < { x.with(|__s| __s.int_) })
            || (({ (*self).with(|__s| __s.int_) } == { x.with(|__s| __s.int_) })
                && ({ (*self).with(|__s| __s.double_) } < { x.with(|__s| __s.double_) }));
    }
    fn get(&self) -> () {
        return (*self).with(|__s| __s.int_);
    }
}
pub trait min_allocator_Emplaceable_Impl {
    fn allocate(&self, n: ()) -> min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__;
    fn deallocate(
        &self,
        p: min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__,
        n: (),
    );
}
impl min_allocator_Emplaceable_Impl for Ptr<min_allocator_Emplaceable_> {
    fn allocate(
        &self,
        mut n: (),
    ) -> min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__ {
        return min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    fn deallocate(
        &self,
        mut p: min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!();
    }
}
pub trait min_allocator_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0___Impl {
    fn allocate ( &self , n : ()  , ) -> min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  ;
    fn deallocate(
        &self,
        p : min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
        n: (),
    );
}
impl min_allocator_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0___Impl
    for Ptr<min_allocator_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0___>
{
    fn allocate ( &self , mut n : ()  , ) -> min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        return min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()   } , )    ;
    }
    fn deallocate(
        &self,
        mut p : min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!();
    }
}
pub trait min_allocator_min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0___Impl
{
    fn allocate ( &self , mut n : ()  , ) -> min_pointer_min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p : min_pointer_min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!()
    }
}
impl min_allocator_min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0___Impl
    for Ptr<min_allocator_min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0___>
{
}
pub trait min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> ();
    fn operator_deref(&self) -> Ptr<Emplaceable>;
    fn operator_arrow(&self) -> Ptr<Emplaceable>;
    fn operator_inc_8(
        &self,
    ) -> Ptr<min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__>;
    fn operator_post_inc_9(
        &self,
        mut _a0: (),
    ) -> min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(
        &self,
    ) -> Ptr<min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__>;
    fn operator_post_dec_11(
        &self,
        mut _a0: (),
    ) -> min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        n: (),
    ) -> Ptr<min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__>;
    fn operator_sub_assign(
        &self,
        n: (),
    ) -> Ptr<min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__>;
    fn operator_add(
        &self,
        n: (),
    ) -> min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__;
    fn operator_sub(
        &self,
        n: (),
    ) -> min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__;
    fn operator_index(&self, mut n: ()) -> Ptr<Emplaceable> {
        unimplemented!()
    }
}
impl min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__>
{
    fn to_(&self) -> () {
        return !(((*self).with(|__s| __s.ptr_.clone())).is_null());
    }
    fn operator_deref(&self) -> Ptr<Emplaceable> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_arrow(&self) -> Ptr<Emplaceable> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_inc_8(
        &self,
    ) -> Ptr<min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__> {
        field!((*self), ptr_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_dec_10(
        &self,
    ) -> Ptr<min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__> {
        field!((*self), ptr_).with_mut(|__v| __v.prefix_dec());
        return (*self).clone();
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() - n)
        };
        return (*self).clone();
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__ {
        let tmp: Value<min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__> =
            Rc::new(RefCell::new((*(*self).upgrade().deref()).clone()));
        ({
            min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__Impl :: operator_add_assign ( &tmp .as_pointer()  , n  , )
        });
        return (*tmp.borrow()).clone();
    }
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__ {
        let tmp: Value<min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__> =
            Rc::new(RefCell::new((*(*self).upgrade().deref()).clone()));
        ({
            min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__Impl :: operator_sub_assign ( &tmp .as_pointer()  , n  , )
        });
        return (*tmp.borrow()).clone();
    }
}
pub trait min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<Emplaceable>;
    fn operator_arrow(&self) -> Ptr<Emplaceable> {
        unimplemented!()
    }
    fn operator_inc_9(
        &self,
    ) -> Ptr<min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_10(
        &self,
        mut _a0: (),
    ) -> min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_11(
        &self,
    ) -> Ptr<min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_12(
        &self,
        mut _a0: (),
    ) -> min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<Emplaceable> {
        unimplemented!()
    }
}
impl min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0__>
{
    fn operator_deref(&self) -> Ptr<Emplaceable> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
}
pub trait min_pointer_const_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__Impl
{
    fn to_(&self) -> ();
    fn operator_deref(
        &self,
    ) -> Ptr<min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__>;
    fn operator_arrow(
        &self,
    ) -> Ptr<min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_inc_9 ( &self , ) -> Ptr< min_pointer_const_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > ;
    fn operator_post_inc_10 ( &self , mut _a0 : ()  , ) -> min_pointer_const_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }
    fn operator_dec_11 ( &self , ) -> Ptr< min_pointer_const_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > ;
    fn operator_post_dec_12 ( &self , mut _a0 : ()  , ) -> min_pointer_const_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }
    fn operator_add_assign ( &self , n : ()  , ) -> Ptr< min_pointer_const_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > ;
    fn operator_sub_assign ( &self , n : ()  , ) -> Ptr< min_pointer_const_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > ;
    fn operator_add ( &self , mut n : ()  , ) -> min_pointer_const_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_sub ( &self , mut n : ()  , ) -> min_pointer_const_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }
    fn operator_index(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
}
impl min_pointer_const_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__Impl for Ptr<min_pointer_const_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__> {
fn to_ ( &self , ) -> ()  { return ! ( ( (* self ) .with(|__s| __s . ptr_ .clone()) ).is_null() )   ;
 } fn operator_deref ( &self , ) -> Ptr< min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__  > { return (* self ) .with(|__s| __s . ptr_ .clone())    ;
 } fn operator_inc_9 ( &self , ) -> Ptr< min_pointer_const_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > { field!((* self ) , ptr_) .with_mut(|__v| __v. prefix_inc ()) ;
 return ((* self )  ).clone()  ;
 } fn operator_dec_11 ( &self , ) -> Ptr< min_pointer_const_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > { field!((* self ) , ptr_) .with_mut(|__v| __v. prefix_dec ()) ;
 return ((* self )  ).clone()  ;
 } fn operator_add_assign ( &self , mut n : ()  , ) -> Ptr< min_pointer_const_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > { { let _ptr = field!((* self ) , ptr_); _ptr.write(_ptr.read() + n ) } ;
 return ((* self )  ).clone()  ;
 } fn operator_sub_assign ( &self , mut n : ()  , ) -> Ptr< min_pointer_const_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > { { let _ptr = field!((* self ) , ptr_); _ptr.write(_ptr.read() - n ) } ;
 return ((* self )  ).clone()  ;
 } }
pub trait min_pointer_const_min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__Impl
{
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(
        &self,
    ) -> Ptr<min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_arrow(
        &self,
    ) -> Ptr<min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }    fn operator_inc_9 ( &self , ) -> Ptr< min_pointer_const_min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_post_inc_10 ( &self , mut _a0 : ()  , ) -> min_pointer_const_min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_dec_11 ( &self , ) -> Ptr< min_pointer_const_min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_post_dec_12 ( &self , mut _a0 : ()  , ) -> min_pointer_const_min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_add_assign ( &self , mut n : ()  , ) -> Ptr< min_pointer_const_min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_sub_assign ( &self , mut n : ()  , ) -> Ptr< min_pointer_const_min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_add ( &self , mut n : ()  , ) -> min_pointer_const_min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_sub ( &self , mut n : ()  , ) -> min_pointer_const_min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }
    fn operator_index(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
}
impl min_pointer_const_min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__Impl for Ptr<min_pointer_const_min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__> {
}
pub trait min_pointer_const_void__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl min_pointer_const_void__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_const_void__std_integral_constant_unsigned_long__0__>
{
}
pub trait min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__Impl
{
    fn to_(&self) -> ();
    fn operator_deref(
        &self,
    ) -> Ptr<min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__>;
    fn operator_arrow(
        &self,
    ) -> Ptr<min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__>;
    fn operator_inc_8 ( &self , ) -> Ptr< min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > ;
    fn operator_post_inc_9 ( &self , mut _a0 : ()  , ) -> min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }
    fn operator_dec_10 ( &self , ) -> Ptr< min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > ;
    fn operator_post_dec_11 ( &self , mut _a0 : ()  , ) -> min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }
    fn operator_add_assign ( &self , n : ()  , ) -> Ptr< min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > ;
    fn operator_sub_assign ( &self , n : ()  , ) -> Ptr< min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > ;
    fn operator_add ( &self , n : ()  , ) -> min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  ;
    fn operator_sub ( &self , n : ()  , ) -> min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  ;
    fn operator_index(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
}
impl min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__Impl for Ptr<min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__> {
fn to_ ( &self , ) -> ()  { return ! ( ( (* self ) .with(|__s| __s . ptr_ .clone()) ).is_null() )   ;
 } fn operator_deref ( &self , ) -> Ptr< min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__  > { return (* self ) .with(|__s| __s . ptr_ .clone())    ;
 } fn operator_arrow ( &self , ) -> Ptr< min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__  >  { return (* self ) .with(|__s| __s . ptr_ .clone())   ;
 } fn operator_inc_8 ( &self , ) -> Ptr< min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > { field!((* self ) , ptr_) .with_mut(|__v| __v. prefix_inc ()) ;
 return ((* self )  ).clone()  ;
 } fn operator_dec_10 ( &self , ) -> Ptr< min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > { field!((* self ) , ptr_) .with_mut(|__v| __v. prefix_dec ()) ;
 return ((* self )  ).clone()  ;
 } fn operator_add_assign ( &self , mut n : ()  , ) -> Ptr< min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > { { let _ptr = field!((* self ) , ptr_); _ptr.write(_ptr.read() + n ) } ;
 return ((* self )  ).clone()  ;
 } fn operator_sub_assign ( &self , mut n : ()  , ) -> Ptr< min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > { { let _ptr = field!((* self ) , ptr_); _ptr.write(_ptr.read() - n ) } ;
 return ((* self )  ).clone()  ;
 } fn operator_add ( &self , mut n : ()  , ) -> min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  { let tmp : Value<min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ > = Rc::new(RefCell::new((( * (* self ) .upgrade().deref() )  ).clone() )) ;
  ;
 ;
 ( { min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__Impl :: operator_add_assign ( &tmp .as_pointer()  , n  , ) } )  ;
 return ((*tmp.borrow()) ).clone()   ;
 } fn operator_sub ( &self , mut n : ()  , ) -> min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  { let tmp : Value<min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ > = Rc::new(RefCell::new((( * (* self ) .upgrade().deref() )  ).clone() )) ;
  ;
 ;
 ( { min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__Impl :: operator_sub_assign ( &tmp .as_pointer()  , n  , ) } )  ;
 return ((*tmp.borrow()) ).clone()   ;
 } }
pub trait min_pointer_min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__Impl
{
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(
        &self,
    ) -> Ptr<min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_arrow(
        &self,
    ) -> Ptr<min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }    fn operator_inc_8 ( &self , ) -> Ptr< min_pointer_min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_post_inc_9 ( &self , mut _a0 : ()  , ) -> min_pointer_min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_dec_10 ( &self , ) -> Ptr< min_pointer_min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_post_dec_11 ( &self , mut _a0 : ()  , ) -> min_pointer_min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_add_assign ( &self , mut n : ()  , ) -> Ptr< min_pointer_min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_sub_assign ( &self , mut n : ()  , ) -> Ptr< min_pointer_min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_add ( &self , mut n : ()  , ) -> min_pointer_min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_sub ( &self , mut n : ()  , ) -> min_pointer_min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }
    fn operator_index(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
}
impl min_pointer_min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__Impl for Ptr<min_pointer_min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__> {
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
pub trait safe_allocator_Emplaceable_Impl {
    fn allocate(&self, n: ()) -> Ptr<Emplaceable>;
    fn deallocate(&self, p: Ptr<Emplaceable>, n: ());
}
impl safe_allocator_Emplaceable_Impl for Ptr<safe_allocator_Emplaceable_> {
    fn allocate(&self, mut n: ()) -> Ptr<Emplaceable> {
        let memory: Value<Ptr<Emplaceable>> = Rc::new(RefCell::new(unimplemented!()));
        {
            ({
                memset_19(
                    (*memory.borrow()).to_any(),
                    0_(),
                    (16usize as ()).wrapping_mul(n),
                )
            });
        }
        return (*memory.borrow()).clone();
    }
    fn deallocate(&self, mut p: Ptr<Emplaceable>, mut n: ()) {
        {
            ({
                let _value: Value<()> = Rc::new(RefCell::new(
                    ({ memset_19((p).to_any(), 0_(), (16usize as ()).wrapping_mul(n)) }),
                ));
                DoNotOptimize_0(_value.as_pointer())
            });
        }
        unimplemented!();
    }
}
pub trait safe_allocator_Emplaceable_ptr_Impl {
    fn allocate(&self, n: ()) -> Ptr<Ptr<Emplaceable>>;
    fn deallocate(&self, p: Ptr<Ptr<Emplaceable>>, n: ());
}
impl safe_allocator_Emplaceable_ptr_Impl for Ptr<safe_allocator_Emplaceable_ptr_> {
    fn allocate(&self, mut n: ()) -> Ptr<Ptr<Emplaceable>> {
        let memory: Value<Ptr<Ptr<Emplaceable>>> = Rc::new(RefCell::new(unimplemented!()));
        {
            ({
                memset_19(
                    (*memory.borrow()).to_any(),
                    0_(),
                    (8usize as ()).wrapping_mul(n),
                )
            });
        }
        return (*memory.borrow()).clone();
    }
    fn deallocate(&self, mut p: Ptr<Ptr<Emplaceable>>, mut n: ()) {
        {
            ({
                let _value: Value<()> = Rc::new(RefCell::new(
                    ({ memset_19((p).to_any(), 0_(), (8usize as ()).wrapping_mul(n)) }),
                ));
                DoNotOptimize_0(_value.as_pointer())
            });
        }
        unimplemented!();
    }
}
pub trait safe_allocator_const_Emplaceable_ptr_Impl {
    fn allocate(&self, mut n: ()) -> Ptr<Ptr<Emplaceable>> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<Ptr<Emplaceable>>, mut n: ()) {
        unimplemented!()
    }
}
impl safe_allocator_const_Emplaceable_ptr_Impl for Ptr<safe_allocator_const_Emplaceable_ptr_> {}
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_4.with(|_| ());
    let _ = alloc_count_5.with(|_| ());
    let _ = dealloc_count_6.with(|_| ());
    let _ = disable_default_constructor_7.with(|_| ());
}
