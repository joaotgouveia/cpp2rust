extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_time_0() {
    let mut t1: () = unimplemented!();
    let t2: Value<()> = Rc::new(RefCell::new(0_()));
    let mut t3: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn print_tm_1(t: ()) {
    let t: Value<()> = Rc::new(RefCell::new(t));
    let tm: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    println!(
        "{}-{}-{} {}:{}:{} wday={} yday={} {} gmtoff={} isdst={}",
        { (*tm.borrow()).tm_year },
        { (*tm.borrow()).tm_mon },
        { (*tm.borrow()).tm_mday },
        { (*tm.borrow()).tm_hour },
        { (*tm.borrow()).tm_min },
        { (*tm.borrow()).tm_sec },
        { (*tm.borrow()).tm_wday },
        { (*tm.borrow()).tm_yday },
        { (*tm.borrow()).tm_zone.clone() },
        { (*tm.borrow()).tm_gmtoff },
        { (*tm.borrow()).tm_isdst }
    );
}
pub fn test_gmtime_r_2() {
    ({ print_tm_1(0_()) });
    ({ print_tm_1(1_()) });
    ({ print_tm_1(86399_()) });
    ({ print_tm_1(86400_()) });
    ({ print_tm_1(951782400_()) });
    ({ print_tm_1(951868799_()) });
    ({ print_tm_1(1704067199_()) });
    ({ print_tm_1(1704067200_()) });
    ({ print_tm_1(1721126096_()) });
    ({ print_tm_1(4102444800_()) });
}
pub fn print_local_tm_3(t: ()) {
    let t: Value<()> = Rc::new(RefCell::new(t));
    let tm: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    println!(
        "{}-{}-{} {}:{}:{} wday={} yday={} {} gmtoff={} isdst={}",
        { (*tm.borrow()).tm_year },
        { (*tm.borrow()).tm_mon },
        { (*tm.borrow()).tm_mday },
        { (*tm.borrow()).tm_hour },
        { (*tm.borrow()).tm_min },
        { (*tm.borrow()).tm_sec },
        { (*tm.borrow()).tm_wday },
        { (*tm.borrow()).tm_yday },
        { (*tm.borrow()).tm_zone.clone() },
        { (*tm.borrow()).tm_gmtoff },
        { (*tm.borrow()).tm_isdst }
    );
}
pub fn test_localtime_r_4() {
    ({ print_local_tm_3(0_()) });
    ({ print_local_tm_3(951782400_()) });
    ({ print_local_tm_3(1704067199_()) });
    ({ print_local_tm_3(1721126096_()) });
    ({ print_local_tm_3(1735689600_()) });
}
pub fn test_strftime_5() {
    let t: Value<()> = Rc::new(RefCell::new(1721126096_()));
    let tm: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    let buf: Value<Box<[()]>> = Rc::new(RefCell::new(
        (0..64).map(|_| unimplemented!()).collect::<Box<[()]>>(),
    ));
    unimplemented!();
    println!("{}", (buf.as_pointer() as Ptr::<()>));
    unimplemented!();
    println!("{}", (buf.as_pointer() as Ptr::<()>));
    unimplemented!();
    println!("{}", (buf.as_pointer() as Ptr::<()>));
    unimplemented!();
    println!("{}", (buf.as_pointer() as Ptr::<()>));
    let small: Value<Box<[()]>> = Rc::new(RefCell::new(
        (0..4).map(|_| unimplemented!()).collect::<Box<[()]>>(),
    ));
    unimplemented!();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    ({ test_time_0() });
    ({ test_gmtime_r_2() });
    ({ test_localtime_r_4() });
    ({ test_strftime_5() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
