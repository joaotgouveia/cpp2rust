extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_optional_std_string___std_optional_std_string__ {}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_optional_TestT___std_optional_TestT__ {}
thread_local!(
    pub static value_2: Value<()> = Rc::new(RefCell::new(true));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct mandate_same_std_optional_std_string___std_optional_std_string__ {}
thread_local!(
    pub static can_make_optional_explicit_impl_3: Value<()> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static can_make_optional_explicit_impl_4: Value<()> =
        Rc::new(RefCell::new(value_2.with(|rc| *rc.borrow())));
);
thread_local!(
    pub static can_make_optional_explicit_impl_5: Value<()> =
        Rc::new(RefCell::new(value_2.with(|rc| *rc.borrow())));
);
thread_local!(
    pub static can_make_optional_explicit_impl_6: Value<()> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static can_make_optional_explicit_7: Value<()> = Rc::new(RefCell::new(
        can_make_optional_explicit_impl_3.with(|rc| *rc.borrow()),
    ));
);
thread_local!(
    pub static can_make_optional_explicit_8: Value<()> = Rc::new(RefCell::new(
        can_make_optional_explicit_impl_4.with(|rc| *rc.borrow()),
    ));
);
thread_local!(
    pub static can_make_optional_explicit_9: Value<()> = Rc::new(RefCell::new(
        can_make_optional_explicit_impl_5.with(|rc| *rc.borrow()),
    ));
);
thread_local!(
    pub static can_make_optional_explicit_10: Value<()> = Rc::new(RefCell::new(
        can_make_optional_explicit_impl_6.with(|rc| *rc.borrow()),
    ));
);
const _: () = assert!(
    true,
    "!can_make_optional_explicit<int, std::initializer_list<int>&>"
);
const _: () = assert!(
    true,
    "can_make_optional_explicit<std::string, std::initializer_list<char>&>"
);
const _: () = assert!(
    true,
    "can_make_optional_explicit<std::string, std::initializer_list<char>&, std::allocator<char>>"
);
const _: () = assert!(
    true,
    "!can_make_optional_explicit<std::string, std::initializer_list<std::string>&>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct TestT {
    #[offset(0)]
    pub x: (),
    #[offset(4)]
    pub size: (),
    #[offset(8)]
    #[byte_size(8)]
    pub ptr: Ptr<()>,
}
impl TestT {
    pub fn new_1(il: ()) -> Self {
        let il: Value<()> = Rc::new(RefCell::new(il));
        Self {
            x: (unimplemented!().read()),
            size: unimplemented!(),
            ptr: Ptr::<()>::null(),
        }
    }
    pub fn new_2(il: (), mut p: Ptr<()>) -> Self {
        let il: Value<()> = Rc::new(RefCell::new(il));
        Self {
            x: (unimplemented!().read()),
            size: unimplemented!(),
            ptr: (p).clone(),
        }
    }
}
pub fn test_11() -> () {
    {
        let opt: Value<()> = Rc::new(RefCell::new(({ make_optional_12(unimplemented!()) })));
        const _: () = assert!(true, "");;
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let i: Value<()> = Rc::new(RefCell::new(42_()));
        let opt: Value<()> = Rc::new(RefCell::new(
            ({
                let ___args: Value<Ptr<()>> = Rc::new(RefCell::new((i.as_pointer())));
                make_optional_13(unimplemented!(), ___args.as_pointer())
            }),
        ));
        const _: () = assert!(true, "");;
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    let test_string: Value<FnPtr<fn()>> = Rc::new(RefCell::new(FnPtr::<fn()>::new(|| {
        {
            let opt: Value<()> = Rc::new(RefCell::new(({ make_optional_14(unimplemented!()) })));
            unimplemented!();
        }
        {
            let opt: Value<()> = Rc::new(RefCell::new(
                ({
                    let ___args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                    make_optional_15(unimplemented!(), ___args.as_pointer())
                }),
            ));
            unimplemented!();
        }
    })));
    {
        ({ (*test_string.borrow()).call() });
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
    ({ test_11() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub fn __cpp2rust_init_globals() {
    let _ = value_2.with(|_| ());
    let _ = can_make_optional_explicit_impl_3.with(|_| ());
    let _ = can_make_optional_explicit_impl_4.with(|_| ());
    let _ = can_make_optional_explicit_impl_5.with(|_| ());
    let _ = can_make_optional_explicit_impl_6.with(|_| ());
    let _ = can_make_optional_explicit_7.with(|_| ());
    let _ = can_make_optional_explicit_8.with(|_| ());
    let _ = can_make_optional_explicit_9.with(|_| ());
    let _ = can_make_optional_explicit_10.with(|_| ());
}
