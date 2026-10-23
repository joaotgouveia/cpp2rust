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
pub struct min_pointer_std_pair_const_int__std_string___std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_std_pair_const_int__std_string___std_integral_constant_unsigned_long__0__ {}
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
pub struct min_pointer_const_std_pair_const_int__std_string___std_integral_constant_unsigned_long__0__
{
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_const_std_pair_const_int__std_string___std_integral_constant_unsigned_long__0__ {
    fn new_1(mut p: Ptr<()>) -> Self {
        Self { ptr_: (p).clone() }
    }
    pub fn pointer_to(
        t: Ptr<()>,
    ) -> min_pointer_const_std_pair_const_int__std_string___std_integral_constant_unsigned_long__0__
    {
        return min_pointer_const_std_pair_const_int__std_string___std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()   } , )    ;
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_const_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: (),
}
impl min_pointer_const_void__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { #[offset(0)] #[byte_size(8)] ptr_ : Ptr< min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__  >  , }
impl min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { fn new_1 ( mut p : Ptr< min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__  >  , ) -> Self { Self { ptr_ : (p ).clone() , }  } pub fn new_2 ( mut _a0 : ()  , ) -> Self { Self { ptr_ : Ptr::<min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__>::null()  , }  } }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_std_pair_const_int__std_string__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___
{}
pub fn testEquality_4(t1: Ptr<()>, t2: Ptr<()>, mut isEqual: ()) -> () {
    if isEqual {
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
    } else {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
    }
    return true;
}
pub fn testEquality_5(t1: Ptr<()>, t2: Ptr<()>, mut isEqual: ()) -> () {
    if isEqual {
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
    } else {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
    }
    return true;
}
pub fn testEquality_6(t1: Ptr<()>, t2: Ptr<()>, mut isEqual: ()) -> () {
    if isEqual {
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
    } else {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
    }
    return true;
}
pub fn operator_lt_7(lhs: Ptr<LessAndEqComp>, rhs: Ptr<LessAndEqComp>) -> () {
    return ({ lhs.with(|__s| __s.value) } < { rhs.with(|__s| __s.value) });
}
pub fn operator_eq_8(lhs: Ptr<LessAndEqComp>, rhs: Ptr<LessAndEqComp>) -> () {
    return ({ lhs.with(|__s| __s.value) } == { rhs.with(|__s| __s.value) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct LessAndEqComp {
    #[offset(0)]
    pub value: (),
}
impl LessAndEqComp {
    pub fn new(mut v: ()) -> Self {
        Self { value: v }
    }
}
impl std::cmp::Ord for LessAndEqComp {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if operator_lt_7(
                Rc::new(RefCell::new(LessAndEqComp {
                    value: self.value.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(LessAndEqComp {
                    value: other.value.clone(),
                }))
                .as_pointer(),
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_7(
                Rc::new(RefCell::new(LessAndEqComp {
                    value: other.value.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(LessAndEqComp {
                    value: self.value.clone(),
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
impl std::cmp::PartialOrd for LessAndEqComp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for LessAndEqComp {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_8(
                Rc::new(RefCell::new(LessAndEqComp {
                    value: self.value.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(LessAndEqComp {
                    value: other.value.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for LessAndEqComp {}
pub fn operator_cmp_9(mut _a0: StrongOrder, mut _a1: StrongOrder) -> () {
    {
        let cmp: Value<()> = Rc::new(RefCell::new(std::cmp::Ord::cmp(&(_a0.value), &(_a1.value))));
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct StrongOrder {
    #[offset(0)]
    pub value: (),
}
impl StrongOrder {
    pub fn new(mut v: ()) -> Self {
        Self { value: v }
    }
}
impl std::cmp::Ord for StrongOrder {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        { operator_cmp_9(self.clone(), other.clone()) }
    }
}
impl std::cmp::PartialOrd for StrongOrder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for StrongOrder {
    fn eq(&self, other: &Self) -> bool {
        { operator_cmp_9(self.clone(), other.clone()) == std::cmp::Ordering::Equal }
    }
}
impl std::cmp::Eq for StrongOrder {}
pub fn operator_cmp_10(mut _a0: WeakOrder, mut _a1: WeakOrder) -> () {
    {
        let cmp: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct WeakOrder {
    #[offset(0)]
    pub value: (),
}
impl WeakOrder {
    pub fn new(mut v: ()) -> Self {
        Self { value: v }
    }
}
impl std::cmp::Ord for WeakOrder {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        { operator_cmp_10(self.clone(), other.clone()) }
    }
}
impl std::cmp::PartialOrd for WeakOrder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for WeakOrder {
    fn eq(&self, other: &Self) -> bool {
        { operator_cmp_10(self.clone(), other.clone()) == std::cmp::Ordering::Equal }
    }
}
impl std::cmp::Eq for WeakOrder {}
pub fn operator_cmp_11(mut lhs: PartialOrder, mut rhs: PartialOrder) -> () {
    if (lhs.value == ({ std_numeric_limits_int_::min() }))
        || (rhs.value == ({ std_numeric_limits_int_::min() }))
    {
        return unimplemented!();
    }
    if (lhs.value == ({ std_numeric_limits_int_::max() }))
        || (rhs.value == ({ std_numeric_limits_int_::max() }))
    {
        return unimplemented!();
    }
    return unimplemented!();
}
pub fn operator_eq_12(mut lhs: PartialOrder, mut rhs: PartialOrder) -> () {
    return unimplemented!();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct PartialOrder {
    #[offset(0)]
    pub value: (),
}
impl PartialOrder {
    pub fn new(mut v: ()) -> Self {
        Self { value: v }
    }
}
impl std::cmp::Ord for PartialOrder {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        { operator_cmp_11(self.clone(), other.clone()) }
    }
}
impl std::cmp::PartialOrd for PartialOrder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for PartialOrder {
    fn eq(&self, other: &Self) -> bool {
        { operator_eq_12(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for PartialOrder {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NonComparable {}
const _: () = assert!(!(false), "!std::equality_comparable<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorEqual<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorGreaterThan<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorGreaterThanEqual<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorLessThan<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorLessThanEqual<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorNotEqual<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorSpaceship<NonComparable>");
pub fn operator_eq_13(_a0: Ptr<EqualityComparable>, _a1: Ptr<EqualityComparable>) -> () {
    return ({ _a0.with(|__s| __s.value_) } == { _a1.with(|__s| __s.value_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct EqualityComparable {
    #[offset(0)]
    value_: (),
}
impl EqualityComparable {
    pub fn new(mut value: ()) -> Self {
        Self { value_: value }
    }
}
impl std::cmp::PartialEq for EqualityComparable {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_13(
                Rc::new(RefCell::new(EqualityComparable {
                    value_: self.value_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(EqualityComparable {
                    value_: other.value_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for EqualityComparable {}
const _: () = assert!(true, "std::equality_comparable<EqualityComparable>");
const _: () = assert!(true, "HasOperatorEqual<EqualityComparable>");
const _: () = assert!(true, "HasOperatorNotEqual<EqualityComparable>");
pub fn operator_eq_14(_a0: Ptr<ThreeWayComparable>, _a1: Ptr<ThreeWayComparable>) -> () {
    return ({ _a0.with(|__s| __s.value_) } == { _a1.with(|__s| __s.value_) });
}
pub fn operator_cmp_15(_a0: Ptr<ThreeWayComparable>, _a1: Ptr<ThreeWayComparable>) -> () {
    {
        let cmp: Value<()> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
            &(_a0.with(|__s| __s.value_)),
            &(_a1.with(|__s| __s.value_)),
        )));
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct ThreeWayComparable {
    #[offset(0)]
    value_: (),
}
impl ThreeWayComparable {
    pub fn new(mut value: ()) -> Self {
        Self { value_: value }
    }
}
impl std::cmp::Ord for ThreeWayComparable {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            operator_cmp_15(
                Rc::new(RefCell::new(ThreeWayComparable {
                    value_: self.value_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(ThreeWayComparable {
                    value_: other.value_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for ThreeWayComparable {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for ThreeWayComparable {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_14(
                Rc::new(RefCell::new(ThreeWayComparable {
                    value_: self.value_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(ThreeWayComparable {
                    value_: other.value_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for ThreeWayComparable {}
const _: () = assert!(true, "std::equality_comparable<ThreeWayComparable>");
const _: () = assert!(true, "std::three_way_comparable<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorEqual<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorGreaterThan<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorGreaterThanEqual<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorLessThan<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorLessThanEqual<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorNotEqual<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorSpaceship<ThreeWayComparable>");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct ImplicitBool_E1 {
    #[offset(0)]
    pub x: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct ImplicitBool_E2 {
    #[offset(0)]
    pub y: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ImplicitBool {
    #[offset(0)]
    pub val: (),
}
pub fn operator_eq_16(mut lhs: ImplicitBool_E1, mut rhs: ImplicitBool_E2) -> ImplicitBool {
    return ImplicitBool {
        val: (lhs.x == rhs.y),
    };
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
        let mut a: [(); 11] = [
            {
                let __tmp_0: Value<()> = Rc::new(RefCell::new(10_()));
                std_pair_int__std_string_::new_1({ __tmp_0.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"ten")
                })
            },
            {
                let __tmp_1: Value<()> = Rc::new(RefCell::new(20_()));
                std_pair_int__std_string_::new_2({ __tmp_1.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"twenty")
                })
            },
            {
                let __tmp_2: Value<()> = Rc::new(RefCell::new(20_()));
                std_pair_int__std_string_::new_3({ __tmp_2.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"twenty 2")
                })
            },
            {
                let __tmp_3: Value<()> = Rc::new(RefCell::new(30_()));
                std_pair_int__std_string_::new_2({ __tmp_3.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"thirty")
                })
            },
            {
                let __tmp_4: Value<()> = Rc::new(RefCell::new(40_()));
                std_pair_int__std_string_::new_4({ __tmp_4.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"forty")
                })
            },
            {
                let __tmp_5: Value<()> = Rc::new(RefCell::new(50_()));
                std_pair_int__std_string_::new_4({ __tmp_5.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"fifty")
                })
            },
            {
                let __tmp_6: Value<()> = Rc::new(RefCell::new(50_()));
                std_pair_int__std_string_::new_5({ __tmp_6.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"fifty 2")
                })
            },
            {
                let __tmp_7: Value<()> = Rc::new(RefCell::new(50_()));
                std_pair_int__std_string_::new_5({ __tmp_7.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"fifty 3")
                })
            },
            {
                let __tmp_8: Value<()> = Rc::new(RefCell::new(60_()));
                std_pair_int__std_string_::new_4({ __tmp_8.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"sixty")
                })
            },
            {
                let __tmp_9: Value<()> = Rc::new(RefCell::new(70_()));
                std_pair_int__std_string_::new_5({ __tmp_9.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"seventy")
                })
            },
            {
                let __tmp_10: Value<()> = Rc::new(RefCell::new(80_()));
                std_pair_int__std_string_::new_2({ __tmp_10.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"eighty")
                })
            },
        ];
        let c1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let c2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let mut a: [(); 11] = [
            {
                let __tmp_11: Value<()> = Rc::new(RefCell::new(10_()));
                std_pair_int__std_string_::new_1({ __tmp_11.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"ten")
                })
            },
            {
                let __tmp_12: Value<()> = Rc::new(RefCell::new(20_()));
                std_pair_int__std_string_::new_2({ __tmp_12.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"twenty")
                })
            },
            {
                let __tmp_13: Value<()> = Rc::new(RefCell::new(20_()));
                std_pair_int__std_string_::new_3({ __tmp_13.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"twenty 2")
                })
            },
            {
                let __tmp_14: Value<()> = Rc::new(RefCell::new(30_()));
                std_pair_int__std_string_::new_2({ __tmp_14.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"thirty")
                })
            },
            {
                let __tmp_15: Value<()> = Rc::new(RefCell::new(40_()));
                std_pair_int__std_string_::new_4({ __tmp_15.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"forty")
                })
            },
            {
                let __tmp_16: Value<()> = Rc::new(RefCell::new(50_()));
                std_pair_int__std_string_::new_4({ __tmp_16.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"fifty")
                })
            },
            {
                let __tmp_17: Value<()> = Rc::new(RefCell::new(50_()));
                std_pair_int__std_string_::new_5({ __tmp_17.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"fifty 2")
                })
            },
            {
                let __tmp_18: Value<()> = Rc::new(RefCell::new(50_()));
                std_pair_int__std_string_::new_5({ __tmp_18.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"fifty 3")
                })
            },
            {
                let __tmp_19: Value<()> = Rc::new(RefCell::new(60_()));
                std_pair_int__std_string_::new_4({ __tmp_19.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"sixty")
                })
            },
            {
                let __tmp_20: Value<()> = Rc::new(RefCell::new(70_()));
                std_pair_int__std_string_::new_5({ __tmp_20.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"seventy")
                })
            },
            {
                let __tmp_21: Value<()> = Rc::new(RefCell::new(80_()));
                std_pair_int__std_string_::new_2({ __tmp_21.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"eighty")
                })
            },
        ];
        let c1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let c2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let mut a: [(); 11] = [
            {
                let __tmp_22: Value<()> = Rc::new(RefCell::new(10_()));
                std_pair_int__std_string_::new_1({ __tmp_22.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"ten")
                })
            },
            {
                let __tmp_23: Value<()> = Rc::new(RefCell::new(20_()));
                std_pair_int__std_string_::new_2({ __tmp_23.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"twenty")
                })
            },
            {
                let __tmp_24: Value<()> = Rc::new(RefCell::new(20_()));
                std_pair_int__std_string_::new_3({ __tmp_24.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"twenty 2")
                })
            },
            {
                let __tmp_25: Value<()> = Rc::new(RefCell::new(30_()));
                std_pair_int__std_string_::new_2({ __tmp_25.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"thirty")
                })
            },
            {
                let __tmp_26: Value<()> = Rc::new(RefCell::new(40_()));
                std_pair_int__std_string_::new_4({ __tmp_26.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"forty")
                })
            },
            {
                let __tmp_27: Value<()> = Rc::new(RefCell::new(50_()));
                std_pair_int__std_string_::new_4({ __tmp_27.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"fifty")
                })
            },
            {
                let __tmp_28: Value<()> = Rc::new(RefCell::new(50_()));
                std_pair_int__std_string_::new_5({ __tmp_28.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"fifty 2")
                })
            },
            {
                let __tmp_29: Value<()> = Rc::new(RefCell::new(50_()));
                std_pair_int__std_string_::new_5({ __tmp_29.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"fifty 3")
                })
            },
            {
                let __tmp_30: Value<()> = Rc::new(RefCell::new(60_()));
                std_pair_int__std_string_::new_4({ __tmp_30.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"sixty")
                })
            },
            {
                let __tmp_31: Value<()> = Rc::new(RefCell::new(70_()));
                std_pair_int__std_string_::new_5({ __tmp_31.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"seventy")
                })
            },
            {
                let __tmp_32: Value<()> = Rc::new(RefCell::new(80_()));
                std_pair_int__std_string_::new_2({ __tmp_32.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"eighty")
                })
            },
        ];
        let c1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let c2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        ({
            let ___x: Value<()> = Rc::new(RefCell::new({
                let __tmp_33: Value<()> = Rc::new(RefCell::new(90_()));
                std_pair_int__std_string_::new_2({ __tmp_33.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"ninety")
                })
            }));
            (*c2.borrow()).insert_45_struct_std_pair_int__std_string__0(___x.as_pointer())
        });
        unimplemented!();
        ({
            let ___x: Value<()> = Rc::new(RefCell::new({
                let __tmp_34: Value<()> = Rc::new(RefCell::new(90_()));
                std_pair_int__std_string_::new_2({ __tmp_34.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"ninety")
                })
            }));
            (*c1.borrow()).insert_45_struct_std_pair_int__std_string__0(___x.as_pointer())
        });
        unimplemented!();
    }
    {
        let mut a: [(); 11] = [
            {
                let __tmp_35: Value<()> = Rc::new(RefCell::new(10_()));
                std_pair_int__std_string_::new_1({ __tmp_35.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"ten")
                })
            },
            {
                let __tmp_36: Value<()> = Rc::new(RefCell::new(20_()));
                std_pair_int__std_string_::new_2({ __tmp_36.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"twenty")
                })
            },
            {
                let __tmp_37: Value<()> = Rc::new(RefCell::new(20_()));
                std_pair_int__std_string_::new_3({ __tmp_37.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"twenty 2")
                })
            },
            {
                let __tmp_38: Value<()> = Rc::new(RefCell::new(30_()));
                std_pair_int__std_string_::new_2({ __tmp_38.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"thirty")
                })
            },
            {
                let __tmp_39: Value<()> = Rc::new(RefCell::new(40_()));
                std_pair_int__std_string_::new_4({ __tmp_39.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"forty")
                })
            },
            {
                let __tmp_40: Value<()> = Rc::new(RefCell::new(50_()));
                std_pair_int__std_string_::new_4({ __tmp_40.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"fifty")
                })
            },
            {
                let __tmp_41: Value<()> = Rc::new(RefCell::new(50_()));
                std_pair_int__std_string_::new_5({ __tmp_41.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"fifty 2")
                })
            },
            {
                let __tmp_42: Value<()> = Rc::new(RefCell::new(50_()));
                std_pair_int__std_string_::new_5({ __tmp_42.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"fifty 3")
                })
            },
            {
                let __tmp_43: Value<()> = Rc::new(RefCell::new(60_()));
                std_pair_int__std_string_::new_4({ __tmp_43.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"sixty")
                })
            },
            {
                let __tmp_44: Value<()> = Rc::new(RefCell::new(70_()));
                std_pair_int__std_string_::new_5({ __tmp_44.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"seventy")
                })
            },
            {
                let __tmp_45: Value<()> = Rc::new(RefCell::new(80_()));
                std_pair_int__std_string_::new_2({ __tmp_45.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"eighty")
                })
            },
        ];
        let c1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let c2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        ({
            let ___x: Value<()> = Rc::new(RefCell::new({
                let __tmp_46: Value<()> = Rc::new(RefCell::new(70_()));
                std_pair_int__std_string_::new_6({ __tmp_46.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"seventy 2")
                })
            }));
            (*c1.borrow()).insert_45_struct_std_pair_int__std_string__0(___x.as_pointer())
        });
        ({
            let ___x: Value<()> = Rc::new(RefCell::new({
                let __tmp_47: Value<()> = Rc::new(RefCell::new(80_()));
                std_pair_int__std_string_::new_3({ __tmp_47.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"eighty 2")
                })
            }));
            (*c2.borrow()).insert_45_struct_std_pair_int__std_string__0(___x.as_pointer())
        });
        unimplemented!();
    }
    {
        let mut a: [(); 11] = [
            {
                let __tmp_48: Value<()> = Rc::new(RefCell::new(10_()));
                std_pair_int__std_string_::new_1({ __tmp_48.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"ten")
                })
            },
            {
                let __tmp_49: Value<()> = Rc::new(RefCell::new(20_()));
                std_pair_int__std_string_::new_2({ __tmp_49.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"twenty")
                })
            },
            {
                let __tmp_50: Value<()> = Rc::new(RefCell::new(20_()));
                std_pair_int__std_string_::new_3({ __tmp_50.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"twenty 2")
                })
            },
            {
                let __tmp_51: Value<()> = Rc::new(RefCell::new(30_()));
                std_pair_int__std_string_::new_2({ __tmp_51.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"thirty")
                })
            },
            {
                let __tmp_52: Value<()> = Rc::new(RefCell::new(40_()));
                std_pair_int__std_string_::new_4({ __tmp_52.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"forty")
                })
            },
            {
                let __tmp_53: Value<()> = Rc::new(RefCell::new(50_()));
                std_pair_int__std_string_::new_4({ __tmp_53.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"fifty")
                })
            },
            {
                let __tmp_54: Value<()> = Rc::new(RefCell::new(50_()));
                std_pair_int__std_string_::new_5({ __tmp_54.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"fifty 2")
                })
            },
            {
                let __tmp_55: Value<()> = Rc::new(RefCell::new(50_()));
                std_pair_int__std_string_::new_5({ __tmp_55.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"fifty 3")
                })
            },
            {
                let __tmp_56: Value<()> = Rc::new(RefCell::new(60_()));
                std_pair_int__std_string_::new_4({ __tmp_56.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"sixty")
                })
            },
            {
                let __tmp_57: Value<()> = Rc::new(RefCell::new(70_()));
                std_pair_int__std_string_::new_5({ __tmp_57.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"seventy")
                })
            },
            {
                let __tmp_58: Value<()> = Rc::new(RefCell::new(80_()));
                std_pair_int__std_string_::new_2({ __tmp_58.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"eighty")
                })
            },
        ];
        let c1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let c2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let mut a: [(); 11] = [
            {
                let __tmp_59: Value<()> = Rc::new(RefCell::new(10_()));
                std_pair_int__std_string_::new_1({ __tmp_59.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"ten")
                })
            },
            {
                let __tmp_60: Value<()> = Rc::new(RefCell::new(20_()));
                std_pair_int__std_string_::new_2({ __tmp_60.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"twenty")
                })
            },
            {
                let __tmp_61: Value<()> = Rc::new(RefCell::new(20_()));
                std_pair_int__std_string_::new_3({ __tmp_61.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"twenty 2")
                })
            },
            {
                let __tmp_62: Value<()> = Rc::new(RefCell::new(30_()));
                std_pair_int__std_string_::new_2({ __tmp_62.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"thirty")
                })
            },
            {
                let __tmp_63: Value<()> = Rc::new(RefCell::new(40_()));
                std_pair_int__std_string_::new_4({ __tmp_63.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"forty")
                })
            },
            {
                let __tmp_64: Value<()> = Rc::new(RefCell::new(50_()));
                std_pair_int__std_string_::new_4({ __tmp_64.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"fifty")
                })
            },
            {
                let __tmp_65: Value<()> = Rc::new(RefCell::new(50_()));
                std_pair_int__std_string_::new_5({ __tmp_65.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"fifty 2")
                })
            },
            {
                let __tmp_66: Value<()> = Rc::new(RefCell::new(50_()));
                std_pair_int__std_string_::new_5({ __tmp_66.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"fifty 3")
                })
            },
            {
                let __tmp_67: Value<()> = Rc::new(RefCell::new(60_()));
                std_pair_int__std_string_::new_4({ __tmp_67.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"sixty")
                })
            },
            {
                let __tmp_68: Value<()> = Rc::new(RefCell::new(70_()));
                std_pair_int__std_string_::new_5({ __tmp_68.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"seventy")
                })
            },
            {
                let __tmp_69: Value<()> = Rc::new(RefCell::new(80_()));
                std_pair_int__std_string_::new_2({ __tmp_69.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"eighty")
                })
            },
        ];
        let c1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let c2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let mut a: [(); 11] = [
            {
                let __tmp_70: Value<()> = Rc::new(RefCell::new(10_()));
                std_pair_int__std_string_::new_1({ __tmp_70.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"ten")
                })
            },
            {
                let __tmp_71: Value<()> = Rc::new(RefCell::new(20_()));
                std_pair_int__std_string_::new_2({ __tmp_71.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"twenty")
                })
            },
            {
                let __tmp_72: Value<()> = Rc::new(RefCell::new(20_()));
                std_pair_int__std_string_::new_3({ __tmp_72.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"twenty 2")
                })
            },
            {
                let __tmp_73: Value<()> = Rc::new(RefCell::new(30_()));
                std_pair_int__std_string_::new_2({ __tmp_73.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"thirty")
                })
            },
            {
                let __tmp_74: Value<()> = Rc::new(RefCell::new(40_()));
                std_pair_int__std_string_::new_4({ __tmp_74.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"forty")
                })
            },
            {
                let __tmp_75: Value<()> = Rc::new(RefCell::new(50_()));
                std_pair_int__std_string_::new_4({ __tmp_75.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"fifty")
                })
            },
            {
                let __tmp_76: Value<()> = Rc::new(RefCell::new(50_()));
                std_pair_int__std_string_::new_5({ __tmp_76.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"fifty 2")
                })
            },
            {
                let __tmp_77: Value<()> = Rc::new(RefCell::new(50_()));
                std_pair_int__std_string_::new_5({ __tmp_77.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"fifty 3")
                })
            },
            {
                let __tmp_78: Value<()> = Rc::new(RefCell::new(60_()));
                std_pair_int__std_string_::new_4({ __tmp_78.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"sixty")
                })
            },
            {
                let __tmp_79: Value<()> = Rc::new(RefCell::new(70_()));
                std_pair_int__std_string_::new_5({ __tmp_79.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"seventy")
                })
            },
            {
                let __tmp_80: Value<()> = Rc::new(RefCell::new(80_()));
                std_pair_int__std_string_::new_2({ __tmp_80.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"eighty")
                })
            },
        ];
        let c1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let c2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        ({
            let ___x: Value<()> = Rc::new(RefCell::new({
                let __tmp_81: Value<()> = Rc::new(RefCell::new(90_()));
                std_pair_int__std_string_::new_2({ __tmp_81.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"ninety")
                })
            }));
            (*c2.borrow()).insert_45_struct_std_pair_int__std_string__0(___x.as_pointer())
        });
        unimplemented!();
        ({
            let ___x: Value<()> = Rc::new(RefCell::new({
                let __tmp_82: Value<()> = Rc::new(RefCell::new(90_()));
                std_pair_int__std_string_::new_2({ __tmp_82.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"ninety")
                })
            }));
            (*c1.borrow()).insert_45_struct_std_pair_int__std_string__0(___x.as_pointer())
        });
        unimplemented!();
    }
    {
        let mut a: [(); 11] = [
            {
                let __tmp_83: Value<()> = Rc::new(RefCell::new(10_()));
                std_pair_int__std_string_::new_1({ __tmp_83.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"ten")
                })
            },
            {
                let __tmp_84: Value<()> = Rc::new(RefCell::new(20_()));
                std_pair_int__std_string_::new_2({ __tmp_84.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"twenty")
                })
            },
            {
                let __tmp_85: Value<()> = Rc::new(RefCell::new(20_()));
                std_pair_int__std_string_::new_3({ __tmp_85.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"twenty 2")
                })
            },
            {
                let __tmp_86: Value<()> = Rc::new(RefCell::new(30_()));
                std_pair_int__std_string_::new_2({ __tmp_86.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"thirty")
                })
            },
            {
                let __tmp_87: Value<()> = Rc::new(RefCell::new(40_()));
                std_pair_int__std_string_::new_4({ __tmp_87.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"forty")
                })
            },
            {
                let __tmp_88: Value<()> = Rc::new(RefCell::new(50_()));
                std_pair_int__std_string_::new_4({ __tmp_88.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"fifty")
                })
            },
            {
                let __tmp_89: Value<()> = Rc::new(RefCell::new(50_()));
                std_pair_int__std_string_::new_5({ __tmp_89.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"fifty 2")
                })
            },
            {
                let __tmp_90: Value<()> = Rc::new(RefCell::new(50_()));
                std_pair_int__std_string_::new_5({ __tmp_90.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"fifty 3")
                })
            },
            {
                let __tmp_91: Value<()> = Rc::new(RefCell::new(60_()));
                std_pair_int__std_string_::new_4({ __tmp_91.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"sixty")
                })
            },
            {
                let __tmp_92: Value<()> = Rc::new(RefCell::new(70_()));
                std_pair_int__std_string_::new_5({ __tmp_92.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"seventy")
                })
            },
            {
                let __tmp_93: Value<()> = Rc::new(RefCell::new(80_()));
                std_pair_int__std_string_::new_2({ __tmp_93.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"eighty")
                })
            },
        ];
        let c1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let c2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        ({
            let ___x: Value<()> = Rc::new(RefCell::new({
                let __tmp_94: Value<()> = Rc::new(RefCell::new(70_()));
                std_pair_int__std_string_::new_6({ __tmp_94.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"seventy 2")
                })
            }));
            (*c1.borrow()).insert_45_struct_std_pair_int__std_string__0(___x.as_pointer())
        });
        ({
            let ___x: Value<()> = Rc::new(RefCell::new({
                let __tmp_95: Value<()> = Rc::new(RefCell::new(80_()));
                std_pair_int__std_string_::new_3({ __tmp_95.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"eighty 2")
                })
            }));
            (*c2.borrow()).insert_45_struct_std_pair_int__std_string__0(___x.as_pointer())
        });
        unimplemented!();
    }
    {
        let mut a: [(); 4] = [
            {
                let __tmp_96: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_97: Value<()> = Rc::new(RefCell::new(('a' as ())));
                std_pair_int__char_::new({ __tmp_96.as_pointer() }, { __tmp_97.as_pointer() })
            },
            {
                let __tmp_98: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_99: Value<()> = Rc::new(RefCell::new(('b' as ())));
                std_pair_int__char_::new({ __tmp_98.as_pointer() }, { __tmp_99.as_pointer() })
            },
            {
                let __tmp_100: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_101: Value<()> = Rc::new(RefCell::new(('d' as ())));
                std_pair_int__char_::new({ __tmp_100.as_pointer() }, { __tmp_101.as_pointer() })
            },
            {
                let __tmp_102: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_103: Value<()> = Rc::new(RefCell::new(('b' as ())));
                std_pair_int__char_::new({ __tmp_102.as_pointer() }, { __tmp_103.as_pointer() })
            },
        ];
        let mut b: [(); 5] = [
            {
                let __tmp_104: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_105: Value<()> = Rc::new(RefCell::new(('a' as ())));
                std_pair_int__char_::new({ __tmp_104.as_pointer() }, { __tmp_105.as_pointer() })
            },
            {
                let __tmp_106: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_107: Value<()> = Rc::new(RefCell::new(('b' as ())));
                std_pair_int__char_::new({ __tmp_106.as_pointer() }, { __tmp_107.as_pointer() })
            },
            {
                let __tmp_108: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_109: Value<()> = Rc::new(RefCell::new(('b' as ())));
                std_pair_int__char_::new({ __tmp_108.as_pointer() }, { __tmp_109.as_pointer() })
            },
            {
                let __tmp_110: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_111: Value<()> = Rc::new(RefCell::new(('d' as ())));
                std_pair_int__char_::new({ __tmp_110.as_pointer() }, { __tmp_111.as_pointer() })
            },
            {
                let __tmp_112: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_113: Value<()> = Rc::new(RefCell::new(('b' as ())));
                std_pair_int__char_::new({ __tmp_112.as_pointer() }, { __tmp_113.as_pointer() })
            },
        ];
        let c1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let c2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let mut a: [(); 4] = [
            {
                let __tmp_114: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_115: Value<()> = Rc::new(RefCell::new(('a' as ())));
                std_pair_int__char_::new({ __tmp_114.as_pointer() }, { __tmp_115.as_pointer() })
            },
            {
                let __tmp_116: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_117: Value<()> = Rc::new(RefCell::new(('b' as ())));
                std_pair_int__char_::new({ __tmp_116.as_pointer() }, { __tmp_117.as_pointer() })
            },
            {
                let __tmp_118: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_119: Value<()> = Rc::new(RefCell::new(('d' as ())));
                std_pair_int__char_::new({ __tmp_118.as_pointer() }, { __tmp_119.as_pointer() })
            },
            {
                let __tmp_120: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_121: Value<()> = Rc::new(RefCell::new(('b' as ())));
                std_pair_int__char_::new({ __tmp_120.as_pointer() }, { __tmp_121.as_pointer() })
            },
        ];
        let mut b: [(); 4] = [
            {
                let __tmp_122: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_123: Value<()> = Rc::new(RefCell::new(('a' as ())));
                std_pair_int__char_::new({ __tmp_122.as_pointer() }, { __tmp_123.as_pointer() })
            },
            {
                let __tmp_124: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_125: Value<()> = Rc::new(RefCell::new(('b' as ())));
                std_pair_int__char_::new({ __tmp_124.as_pointer() }, { __tmp_125.as_pointer() })
            },
            {
                let __tmp_126: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_127: Value<()> = Rc::new(RefCell::new(('E' as ())));
                std_pair_int__char_::new({ __tmp_126.as_pointer() }, { __tmp_127.as_pointer() })
            },
            {
                let __tmp_128: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_129: Value<()> = Rc::new(RefCell::new(('b' as ())));
                std_pair_int__char_::new({ __tmp_128.as_pointer() }, { __tmp_129.as_pointer() })
            },
        ];
        let c1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let c2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    return 0_();
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__;
pub trait ImplicitBoolImpl {
    fn to__1(&self) -> ();
}
impl ImplicitBoolImpl for Ptr<ImplicitBool> {
    fn to__1(&self) -> () {
        return (*self).with(|__s| __s.val);
    }
}
pub trait min_allocator_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___Impl
{
    fn allocate ( &self , n : ()  , ) -> min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  ;
    fn deallocate(
        &self,
        p : min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
        n: (),
    );
}
impl min_allocator_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___Impl for Ptr<min_allocator_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___> {
fn allocate ( &self , mut n : ()  , ) -> min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  { return min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()   } , )    ;
 } fn deallocate ( &self , mut p : min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  , mut n : ()  , ) { unimplemented!()  ;
 } }
