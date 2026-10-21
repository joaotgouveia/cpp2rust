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
        field!(o, v).write(unimplemented!());
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
        Self {
            mark: unimplemented!(),
        }
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
            copies: unimplemented!(),
            moves: unimplemented!(),
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
        field!(o, v).write(unimplemented!());
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
            copies: unimplemented!(),
            moves: unimplemented!(),
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
        field!(o, v).write(unimplemented!());
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
    let mut m: MoveOnly = MoveOnly::new({ v });
    return MoveOnly::move_from({ unimplemented!() });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut a: MoveOnly = MoveOnly::new({ 1_() });
    let mut b: MoveOnly = MoveOnly::move_from({ unimplemented!() });
    unimplemented!();
    unimplemented!();
    let mut c: MoveOnly = MoveOnly::move_from({ unimplemented!() });
    unimplemented!();
    unimplemented!();
    let mut d: MoveOnly = MoveOnly::move_from({ unimplemented!() });
    unimplemented!();
    unimplemented!();
    let mut e: MoveOnly = ({ make_1(5_()) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut vec_: () = unimplemented!();
    unimplemented!();
    let mut f: MoveOnly = MoveOnly::new({ 8_() });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut m: ConstMove = ConstMove::new();
    let mut m1: ConstMove = ConstMove::new_1({ unimplemented!() });
    let mut cm: ConstMove = ConstMove::new();
    let mut m2: ConstMove = ConstMove::new_2({ unimplemented!() });
    unimplemented!();
    unimplemented!();
    let mut t: ThrowingMove = ThrowingMove::new({ 1_() });
    let mut t1: ThrowingMove = ThrowingMove::copy_from({ unimplemented!() });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut n: NoexceptMove = NoexceptMove::new({ 2_() });
    let mut n1: NoexceptMove = NoexceptMove::move_from({ unimplemented!() });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut g: MoveOnly = MoveOnly::new({ 3_() });
    let mut g1: MoveOnly = MoveOnly::move_from({ unimplemented!() });
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
