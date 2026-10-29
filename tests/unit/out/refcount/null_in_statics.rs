extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static p_mut_0: Value<Ptr<()>> = Rc::new(RefCell::new(Ptr::<()>::null()));
);
thread_local!(
    pub static p_const_1: Value<Ptr<()>> = Rc::new(RefCell::new(Ptr::<()>::null()));
);
thread_local!(
    pub static cp_2: Value<Ptr<()>> = Rc::new(RefCell::new(Ptr::<()>::null()));
);
thread_local!(
    pub static arr_of_ptr_3: Value<Box<[Ptr<()>]>> = Rc::new(RefCell::new(
        (0..4)
            .map(|_| Ptr::<()>::null())
            .collect::<Box<[Ptr<()>]>>(),
    ));
);
thread_local!(
    pub static pp_4: Value<Ptr<Ptr<()>>> = Rc::new(RefCell::new(Ptr::<Ptr<()>>::null()));
);
thread_local!(
    pub static const_arr_of_ptr_5: Value<Box<[Ptr<()>]>> = Rc::new(RefCell::new(
        (0..3)
            .map(|_| Ptr::<()>::null())
            .collect::<Box<[Ptr<()>]>>(),
    ));
);
thread_local!(
    pub static cp_explicit_null_6: Value<Ptr<()>> = Rc::new(RefCell::new(Ptr::<()>::null()));
);
thread_local!(
    pub static p_zero_7: Value<Ptr<()>> = Rc::new(RefCell::new(Ptr::<()>::null()));
);
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < 4_()) {
        unimplemented!();
        i.prefix_inc();
    }
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < 3_()) {
        unimplemented!();
        i.prefix_inc();
    }
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {
    let _ = p_mut_0.with(|_| ());
    let _ = p_const_1.with(|_| ());
    let _ = cp_2.with(|_| ());
    let _ = arr_of_ptr_3.with(|_| ());
    let _ = pp_4.with(|_| ());
    let _ = const_arr_of_ptr_5.with(|_| ());
    let _ = cp_explicit_null_6.with(|_| ());
    let _ = p_zero_7.with(|_| ());
}
