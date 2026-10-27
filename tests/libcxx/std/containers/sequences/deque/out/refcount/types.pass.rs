extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(40)]
pub struct test_allocator_statistics {
    #[offset(0)]
    pub time_to_throw: (),
    #[offset(4)]
    pub throw_after: (),
    #[offset(8)]
    pub count: (),
    #[offset(12)]
    pub alloc_count: (),
    #[offset(16)]
    pub allocated_size: (),
    #[offset(20)]
    pub construct_count: (),
    #[offset(24)]
    pub destroy_count: (),
    #[offset(28)]
    pub copied: (),
    #[offset(32)]
    pub moved: (),
    #[offset(36)]
    pub converted: (),
}
impl Default for test_allocator_statistics {
    fn default() -> Self {
        {
            test_allocator_statistics {
                time_to_throw: 0_(),
                throw_after: 2147483647_(),
                count: 0_(),
                alloc_count: 0_(),
                allocated_size: 0_(),
                construct_count: 0_(),
                destroy_count: 0_(),
                copied: 0_(),
                moved: 0_(),
                converted: 0_(),
            }
        }
    }
}
thread_local!(
    pub static destructed_value_0: Value<()> = Rc::new(RefCell::new(-1_()));
);
thread_local!(
    pub static moved_value_1: Value<()> = Rc::new(RefCell::new(2147483647_()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_alloc_base {}
pub fn operator_eq_2(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_3(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> () {
    return !({
        let _x: Ptr<test_allocator_void_> = (x).clone();
        let _y: Ptr<test_allocator_void_> = (y).clone();
        operator_eq_2(_x, _y)
    });
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor)]
#[byte_size(16)]
pub struct test_allocator_void_ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_void_ {
    pub fn new_1(mut stats: Ptr<test_allocator_statistics>) -> Self {
        Self {
            data_: 0_(),
            id_: 0_(),
            stats_: (stats).clone(),
        }
    }
    pub fn new_2(mut data: ()) -> Self {
        Self {
            data_: data,
            id_: 0_(),
            stats_: Ptr::<test_allocator_statistics>::null(),
        }
    }
    pub fn new_3(mut data: (), mut stats: Ptr<test_allocator_statistics>) -> Self {
        Self {
            data_: data,
            id_: 0_(),
            stats_: (stats).clone(),
        }
    }
    pub fn new_4(mut data: (), mut id: ()) -> Self {
        Self {
            data_: data,
            id_: id,
            stats_: Ptr::<test_allocator_statistics>::null(),
        }
    }
    pub fn new_5(mut data: (), mut id: (), mut stats: Ptr<test_allocator_statistics>) -> Self {
        Self {
            data_: data,
            id_: id,
            stats_: (stats).clone(),
        }
    }
    pub fn copy_from(a: Ptr<test_allocator_void_>) -> Self {
        Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }
    }
}
impl std::cmp::PartialEq for test_allocator_void_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_2(
                Rc::new(RefCell::new(test_allocator_void_ {
                    data_: self.data_.clone(),
                    id_: self.id_.clone(),
                    stats_: self.stats_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(test_allocator_void_ {
                    data_: other.data_.clone(),
                    id_: other.id_.clone(),
                    stats_: other.stats_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for test_allocator_void_ {}
impl Clone for test_allocator_void_ {
    fn clone(&self) -> Self {
        let __src: Value<test_allocator_void_> = Rc::new(RefCell::new(test_allocator_void_ {
            data_: self.data_.clone(),
            id_: self.id_.clone(),
            stats_: self.stats_.clone(),
        }));
        test_allocator_void_::copy_from(__src.as_pointer())
    }
}
impl Default for test_allocator_void_ {
    fn default() -> Self {
        {
            test_allocator_void_ {
                data_: 0_(),
                id_: 0_(),
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_int__rebind_int_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_int__rebind_int_ptr_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_int__rebind_const_int_ptr_ {}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(16)]
pub struct test_allocator_int_ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_int_ {}
impl Default for test_allocator_int_ {
    fn default() -> Self {
        {
            test_allocator_int_ {
                data_: unimplemented!(),
                id_: unimplemented!(),
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_Copyable__rebind_Copyable_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_Copyable__rebind_Copyable_ptr_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_Copyable__rebind_const_Copyable_ptr_ {}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(16)]
pub struct test_allocator_Copyable_ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_Copyable_ {}
impl Default for test_allocator_Copyable_ {
    fn default() -> Self {
        {
            test_allocator_Copyable_ {
                data_: unimplemented!(),
                id_: unimplemented!(),
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(16)]
pub struct test_allocator_int_ptr_ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_int_ptr_ {}
impl Default for test_allocator_int_ptr_ {
    fn default() -> Self {
        {
            test_allocator_int_ptr_ {
                data_: unimplemented!(),
                id_: unimplemented!(),
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(16)]
pub struct test_allocator_const_int_ptr_ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_const_int_ptr_ {}
impl Default for test_allocator_const_int_ptr_ {
    fn default() -> Self {
        {
            test_allocator_const_int_ptr_ {
                data_: unimplemented!(),
                id_: unimplemented!(),
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(16)]
pub struct test_allocator_Copyable_ptr_ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_Copyable_ptr_ {}
impl Default for test_allocator_Copyable_ptr_ {
    fn default() -> Self {
        {
            test_allocator_Copyable_ptr_ {
                data_: unimplemented!(),
                id_: unimplemented!(),
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(16)]
pub struct test_allocator_const_Copyable_ptr_ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_const_Copyable_ptr_ {}
impl Default for test_allocator_const_Copyable_ptr_ {
    fn default() -> Self {
        {
            test_allocator_const_Copyable_ptr_ {
                data_: unimplemented!(),
                id_: unimplemented!(),
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Ctor_Tag {}
#[derive(Record, ByteRepr, FnPtrArg, CopyAssign, MoveAssign, Default)]
#[byte_size(1)]
pub struct Tag_X {}
impl Tag_X {
    pub fn new_1(mut _a0: Ctor_Tag) -> Self {
        Self {}
    }
    pub fn new_2(mut _a0: Ctor_Tag, _a1: Ptr<Tag_X>) -> Self {
        Self {}
    }
    pub fn new_3(mut _a0: Ctor_Tag, _a1: Ptr<Tag_X>) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Copyable {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> () {
        unimplemented!();
        return ((alloc_count_5.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_6.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        unimplemented!();
        disable_default_constructor_7.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_4.with(|rc| *rc.borrow_mut() = 0_());
        alloc_count_5.with(|rc| *rc.borrow_mut() = 0_());
        dealloc_count_6.with(|rc| *rc.borrow_mut() = 0_());
    }
}
thread_local!(
    pub static outstanding_bytes_4: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static alloc_count_5: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static dealloc_count_6: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static disable_default_constructor_7: Value<()> = Rc::new(RefCell::new(false));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_short__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_short__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_const_short__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_const_short__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_min_pointer_short__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__
{
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<min_pointer_short__std_integral_constant_unsigned_long__0__>,
}
impl min_pointer_min_pointer_short__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_const_min_pointer_short__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__
{
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<min_pointer_short__std_integral_constant_unsigned_long__0__>,
}
impl min_pointer_const_min_pointer_short__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_min_pointer_const_short__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__
{
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<min_pointer_const_short__std_integral_constant_unsigned_long__0__>,
}
impl min_pointer_min_pointer_const_short__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_const_min_pointer_const_short__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__
{
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<min_pointer_const_short__std_integral_constant_unsigned_long__0__>,
}
impl min_pointer_const_min_pointer_const_short__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_short_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_min_pointer_short__std_integral_constant_unsigned_long__0___ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_min_pointer_const_short__std_integral_constant_unsigned_long__0___ {}
pub fn test_8() {
    const _: () = assert!(true, "(std::is_same<typename C::value_type, T>::value)");;
    const _: () = assert!(
        true,
        "(std::is_same<typename C::value_type, typename std::allocator_traits<Allocator>::value_type>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename C::allocator_type, Allocator>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename C::size_type, typename std::allocator_traits<Allocator>::size_type>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename C::difference_type, typename std::allocator_traits<Allocator>::difference_type>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename C::reference, typename std::allocator_traits<Allocator>::value_type&>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename C::const_reference, const typename std::allocator_traits<Allocator>::value_type&>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename C::pointer, typename std::allocator_traits<Allocator>::pointer>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename C::const_pointer, typename std::allocator_traits<Allocator>::const_pointer>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same< typename std::iterator_traits<typename C::iterator>::iterator_category,\n                               std::random_access_iterator_tag>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same< typename std::iterator_traits<typename C::const_iterator>::iterator_category,\n                               std::random_access_iterator_tag>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same< typename C::reverse_iterator, std::reverse_iterator<typename C::iterator> >::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same< typename C::const_reverse_iterator, std::reverse_iterator<typename C::const_iterator> >::value)"
    );;
    const _: () = assert!(true, "(std::is_signed<typename C::difference_type>::value)");;
    const _: () = assert!(true, "(std::is_unsigned<typename C::size_type>::value)");;
    const _: () = assert!(
        true,
        "(std::is_same<typename C::difference_type,\n                              typename std::iterator_traits<typename C::iterator>::difference_type>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename C::difference_type,\n                              typename std::iterator_traits<typename C::const_iterator>::difference_type>::value)"
    );;
}
pub fn test_9() {
    const _: () = assert!(true, "(std::is_same<typename C::value_type, T>::value)");;
    const _: () = assert!(
        true,
        "(std::is_same<typename C::value_type, typename std::allocator_traits<Allocator>::value_type>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename C::allocator_type, Allocator>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename C::size_type, typename std::allocator_traits<Allocator>::size_type>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename C::difference_type, typename std::allocator_traits<Allocator>::difference_type>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename C::reference, typename std::allocator_traits<Allocator>::value_type&>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename C::const_reference, const typename std::allocator_traits<Allocator>::value_type&>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename C::pointer, typename std::allocator_traits<Allocator>::pointer>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename C::const_pointer, typename std::allocator_traits<Allocator>::const_pointer>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same< typename std::iterator_traits<typename C::iterator>::iterator_category,\n                               std::random_access_iterator_tag>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same< typename std::iterator_traits<typename C::const_iterator>::iterator_category,\n                               std::random_access_iterator_tag>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same< typename C::reverse_iterator, std::reverse_iterator<typename C::iterator> >::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same< typename C::const_reverse_iterator, std::reverse_iterator<typename C::const_iterator> >::value)"
    );;
    const _: () = assert!(true, "(std::is_signed<typename C::difference_type>::value)");;
    const _: () = assert!(true, "(std::is_unsigned<typename C::size_type>::value)");;
    const _: () = assert!(
        true,
        "(std::is_same<typename C::difference_type,\n                              typename std::iterator_traits<typename C::iterator>::difference_type>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename C::difference_type,\n                              typename std::iterator_traits<typename C::const_iterator>::difference_type>::value)"
    );;
}
pub fn test_10() {
    const _: () = assert!(true, "(std::is_same<typename C::value_type, T>::value)");;
    const _: () = assert!(
        true,
        "(std::is_same<typename C::value_type, typename std::allocator_traits<Allocator>::value_type>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename C::allocator_type, Allocator>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename C::size_type, typename std::allocator_traits<Allocator>::size_type>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename C::difference_type, typename std::allocator_traits<Allocator>::difference_type>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename C::reference, typename std::allocator_traits<Allocator>::value_type&>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename C::const_reference, const typename std::allocator_traits<Allocator>::value_type&>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename C::pointer, typename std::allocator_traits<Allocator>::pointer>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename C::const_pointer, typename std::allocator_traits<Allocator>::const_pointer>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same< typename std::iterator_traits<typename C::iterator>::iterator_category,\n                               std::random_access_iterator_tag>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same< typename std::iterator_traits<typename C::const_iterator>::iterator_category,\n                               std::random_access_iterator_tag>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same< typename C::reverse_iterator, std::reverse_iterator<typename C::iterator> >::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same< typename C::const_reverse_iterator, std::reverse_iterator<typename C::const_iterator> >::value)"
    );;
    const _: () = assert!(true, "(std::is_signed<typename C::difference_type>::value)");;
    const _: () = assert!(true, "(std::is_unsigned<typename C::size_type>::value)");;
    const _: () = assert!(
        true,
        "(std::is_same<typename C::difference_type,\n                              typename std::iterator_traits<typename C::iterator>::difference_type>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename C::difference_type,\n                              typename std::iterator_traits<typename C::const_iterator>::difference_type>::value)"
    );;
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
    ({ test_8() });
    ({ test_9() });
    ({ test_10() });
    const _: () = assert!(
        true,
        "(std::is_same<std::deque<char>::allocator_type, std::allocator<char> >::value)"
    );;
    {
        const _: () = assert!(true, "(std::is_same<C::value_type, short>::value)");;
        const _: () = assert!(
            true,
            "(std::is_same<C::allocator_type, min_allocator<C::value_type> >::value)"
        );;
        const _: () = assert!(true, "(std::is_same<C::reference, C::value_type&>::value)");;
        const _: () = assert!(
            true,
            "(std::is_same<C::const_reference, const C::value_type&>::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<C::pointer, min_pointer<C::value_type>>::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<C::const_pointer, min_pointer<const C::value_type>>::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<C::size_type, std::make_unsigned<C::difference_type>::type>::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<C::difference_type, std::ptrdiff_t>::value)"
        );;
        const _: () = assert!(true, "(std::is_signed<typename C::difference_type>::value)");;
        const _: () = assert!(true, "(std::is_unsigned<typename C::size_type>::value)");;
        const _: () = assert!(
            true,
            "(std::is_same<typename C::difference_type,\n                                typename std::iterator_traits<typename C::iterator>::difference_type>::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<typename C::difference_type,\n                                typename std::iterator_traits<typename C::const_iterator>::difference_type>::value)"
        );;
    }
    return 0_();
}
pub trait Tag_XImpl {
    fn copy_assign(&self, _a0: Ptr<Tag_X>) -> Ptr<Tag_X>;
    fn move_assign(&self, _a0: Ptr<Tag_X>) -> Ptr<Tag_X>;
}
impl Tag_XImpl for Ptr<Tag_X> {
    fn copy_assign(&self, _a0: Ptr<Tag_X>) -> Ptr<Tag_X> {
        return (*self).clone();
    }
    fn move_assign(&self, _a0: Ptr<Tag_X>) -> Ptr<Tag_X> {
        return (*self).clone();
    }
}
pub trait min_allocator_min_pointer_const_short__std_integral_constant_unsigned_long__0___Impl {
    fn allocate ( &self , mut n : ()  , ) -> min_pointer_min_pointer_const_short__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p : min_pointer_min_pointer_const_short__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!()
    }
}
impl min_allocator_min_pointer_const_short__std_integral_constant_unsigned_long__0___Impl
    for Ptr<min_allocator_min_pointer_const_short__std_integral_constant_unsigned_long__0___>
{
}
pub trait min_allocator_min_pointer_short__std_integral_constant_unsigned_long__0___Impl {
    fn allocate ( &self , mut n : ()  , ) -> min_pointer_min_pointer_short__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p : min_pointer_min_pointer_short__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!()
    }
}
impl min_allocator_min_pointer_short__std_integral_constant_unsigned_long__0___Impl
    for Ptr<min_allocator_min_pointer_short__std_integral_constant_unsigned_long__0___>
{
}
pub trait min_allocator_short_Impl {
    fn allocate(&self, mut n: ()) -> min_pointer_short__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p: min_pointer_short__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!()
    }
}
impl min_allocator_short_Impl for Ptr<min_allocator_short_> {}
pub trait min_pointer_const_min_pointer_const_short__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__Impl
{
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(
        &self,
    ) -> Ptr<min_pointer_const_short__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_arrow(
        &self,
    ) -> Ptr<min_pointer_const_short__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }    fn operator_inc_9 ( &self , ) -> Ptr< min_pointer_const_min_pointer_const_short__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_post_inc_10 ( &self , mut _a0 : ()  , ) -> min_pointer_const_min_pointer_const_short__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_dec_11 ( &self , ) -> Ptr< min_pointer_const_min_pointer_const_short__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_post_dec_12 ( &self , mut _a0 : ()  , ) -> min_pointer_const_min_pointer_const_short__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_add_assign ( &self , mut n : ()  , ) -> Ptr< min_pointer_const_min_pointer_const_short__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_sub_assign ( &self , mut n : ()  , ) -> Ptr< min_pointer_const_min_pointer_const_short__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_add ( &self , mut n : ()  , ) -> min_pointer_const_min_pointer_const_short__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_sub ( &self , mut n : ()  , ) -> min_pointer_const_min_pointer_const_short__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }
    fn operator_index(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_short__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
}
impl min_pointer_const_min_pointer_const_short__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__Impl for Ptr<min_pointer_const_min_pointer_const_short__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__> {
}
pub trait min_pointer_const_min_pointer_short__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__Impl
{
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<min_pointer_short__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<min_pointer_short__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }    fn operator_inc_9 ( &self , ) -> Ptr< min_pointer_const_min_pointer_short__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_post_inc_10 ( &self , mut _a0 : ()  , ) -> min_pointer_const_min_pointer_short__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_dec_11 ( &self , ) -> Ptr< min_pointer_const_min_pointer_short__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_post_dec_12 ( &self , mut _a0 : ()  , ) -> min_pointer_const_min_pointer_short__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_add_assign ( &self , mut n : ()  , ) -> Ptr< min_pointer_const_min_pointer_short__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_sub_assign ( &self , mut n : ()  , ) -> Ptr< min_pointer_const_min_pointer_short__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_add ( &self , mut n : ()  , ) -> min_pointer_const_min_pointer_short__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_sub ( &self , mut n : ()  , ) -> min_pointer_const_min_pointer_short__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }
    fn operator_index(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_short__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
}
impl min_pointer_const_min_pointer_short__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__Impl for Ptr<min_pointer_const_min_pointer_short__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__> {
}
pub trait min_pointer_const_short__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_inc_9(
        &self,
    ) -> Ptr<min_pointer_const_short__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_10(
        &self,
        mut _a0: (),
    ) -> min_pointer_const_short__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_11(
        &self,
    ) -> Ptr<min_pointer_const_short__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_12(
        &self,
        mut _a0: (),
    ) -> min_pointer_const_short__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_short__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_short__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_const_short__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_const_short__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
}
impl min_pointer_const_short__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_const_short__std_integral_constant_unsigned_long__0__>
{
}
pub trait min_pointer_min_pointer_const_short__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__Impl
{
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(
        &self,
    ) -> Ptr<min_pointer_const_short__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_arrow(
        &self,
    ) -> Ptr<min_pointer_const_short__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }    fn operator_inc_8 ( &self , ) -> Ptr< min_pointer_min_pointer_const_short__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_post_inc_9 ( &self , mut _a0 : ()  , ) -> min_pointer_min_pointer_const_short__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_dec_10 ( &self , ) -> Ptr< min_pointer_min_pointer_const_short__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_post_dec_11 ( &self , mut _a0 : ()  , ) -> min_pointer_min_pointer_const_short__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_add_assign ( &self , mut n : ()  , ) -> Ptr< min_pointer_min_pointer_const_short__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_sub_assign ( &self , mut n : ()  , ) -> Ptr< min_pointer_min_pointer_const_short__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_add ( &self , mut n : ()  , ) -> min_pointer_min_pointer_const_short__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_sub ( &self , mut n : ()  , ) -> min_pointer_min_pointer_const_short__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }
    fn operator_index(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_short__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
}
impl min_pointer_min_pointer_const_short__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__Impl for Ptr<min_pointer_min_pointer_const_short__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__> {
}
pub trait min_pointer_min_pointer_short__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__Impl
{
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<min_pointer_short__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<min_pointer_short__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }    fn operator_inc_8 ( &self , ) -> Ptr< min_pointer_min_pointer_short__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_post_inc_9 ( &self , mut _a0 : ()  , ) -> min_pointer_min_pointer_short__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_dec_10 ( &self , ) -> Ptr< min_pointer_min_pointer_short__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_post_dec_11 ( &self , mut _a0 : ()  , ) -> min_pointer_min_pointer_short__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_add_assign ( &self , mut n : ()  , ) -> Ptr< min_pointer_min_pointer_short__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_sub_assign ( &self , mut n : ()  , ) -> Ptr< min_pointer_min_pointer_short__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_add ( &self , mut n : ()  , ) -> min_pointer_min_pointer_short__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_sub ( &self , mut n : ()  , ) -> min_pointer_min_pointer_short__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }
    fn operator_index(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_short__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
}
impl min_pointer_min_pointer_short__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__Impl for Ptr<min_pointer_min_pointer_short__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__> {
}
pub trait min_pointer_short__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_inc_8(&self) -> Ptr<min_pointer_short__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_9(
        &self,
        mut _a0: (),
    ) -> min_pointer_short__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(&self) -> Ptr<min_pointer_short__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_11(
        &self,
        mut _a0: (),
    ) -> min_pointer_short__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_short__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_short__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_short__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_short__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
}
impl min_pointer_short__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_short__std_integral_constant_unsigned_long__0__>
{
}
pub trait test_allocator_Copyable_Impl {
    fn destructor(&self) {
        unimplemented!()
    }
    fn address_11(&self, x: ()) -> Ptr<Copyable> {
        unimplemented!()
    }
    fn address_12(&self, x: ()) -> Ptr<Copyable> {
        unimplemented!()
    }
    fn allocate(&self, mut n: (), _a1: Option<()>) -> Ptr<Copyable> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<Copyable>, mut s: ()) {
        unimplemented!()
    }
    fn max_size(&self) -> () {
        unimplemented!()
    }
    fn destroy(&self, mut p: Ptr<Copyable>) {
        unimplemented!()
    }
    fn get_data(&self) -> () {
        unimplemented!()
    }
    fn get_id(&self) -> () {
        unimplemented!()
    }
}
impl test_allocator_Copyable_Impl for Ptr<test_allocator_Copyable_> {}
pub trait test_allocator_Copyable_ptr_Impl {
    fn destructor(&self) {
        unimplemented!()
    }
    fn address_11(&self, x: ()) -> Ptr<Ptr<Copyable>> {
        unimplemented!()
    }
    fn address_12(&self, x: ()) -> Ptr<Ptr<Copyable>> {
        unimplemented!()
    }
    fn allocate(&self, mut n: (), mut _a1: ()) -> Ptr<Ptr<Copyable>> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<Ptr<Copyable>>, mut s: ()) {
        unimplemented!()
    }
    fn max_size(&self) -> () {
        unimplemented!()
    }
    fn destroy(&self, mut p: Ptr<Ptr<Copyable>>) {
        unimplemented!()
    }
    fn get_data(&self) -> () {
        unimplemented!()
    }
    fn get_id(&self) -> () {
        unimplemented!()
    }
}
impl test_allocator_Copyable_ptr_Impl for Ptr<test_allocator_Copyable_ptr_> {}
pub trait test_allocator_const_Copyable_ptr_Impl {
    fn destructor(&self) {
        unimplemented!()
    }
    fn address_11(&self, x: ()) -> Ptr<Ptr<Copyable>> {
        unimplemented!()
    }
    fn address_12(&self, x: ()) -> Ptr<Ptr<Copyable>> {
        unimplemented!()
    }
    fn allocate(&self, mut n: (), mut _a1: ()) -> Ptr<Ptr<Copyable>> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<Ptr<Copyable>>, mut s: ()) {
        unimplemented!()
    }
    fn max_size(&self) -> () {
        unimplemented!()
    }
    fn destroy(&self, mut p: Ptr<Ptr<Copyable>>) {
        unimplemented!()
    }
    fn get_data(&self) -> () {
        unimplemented!()
    }
    fn get_id(&self) -> () {
        unimplemented!()
    }
}
impl test_allocator_const_Copyable_ptr_Impl for Ptr<test_allocator_const_Copyable_ptr_> {}
pub trait test_allocator_const_int_ptr_Impl {
    fn destructor(&self) {
        unimplemented!()
    }
    fn address_11(&self, x: ()) -> Ptr<Ptr<()>> {
        unimplemented!()
    }
    fn address_12(&self, x: ()) -> Ptr<Ptr<()>> {
        unimplemented!()
    }
    fn allocate(&self, mut n: (), mut _a1: ()) -> Ptr<Ptr<()>> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<Ptr<()>>, mut s: ()) {
        unimplemented!()
    }
    fn max_size(&self) -> () {
        unimplemented!()
    }
    fn destroy(&self, mut p: Ptr<Ptr<()>>) {
        unimplemented!()
    }
    fn get_data(&self) -> () {
        unimplemented!()
    }
    fn get_id(&self) -> () {
        unimplemented!()
    }
}
impl test_allocator_const_int_ptr_Impl for Ptr<test_allocator_const_int_ptr_> {}
pub trait test_allocator_int_Impl {
    fn destructor(&self) {
        unimplemented!()
    }
    fn address_11(&self, x: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn address_12(&self, x: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn allocate(&self, mut n: (), _a1: Option<()>) -> Ptr<()> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<()>, mut s: ()) {
        unimplemented!()
    }
    fn max_size(&self) -> () {
        unimplemented!()
    }
    fn destroy(&self, mut p: Ptr<()>) {
        unimplemented!()
    }
    fn get_data(&self) -> () {
        unimplemented!()
    }
    fn get_id(&self) -> () {
        unimplemented!()
    }
}
impl test_allocator_int_Impl for Ptr<test_allocator_int_> {}
pub trait test_allocator_int_ptr_Impl {
    fn destructor(&self) {
        unimplemented!()
    }
    fn address_11(&self, x: ()) -> Ptr<Ptr<()>> {
        unimplemented!()
    }
    fn address_12(&self, x: ()) -> Ptr<Ptr<()>> {
        unimplemented!()
    }
    fn allocate(&self, mut n: (), mut _a1: ()) -> Ptr<Ptr<()>> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<Ptr<()>>, mut s: ()) {
        unimplemented!()
    }
    fn max_size(&self) -> () {
        unimplemented!()
    }
    fn destroy(&self, mut p: Ptr<Ptr<()>>) {
        unimplemented!()
    }
    fn get_data(&self) -> () {
        unimplemented!()
    }
    fn get_id(&self) -> () {
        unimplemented!()
    }
}
impl test_allocator_int_ptr_Impl for Ptr<test_allocator_int_ptr_> {}
pub trait test_allocator_statisticsImpl {
    fn clear(&self);
    fn clear_ctor_counters(&self);
}
impl test_allocator_statisticsImpl for Ptr<test_allocator_statistics> {
    fn clear(&self) {
        unimplemented!();
        field!((*self), count).write(0_());
        field!((*self), time_to_throw).write(0_());
        field!((*self), alloc_count).write(0_());
        field!((*self), allocated_size).write(0_());
        field!((*self), construct_count).write(0_());
        field!((*self), destroy_count).write(0_());
        field!((*self), throw_after).write(2147483647_());
        ({ test_allocator_statisticsImpl::clear_ctor_counters(self) });
    }
    fn clear_ctor_counters(&self) {
        field!((*self), copied).write(0_());
        field!((*self), moved).write(0_());
        field!((*self), converted).write(0_());
    }
}
pub trait test_allocator_void_Impl {
    fn destructor(&self);
    fn get_id(&self) -> ();
    fn get_data(&self) -> ();
    fn copy_assign(&self, _a0: Ptr<test_allocator_void_>) -> Ptr<test_allocator_void_>;
}
impl test_allocator_void_Impl for Ptr<test_allocator_void_> {
    fn destructor(&self) {
        field!((*self), data_).write(destructed_value_0.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_0.with(|rc| *rc.borrow()));
    }
    fn get_id(&self) -> () {
        return (*self).with(|__s| __s.id_);
    }
    fn get_data(&self) -> () {
        return (*self).with(|__s| __s.data_);
    }
    fn copy_assign(&self, _a0: Ptr<test_allocator_void_>) -> Ptr<test_allocator_void_> {
        field!((*self), data_).write({ _a0.with(|__s| __s.data_) });
        field!((*self), id_).write({ _a0.with(|__s| __s.id_) });
        field!((*self), stats_).write({ _a0.with(|__s| __s.stats_.clone()) });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = destructed_value_0.with(|_| ());
    let _ = moved_value_1.with(|_| ());
    let _ = outstanding_bytes_4.with(|_| ());
    let _ = alloc_count_5.with(|_| ());
    let _ = dealloc_count_6.with(|_| ());
    let _ = disable_default_constructor_7.with(|_| ());
}
