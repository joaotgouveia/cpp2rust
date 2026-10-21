extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn fill_row_0(mut row: Ptr<()>, mut c: ()) {
    elem!(row, 0_()).write({ c });
    elem!(row, 1_()).write(('\0' as ()));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let grid: Value<Box<[Box<[()]>]>> = Rc::new(RefCell::new(
        (0..3)
            .map(|_| (0..6).map(|_| unimplemented!()).collect::<Box<[()]>>())
            .collect::<Box<[Box<[()]>]>>(),
    ));
    let mut i: () = 0_();
    'loop_: while (((i < 3_()) as ()) != 0) {
        ({
            let _row: Ptr<()> =
                (((grid.as_pointer() as Ptr<Box<[()]>>).offset(i).as_pointer()) as Ptr<()>);
            let _c: () = ((('a' as ()) + i) as ());
            fill_row_0(_row, _c)
        });
        i.postfix_inc();
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*grid.borrow_mut())[(2_()) as usize][(5_()) as usize] = ('z' as ());
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
