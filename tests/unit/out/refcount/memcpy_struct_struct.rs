extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Entry {
    #[offset(0)]
    pub bits: u8,
    #[offset(2)]
    pub value: u16,
}
impl CppMove for Entry {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let table: Value<Box<[Entry]>> = Rc::new(RefCell::new(Box::new([
        Entry {
            bits: 1_u8,
            value: 4369_u16,
        },
        Entry {
            bits: 2_u8,
            value: 8738_u16,
        },
        Entry {
            bits: 3_u8,
            value: 13107_u16,
        },
        Entry {
            bits: 4_u8,
            value: 17476_u16,
        },
        Entry {
            bits: 0_u8,
            value: 0_u16,
        },
        Entry {
            bits: 0_u8,
            value: 0_u16,
        },
        Entry {
            bits: 0_u8,
            value: 0_u16,
        },
        Entry {
            bits: 0_u8,
            value: 0_u16,
        },
    ])));
    let mut table_size: u64 = 4_u64;
    ({
        let ___dest: AnyPtr =
            (((table.as_pointer() as Ptr<Entry>).offset(table_size)) as Ptr<Entry>).to_any();
        let ___src: AnyPtr =
            (((table.as_pointer() as Ptr<Entry>).offset(0)) as Ptr<Entry>).to_any();
        let ___n: u64 = (table_size).wrapping_mul((4usize as u64));
        memcpy_0(___dest, ___src, ___n)
    });
    ({
        cpp2rust_assert_fail_1(
            (({ (*table.borrow())[(4) as usize].bits } as i32) == 1)
                && (({ (*table.borrow())[(4) as usize].value } as i32) == 4369),
        )
    });
    ({
        cpp2rust_assert_fail_1(
            (({ (*table.borrow())[(5) as usize].bits } as i32) == 2)
                && (({ (*table.borrow())[(5) as usize].value } as i32) == 8738),
        )
    });
    ({
        cpp2rust_assert_fail_1(
            (({ (*table.borrow())[(6) as usize].bits } as i32) == 3)
                && (({ (*table.borrow())[(6) as usize].value } as i32) == 13107),
        )
    });
    ({
        cpp2rust_assert_fail_1(
            (({ (*table.borrow())[(7) as usize].bits } as i32) == 4)
                && (({ (*table.borrow())[(7) as usize].value } as i32) == 17476),
        )
    });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