pub trait min_allocator_std_pair_const_int__std_string__Impl {
    fn allocate(
        &self,
        mut n: (),
    ) -> min_pointer_std_pair_const_int__std_string___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p : min_pointer_std_pair_const_int__std_string___std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!()
    }
}
impl min_allocator_std_pair_const_int__std_string__Impl
    for Ptr<min_allocator_std_pair_const_int__std_string__>
{
}
pub trait min_pointer_const_std_pair_const_int__std_string___std_integral_constant_unsigned_long__0__Impl
{
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<()>;
    fn operator_inc_9(
        &self,
    ) -> Ptr<
        min_pointer_const_std_pair_const_int__std_string___std_integral_constant_unsigned_long__0__,
    > {
        unimplemented!()
    }
    fn operator_post_inc_10(
        &self,
        mut _a0: (),
    ) -> min_pointer_const_std_pair_const_int__std_string___std_integral_constant_unsigned_long__0__
    {
        unimplemented!()
    }
    fn operator_dec_11(
        &self,
    ) -> Ptr<
        min_pointer_const_std_pair_const_int__std_string___std_integral_constant_unsigned_long__0__,
    > {
        unimplemented!()
    }
    fn operator_post_dec_12(
        &self,
        mut _a0: (),
    ) -> min_pointer_const_std_pair_const_int__std_string___std_integral_constant_unsigned_long__0__
    {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<
        min_pointer_const_std_pair_const_int__std_string___std_integral_constant_unsigned_long__0__,
    > {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<
        min_pointer_const_std_pair_const_int__std_string___std_integral_constant_unsigned_long__0__,
    > {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_const_std_pair_const_int__std_string___std_integral_constant_unsigned_long__0__
    {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_const_std_pair_const_int__std_string___std_integral_constant_unsigned_long__0__
    {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
}
impl min_pointer_const_std_pair_const_int__std_string___std_integral_constant_unsigned_long__0__Impl
    for Ptr<
        min_pointer_const_std_pair_const_int__std_string___std_integral_constant_unsigned_long__0__,
    >
{
    fn operator_arrow(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
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
pub trait min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__Impl
{
    fn to_(&self) -> ();
    fn operator_deref ( &self , ) -> Ptr< min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__  > ;
    fn operator_arrow ( &self , ) -> Ptr< min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }
    fn operator_inc_8 ( &self , ) -> Ptr< min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > ;
    fn operator_post_inc_9 ( &self , mut _a0 : ()  , ) -> min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_dec_10 ( &self , ) -> Ptr< min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_post_dec_11 ( &self , mut _a0 : ()  , ) -> min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_add_assign ( &self , mut n : ()  , ) -> Ptr< min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_sub_assign ( &self , mut n : ()  , ) -> Ptr< min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_add ( &self , mut n : ()  , ) -> min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_sub ( &self , mut n : ()  , ) -> min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }
    fn operator_index ( &self , n : ()  , ) -> Ptr< min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__  > ;
}
impl min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__Impl for Ptr<min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__> {
fn to_ ( &self , ) -> ()  { return ! ( ( (* self ) .with(|__s| __s . ptr_ .clone()) ).is_null() )   ;
 } fn operator_deref ( &self , ) -> Ptr< min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__  > { return (* self ) .with(|__s| __s . ptr_ .clone())    ;
 } fn operator_inc_8 ( &self , ) -> Ptr< min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > { field!((* self ) , ptr_) .with_mut(|__v| __v. prefix_inc ()) ;
 return ((* self )  ).clone()  ;
 } fn operator_index ( &self , mut n : ()  , ) -> Ptr< min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__  > { return (* self ) .with(|__s| __s . ptr_ .clone()) . offset ( ( n ) as isize )   ;
 } }
pub trait min_pointer_std_pair_const_int__std_string___std_integral_constant_unsigned_long__0__Impl
{
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_inc_8(
        &self,
    ) -> Ptr<min_pointer_std_pair_const_int__std_string___std_integral_constant_unsigned_long__0__>
    {
        unimplemented!()
    }
    fn operator_post_inc_9(
        &self,
        mut _a0: (),
    ) -> min_pointer_std_pair_const_int__std_string___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(
        &self,
    ) -> Ptr<min_pointer_std_pair_const_int__std_string___std_integral_constant_unsigned_long__0__>
    {
        unimplemented!()
    }
    fn operator_post_dec_11(
        &self,
        mut _a0: (),
    ) -> min_pointer_std_pair_const_int__std_string___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_std_pair_const_int__std_string___std_integral_constant_unsigned_long__0__>
    {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_std_pair_const_int__std_string___std_integral_constant_unsigned_long__0__>
    {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_std_pair_const_int__std_string___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_std_pair_const_int__std_string___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
}
impl min_pointer_std_pair_const_int__std_string___std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_std_pair_const_int__std_string___std_integral_constant_unsigned_long__0__>
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
}
