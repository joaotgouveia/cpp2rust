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
    pub val: (),
    #[offset(8)]
    #[byte_size(8)]
    pub tag: Ptr<()>,
}
impl Tag {
    pub fn new_1() -> Self {
        Self {
            val: 0_(),
            tag: Ptr::<()>::from_string_literal(b"Default"),
        }
    }
    pub fn new_2(mut a: (), mut b: Ptr<()>) -> Self {
        Self {
            val: a,
            tag: (b).clone(),
        }
    }
}
impl std::cmp::Ord for Tag {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if operator_lt_0(
                Rc::new(RefCell::new(Tag {
                    val: self.val.clone(),
                    tag: self.tag.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(Tag {
                    val: other.val.clone(),
                    tag: other.tag.clone(),
                }))
                .as_pointer(),
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_0(
                Rc::new(RefCell::new(Tag {
                    val: other.val.clone(),
                    tag: other.tag.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(Tag {
                    val: self.val.clone(),
                    tag: self.tag.clone(),
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
impl std::cmp::PartialOrd for Tag {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Tag {
    fn eq(&self, other: &Self) -> bool {
        {
            !(operator_lt_0(
                Rc::new(RefCell::new(Tag {
                    val: self.val.clone(),
                    tag: self.tag.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(Tag {
                    val: other.val.clone(),
                    tag: other.tag.clone(),
                }))
                .as_pointer(),
            )) && !(operator_lt_0(
                Rc::new(RefCell::new(Tag {
                    val: other.val.clone(),
                    tag: other.tag.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(Tag {
                    val: self.val.clone(),
                    tag: self.tag.clone(),
                }))
                .as_pointer(),
            ))
        }
    }
}
impl std::cmp::Eq for Tag {}
impl Default for Tag {
    fn default() -> Self {
        { Tag::new_1() }
    }
}
pub fn eq_1(rhs: Ptr<Tag>, lhs: Ptr<Tag>) -> () {
    return ({ rhs.with(|__s| __s.val) } == { lhs.with(|__s| __s.val) })
        && ({ rhs.with(|__s| __s.tag.clone()) } == { lhs.with(|__s| __s.tag.clone()) });
}
pub fn operator_lt_0(rhs: Ptr<Tag>, lhs: Ptr<Tag>) -> () {
    return ({ rhs.with(|__s| __s.val) } < { lhs.with(|__s| __s.val) });
}
pub fn test_2(a: Ptr<()>, lo: Ptr<()>, hi: Ptr<()>, x: Ptr<()>) {
    unimplemented!();
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
        let x: Value<()> = Rc::new(RefCell::new(0_()));
        let y: Value<()> = Rc::new(RefCell::new(0_()));
        let z: Value<()> = Rc::new(RefCell::new(0_()));
        ({
            let _a: Ptr<()> = x.as_pointer();
            let _x: Ptr<()> = x.as_pointer();
            test_2(_a, y.as_pointer(), z.as_pointer(), _x)
        });
        ({
            let _a: Ptr<()> = y.as_pointer();
            let _x: Ptr<()> = y.as_pointer();
            test_2(_a, x.as_pointer(), z.as_pointer(), _x)
        });
    }
    {
        let x: Value<()> = Rc::new(RefCell::new(0_()));
        let y: Value<()> = Rc::new(RefCell::new(1_()));
        let z: Value<()> = Rc::new(RefCell::new(2_()));
        ({
            let _lo: Ptr<()> = y.as_pointer();
            let _x: Ptr<()> = y.as_pointer();
            test_2(x.as_pointer(), _lo, z.as_pointer(), _x)
        });
        ({
            let _a: Ptr<()> = y.as_pointer();
            let _x: Ptr<()> = y.as_pointer();
            test_2(_a, x.as_pointer(), z.as_pointer(), _x)
        });
    }
    {
        let x: Value<()> = Rc::new(RefCell::new(1_()));
        let y: Value<()> = Rc::new(RefCell::new(0_()));
        let z: Value<()> = Rc::new(RefCell::new(1_()));
        ({
            let _a: Ptr<()> = x.as_pointer();
            let _x: Ptr<()> = x.as_pointer();
            test_2(_a, y.as_pointer(), z.as_pointer(), _x)
        });
        ({
            let _lo: Ptr<()> = x.as_pointer();
            let _x: Ptr<()> = x.as_pointer();
            test_2(y.as_pointer(), _lo, z.as_pointer(), _x)
        });
    }
    {
        let x: Value<Tag> = Rc::new(RefCell::new(Tag::new_2({ 0_() }, {
            Ptr::<()>::from_string_literal(b"Zero-x")
        })));
        let _dtor_x = ScopedDestructor::new(&x, |__p| __p.destructor());
        let y: Value<Tag> = Rc::new(RefCell::new(Tag::new_2({ 0_() }, {
            Ptr::<()>::from_string_literal(b"Zero-y")
        })));
        let _dtor_y = ScopedDestructor::new(&y, |__p| __p.destructor());
        let z: Value<Tag> = Rc::new(RefCell::new(Tag::new_2({ 0_() }, {
            Ptr::<()>::from_string_literal(b"Zero-z")
        })));
        let _dtor_z = ScopedDestructor::new(&z, |__p| __p.destructor());
        unimplemented!();
        unimplemented!();
    }
    {
        let x: Value<Tag> = Rc::new(RefCell::new(Tag::new_2({ 0_() }, {
            Ptr::<()>::from_string_literal(b"Zero-x")
        })));
        let _dtor_x = ScopedDestructor::new(&x, |__p| __p.destructor());
        let y: Value<Tag> = Rc::new(RefCell::new(Tag::new_2({ 0_() }, {
            Ptr::<()>::from_string_literal(b"Zero-y")
        })));
        let _dtor_y = ScopedDestructor::new(&y, |__p| __p.destructor());
        let z: Value<Tag> = Rc::new(RefCell::new(Tag::new_2({ 1_() }, {
            Ptr::<()>::from_string_literal(b"One-z")
        })));
        let _dtor_z = ScopedDestructor::new(&z, |__p| __p.destructor());
        unimplemented!();
        unimplemented!();
    }
    {
        let x: Value<Tag> = Rc::new(RefCell::new(Tag::new_2({ 1_() }, {
            Ptr::<()>::from_string_literal(b"One-x")
        })));
        let _dtor_x = ScopedDestructor::new(&x, |__p| __p.destructor());
        let y: Value<Tag> = Rc::new(RefCell::new(Tag::new_2({ 0_() }, {
            Ptr::<()>::from_string_literal(b"Zero-y")
        })));
        let _dtor_y = ScopedDestructor::new(&y, |__p| __p.destructor());
        let z: Value<Tag> = Rc::new(RefCell::new(Tag::new_2({ 1_() }, {
            Ptr::<()>::from_string_literal(b"One-z")
        })));
        let _dtor_z = ScopedDestructor::new(&z, |__p| __p.destructor());
        unimplemented!();
        unimplemented!();
    }
    {
        let x: Value<Tag> = Rc::new(RefCell::new(Tag::new_2({ 1_() }, {
            Ptr::<()>::from_string_literal(b"One-x")
        })));
        let _dtor_x = ScopedDestructor::new(&x, |__p| __p.destructor());
        let y: Value<Tag> = Rc::new(RefCell::new(Tag::new_2({ 0_() }, {
            Ptr::<()>::from_string_literal(b"Zero-y")
        })));
        let _dtor_y = ScopedDestructor::new(&y, |__p| __p.destructor());
        let z: Value<Tag> = Rc::new(RefCell::new(Tag::new_2({ 2_() }, {
            Ptr::<()>::from_string_literal(b"Two-z")
        })));
        let _dtor_z = ScopedDestructor::new(&z, |__p| __p.destructor());
        unimplemented!();
        unimplemented!();
    }
    {
        let x: Value<Tag> = Rc::new(RefCell::new(Tag::new_2({ 0_() }, {
            Ptr::<()>::from_string_literal(b"Zero-x")
        })));
        let _dtor_x = ScopedDestructor::new(&x, |__p| __p.destructor());
        let y: Value<Tag> = Rc::new(RefCell::new(Tag::new_2({ 1_() }, {
            Ptr::<()>::from_string_literal(b"One-y")
        })));
        let _dtor_y = ScopedDestructor::new(&y, |__p| __p.destructor());
        let z: Value<Tag> = Rc::new(RefCell::new(Tag::new_2({ 2_() }, {
            Ptr::<()>::from_string_literal(b"Two-z")
        })));
        let _dtor_z = ScopedDestructor::new(&z, |__p| __p.destructor());
        unimplemented!();
        unimplemented!();
    }
    {
        let x: Value<Tag> = Rc::new(RefCell::new(Tag::new_2({ 2_() }, {
            Ptr::<()>::from_string_literal(b"Two-x")
        })));
        let _dtor_x = ScopedDestructor::new(&x, |__p| __p.destructor());
        let y: Value<Tag> = Rc::new(RefCell::new(Tag::new_2({ 0_() }, {
            Ptr::<()>::from_string_literal(b"Zero-y")
        })));
        let _dtor_y = ScopedDestructor::new(&y, |__p| __p.destructor());
        let z: Value<Tag> = Rc::new(RefCell::new(Tag::new_2({ 1_() }, {
            Ptr::<()>::from_string_literal(b"One-z")
        })));
        let _dtor_z = ScopedDestructor::new(&z, |__p| __p.destructor());
        unimplemented!();
        unimplemented!();
    }
    {
        let mut x: () = 1_();
        let mut y: () = 0_();
        let mut z: () = 1_();
        const _: () = assert!(true, "std::clamp(x, y, z) == x");;
        const _: () = assert!(true, "std::clamp(y, x, z) == x");;
    }
    return 0_();
}
pub trait TagImpl {
    fn destructor(&self);
}
impl TagImpl for Ptr<Tag> {
    fn destructor(&self) {}
}
pub fn __cpp2rust_init_globals() {}
