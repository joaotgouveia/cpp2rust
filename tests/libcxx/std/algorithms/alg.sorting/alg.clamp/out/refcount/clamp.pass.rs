extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(16)]
pub struct Tag {
    #[offset(0)]
    pub val: i32,
    #[offset(8)]
    #[byte_size(8)]
    pub tag: Ptr<i8>,
}
impl Tag {
    pub fn new_1() -> Self {
        Self {
            val: 0,
            tag: Ptr::<i8>::from_string_literal(b"Default"),
        }
    }
    pub fn new_2(mut a: i32, mut b: Ptr<i8>) -> Self {
        Self {
            val: a,
            tag: (b).clone(),
        }
    }
}
impl CppLess for Tag {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_0(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for Tag {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        !(operator_lt_0(__lhs.clone(), __rhs.clone()))
            && !(operator_lt_0(__rhs.clone(), __lhs.clone()))
    }
}
impl CppMove for Tag {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for Tag {
    fn default() -> Self {
        { Tag::new_1() }
    }
}
pub fn eq_1(rhs: Ptr<Tag>, lhs: Ptr<Tag>) -> bool {
    return ({ rhs.with(|__s| __s.val) } == { lhs.with(|__s| __s.val) })
        && ({ rhs.with(|__s| __s.tag.clone()) } == { lhs.with(|__s| __s.tag.clone()) });
}
pub fn operator_lt_0(rhs: Ptr<Tag>, lhs: Ptr<Tag>) -> bool {
    return ({ rhs.with(|__s| __s.val) } < { lhs.with(|__s| __s.val) });
}
pub fn test_2(a: Ptr<i32>, lo: Ptr<i32>, hi: Ptr<i32>, x: Ptr<i32>) {
    assert!(
        ({
            ({
                let ___val: Ptr<i32> = (a).clone();
                let ___lo: Ptr<i32> = (lo).clone();
                let ___hi: Ptr<i32> = (hi).clone();
                clamp_3(___val, ___lo, ___hi)
            })
        } == { (x).clone() })
    );
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
        let x: Value<i32> = Rc::new(RefCell::new(0));
        let y: Value<i32> = Rc::new(RefCell::new(0));
        let z: Value<i32> = Rc::new(RefCell::new(0));
        ({
            let _a: Ptr<i32> = x.as_pointer();
            let _x: Ptr<i32> = x.as_pointer();
            test_2(_a, y.as_pointer(), z.as_pointer(), _x)
        });
        ({
            let _a: Ptr<i32> = y.as_pointer();
            let _x: Ptr<i32> = y.as_pointer();
            test_2(_a, x.as_pointer(), z.as_pointer(), _x)
        });
    }
    {
        let x: Value<i32> = Rc::new(RefCell::new(0));
        let y: Value<i32> = Rc::new(RefCell::new(1));
        let z: Value<i32> = Rc::new(RefCell::new(2));
        ({
            let _lo: Ptr<i32> = y.as_pointer();
            let _x: Ptr<i32> = y.as_pointer();
            test_2(x.as_pointer(), _lo, z.as_pointer(), _x)
        });
        ({
            let _a: Ptr<i32> = y.as_pointer();
            let _x: Ptr<i32> = y.as_pointer();
            test_2(_a, x.as_pointer(), z.as_pointer(), _x)
        });
    }
    {
        let x: Value<i32> = Rc::new(RefCell::new(1));
        let y: Value<i32> = Rc::new(RefCell::new(0));
        let z: Value<i32> = Rc::new(RefCell::new(1));
        ({
            let _a: Ptr<i32> = x.as_pointer();
            let _x: Ptr<i32> = x.as_pointer();
            test_2(_a, y.as_pointer(), z.as_pointer(), _x)
        });
        ({
            let _lo: Ptr<i32> = x.as_pointer();
            let _x: Ptr<i32> = x.as_pointer();
            test_2(y.as_pointer(), _lo, z.as_pointer(), _x)
        });
    }
    {
        let x: Value<Tag> = Rc::new(RefCell::new(Tag::new_2({ 0 }, {
            Ptr::<i8>::from_string_literal(b"Zero-x")
        })));
        let _dtor_x = ScopedDestructor::new(&x, |__p| __p.destructor());
        let y: Value<Tag> = Rc::new(RefCell::new(Tag::new_2({ 0 }, {
            Ptr::<i8>::from_string_literal(b"Zero-y")
        })));
        let _dtor_y = ScopedDestructor::new(&y, |__p| __p.destructor());
        let z: Value<Tag> = Rc::new(RefCell::new(Tag::new_2({ 0 }, {
            Ptr::<i8>::from_string_literal(b"Zero-z")
        })));
        let _dtor_z = ScopedDestructor::new(&z, |__p| __p.destructor());
        assert!(
            ({
                let _rhs: Ptr<Tag> = ({ clamp_4(x.as_pointer(), y.as_pointer(), z.as_pointer()) });
                let _lhs: Ptr<Tag> = x.as_pointer();
                eq_1(_rhs, _lhs)
            })
        );
        assert!(
            ({
                let _rhs: Ptr<Tag> = ({ clamp_4(y.as_pointer(), x.as_pointer(), z.as_pointer()) });
                let _lhs: Ptr<Tag> = y.as_pointer();
                eq_1(_rhs, _lhs)
            })
        );
    }
    {
        let x: Value<Tag> = Rc::new(RefCell::new(Tag::new_2({ 0 }, {
            Ptr::<i8>::from_string_literal(b"Zero-x")
        })));
        let _dtor_x = ScopedDestructor::new(&x, |__p| __p.destructor());
        let y: Value<Tag> = Rc::new(RefCell::new(Tag::new_2({ 0 }, {
            Ptr::<i8>::from_string_literal(b"Zero-y")
        })));
        let _dtor_y = ScopedDestructor::new(&y, |__p| __p.destructor());
        let z: Value<Tag> = Rc::new(RefCell::new(Tag::new_2({ 1 }, {
            Ptr::<i8>::from_string_literal(b"One-z")
        })));
        let _dtor_z = ScopedDestructor::new(&z, |__p| __p.destructor());
        assert!(
            ({
                let _rhs: Ptr<Tag> = ({ clamp_4(x.as_pointer(), y.as_pointer(), z.as_pointer()) });
                let _lhs: Ptr<Tag> = x.as_pointer();
                eq_1(_rhs, _lhs)
            })
        );
        assert!(
            ({
                let _rhs: Ptr<Tag> = ({ clamp_4(y.as_pointer(), x.as_pointer(), z.as_pointer()) });
                let _lhs: Ptr<Tag> = y.as_pointer();
                eq_1(_rhs, _lhs)
            })
        );
    }
    {
        let x: Value<Tag> = Rc::new(RefCell::new(Tag::new_2({ 1 }, {
            Ptr::<i8>::from_string_literal(b"One-x")
        })));
        let _dtor_x = ScopedDestructor::new(&x, |__p| __p.destructor());
        let y: Value<Tag> = Rc::new(RefCell::new(Tag::new_2({ 0 }, {
            Ptr::<i8>::from_string_literal(b"Zero-y")
        })));
        let _dtor_y = ScopedDestructor::new(&y, |__p| __p.destructor());
        let z: Value<Tag> = Rc::new(RefCell::new(Tag::new_2({ 1 }, {
            Ptr::<i8>::from_string_literal(b"One-z")
        })));
        let _dtor_z = ScopedDestructor::new(&z, |__p| __p.destructor());
        assert!(
            ({
                let _rhs: Ptr<Tag> = ({ clamp_4(x.as_pointer(), y.as_pointer(), z.as_pointer()) });
                let _lhs: Ptr<Tag> = x.as_pointer();
                eq_1(_rhs, _lhs)
            })
        );
        assert!(
            ({
                let _rhs: Ptr<Tag> = ({ clamp_4(z.as_pointer(), y.as_pointer(), x.as_pointer()) });
                let _lhs: Ptr<Tag> = z.as_pointer();
                eq_1(_rhs, _lhs)
            })
        );
    }
    {
        let x: Value<Tag> = Rc::new(RefCell::new(Tag::new_2({ 1 }, {
            Ptr::<i8>::from_string_literal(b"One-x")
        })));
        let _dtor_x = ScopedDestructor::new(&x, |__p| __p.destructor());
        let y: Value<Tag> = Rc::new(RefCell::new(Tag::new_2({ 0 }, {
            Ptr::<i8>::from_string_literal(b"Zero-y")
        })));
        let _dtor_y = ScopedDestructor::new(&y, |__p| __p.destructor());
        let z: Value<Tag> = Rc::new(RefCell::new(Tag::new_2({ 2 }, {
            Ptr::<i8>::from_string_literal(b"Two-z")
        })));
        let _dtor_z = ScopedDestructor::new(&z, |__p| __p.destructor());
        assert!(
            ({
                let _rhs: Ptr<Tag> = ({ clamp_4(x.as_pointer(), y.as_pointer(), z.as_pointer()) });
                let _lhs: Ptr<Tag> = x.as_pointer();
                eq_1(_rhs, _lhs)
            })
        );
        assert!(
            ({
                let _rhs: Ptr<Tag> = ({ clamp_4(y.as_pointer(), x.as_pointer(), z.as_pointer()) });
                let _lhs: Ptr<Tag> = x.as_pointer();
                eq_1(_rhs, _lhs)
            })
        );
    }
    {
        let x: Value<Tag> = Rc::new(RefCell::new(Tag::new_2({ 0 }, {
            Ptr::<i8>::from_string_literal(b"Zero-x")
        })));
        let _dtor_x = ScopedDestructor::new(&x, |__p| __p.destructor());
        let y: Value<Tag> = Rc::new(RefCell::new(Tag::new_2({ 1 }, {
            Ptr::<i8>::from_string_literal(b"One-y")
        })));
        let _dtor_y = ScopedDestructor::new(&y, |__p| __p.destructor());
        let z: Value<Tag> = Rc::new(RefCell::new(Tag::new_2({ 2 }, {
            Ptr::<i8>::from_string_literal(b"Two-z")
        })));
        let _dtor_z = ScopedDestructor::new(&z, |__p| __p.destructor());
        assert!(
            ({
                let _rhs: Ptr<Tag> = ({ clamp_4(x.as_pointer(), y.as_pointer(), z.as_pointer()) });
                let _lhs: Ptr<Tag> = y.as_pointer();
                eq_1(_rhs, _lhs)
            })
        );
        assert!(
            ({
                let _rhs: Ptr<Tag> = ({ clamp_4(y.as_pointer(), x.as_pointer(), z.as_pointer()) });
                let _lhs: Ptr<Tag> = y.as_pointer();
                eq_1(_rhs, _lhs)
            })
        );
    }
    {
        let x: Value<Tag> = Rc::new(RefCell::new(Tag::new_2({ 2 }, {
            Ptr::<i8>::from_string_literal(b"Two-x")
        })));
        let _dtor_x = ScopedDestructor::new(&x, |__p| __p.destructor());
        let y: Value<Tag> = Rc::new(RefCell::new(Tag::new_2({ 0 }, {
            Ptr::<i8>::from_string_literal(b"Zero-y")
        })));
        let _dtor_y = ScopedDestructor::new(&y, |__p| __p.destructor());
        let z: Value<Tag> = Rc::new(RefCell::new(Tag::new_2({ 1 }, {
            Ptr::<i8>::from_string_literal(b"One-z")
        })));
        let _dtor_z = ScopedDestructor::new(&z, |__p| __p.destructor());
        assert!(
            ({
                let _rhs: Ptr<Tag> = ({ clamp_4(x.as_pointer(), y.as_pointer(), z.as_pointer()) });
                let _lhs: Ptr<Tag> = z.as_pointer();
                eq_1(_rhs, _lhs)
            })
        );
        assert!(
            ({
                let _rhs: Ptr<Tag> = ({ clamp_4(y.as_pointer(), z.as_pointer(), x.as_pointer()) });
                let _lhs: Ptr<Tag> = z.as_pointer();
                eq_1(_rhs, _lhs)
            })
        );
    }
    {
        let x: Value<i32> = Rc::new(RefCell::new(1));
        let y: Value<i32> = Rc::new(RefCell::new(0));
        let z: Value<i32> = Rc::new(RefCell::new(1));
        const _: () = assert!(true, "std::clamp(x, y, z) == x");;
        const _: () = assert!(true, "std::clamp(y, x, z) == x");;
    }
    return 0;
}
pub trait TagImpl {
    fn destructor(&self);
}
impl TagImpl for Ptr<Tag> {
    fn destructor(&self) {}
}
pub fn __cpp2rust_init_globals() {}
