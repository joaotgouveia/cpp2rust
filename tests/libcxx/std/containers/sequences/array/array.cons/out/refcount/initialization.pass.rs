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
pub struct NoDefault {}
impl NoDefault {
    pub fn new(mut _a0: ()) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_initialization {}
pub fn test_initializer_list_0() -> () {
    {
        let a3_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let a3_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let a3_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let a3_3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Empty {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Trivial {
    #[offset(0)]
    pub i: (),
    #[offset(4)]
    pub j: (),
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign)]
#[byte_size(1)]
pub struct NonTrivial {}
impl NonTrivial {
    pub fn new() -> Self {
        Self {}
    }
    pub fn copy_from(_a0: Ptr<NonTrivial>) -> Self {
        Self {}
    }
}
impl Clone for NonTrivial {
    fn clone(&self) -> Self {
        let __src: Value<NonTrivial> = Rc::new(RefCell::new(NonTrivial {}));
        NonTrivial::copy_from(__src.as_pointer())
    }
}
impl Default for NonTrivial {
    fn default() -> Self {
        { NonTrivial::new() }
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign)]
#[byte_size(8)]
pub struct NonEmptyNonTrivial {
    #[offset(0)]
    pub i: (),
    #[offset(4)]
    pub j: (),
}
impl NonEmptyNonTrivial {
    pub fn new() -> Self {
        Self { i: 22_(), j: 33_() }
    }
    pub fn copy_from(_a0: Ptr<NonEmptyNonTrivial>) -> Self {
        Self { i: 22_(), j: 33_() }
    }
}
impl Clone for NonEmptyNonTrivial {
    fn clone(&self) -> Self {
        let __src: Value<NonEmptyNonTrivial> = Rc::new(RefCell::new(NonEmptyNonTrivial {
            i: self.i.clone(),
            j: self.j.clone(),
        }));
        NonEmptyNonTrivial::copy_from(__src.as_pointer())
    }
}
impl Default for NonEmptyNonTrivial {
    fn default() -> Self {
        { NonEmptyNonTrivial::new() }
    }
}
pub fn with_all_types_1() -> () {
    ({
        test_initializationImpl::operator_call_1_char(
            &Rc::new(RefCell::new(<test_initialization>::default())).as_pointer(),
        )
    });
    ({
        test_initializationImpl::operator_call_1_int(
            &Rc::new(RefCell::new(<test_initialization>::default())).as_pointer(),
        )
    });
    ({
        test_initializationImpl::operator_call_1_long(
            &Rc::new(RefCell::new(<test_initialization>::default())).as_pointer(),
        )
    });
    ({
        test_initializationImpl::operator_call_1_float(
            &Rc::new(RefCell::new(<test_initialization>::default())).as_pointer(),
        )
    });
    ({
        test_initializationImpl::operator_call_1_double(
            &Rc::new(RefCell::new(<test_initialization>::default())).as_pointer(),
        )
    });
    ({
        test_initializationImpl::operator_call_1_long_double(
            &Rc::new(RefCell::new(<test_initialization>::default())).as_pointer(),
        )
    });
    ({
        test_initializationImpl::operator_call_1_struct_Empty(
            &Rc::new(RefCell::new(<test_initialization>::default())).as_pointer(),
        )
    });
    ({
        test_initializationImpl::operator_call_1_struct_Trivial(
            &Rc::new(RefCell::new(<test_initialization>::default())).as_pointer(),
        )
    });
    ({
        test_initializationImpl::operator_call_1_struct_NonTrivial(
            &Rc::new(RefCell::new(<test_initialization>::default())).as_pointer(),
        )
    });
    ({
        test_initializationImpl::operator_call_1_struct_NonEmptyNonTrivial(
            &Rc::new(RefCell::new(<test_initialization>::default())).as_pointer(),
        )
    });
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Foo {}
const _: () = assert!(!(false), "!is_list_initializable_int<std::array<Foo, 0>>");
const _: () = assert!(!(false), "!is_list_initializable_int<std::array<Foo, 1>>");
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
    ({ with_all_types_1() });
    ({ test_initializer_list_0() });
    const _: () = assert!(true, "with_all_types<test_initialization>()");;
    const _: () = assert!(true, "test_initializer_list()");;
    return 0_();
}
pub trait NonEmptyNonTrivialImpl {
    fn copy_assign(&self, _a0: Ptr<NonEmptyNonTrivial>) -> Ptr<NonEmptyNonTrivial>;
}
impl NonEmptyNonTrivialImpl for Ptr<NonEmptyNonTrivial> {
    fn copy_assign(&self, _a0: Ptr<NonEmptyNonTrivial>) -> Ptr<NonEmptyNonTrivial> {
        field!((*self), i).write({ _a0.with(|__s| __s.i) });
        field!((*self), j).write({ _a0.with(|__s| __s.j) });
        return (*self).clone();
    }
}
pub trait NonTrivialImpl {
    fn copy_assign(&self, _a0: Ptr<NonTrivial>) -> Ptr<NonTrivial>;
}
impl NonTrivialImpl for Ptr<NonTrivial> {
    fn copy_assign(&self, _a0: Ptr<NonTrivial>) -> Ptr<NonTrivial> {
        return (*self).clone();
    }
}
pub trait test_initializationImpl {
    fn operator_call_1_char(&self);
    fn operator_call_1_int(&self);
    fn operator_call_1_long(&self);
    fn operator_call_1_float(&self);
    fn operator_call_1_double(&self);
    fn operator_call_1_long_double(&self);
    fn operator_call_1_struct_Empty(&self);
    fn operator_call_1_struct_Trivial(&self);
    fn operator_call_1_struct_NonTrivial(&self);
    fn operator_call_1_struct_NonEmptyNonTrivial(&self);
}
impl test_initializationImpl for Ptr<test_initialization> {
    fn operator_call_1_char(&self) {
        {
            let a0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*a0.borrow_mut());
            {
                let a1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1.borrow_mut());
                let a2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2.borrow_mut());
                let a3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3.borrow_mut());
            }
            let nodefault: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*nodefault.borrow_mut());
        }
        {
            let a0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*a0.borrow_mut());
            let nodefault: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*nodefault.borrow_mut());
        }
        {
            {
                let a0_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a0_0.borrow_mut());
            }
            {
                let a1_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1_0.borrow_mut());
                let a1_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1_1.borrow_mut());
            }
            {
                let a2_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_0.borrow_mut());
                let a2_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_1.borrow_mut());
                let a2_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_2.borrow_mut());
            }
            {
                let a3_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_0.borrow_mut());
                let a3_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_1.borrow_mut());
                let a3_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_2.borrow_mut());
                let a3_3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_3.borrow_mut());
            }
            let nodefault: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*nodefault.borrow_mut());
        }
        {
            {
                let a0_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a0_0.borrow_mut());
            }
            {
                let a1_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1_0.borrow_mut());
                let a1_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1_1.borrow_mut());
            }
            {
                let a2_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_0.borrow_mut());
                let a2_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_1.borrow_mut());
                let a2_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_2.borrow_mut());
            }
            {
                let a3_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_0.borrow_mut());
                let a3_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_1.borrow_mut());
                let a3_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_2.borrow_mut());
                let a3_3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_3.borrow_mut());
            }
            let nodefault: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*nodefault.borrow_mut());
        }
        {
            {
                let a0_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a0_0.borrow_mut());
            }
            {
                let a1_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1_0.borrow_mut());
                let a1_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1_1.borrow_mut());
            }
            {
                let a2_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_0.borrow_mut());
                let a2_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_1.borrow_mut());
                let a2_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_2.borrow_mut());
            }
            {
                let a3_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_0.borrow_mut());
                let a3_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_1.borrow_mut());
                let a3_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_2.borrow_mut());
                let a3_3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_3.borrow_mut());
            }
            let nodefault: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*nodefault.borrow_mut());
        }
    }
    fn operator_call_1_int(&self) {
        {
            let a0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*a0.borrow_mut());
            {
                let a1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1.borrow_mut());
                let a2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2.borrow_mut());
                let a3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3.borrow_mut());
            }
            let nodefault: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*nodefault.borrow_mut());
        }
        {
            let a0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*a0.borrow_mut());
            let nodefault: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*nodefault.borrow_mut());
        }
        {
            {
                let a0_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a0_0.borrow_mut());
            }
            {
                let a1_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1_0.borrow_mut());
                let a1_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1_1.borrow_mut());
            }
            {
                let a2_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_0.borrow_mut());
                let a2_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_1.borrow_mut());
                let a2_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_2.borrow_mut());
            }
            {
                let a3_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_0.borrow_mut());
                let a3_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_1.borrow_mut());
                let a3_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_2.borrow_mut());
                let a3_3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_3.borrow_mut());
            }
            let nodefault: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*nodefault.borrow_mut());
        }
        {
            {
                let a0_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a0_0.borrow_mut());
            }
            {
                let a1_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1_0.borrow_mut());
                let a1_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1_1.borrow_mut());
            }
            {
                let a2_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_0.borrow_mut());
                let a2_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_1.borrow_mut());
                let a2_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_2.borrow_mut());
            }
            {
                let a3_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_0.borrow_mut());
                let a3_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_1.borrow_mut());
                let a3_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_2.borrow_mut());
                let a3_3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_3.borrow_mut());
            }
            let nodefault: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*nodefault.borrow_mut());
        }
        {
            {
                let a0_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a0_0.borrow_mut());
            }
            {
                let a1_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1_0.borrow_mut());
                let a1_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1_1.borrow_mut());
            }
            {
                let a2_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_0.borrow_mut());
                let a2_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_1.borrow_mut());
                let a2_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_2.borrow_mut());
            }
            {
                let a3_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_0.borrow_mut());
                let a3_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_1.borrow_mut());
                let a3_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_2.borrow_mut());
                let a3_3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_3.borrow_mut());
            }
            let nodefault: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*nodefault.borrow_mut());
        }
    }
    fn operator_call_1_long(&self) {
        {
            let a0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*a0.borrow_mut());
            {
                let a1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1.borrow_mut());
                let a2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2.borrow_mut());
                let a3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3.borrow_mut());
            }
            let nodefault: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*nodefault.borrow_mut());
        }
        {
            let a0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*a0.borrow_mut());
            let nodefault: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*nodefault.borrow_mut());
        }
        {
            {
                let a0_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a0_0.borrow_mut());
            }
            {
                let a1_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1_0.borrow_mut());
                let a1_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1_1.borrow_mut());
            }
            {
                let a2_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_0.borrow_mut());
                let a2_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_1.borrow_mut());
                let a2_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_2.borrow_mut());
            }
            {
                let a3_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_0.borrow_mut());
                let a3_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_1.borrow_mut());
                let a3_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_2.borrow_mut());
                let a3_3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_3.borrow_mut());
            }
            let nodefault: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*nodefault.borrow_mut());
        }
        {
            {
                let a0_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a0_0.borrow_mut());
            }
            {
                let a1_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1_0.borrow_mut());
                let a1_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1_1.borrow_mut());
            }
            {
                let a2_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_0.borrow_mut());
                let a2_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_1.borrow_mut());
                let a2_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_2.borrow_mut());
            }
            {
                let a3_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_0.borrow_mut());
                let a3_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_1.borrow_mut());
                let a3_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_2.borrow_mut());
                let a3_3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_3.borrow_mut());
            }
            let nodefault: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*nodefault.borrow_mut());
        }
        {
            {
                let a0_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a0_0.borrow_mut());
            }
            {
                let a1_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1_0.borrow_mut());
                let a1_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1_1.borrow_mut());
            }
            {
                let a2_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_0.borrow_mut());
                let a2_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_1.borrow_mut());
                let a2_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_2.borrow_mut());
            }
            {
                let a3_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_0.borrow_mut());
                let a3_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_1.borrow_mut());
                let a3_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_2.borrow_mut());
                let a3_3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_3.borrow_mut());
            }
            let nodefault: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*nodefault.borrow_mut());
        }
    }
    fn operator_call_1_float(&self) {
        {
            let a0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*a0.borrow_mut());
            {
                let a1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1.borrow_mut());
                let a2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2.borrow_mut());
                let a3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3.borrow_mut());
            }
            let nodefault: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*nodefault.borrow_mut());
        }
        {
            let a0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*a0.borrow_mut());
            let nodefault: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*nodefault.borrow_mut());
        }
        {
            {
                let a0_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a0_0.borrow_mut());
            }
            {
                let a1_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1_0.borrow_mut());
                let a1_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1_1.borrow_mut());
            }
            {
                let a2_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_0.borrow_mut());
                let a2_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_1.borrow_mut());
                let a2_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_2.borrow_mut());
            }
            {
                let a3_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_0.borrow_mut());
                let a3_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_1.borrow_mut());
                let a3_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_2.borrow_mut());
                let a3_3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_3.borrow_mut());
            }
            let nodefault: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*nodefault.borrow_mut());
        }
        {
            {
                let a0_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a0_0.borrow_mut());
            }
            {
                let a1_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1_0.borrow_mut());
                let a1_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1_1.borrow_mut());
            }
            {
                let a2_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_0.borrow_mut());
                let a2_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_1.borrow_mut());
                let a2_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_2.borrow_mut());
            }
            {
                let a3_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_0.borrow_mut());
                let a3_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_1.borrow_mut());
                let a3_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_2.borrow_mut());
                let a3_3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_3.borrow_mut());
            }
            let nodefault: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*nodefault.borrow_mut());
        }
        {
            {
                let a0_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a0_0.borrow_mut());
            }
            {
                let a1_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1_0.borrow_mut());
                let a1_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1_1.borrow_mut());
            }
            {
                let a2_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_0.borrow_mut());
                let a2_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_1.borrow_mut());
                let a2_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_2.borrow_mut());
            }
            {
                let a3_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_0.borrow_mut());
                let a3_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_1.borrow_mut());
                let a3_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_2.borrow_mut());
                let a3_3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_3.borrow_mut());
            }
            let nodefault: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*nodefault.borrow_mut());
        }
    }
    fn operator_call_1_double(&self) {
        {
            let a0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*a0.borrow_mut());
            {
                let a1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1.borrow_mut());
                let a2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2.borrow_mut());
                let a3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3.borrow_mut());
            }
            let nodefault: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*nodefault.borrow_mut());
        }
        {
            let a0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*a0.borrow_mut());
            let nodefault: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*nodefault.borrow_mut());
        }
        {
            {
                let a0_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a0_0.borrow_mut());
            }
            {
                let a1_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1_0.borrow_mut());
                let a1_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1_1.borrow_mut());
            }
            {
                let a2_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_0.borrow_mut());
                let a2_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_1.borrow_mut());
                let a2_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_2.borrow_mut());
            }
            {
                let a3_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_0.borrow_mut());
                let a3_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_1.borrow_mut());
                let a3_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_2.borrow_mut());
                let a3_3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_3.borrow_mut());
            }
            let nodefault: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*nodefault.borrow_mut());
        }
        {
            {
                let a0_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a0_0.borrow_mut());
            }
            {
                let a1_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1_0.borrow_mut());
                let a1_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1_1.borrow_mut());
            }
            {
                let a2_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_0.borrow_mut());
                let a2_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_1.borrow_mut());
                let a2_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_2.borrow_mut());
            }
            {
                let a3_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_0.borrow_mut());
                let a3_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_1.borrow_mut());
                let a3_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_2.borrow_mut());
                let a3_3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_3.borrow_mut());
            }
            let nodefault: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*nodefault.borrow_mut());
        }
        {
            {
                let a0_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a0_0.borrow_mut());
            }
            {
                let a1_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1_0.borrow_mut());
                let a1_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1_1.borrow_mut());
            }
            {
                let a2_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_0.borrow_mut());
                let a2_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_1.borrow_mut());
                let a2_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_2.borrow_mut());
            }
            {
                let a3_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_0.borrow_mut());
                let a3_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_1.borrow_mut());
                let a3_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_2.borrow_mut());
                let a3_3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_3.borrow_mut());
            }
            let nodefault: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*nodefault.borrow_mut());
        }
    }
    fn operator_call_1_long_double(&self) {
        {
            let a0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*a0.borrow_mut());
            {
                let a1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1.borrow_mut());
                let a2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2.borrow_mut());
                let a3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3.borrow_mut());
            }
            let nodefault: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*nodefault.borrow_mut());
        }
        {
            let a0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*a0.borrow_mut());
            let nodefault: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*nodefault.borrow_mut());
        }
        {
            {
                let a0_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a0_0.borrow_mut());
            }
            {
                let a1_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1_0.borrow_mut());
                let a1_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1_1.borrow_mut());
            }
            {
                let a2_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_0.borrow_mut());
                let a2_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_1.borrow_mut());
                let a2_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_2.borrow_mut());
            }
            {
                let a3_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_0.borrow_mut());
                let a3_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_1.borrow_mut());
                let a3_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_2.borrow_mut());
                let a3_3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_3.borrow_mut());
            }
            let nodefault: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*nodefault.borrow_mut());
        }
        {
            {
                let a0_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a0_0.borrow_mut());
            }
            {
                let a1_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1_0.borrow_mut());
                let a1_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1_1.borrow_mut());
            }
            {
                let a2_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_0.borrow_mut());
                let a2_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_1.borrow_mut());
                let a2_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_2.borrow_mut());
            }
            {
                let a3_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_0.borrow_mut());
                let a3_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_1.borrow_mut());
                let a3_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_2.borrow_mut());
                let a3_3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_3.borrow_mut());
            }
            let nodefault: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*nodefault.borrow_mut());
        }
        {
            {
                let a0_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a0_0.borrow_mut());
            }
            {
                let a1_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1_0.borrow_mut());
                let a1_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1_1.borrow_mut());
            }
            {
                let a2_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_0.borrow_mut());
                let a2_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_1.borrow_mut());
                let a2_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_2.borrow_mut());
            }
            {
                let a3_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_0.borrow_mut());
                let a3_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_1.borrow_mut());
                let a3_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_2.borrow_mut());
                let a3_3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_3.borrow_mut());
            }
            let nodefault: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*nodefault.borrow_mut());
        }
    }
    fn operator_call_1_struct_Empty(&self) {
        {
            let a0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*a0.borrow_mut());
            {
                let a1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1.borrow_mut());
                let a2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2.borrow_mut());
                let a3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3.borrow_mut());
            }
            let nodefault: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*nodefault.borrow_mut());
        }
        {
            let a0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*a0.borrow_mut());
            let nodefault: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*nodefault.borrow_mut());
        }
        {
            {
                let a0_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a0_0.borrow_mut());
            }
            {
                let a1_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1_0.borrow_mut());
                let a1_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1_1.borrow_mut());
            }
            {
                let a2_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_0.borrow_mut());
                let a2_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_1.borrow_mut());
                let a2_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_2.borrow_mut());
            }
            {
                let a3_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_0.borrow_mut());
                let a3_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_1.borrow_mut());
                let a3_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_2.borrow_mut());
                let a3_3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_3.borrow_mut());
            }
            let nodefault: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*nodefault.borrow_mut());
        }
        {
            {
                let a0_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a0_0.borrow_mut());
            }
            {
                let a1_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1_0.borrow_mut());
                let a1_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1_1.borrow_mut());
            }
            {
                let a2_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_0.borrow_mut());
                let a2_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_1.borrow_mut());
                let a2_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_2.borrow_mut());
            }
            {
                let a3_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_0.borrow_mut());
                let a3_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_1.borrow_mut());
                let a3_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_2.borrow_mut());
                let a3_3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_3.borrow_mut());
            }
            let nodefault: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*nodefault.borrow_mut());
        }
        {
            {
                let a0_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a0_0.borrow_mut());
            }
            {
                let a1_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1_0.borrow_mut());
                let a1_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1_1.borrow_mut());
            }
            {
                let a2_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_0.borrow_mut());
                let a2_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_1.borrow_mut());
                let a2_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_2.borrow_mut());
            }
            {
                let a3_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_0.borrow_mut());
                let a3_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_1.borrow_mut());
                let a3_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_2.borrow_mut());
                let a3_3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_3.borrow_mut());
            }
            let nodefault: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*nodefault.borrow_mut());
        }
    }
    fn operator_call_1_struct_Trivial(&self) {
        {
            let a0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*a0.borrow_mut());
            {
                let a1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1.borrow_mut());
                let a2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2.borrow_mut());
                let a3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3.borrow_mut());
            }
            let nodefault: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*nodefault.borrow_mut());
        }
        {
            let a0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*a0.borrow_mut());
            let nodefault: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*nodefault.borrow_mut());
        }
        {
            {
                let a0_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a0_0.borrow_mut());
            }
            {
                let a1_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1_0.borrow_mut());
                let a1_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1_1.borrow_mut());
            }
            {
                let a2_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_0.borrow_mut());
                let a2_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_1.borrow_mut());
                let a2_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_2.borrow_mut());
            }
            {
                let a3_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_0.borrow_mut());
                let a3_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_1.borrow_mut());
                let a3_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_2.borrow_mut());
                let a3_3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_3.borrow_mut());
            }
            let nodefault: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*nodefault.borrow_mut());
        }
        {
            {
                let a0_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a0_0.borrow_mut());
            }
            {
                let a1_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1_0.borrow_mut());
                let a1_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1_1.borrow_mut());
            }
            {
                let a2_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_0.borrow_mut());
                let a2_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_1.borrow_mut());
                let a2_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_2.borrow_mut());
            }
            {
                let a3_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_0.borrow_mut());
                let a3_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_1.borrow_mut());
                let a3_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_2.borrow_mut());
                let a3_3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_3.borrow_mut());
            }
            let nodefault: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*nodefault.borrow_mut());
        }
        {
            {
                let a0_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a0_0.borrow_mut());
            }
            {
                let a1_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1_0.borrow_mut());
                let a1_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1_1.borrow_mut());
            }
            {
                let a2_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_0.borrow_mut());
                let a2_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_1.borrow_mut());
                let a2_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_2.borrow_mut());
            }
            {
                let a3_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_0.borrow_mut());
                let a3_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_1.borrow_mut());
                let a3_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_2.borrow_mut());
                let a3_3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_3.borrow_mut());
            }
            let nodefault: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*nodefault.borrow_mut());
        }
    }
    fn operator_call_1_struct_NonTrivial(&self) {
        {
            let a0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*a0.borrow_mut());
            {
                let a1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1.borrow_mut());
                let a2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2.borrow_mut());
                let a3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3.borrow_mut());
            }
            let nodefault: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*nodefault.borrow_mut());
        }
        {
            let a0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*a0.borrow_mut());
            let nodefault: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*nodefault.borrow_mut());
        }
        {
            {
                let a0_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a0_0.borrow_mut());
            }
            {
                let a1_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1_0.borrow_mut());
                let a1_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1_1.borrow_mut());
            }
            {
                let a2_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_0.borrow_mut());
                let a2_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_1.borrow_mut());
                let a2_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_2.borrow_mut());
            }
            {
                let a3_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_0.borrow_mut());
                let a3_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_1.borrow_mut());
                let a3_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_2.borrow_mut());
                let a3_3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_3.borrow_mut());
            }
            let nodefault: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*nodefault.borrow_mut());
        }
        {
            {
                let a0_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a0_0.borrow_mut());
            }
            {
                let a1_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1_0.borrow_mut());
                let a1_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1_1.borrow_mut());
            }
            {
                let a2_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_0.borrow_mut());
                let a2_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_1.borrow_mut());
                let a2_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_2.borrow_mut());
            }
            {
                let a3_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_0.borrow_mut());
                let a3_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_1.borrow_mut());
                let a3_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_2.borrow_mut());
                let a3_3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_3.borrow_mut());
            }
            let nodefault: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*nodefault.borrow_mut());
        }
        {
            {
                let a0_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a0_0.borrow_mut());
            }
            {
                let a1_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1_0.borrow_mut());
                let a1_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1_1.borrow_mut());
            }
            {
                let a2_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_0.borrow_mut());
                let a2_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_1.borrow_mut());
                let a2_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_2.borrow_mut());
            }
            {
                let a3_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_0.borrow_mut());
                let a3_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_1.borrow_mut());
                let a3_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_2.borrow_mut());
                let a3_3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_3.borrow_mut());
            }
            let nodefault: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*nodefault.borrow_mut());
        }
    }
    fn operator_call_1_struct_NonEmptyNonTrivial(&self) {
        {
            let a0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*a0.borrow_mut());
            {
                let a1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1.borrow_mut());
                let a2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2.borrow_mut());
                let a3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3.borrow_mut());
            }
            let nodefault: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*nodefault.borrow_mut());
        }
        {
            let a0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*a0.borrow_mut());
            let nodefault: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*nodefault.borrow_mut());
        }
        {
            {
                let a0_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a0_0.borrow_mut());
            }
            {
                let a1_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1_0.borrow_mut());
                let a1_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1_1.borrow_mut());
            }
            {
                let a2_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_0.borrow_mut());
                let a2_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_1.borrow_mut());
                let a2_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_2.borrow_mut());
            }
            {
                let a3_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_0.borrow_mut());
                let a3_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_1.borrow_mut());
                let a3_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_2.borrow_mut());
                let a3_3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_3.borrow_mut());
            }
            let nodefault: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*nodefault.borrow_mut());
        }
        {
            {
                let a0_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a0_0.borrow_mut());
            }
            {
                let a1_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1_0.borrow_mut());
                let a1_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1_1.borrow_mut());
            }
            {
                let a2_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_0.borrow_mut());
                let a2_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_1.borrow_mut());
                let a2_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_2.borrow_mut());
            }
            {
                let a3_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_0.borrow_mut());
                let a3_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_1.borrow_mut());
                let a3_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_2.borrow_mut());
                let a3_3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_3.borrow_mut());
            }
            let nodefault: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*nodefault.borrow_mut());
        }
        {
            {
                let a0_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a0_0.borrow_mut());
            }
            {
                let a1_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1_0.borrow_mut());
                let a1_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a1_1.borrow_mut());
            }
            {
                let a2_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_0.borrow_mut());
                let a2_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_1.borrow_mut());
                let a2_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a2_2.borrow_mut());
            }
            {
                let a3_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_0.borrow_mut());
                let a3_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_1.borrow_mut());
                let a3_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_2.borrow_mut());
                let a3_3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                &(*a3_3.borrow_mut());
            }
            let nodefault: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*nodefault.borrow_mut());
        }
    }
}
pub fn __cpp2rust_init_globals() {}
