extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct SelfPtr {
    #[offset(0)]
    pub value_: i32,
    #[offset(8)]
    #[byte_size(8)]
    pub this_: Ptr<SelfPtr>,
}
impl Default for SelfPtr {
    fn default() -> Self {
        {
            SelfPtr {
                value_: 0_i32,
                this_: (*self).clone(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Alloc {
    #[offset(0)]
    pub id: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
pub struct RefToMember {
    #[offset(0)]
    #[byte_size(4)]
    pub a_: Alloc,
    #[offset(8)]
    #[byte_size(8)]
    pub alloc_: Ptr<Alloc>,
    #[offset(16)]
    pub constructed_: bool,
}
impl Default for RefToMember {
    fn default() -> Self {
        {
            RefToMember {
                a_: Alloc { id: 7 },
                alloc_: field_ptr!((*self), a_),
                constructed_: false,
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct CtorSelfPtr {
    #[offset(0)]
    pub value_: i32,
    #[offset(8)]
    #[byte_size(8)]
    pub this_: Ptr<CtorSelfPtr>,
}
impl CtorSelfPtr {
    pub fn new(mut v: i32) -> Self {
        let __this: Value<CtorSelfPtr> = Rc::new(RefCell::new(Self {
            value_: v,
            this_: (this).clone(),
        }));
        let this: Ptr<CtorSelfPtr> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct CtorReadsMember {
    #[offset(0)]
    pub size: i32,
    #[offset(4)]
    pub doubled: i32,
}
impl CtorReadsMember {
    pub fn new(mut s: i32) -> Self {
        let __this: Value<CtorReadsMember> = Rc::new(RefCell::new(Self {
            size: s,
            doubled: (this.with(|__s| __s.size) * 2),
        }));
        let this: Ptr<CtorReadsMember> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct list_head {
    #[offset(0)]
    #[byte_size(8)]
    pub next: Ptr<list_head>,
}
pub fn f_0() -> bool {
    let list: Value<list_head> = <Value<list_head>>::default();
    *list.borrow_mut() = list_head {
        next: (list.as_pointer()),
    };
    return ({ { (*list.borrow()).next.clone() } } == { (list.as_pointer()) });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<SelfPtr> = Rc::new(RefCell::new(<SelfPtr>::default()));
    (if ({ { (*s.borrow()).this_.clone() } } == { (s.as_pointer()) }) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"s.this_ == &s"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({ (*s.borrow()).value_ } == 0) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"s.value_ == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let r: Value<RefToMember> = Rc::new(RefCell::new(<RefToMember>::default()));
    (if ({ ({ (*r.borrow()).alloc_.clone() }).clone() } == { (field_ptr!(r.as_pointer(), a_)) }) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"&r.alloc_ == &r.a_"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({ (*r.borrow()).alloc_.clone() }.with(|__s| __s.id) == 7) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"r.alloc_.id == 7"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if !({ (*r.borrow()).constructed_ }) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"!r.constructed_"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let c: Value<CtorSelfPtr> = Rc::new(RefCell::new(CtorSelfPtr::new({ 3 })));
    (if ({ { (*c.borrow()).this_.clone() } } == { (c.as_pointer()) }) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"c.this_ == &c"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({ (*c.borrow()).value_ } == 3) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"c.value_ == 3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut m: CtorReadsMember = CtorReadsMember::new({ 4 });
    (if (m.doubled == 8) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"m.doubled == 8"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({ f_0() }) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"f()"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    return 0;
}
pub fn __assert_fail_1(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {}
