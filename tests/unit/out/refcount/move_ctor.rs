extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, Default)]
#[byte_size(4)]
pub struct MoveOnly {
    #[offset(0)]
    pub v: (),
}
impl MoveOnly {
    pub fn new(mut v: ()) -> Self {
        Self { v: v }
    }
    pub fn move_from(o: Ptr<MoveOnly>) -> Self {
        let __this: MoveOnly = Self {
            v: o.with(|__s| __s.v),
        };
        field!(o, v).write(0_());
        __this
    }
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(4)]
pub struct ConstMove {
    #[offset(0)]
    pub mark: (),
}
impl ConstMove {
    pub fn new() -> Self {
        Self { mark: 0_() }
    }
    pub fn new_1(o: Ptr<ConstMove>) -> Self {
        Self {
            mark: (o.with(|__s| __s.mark) + 1_()),
        }
    }
    pub fn new_2(o: Ptr<ConstMove>) -> Self {
        Self {
            mark: (o.with(|__s| __s.mark) + 10_()),
        }
    }
}
impl Default for ConstMove {
    fn default() -> Self {
        { ConstMove::new() }
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Default)]
#[byte_size(12)]
pub struct ThrowingMove {
    #[offset(0)]
    pub v: (),
    #[offset(4)]
    pub copies: (),
    #[offset(8)]
    pub moves: (),
}
impl ThrowingMove {
    pub fn new(mut v: ()) -> Self {
        Self {
            v: v,
            copies: 0_(),
            moves: 0_(),
        }
    }
    pub fn copy_from(o: Ptr<ThrowingMove>) -> Self {
        Self {
            v: o.with(|__s| __s.v),
            copies: (o.with(|__s| __s.copies) + 1_()),
            moves: o.with(|__s| __s.moves),
        }
    }
    pub fn move_from(o: Ptr<ThrowingMove>) -> Self {
        let __this: ThrowingMove = Self {
            v: o.with(|__s| __s.v),
            copies: o.with(|__s| __s.copies),
            moves: (o.with(|__s| __s.moves) + 1_()),
        };
        field!(o, v).write(0_());
        __this
    }
}
impl Clone for ThrowingMove {
    fn clone(&self) -> Self {
        let __src: Value<ThrowingMove> = Rc::new(RefCell::new(ThrowingMove {
            v: self.v.clone(),
            copies: self.copies.clone(),
            moves: self.moves.clone(),
        }));
        ThrowingMove::copy_from(__src.as_pointer())
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Default)]
#[byte_size(12)]
pub struct NoexceptMove {
    #[offset(0)]
    pub v: (),
    #[offset(4)]
    pub copies: (),
    #[offset(8)]
    pub moves: (),
}
impl NoexceptMove {
    pub fn new(mut v: ()) -> Self {
        Self {
            v: v,
            copies: 0_(),
            moves: 0_(),
        }
    }
    pub fn copy_from(o: Ptr<NoexceptMove>) -> Self {
        Self {
            v: o.with(|__s| __s.v),
            copies: (o.with(|__s| __s.copies) + 1_()),
            moves: o.with(|__s| __s.moves),
        }
    }
    pub fn move_from(o: Ptr<NoexceptMove>) -> Self {
        let __this: NoexceptMove = Self {
            v: o.with(|__s| __s.v),
            copies: o.with(|__s| __s.copies),
            moves: (o.with(|__s| __s.moves) + 1_()),
        };
        field!(o, v).write(0_());
        __this
    }
}
impl Clone for NoexceptMove {
    fn clone(&self) -> Self {
        let __src: Value<NoexceptMove> = Rc::new(RefCell::new(NoexceptMove {
            v: self.v.clone(),
            copies: self.copies.clone(),
            moves: self.moves.clone(),
        }));
        NoexceptMove::copy_from(__src.as_pointer())
    }
}
pub fn by_value_0(mut m: MoveOnly) -> () {
    return m.v;
}
pub fn make_1(mut v: ()) -> MoveOnly {
    let m: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ v })));
    return MoveOnly::move_from({ m.as_pointer() });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let a: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ 1_() })));
    let b: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::move_from({ a.as_pointer() })));
    unimplemented!();
    unimplemented!();
    let c: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::move_from({ b.as_pointer() })));
    unimplemented!();
    unimplemented!();
    let mut d: MoveOnly = MoveOnly::move_from({ c.as_pointer() });
    unimplemented!();
    unimplemented!();
    let e: Value<MoveOnly> = Rc::new(RefCell::new(({ make_1(5_()) })));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let vec_: Value<std_vector_MoveOnly__std_allocator_MoveOnly__> = Rc::new(RefCell::new(
        <std_vector_MoveOnly__std_allocator_MoveOnly__>::default(),
    ));
    ({
        let ___x: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ 7_() })));
        (*vec_.borrow()).push_back_58(___x.as_pointer())
    });
    let f: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ 8_() })));
    ({ (*vec_.borrow()).push_back_58(f.as_pointer()) });
    unimplemented!();
    unimplemented!();
    let m: Value<ConstMove> = Rc::new(RefCell::new(ConstMove::new()));
    let mut m1: ConstMove = ConstMove::new_1({ m.as_pointer() });
    let cm: Value<ConstMove> = Rc::new(RefCell::new(ConstMove::new()));
    let mut m2: ConstMove = ConstMove::new_2({ cm.as_pointer() });
    unimplemented!();
    unimplemented!();
    let t: Value<ThrowingMove> = Rc::new(RefCell::new(ThrowingMove::new({ 1_() })));
    let mut t1: ThrowingMove = ThrowingMove::copy_from({ t.as_pointer() });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let n: Value<NoexceptMove> = Rc::new(RefCell::new(NoexceptMove::new({ 2_() })));
    let mut n1: NoexceptMove = NoexceptMove::move_from({ n.as_pointer() });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let g: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ 3_() })));
    let mut g1: MoveOnly = MoveOnly::move_from({ g.as_pointer() });
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
