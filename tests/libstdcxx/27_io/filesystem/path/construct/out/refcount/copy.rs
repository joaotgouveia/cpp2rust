extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn compare_paths_0(p1: Ptr<Vec<i8>>, p2: Ptr<Vec<i8>>) {
    if !(unimplemented!()) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
    if !(unimplemented!()) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
    if ({ (unimplemented!() as i32) } != { (unimplemented!() as i32) }) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
    if ({ (unimplemented!() as i32) } != { (unimplemented!() as i32) }) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
    if ({ (unimplemented!() as i32) } != { (unimplemented!() as i32) }) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
    if ({ (unimplemented!() as i32) } != { (unimplemented!() as i32) }) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
    if ({ (unimplemented!() as i32) } != { (unimplemented!() as i32) }) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
    if ({ (unimplemented!() as i32) } != { (unimplemented!() as i32) }) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
    if ({ (unimplemented!() as i32) } != { (unimplemented!() as i32) }) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
    if ({ (unimplemented!() as i32) } != { (unimplemented!() as i32) }) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
    if ({ (unimplemented!() as i32) } != { (unimplemented!() as i32) }) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
    if ({ (unimplemented!() as i32) } != { (unimplemented!() as i32) }) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
    if ({ (unimplemented!() as i32) } != { (unimplemented!() as i32) }) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
    let mut d1: isize = unimplemented!();
    let mut d2: isize = unimplemented!();
    if (d1 != d2) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
    if !(unimplemented!()) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
}
thread_local!(
    pub static test_paths_1: Value<Box<[Vec<i8>]>> = Rc::new(RefCell::new(Box::new([
        {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        },
        {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"/").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        },
        {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"//").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        },
        {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"/.").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        },
        {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"/./").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        },
        {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"/a").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        },
        {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"/a/").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        },
        {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"/a//").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        },
        {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"/a/b/c/d").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        },
        {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"/a//b").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        },
        {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"a").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        },
        {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"a/b").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        },
        {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"a/b/").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        },
        {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"a/b/c").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        },
        {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"a/b/c.d").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        },
        {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"a/b/..").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        },
        {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"a/b/c.").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        },
        {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"a/b/.c").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        },
    ])));
);
pub fn root_path_2() -> Vec<i8> {
    return {
        let mut __v: Vec<i8> = (b"/" as Ptr<i8>)
            .path_bytes()
            .into_iter()
            .map(|__b| __b as i8)
            .collect();
        __v.push(0);
        __v
    };
}
pub fn nonexistent_path_3(file: Option<Vec<i8>>) -> Vec<i8> {
    let file: Value<Vec<i8>> = Rc::new(RefCell::new(file.unwrap_or_else(|| {
        let mut __bytes = Ptr::<i8>::from_string_literal(file!().as_bytes()).to_c_bytes();
        __bytes.push(0_i8);
        __bytes
    })));
    if (((*file.borrow()).len() - 1) > 3_usize) && (unimplemented!() == 0) {
        {
            (*file.borrow_mut()).pop();
            (*file.borrow_mut()).resize(
                (((*file.borrow()).len() - 1).wrapping_sub(3_usize)) as usize,
                0_i8,
            );
            (*file.borrow_mut()).push(0_i8)
        };
    }
    let mut pos: usize = {
        let __n = (*file.borrow()).len().saturating_sub(1);
        let __end = usize::MAX.saturating_add(1).min(__n);
        Ptr::<i8>::from_string_literal(b"/\\").with_c_str(|__lookup| {
            (*file.borrow())
                .iter()
                .take(__end)
                .rposition(|__x| __lookup.contains(__x))
                .unwrap_or(usize::MAX)
        })
    };
    if (pos != usize::MAX) {
        unimplemented!();
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let p: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    let mut fd: i32 = unimplemented!();
    if (fd == -1_i32) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
    match nix::unistd::unlink(file.decay().to_rust_string().as_str()) {
        Ok(()) => 0,
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            -1
        }
    };
    FdRegistry::close(fd);
    unimplemented!();
    return unimplemented!();
}
pub type __gnu_test_scoped_file_adopt_file_t = u32;
pub const __gnu_test_scoped_file_adopt_file_t_adopt_file: __gnu_test_scoped_file_adopt_file_t = 0;
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Destructor)]
#[byte_size(40)]
pub struct __gnu_test_scoped_file {
    #[offset(0)]
    #[byte_size(40)]
    pub path: Vec<i8>,
}
impl __gnu_test_scoped_file {
    pub fn new_1(p: Option<Ptr<Vec<i8>>>) -> Self {
        let mut __p_default: Option<Value<Vec<i8>>> = None;
        let p: Ptr<Vec<i8>> = p.unwrap_or_else(|| {
            __p_default
                .insert(Rc::new(RefCell::new(({ nonexistent_path_3(None) }))))
                .as_pointer()
        });
        let __this: __gnu_test_scoped_file = Self {
            path: unimplemented!(),
        };
        ::std::fs::File::create((unimplemented!()).reinterpret_cast::<i8>().to_string())
            .expect("Failed to open file");
        __this
    }
    pub fn new_2(p: Vec<i8>, mut _a1: __gnu_test_scoped_file_adopt_file_t) -> Self {
        let p: Value<Vec<i8>> = Rc::new(RefCell::new(p));
        Self {
            path: unimplemented!(),
        }
    }
    pub fn move_from(_a0: Ptr<__gnu_test_scoped_file>) -> Self {
        Self {
            path: unimplemented!(),
        }
    }
}
impl Default for __gnu_test_scoped_file {
    fn default() -> Self {
        { __gnu_test_scoped_file::new_1(None) }
    }
}
pub fn permissions_are_testable_5(print_msg: Option<bool>) -> bool {
    let mut print_msg: bool = print_msg.unwrap_or_else(|| true);
    let testable: Value<bool> = Rc::new(RefCell::new(false));
    if ((nix::unistd::geteuid().as_raw() as ::libc::uid_t) != (0 as ::libc::uid_t)) {
        (*testable.borrow_mut()) = true;
    }
    if (print_msg) && (!(*testable.borrow())) {
        {
            let mut __bytes = Ptr::<i8>::from_string_literal(
                b"Skipping tests that depend on filesystem permissions",
            )
            .to_c_u8_bytes();
            __bytes.push(b'\n');
            match libcc2rs::c_stdout().with_mut(|__f| __f.write(&__bytes)) == __bytes.len() {
                true => 0,
                false => -1,
            }
        };
    }
    return (*testable.borrow());
}
thread_local!(
    static counter_4: Value<u32> = Rc::new(RefCell::new((unimplemented!() as u32)));
);
pub fn test01_6() {
    'loop_: for p in IterRange::new(
        (test_paths_1.with(|v| v.as_pointer()) as Ptr<Vec<i8>>),
        (test_paths_1.with(|v| v.as_pointer()) as Ptr<Vec<i8>>).offset((18_i64) as isize),
    ) {
        let p: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __v: Vec<i8> = (p)
                .clone()
                .path_bytes()
                .into_iter()
                .map(|__b| __b as i8)
                .collect();
            __v.push(0);
            __v
        }));
        let copy: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        ({ compare_paths_0(p.as_pointer(), copy.as_pointer()) });
    }
}
pub fn test02_7() {
    'loop_: for p in IterRange::new(
        (test_paths_1.with(|v| v.as_pointer()) as Ptr<Vec<i8>>),
        (test_paths_1.with(|v| v.as_pointer()) as Ptr<Vec<i8>>).offset((18_i64) as isize),
    ) {
        let p: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __v: Vec<i8> = (p)
                .clone()
                .path_bytes()
                .into_iter()
                .map(|__b| __b as i8)
                .collect();
            __v.push(0);
            __v
        }));
        let copy: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        let move_: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        ({ compare_paths_0(p.as_pointer(), move_.as_pointer()) });
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test01_6() });
    ({ test02_7() });
    0
}
pub trait __gnu_test_scoped_fileImpl {
    fn destructor(&self);
    fn move_assign(&self, _a0: Ptr<__gnu_test_scoped_file>) -> Ptr<__gnu_test_scoped_file>;
}
impl __gnu_test_scoped_fileImpl for Ptr<__gnu_test_scoped_file> {
    fn destructor(&self) {
        if !(unimplemented!()) {
            unimplemented!();
        }
    }
    fn move_assign(&self, _a0: Ptr<__gnu_test_scoped_file>) -> Ptr<__gnu_test_scoped_file> {
        unimplemented!();
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = test_paths_1.with(|_| ());
}
