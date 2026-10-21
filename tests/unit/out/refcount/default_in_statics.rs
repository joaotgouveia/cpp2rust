extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Inner {
    #[offset(0)]
    pub v: (),
    #[offset(8)]
    #[byte_size(8)]
    pub name: Ptr<()>,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(88)]
pub struct Outer {
    #[offset(0)]
    #[byte_size(8)]
    pub p1: Ptr<()>,
    #[offset(8)]
    #[byte_size(8)]
    pub p2: Ptr<()>,
    #[offset(16)]
    #[byte_size(24)]
    pub arr: Box<[Ptr<()>]>,
    #[offset(40)]
    #[byte_size(8)]
    pub cp: Ptr<()>,
    #[offset(48)]
    #[byte_size(8)]
    pub pp: Ptr<Ptr<()>>,
    #[offset(56)]
    #[byte_size(16)]
    pub inner: Inner,
    #[offset(72)]
    pub x: (),
    #[offset(80)]
    #[byte_size(8)]
    pub fn_: FnPtr<fn(()) -> ()>,
}
impl Default for Outer {
    fn default() -> Self {
        {
            Outer {
                p1: Ptr::<()>::null(),
                p2: Ptr::<()>::null(),
                arr: (0..3)
                    .map(|_| Ptr::<()>::null())
                    .collect::<Box<[Ptr<()>]>>(),
                cp: Ptr::<()>::null(),
                pp: Ptr::<Ptr<()>>::null(),
                inner: <Inner>::default(),
                x: unimplemented!(),
                fn_: FnPtr::<fn(()) -> ()>::null(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(40)]
pub struct Foo {
    #[offset(0)]
    #[byte_size(8)]
    pub s1: Ptr<()>,
    #[offset(8)]
    #[byte_size(8)]
    pub s2: Ptr<()>,
    #[offset(16)]
    #[byte_size(8)]
    pub fn1: FnPtr<fn(()) -> ()>,
    #[offset(24)]
    #[byte_size(8)]
    pub fn2: FnPtr<fn(()) -> ()>,
    #[offset(32)]
    pub n: (),
}
thread_local!(
    pub static static_fn_0: Value<FnPtr<fn(()) -> ()>> =
        Rc::new(RefCell::new(FnPtr::<fn(()) -> ()>::null()));
);
thread_local!(
    pub static static_outer_1: Value<Outer> = Rc::new(RefCell::new(<Outer>::default()));
);
thread_local!(
    pub static static_inner_array_2: Value<Box<[Inner]>> = Rc::new(RefCell::new(
        (0..2).map(|_| <Inner>::default()).collect::<Box<[Inner]>>(),
    ));
);
thread_local!(
    pub static static_foo_3: Value<Foo> = Rc::new(RefCell::new(Foo {
        s1: Ptr::<()>::from_string_literal(b"hello"),
        s2: Ptr::<()>::null(),
        fn1: FnPtr::<fn(()) -> ()>::null(),
        fn2: FnPtr::<fn(()) -> ()>::null(),
        n: 42_(),
    }));
);
thread_local!(
    pub static static_foo_array_4: Value<Box<[Foo]>> = Rc::new(RefCell::new(Box::new([
        Foo {
            s1: Ptr::<()>::from_string_literal(b"first"),
            s2: Ptr::<()>::null(),
            fn1: FnPtr::<fn(()) -> ()>::null(),
            fn2: FnPtr::<fn(()) -> ()>::null(),
            n: 1_(),
        },
        Foo {
            s1: Ptr::<()>::from_string_literal(b"second"),
            s2: Ptr::<()>::null(),
            fn1: FnPtr::<fn(()) -> ()>::null(),
            fn2: FnPtr::<fn(()) -> ()>::null(),
            n: 2_(),
        },
    ])));
);
pub fn check_local_static_5() {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
thread_local!(
    static local_outer_6: Value<Outer> = Rc::new(RefCell::new(<Outer>::default()));
);
thread_local!(
    static local_fn_7: Value<FnPtr<fn(()) -> ()>> =
        Rc::new(RefCell::new(FnPtr::<fn(()) -> ()>::null()));
);
thread_local!(
    static local_p_8: Value<Ptr<()>> = Rc::new(RefCell::new(Ptr::<()>::null()));
);
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < 3_()) {
        unimplemented!();
        i.prefix_inc();
    }
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < 2_()) {
        unimplemented!();
        i.prefix_inc();
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < 2_()) {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        i.prefix_inc();
    }
    ({ check_local_static_5() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {
    let _ = static_fn_0.with(|_| ());
    let _ = static_outer_1.with(|_| ());
    let _ = static_inner_array_2.with(|_| ());
    let _ = static_foo_3.with(|_| ());
    let _ = static_foo_array_4.with(|_| ());
}
