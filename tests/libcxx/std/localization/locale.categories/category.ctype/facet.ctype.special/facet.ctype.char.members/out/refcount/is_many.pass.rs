extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};

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
    let l: Value<std_locale> = Rc::new(RefCell::new(
        (*({ std_locale::classic() }).upgrade().deref()).clone(),
    ));
    {
        let f: Ptr<std_ctype_char_> = ({ use_facet_0(l.as_pointer()) });
        let in_: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b" A\x07.a1").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        let m: Value<Vec<u16>> = Rc::new(RefCell::new(
            (0..({ ((*in_.borrow()).len() - 1) }) as usize)
                .map(|_| 0_u16)
                .collect::<Vec<_>>(),
        ));
        let mut h: Ptr<i8> = ({
            let ___low: Ptr<i8> = in_.decay();
            let ___high: Ptr<i8> = in_.decay().offset(((*in_.borrow()).len() - 1) as isize);
            let ___vec: Ptr<u16> = m.decay();
            (*f.upgrade().deref()).is_4(___low, ___high, ___vec)
        });
        assert!(
            ({ ({ (h).clone() } == { in_.decay().offset(((*in_.borrow()).len() - 1) as isize) }) })
        );
        assert!(
            ((((m.decay().offset(0_usize as isize).read()) as i32)
                & (space_1.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            ((((m.decay().offset(0_usize as isize).read()) as i32)
                & (print_2.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            !((((m.decay().offset(0_usize as isize).read()) as i32)
                & (cntrl_3.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            !((((m.decay().offset(0_usize as isize).read()) as i32)
                & (upper_4.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            !((((m.decay().offset(0_usize as isize).read()) as i32)
                & (lower_5.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            !((((m.decay().offset(0_usize as isize).read()) as i32)
                & (alpha_6.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            !((((m.decay().offset(0_usize as isize).read()) as i32)
                & (digit_7.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            !((((m.decay().offset(0_usize as isize).read()) as i32)
                & (punct_8.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            !((((m.decay().offset(0_usize as isize).read()) as i32)
                & (xdigit_9.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            ((((m.decay().offset(0_usize as isize).read()) as i32)
                & (blank_10.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            !((((m.decay().offset(0_usize as isize).read()) as i32)
                & (alnum_11.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            !((((m.decay().offset(0_usize as isize).read()) as i32)
                & (graph_12.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            !((((m.decay().offset(1_usize as isize).read()) as i32)
                & (space_1.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            ((((m.decay().offset(1_usize as isize).read()) as i32)
                & (print_2.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            !((((m.decay().offset(1_usize as isize).read()) as i32)
                & (cntrl_3.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            ((((m.decay().offset(1_usize as isize).read()) as i32)
                & (upper_4.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            !((((m.decay().offset(1_usize as isize).read()) as i32)
                & (lower_5.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            ((((m.decay().offset(1_usize as isize).read()) as i32)
                & (alpha_6.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            !((((m.decay().offset(1_usize as isize).read()) as i32)
                & (digit_7.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            !((((m.decay().offset(1_usize as isize).read()) as i32)
                & (punct_8.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            ((((m.decay().offset(1_usize as isize).read()) as i32)
                & (xdigit_9.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            !((((m.decay().offset(1_usize as isize).read()) as i32)
                & (blank_10.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            ((((m.decay().offset(1_usize as isize).read()) as i32)
                & (alnum_11.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            ((((m.decay().offset(1_usize as isize).read()) as i32)
                & (graph_12.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            !((((m.decay().offset(2_usize as isize).read()) as i32)
                & (space_1.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            !((((m.decay().offset(2_usize as isize).read()) as i32)
                & (print_2.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            ((((m.decay().offset(2_usize as isize).read()) as i32)
                & (cntrl_3.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            !((((m.decay().offset(2_usize as isize).read()) as i32)
                & (upper_4.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            !((((m.decay().offset(2_usize as isize).read()) as i32)
                & (lower_5.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            !((((m.decay().offset(2_usize as isize).read()) as i32)
                & (alpha_6.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            !((((m.decay().offset(2_usize as isize).read()) as i32)
                & (digit_7.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            !((((m.decay().offset(2_usize as isize).read()) as i32)
                & (punct_8.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            !((((m.decay().offset(2_usize as isize).read()) as i32)
                & (xdigit_9.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            !((((m.decay().offset(2_usize as isize).read()) as i32)
                & (blank_10.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            !((((m.decay().offset(2_usize as isize).read()) as i32)
                & (alnum_11.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            !((((m.decay().offset(2_usize as isize).read()) as i32)
                & (graph_12.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            !((((m.decay().offset(3_usize as isize).read()) as i32)
                & (space_1.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            ((((m.decay().offset(3_usize as isize).read()) as i32)
                & (print_2.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            !((((m.decay().offset(3_usize as isize).read()) as i32)
                & (cntrl_3.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            !((((m.decay().offset(3_usize as isize).read()) as i32)
                & (upper_4.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            !((((m.decay().offset(3_usize as isize).read()) as i32)
                & (lower_5.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            !((((m.decay().offset(3_usize as isize).read()) as i32)
                & (alpha_6.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            !((((m.decay().offset(3_usize as isize).read()) as i32)
                & (digit_7.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            ((((m.decay().offset(3_usize as isize).read()) as i32)
                & (punct_8.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            !((((m.decay().offset(3_usize as isize).read()) as i32)
                & (xdigit_9.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            !((((m.decay().offset(3_usize as isize).read()) as i32)
                & (blank_10.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            !((((m.decay().offset(3_usize as isize).read()) as i32)
                & (alnum_11.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            ((((m.decay().offset(3_usize as isize).read()) as i32)
                & (graph_12.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            !((((m.decay().offset(4_usize as isize).read()) as i32)
                & (space_1.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            ((((m.decay().offset(4_usize as isize).read()) as i32)
                & (print_2.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            !((((m.decay().offset(4_usize as isize).read()) as i32)
                & (cntrl_3.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            !((((m.decay().offset(4_usize as isize).read()) as i32)
                & (upper_4.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            ((((m.decay().offset(4_usize as isize).read()) as i32)
                & (lower_5.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            ((((m.decay().offset(4_usize as isize).read()) as i32)
                & (alpha_6.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            !((((m.decay().offset(4_usize as isize).read()) as i32)
                & (digit_7.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            !((((m.decay().offset(4_usize as isize).read()) as i32)
                & (punct_8.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            ((((m.decay().offset(4_usize as isize).read()) as i32)
                & (xdigit_9.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            !((((m.decay().offset(4_usize as isize).read()) as i32)
                & (blank_10.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            ((((m.decay().offset(4_usize as isize).read()) as i32)
                & (alnum_11.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            ((((m.decay().offset(4_usize as isize).read()) as i32)
                & (graph_12.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            !((((m.decay().offset(5_usize as isize).read()) as i32)
                & (space_1.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            ((((m.decay().offset(5_usize as isize).read()) as i32)
                & (print_2.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            !((((m.decay().offset(5_usize as isize).read()) as i32)
                & (cntrl_3.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            !((((m.decay().offset(5_usize as isize).read()) as i32)
                & (upper_4.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            !((((m.decay().offset(5_usize as isize).read()) as i32)
                & (lower_5.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            !((((m.decay().offset(5_usize as isize).read()) as i32)
                & (alpha_6.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            ((((m.decay().offset(5_usize as isize).read()) as i32)
                & (digit_7.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            !((((m.decay().offset(5_usize as isize).read()) as i32)
                & (punct_8.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            ((((m.decay().offset(5_usize as isize).read()) as i32)
                & (xdigit_9.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            !((((m.decay().offset(5_usize as isize).read()) as i32)
                & (blank_10.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            ((((m.decay().offset(5_usize as isize).read()) as i32)
                & (alnum_11.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
        assert!(
            ((((m.decay().offset(5_usize as isize).read()) as i32)
                & (graph_12.with(|rc| *rc.borrow()) as i32))
                != 0)
        );
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
