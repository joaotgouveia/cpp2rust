extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_into(Ptr< i8  >  , to_pconsti8)]
#[cpp_into(Ptr< i32  >  , to_pconsti32)]
#[cpp_into(Ptr< u8  >  , to_pconstu8)]
#[cpp_into(Ptr< u16  >  , to_pconstu16)]
#[cpp_into(Ptr< u32  >  , to_pconstu32)]
#[byte_size(80)]
pub struct MultiStringType {
    #[offset(0)]
    #[byte_size(8)]
    pub w_: Ptr<i32>,
    #[offset(8)]
    pub wn_: usize,
    #[offset(16)]
    #[byte_size(8)]
    pub u8_: Ptr<u8>,
    #[offset(24)]
    pub u8n_: usize,
    #[offset(32)]
    #[byte_size(8)]
    pub u16_: Ptr<u16>,
    #[offset(40)]
    pub u16n_: usize,
    #[offset(48)]
    #[byte_size(8)]
    pub u32_: Ptr<u32>,
    #[offset(56)]
    pub u32n_: usize,
    #[offset(64)]
    #[byte_size(8)]
    pub s_: Ptr<i8>,
    #[offset(72)]
    pub sn_: usize,
}
impl MultiStringType {
    pub fn new(
        mut w: Ptr<i32>,
        mut wn: usize,
        mut u8: Ptr<u8>,
        mut u8n: usize,
        mut u16: Ptr<u16>,
        mut u16n: usize,
        mut u32: Ptr<u32>,
        mut u32n: usize,
        mut s: Ptr<i8>,
        mut sn: usize,
    ) -> Self {
        Self {
            w_: (w).clone(),
            wn_: wn,
            u8_: (u8).clone(),
            u8n_: u8n,
            u16_: (u16).clone(),
            u16n_: u16n,
            u32_: (u32).clone(),
            u32n_: u32n,
            s_: (s).clone(),
            sn_: sn,
        }
    }
}
impl CppMove for MultiStringType {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(40)]
pub struct test_allocator_statistics {
    #[offset(0)]
    pub time_to_throw: i32,
    #[offset(4)]
    pub throw_after: i32,
    #[offset(8)]
    pub count: i32,
    #[offset(12)]
    pub alloc_count: i32,
    #[offset(16)]
    pub allocated_size: i32,
    #[offset(20)]
    pub construct_count: i32,
    #[offset(24)]
    pub destroy_count: i32,
    #[offset(28)]
    pub copied: i32,
    #[offset(32)]
    pub moved: i32,
    #[offset(36)]
    pub converted: i32,
}
impl CppMove for test_allocator_statistics {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for test_allocator_statistics {
    fn default() -> Self {
        {
            test_allocator_statistics {
                time_to_throw: 0,
                throw_after: 2147483647,
                count: 0,
                alloc_count: 0,
                allocated_size: 0,
                construct_count: 0,
                destroy_count: 0,
                copied: 0,
                moved: 0,
                converted: 0,
            }
        }
    }
}
thread_local!(
    pub static destructed_value_0: Value<i32> = Rc::new(RefCell::new(-1_i32));
);
thread_local!(
    pub static moved_value_1: Value<i32> = Rc::new(RefCell::new(2147483647));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_alloc_base {}
impl CppMove for test_alloc_base {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn operator_eq_2(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_3(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> bool {
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
    data_: i32,
    #[offset(4)]
    id_: i32,
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_void_ {
    pub fn new_1(mut stats: Ptr<test_allocator_statistics>) -> Self {
        Self {
            data_: 0,
            id_: 0,
            stats_: (stats).clone(),
        }
    }
    pub fn new_2(mut data: i32) -> Self {
        Self {
            data_: data,
            id_: 0,
            stats_: Ptr::<test_allocator_statistics>::null(),
        }
    }
    pub fn new_3(mut data: i32, mut stats: Ptr<test_allocator_statistics>) -> Self {
        Self {
            data_: data,
            id_: 0,
            stats_: (stats).clone(),
        }
    }
    pub fn new_4(mut data: i32, mut id: i32) -> Self {
        Self {
            data_: data,
            id_: id,
            stats_: Ptr::<test_allocator_statistics>::null(),
        }
    }
    pub fn new_5(mut data: i32, mut id: i32, mut stats: Ptr<test_allocator_statistics>) -> Self {
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
impl CppEq for test_allocator_void_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_2(__lhs.clone(), __rhs.clone())
    }
}
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
impl CppMove for test_allocator_void_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for test_allocator_void_ {
    fn default() -> Self {
        {
            test_allocator_void_ {
                data_: 0,
                id_: 0,
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Ctor_Tag {}
impl CppMove for Ctor_Tag {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Record, ByteRepr, FnPtrArg, CppFrom, CopyAssign, MoveAssign, Default)]
#[cpp_from(Ctor_Tag, new_1, value)]
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct SocccAllocator_char_ {
    #[offset(0)]
    pub count_: i32,
}
impl SocccAllocator_char_ {
    pub fn new(mut i: i32) -> Self {
        Self { count_: i }
    }
}
impl CppEq for SocccAllocator_char_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        SocccAllocator_char_Impl::operator_eq(__lhs, __rhs.clone())
    }
}
impl CppMove for SocccAllocator_char_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for SocccAllocator_char_ {
    fn default() -> Self {
        { SocccAllocator_char_ { count_: 0_i32 } }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct SocccAllocator_wchar_t_ {
    #[offset(0)]
    pub count_: i32,
}
impl SocccAllocator_wchar_t_ {
    pub fn new(mut i: i32) -> Self {
        Self { count_: i }
    }
}
impl CppEq for SocccAllocator_wchar_t_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        SocccAllocator_wchar_t_Impl::operator_eq(__lhs, __rhs.clone())
    }
}
impl CppMove for SocccAllocator_wchar_t_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for SocccAllocator_wchar_t_ {
    fn default() -> Self {
        { SocccAllocator_wchar_t_ { count_: 0_i32 } }
    }
}
pub fn test_soccc_behavior_4() {
    {
        let ss: Value<std_basic_ostringstream_char__std_char_traits_char___SocccAllocator_char__> =
            Rc::new(RefCell::new({
                let __tmp_0: Value<SocccAllocator_char_> =
                    Rc::new(RefCell::new(SocccAllocator_char_::new({ 10 })));
                std_basic_ostringstream_char__std_char_traits_char___SocccAllocator_char__::new_1(
                    { 16_i32 },
                    { __tmp_0.as_pointer() },
                )
            }));
        let copied: Value<Vec<i8>> = Rc::new(RefCell::new(({ (*ss.borrow()).str_16() })));
        assert!(({ ({ (*copied.borrow()).get_allocator() }).count_ } == 10));
        assert!(
            ({ ({ (*({ (*ss.borrow()).rdbuf() }).upgrade().deref()).get_allocator() }).count_ }
                == 10)
        );
        assert!(({ (*copied.borrow()).len() <= 1 }));
        assert!(({ ({ (*copied.borrow()).get_allocator() }).count_ } == 11));
        let moved: Value<Vec<i8>> = Rc::new(RefCell::new(({ (*ss.borrow()).str_18() })));
        assert!(({ ({ (*moved.borrow()).get_allocator() }).count_ } == 10));
        assert!(
            ({ ({ (*({ (*ss.borrow()).rdbuf() }).upgrade().deref()).get_allocator() }).count_ }
                == 10)
        );
        assert!(({ (*moved.borrow()).len() <= 1 }));
    }
}
pub fn test_soccc_behavior_5() {
    {
        let ss: Value<
            std_basic_ostringstream_wchar_t__std_char_traits_wchar_t___SocccAllocator_wchar_t__,
        > = Rc::new(RefCell::new({
            let __tmp_1: Value<SocccAllocator_wchar_t_> =
                Rc::new(RefCell::new(SocccAllocator_wchar_t_::new({ 10 })));
            std_basic_ostringstream_wchar_t__std_char_traits_wchar_t___SocccAllocator_wchar_t__ :: new_1 ( { 16_i32  } , { __tmp_1.as_pointer() } , )
        }));
        let copied: Value<Vec<i32>> = Rc::new(RefCell::new(({ (*ss.borrow()).str_16() })));
        assert!(({ ({ (*copied.borrow()).get_allocator() }).count_ } == 10));
        assert!(
            ({ ({ (*({ (*ss.borrow()).rdbuf() }).upgrade().deref()).get_allocator() }).count_ }
                == 10)
        );
        assert!(({ (*copied.borrow()).len() <= 1 }));
        assert!(({ ({ (*copied.borrow()).get_allocator() }).count_ } == 11));
        let moved: Value<Vec<i32>> = Rc::new(RefCell::new(({ (*ss.borrow()).str_18() })));
        assert!(({ ({ (*moved.borrow()).get_allocator() }).count_ } == 10));
        assert!(
            ({ ({ (*({ (*ss.borrow()).rdbuf() }).upgrade().deref()).get_allocator() }).count_ }
                == 10)
        );
        assert!(({ (*moved.borrow()).len() <= 1 }));
    }
}
pub fn test_allocation_is_pilfered_6() {
    let buf: Value<Box<[i8]>> = Rc::new(RefCell::new((0..80).map(|_| 0_i8).collect::<Box<[i8]>>()));
    let mut initial: Ptr<i8> = ({
        MultiStringTypeImpl :: as_ptr_2 ( &Rc::new(RefCell::new(MultiStringType :: new ( { Ptr::<i32 >::from_string_literal(&[97 as i32 , 32 as i32 , 118 as i32 , 101 as i32 , 114 as i32 , 121 as i32 , 32 as i32 , 108 as i32 , 111 as i32 , 110 as i32 , 103 as i32 , 32 as i32 , 115 as i32 , 116 as i32 , 114 as i32 , 105 as i32 , 110 as i32 , 103 as i32 , 32 as i32 , 116 as i32 , 104 as i32 , 97 as i32 , 116 as i32 , 32 as i32 , 101 as i32 , 120 as i32 , 99 as i32 , 101 as i32 , 101 as i32 , 100 as i32 , 115 as i32 , 32 as i32 , 116 as i32 , 104 as i32 , 101 as i32 , 32 as i32 , 115 as i32 , 109 as i32 , 97 as i32 , 108 as i32 , 108 as i32 , 32 as i32 , 115 as i32 , 116 as i32 , 114 as i32 , 105 as i32 , 110 as i32 , 103 as i32 , 32 as i32 , 111 as i32 , 112 as i32 , 116 as i32 , 105 as i32 , 109 as i32 , 105 as i32 , 122 as i32 , 97 as i32 , 116 as i32 , 105 as i32 , 111 as i32 , 110 as i32 , 32 as i32 , 98 as i32 , 117 as i32 , 102 as i32 , 102 as i32 , 101 as i32 , 114 as i32 , 32 as i32 , 108 as i32 , 101 as i32 , 110 as i32 , 103 as i32 , 116 as i32 , 104 as i32 , 0 as i32 , ] )  } , { ( ( ( ( ::std::mem::size_of::<[ i32 ; 76]>() as usize ) ) . wrapping_div ( ( ::std::mem::size_of::<i32>() as usize ) ) as usize ) ) . wrapping_sub ( 1_usize )  } , { Ptr::<u8 >::from_string_literal(&[97 as u8 , 32 as u8 , 118 as u8 , 101 as u8 , 114 as u8 , 121 as u8 , 32 as u8 , 108 as u8 , 111 as u8 , 110 as u8 , 103 as u8 , 32 as u8 , 115 as u8 , 116 as u8 , 114 as u8 , 105 as u8 , 110 as u8 , 103 as u8 , 32 as u8 , 116 as u8 , 104 as u8 , 97 as u8 , 116 as u8 , 32 as u8 , 101 as u8 , 120 as u8 , 99 as u8 , 101 as u8 , 101 as u8 , 100 as u8 , 115 as u8 , 32 as u8 , 116 as u8 , 104 as u8 , 101 as u8 , 32 as u8 , 115 as u8 , 109 as u8 , 97 as u8 , 108 as u8 , 108 as u8 , 32 as u8 , 115 as u8 , 116 as u8 , 114 as u8 , 105 as u8 , 110 as u8 , 103 as u8 , 32 as u8 , 111 as u8 , 112 as u8 , 116 as u8 , 105 as u8 , 109 as u8 , 105 as u8 , 122 as u8 , 97 as u8 , 116 as u8 , 105 as u8 , 111 as u8 , 110 as u8 , 32 as u8 , 98 as u8 , 117 as u8 , 102 as u8 , 102 as u8 , 101 as u8 , 114 as u8 , 32 as u8 , 108 as u8 , 101 as u8 , 110 as u8 , 103 as u8 , 116 as u8 , 104 as u8 , 0 as u8 , ] )  } , { ( ( ( ( ::std::mem::size_of::<[ u8 ; 76]>() as usize ) ) . wrapping_div ( ( ::std::mem::size_of::<u8>() as usize ) ) as usize ) ) . wrapping_sub ( 1_usize )  } , { Ptr::<u16 >::from_string_literal(&[97 as u16 , 32 as u16 , 118 as u16 , 101 as u16 , 114 as u16 , 121 as u16 , 32 as u16 , 108 as u16 , 111 as u16 , 110 as u16 , 103 as u16 , 32 as u16 , 115 as u16 , 116 as u16 , 114 as u16 , 105 as u16 , 110 as u16 , 103 as u16 , 32 as u16 , 116 as u16 , 104 as u16 , 97 as u16 , 116 as u16 , 32 as u16 , 101 as u16 , 120 as u16 , 99 as u16 , 101 as u16 , 101 as u16 , 100 as u16 , 115 as u16 , 32 as u16 , 116 as u16 , 104 as u16 , 101 as u16 , 32 as u16 , 115 as u16 , 109 as u16 , 97 as u16 , 108 as u16 , 108 as u16 , 32 as u16 , 115 as u16 , 116 as u16 , 114 as u16 , 105 as u16 , 110 as u16 , 103 as u16 , 32 as u16 , 111 as u16 , 112 as u16 , 116 as u16 , 105 as u16 , 109 as u16 , 105 as u16 , 122 as u16 , 97 as u16 , 116 as u16 , 105 as u16 , 111 as u16 , 110 as u16 , 32 as u16 , 98 as u16 , 117 as u16 , 102 as u16 , 102 as u16 , 101 as u16 , 114 as u16 , 32 as u16 , 108 as u16 , 101 as u16 , 110 as u16 , 103 as u16 , 116 as u16 , 104 as u16 , 0 as u16 , ] )  } , { ( ( ( ( ::std::mem::size_of::<[ u16 ; 76]>() as usize ) ) . wrapping_div ( ( ::std::mem::size_of::<u16>() as usize ) ) as usize ) ) . wrapping_sub ( 1_usize )  } , { Ptr::<u32 >::from_string_literal(&[97 as u32 , 32 as u32 , 118 as u32 , 101 as u32 , 114 as u32 , 121 as u32 , 32 as u32 , 108 as u32 , 111 as u32 , 110 as u32 , 103 as u32 , 32 as u32 , 115 as u32 , 116 as u32 , 114 as u32 , 105 as u32 , 110 as u32 , 103 as u32 , 32 as u32 , 116 as u32 , 104 as u32 , 97 as u32 , 116 as u32 , 32 as u32 , 101 as u32 , 120 as u32 , 99 as u32 , 101 as u32 , 101 as u32 , 100 as u32 , 115 as u32 , 32 as u32 , 116 as u32 , 104 as u32 , 101 as u32 , 32 as u32 , 115 as u32 , 109 as u32 , 97 as u32 , 108 as u32 , 108 as u32 , 32 as u32 , 115 as u32 , 116 as u32 , 114 as u32 , 105 as u32 , 110 as u32 , 103 as u32 , 32 as u32 , 111 as u32 , 112 as u32 , 116 as u32 , 105 as u32 , 109 as u32 , 105 as u32 , 122 as u32 , 97 as u32 , 116 as u32 , 105 as u32 , 111 as u32 , 110 as u32 , 32 as u32 , 98 as u32 , 117 as u32 , 102 as u32 , 102 as u32 , 101 as u32 , 114 as u32 , 32 as u32 , 108 as u32 , 101 as u32 , 110 as u32 , 103 as u32 , 116 as u32 , 104 as u32 , 0 as u32 , ] )  } , { ( ( ( ( ::std::mem::size_of::<[ u32 ; 76]>() as usize ) ) . wrapping_div ( ( ::std::mem::size_of::<u32>() as usize ) ) as usize ) ) . wrapping_sub ( 1_usize )  } , { Ptr::<i8 >::from_string_literal(b"a very long string that exceeds the small string optimization buffer length" )  } , { ( ( ::std::mem::size_of::<[ i8 ; 76]>() as usize ) ) . wrapping_sub ( 1_usize )  } , )  )).as_pointer() , Ptr::<i8>::null()  , )
    });
    {
        ({ set_default_resource_7(({ null_memory_resource_8() })) });
        let mr1: Value<std_pmr_monotonic_buffer_resource> =
            Rc::new(RefCell::new(std_pmr_monotonic_buffer_resource::new_1(
                { ((buf.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any() },
                { ::std::mem::size_of::<[i8; 80]>() },
                { ({ null_memory_resource_8() }) },
            )));
        let ss : Value<std_basic_ostringstream_char__std_char_traits_char___std_pmr_polymorphic_allocator_char__ > = Rc::new(RefCell::new({ let __tmp_2 : Value<Vec<i8  > > = Rc::new(RefCell::new({let mut __bytes = initial .to_c_bytes();
    __bytes.push(0_i8);
    __bytes} )); std_basic_ostringstream_char__std_char_traits_char___std_pmr_polymorphic_allocator_char__ :: new_1 ( { __tmp_2.as_pointer() } , None , )  } )) ;
        let mut s: Vec<i8> = ({ (*ss.borrow()).str_18() });
        assert!(initial.with_c_str(|__s| s[..s.len().saturating_sub(1)] == *__s));
    }
}
pub fn test_allocation_is_pilfered_9() {
    let buf: Value<Box<[i8]>> =
        Rc::new(RefCell::new((0..320).map(|_| 0_i8).collect::<Box<[i8]>>()));
    let mut initial: Ptr<i32> = ({
        MultiStringTypeImpl :: as_ptr_3 ( &Rc::new(RefCell::new(MultiStringType :: new ( { Ptr::<i32 >::from_string_literal(&[97 as i32 , 32 as i32 , 118 as i32 , 101 as i32 , 114 as i32 , 121 as i32 , 32 as i32 , 108 as i32 , 111 as i32 , 110 as i32 , 103 as i32 , 32 as i32 , 115 as i32 , 116 as i32 , 114 as i32 , 105 as i32 , 110 as i32 , 103 as i32 , 32 as i32 , 116 as i32 , 104 as i32 , 97 as i32 , 116 as i32 , 32 as i32 , 101 as i32 , 120 as i32 , 99 as i32 , 101 as i32 , 101 as i32 , 100 as i32 , 115 as i32 , 32 as i32 , 116 as i32 , 104 as i32 , 101 as i32 , 32 as i32 , 115 as i32 , 109 as i32 , 97 as i32 , 108 as i32 , 108 as i32 , 32 as i32 , 115 as i32 , 116 as i32 , 114 as i32 , 105 as i32 , 110 as i32 , 103 as i32 , 32 as i32 , 111 as i32 , 112 as i32 , 116 as i32 , 105 as i32 , 109 as i32 , 105 as i32 , 122 as i32 , 97 as i32 , 116 as i32 , 105 as i32 , 111 as i32 , 110 as i32 , 32 as i32 , 98 as i32 , 117 as i32 , 102 as i32 , 102 as i32 , 101 as i32 , 114 as i32 , 32 as i32 , 108 as i32 , 101 as i32 , 110 as i32 , 103 as i32 , 116 as i32 , 104 as i32 , 0 as i32 , ] )  } , { ( ( ( ( ::std::mem::size_of::<[ i32 ; 76]>() as usize ) ) . wrapping_div ( ( ::std::mem::size_of::<i32>() as usize ) ) as usize ) ) . wrapping_sub ( 1_usize )  } , { Ptr::<u8 >::from_string_literal(&[97 as u8 , 32 as u8 , 118 as u8 , 101 as u8 , 114 as u8 , 121 as u8 , 32 as u8 , 108 as u8 , 111 as u8 , 110 as u8 , 103 as u8 , 32 as u8 , 115 as u8 , 116 as u8 , 114 as u8 , 105 as u8 , 110 as u8 , 103 as u8 , 32 as u8 , 116 as u8 , 104 as u8 , 97 as u8 , 116 as u8 , 32 as u8 , 101 as u8 , 120 as u8 , 99 as u8 , 101 as u8 , 101 as u8 , 100 as u8 , 115 as u8 , 32 as u8 , 116 as u8 , 104 as u8 , 101 as u8 , 32 as u8 , 115 as u8 , 109 as u8 , 97 as u8 , 108 as u8 , 108 as u8 , 32 as u8 , 115 as u8 , 116 as u8 , 114 as u8 , 105 as u8 , 110 as u8 , 103 as u8 , 32 as u8 , 111 as u8 , 112 as u8 , 116 as u8 , 105 as u8 , 109 as u8 , 105 as u8 , 122 as u8 , 97 as u8 , 116 as u8 , 105 as u8 , 111 as u8 , 110 as u8 , 32 as u8 , 98 as u8 , 117 as u8 , 102 as u8 , 102 as u8 , 101 as u8 , 114 as u8 , 32 as u8 , 108 as u8 , 101 as u8 , 110 as u8 , 103 as u8 , 116 as u8 , 104 as u8 , 0 as u8 , ] )  } , { ( ( ( ( ::std::mem::size_of::<[ u8 ; 76]>() as usize ) ) . wrapping_div ( ( ::std::mem::size_of::<u8>() as usize ) ) as usize ) ) . wrapping_sub ( 1_usize )  } , { Ptr::<u16 >::from_string_literal(&[97 as u16 , 32 as u16 , 118 as u16 , 101 as u16 , 114 as u16 , 121 as u16 , 32 as u16 , 108 as u16 , 111 as u16 , 110 as u16 , 103 as u16 , 32 as u16 , 115 as u16 , 116 as u16 , 114 as u16 , 105 as u16 , 110 as u16 , 103 as u16 , 32 as u16 , 116 as u16 , 104 as u16 , 97 as u16 , 116 as u16 , 32 as u16 , 101 as u16 , 120 as u16 , 99 as u16 , 101 as u16 , 101 as u16 , 100 as u16 , 115 as u16 , 32 as u16 , 116 as u16 , 104 as u16 , 101 as u16 , 32 as u16 , 115 as u16 , 109 as u16 , 97 as u16 , 108 as u16 , 108 as u16 , 32 as u16 , 115 as u16 , 116 as u16 , 114 as u16 , 105 as u16 , 110 as u16 , 103 as u16 , 32 as u16 , 111 as u16 , 112 as u16 , 116 as u16 , 105 as u16 , 109 as u16 , 105 as u16 , 122 as u16 , 97 as u16 , 116 as u16 , 105 as u16 , 111 as u16 , 110 as u16 , 32 as u16 , 98 as u16 , 117 as u16 , 102 as u16 , 102 as u16 , 101 as u16 , 114 as u16 , 32 as u16 , 108 as u16 , 101 as u16 , 110 as u16 , 103 as u16 , 116 as u16 , 104 as u16 , 0 as u16 , ] )  } , { ( ( ( ( ::std::mem::size_of::<[ u16 ; 76]>() as usize ) ) . wrapping_div ( ( ::std::mem::size_of::<u16>() as usize ) ) as usize ) ) . wrapping_sub ( 1_usize )  } , { Ptr::<u32 >::from_string_literal(&[97 as u32 , 32 as u32 , 118 as u32 , 101 as u32 , 114 as u32 , 121 as u32 , 32 as u32 , 108 as u32 , 111 as u32 , 110 as u32 , 103 as u32 , 32 as u32 , 115 as u32 , 116 as u32 , 114 as u32 , 105 as u32 , 110 as u32 , 103 as u32 , 32 as u32 , 116 as u32 , 104 as u32 , 97 as u32 , 116 as u32 , 32 as u32 , 101 as u32 , 120 as u32 , 99 as u32 , 101 as u32 , 101 as u32 , 100 as u32 , 115 as u32 , 32 as u32 , 116 as u32 , 104 as u32 , 101 as u32 , 32 as u32 , 115 as u32 , 109 as u32 , 97 as u32 , 108 as u32 , 108 as u32 , 32 as u32 , 115 as u32 , 116 as u32 , 114 as u32 , 105 as u32 , 110 as u32 , 103 as u32 , 32 as u32 , 111 as u32 , 112 as u32 , 116 as u32 , 105 as u32 , 109 as u32 , 105 as u32 , 122 as u32 , 97 as u32 , 116 as u32 , 105 as u32 , 111 as u32 , 110 as u32 , 32 as u32 , 98 as u32 , 117 as u32 , 102 as u32 , 102 as u32 , 101 as u32 , 114 as u32 , 32 as u32 , 108 as u32 , 101 as u32 , 110 as u32 , 103 as u32 , 116 as u32 , 104 as u32 , 0 as u32 , ] )  } , { ( ( ( ( ::std::mem::size_of::<[ u32 ; 76]>() as usize ) ) . wrapping_div ( ( ::std::mem::size_of::<u32>() as usize ) ) as usize ) ) . wrapping_sub ( 1_usize )  } , { Ptr::<i8 >::from_string_literal(b"a very long string that exceeds the small string optimization buffer length" )  } , { ( ( ::std::mem::size_of::<[ i8 ; 76]>() as usize ) ) . wrapping_sub ( 1_usize )  } , )  )).as_pointer() , Ptr::<i32>::null()  , )
    });
    {
        ({ set_default_resource_7(({ null_memory_resource_8() })) });
        let mr1: Value<std_pmr_monotonic_buffer_resource> =
            Rc::new(RefCell::new(std_pmr_monotonic_buffer_resource::new_1(
                { ((buf.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any() },
                { ::std::mem::size_of::<[i8; 320]>() },
                { ({ null_memory_resource_8() }) },
            )));
        let ss : Value<std_basic_ostringstream_wchar_t__std_char_traits_wchar_t___std_pmr_polymorphic_allocator_wchar_t__ > = Rc::new(RefCell::new({ let __tmp_3 : Value<Vec<i32  > > = Rc::new(RefCell::new({let mut __bytes = initial .to_c_bytes();
    __bytes.push(0_i32);
    __bytes} )); std_basic_ostringstream_wchar_t__std_char_traits_wchar_t___std_pmr_polymorphic_allocator_wchar_t__ :: new_1 ( { __tmp_3.as_pointer() } , None , )  } )) ;
        let mut s: Vec<i32> = ({ (*ss.borrow()).str_18() });
        assert!(initial.with_c_str(|__s| s[..s.len().saturating_sub(1)] == *__s));
    }
}
pub fn test_no_foreign_allocations_10() {
    let mut initial: Ptr<i8> = ({
        MultiStringTypeImpl :: as_ptr_2 ( &Rc::new(RefCell::new(MultiStringType :: new ( { Ptr::<i32 >::from_string_literal(&[97 as i32 , 32 as i32 , 118 as i32 , 101 as i32 , 114 as i32 , 121 as i32 , 32 as i32 , 108 as i32 , 111 as i32 , 110 as i32 , 103 as i32 , 32 as i32 , 115 as i32 , 116 as i32 , 114 as i32 , 105 as i32 , 110 as i32 , 103 as i32 , 32 as i32 , 116 as i32 , 104 as i32 , 97 as i32 , 116 as i32 , 32 as i32 , 101 as i32 , 120 as i32 , 99 as i32 , 101 as i32 , 101 as i32 , 100 as i32 , 115 as i32 , 32 as i32 , 116 as i32 , 104 as i32 , 101 as i32 , 32 as i32 , 115 as i32 , 109 as i32 , 97 as i32 , 108 as i32 , 108 as i32 , 32 as i32 , 115 as i32 , 116 as i32 , 114 as i32 , 105 as i32 , 110 as i32 , 103 as i32 , 32 as i32 , 111 as i32 , 112 as i32 , 116 as i32 , 105 as i32 , 109 as i32 , 105 as i32 , 122 as i32 , 97 as i32 , 116 as i32 , 105 as i32 , 111 as i32 , 110 as i32 , 32 as i32 , 98 as i32 , 117 as i32 , 102 as i32 , 102 as i32 , 101 as i32 , 114 as i32 , 32 as i32 , 108 as i32 , 101 as i32 , 110 as i32 , 103 as i32 , 116 as i32 , 104 as i32 , 0 as i32 , ] )  } , { ( ( ( ( ::std::mem::size_of::<[ i32 ; 76]>() as usize ) ) . wrapping_div ( ( ::std::mem::size_of::<i32>() as usize ) ) as usize ) ) . wrapping_sub ( 1_usize )  } , { Ptr::<u8 >::from_string_literal(&[97 as u8 , 32 as u8 , 118 as u8 , 101 as u8 , 114 as u8 , 121 as u8 , 32 as u8 , 108 as u8 , 111 as u8 , 110 as u8 , 103 as u8 , 32 as u8 , 115 as u8 , 116 as u8 , 114 as u8 , 105 as u8 , 110 as u8 , 103 as u8 , 32 as u8 , 116 as u8 , 104 as u8 , 97 as u8 , 116 as u8 , 32 as u8 , 101 as u8 , 120 as u8 , 99 as u8 , 101 as u8 , 101 as u8 , 100 as u8 , 115 as u8 , 32 as u8 , 116 as u8 , 104 as u8 , 101 as u8 , 32 as u8 , 115 as u8 , 109 as u8 , 97 as u8 , 108 as u8 , 108 as u8 , 32 as u8 , 115 as u8 , 116 as u8 , 114 as u8 , 105 as u8 , 110 as u8 , 103 as u8 , 32 as u8 , 111 as u8 , 112 as u8 , 116 as u8 , 105 as u8 , 109 as u8 , 105 as u8 , 122 as u8 , 97 as u8 , 116 as u8 , 105 as u8 , 111 as u8 , 110 as u8 , 32 as u8 , 98 as u8 , 117 as u8 , 102 as u8 , 102 as u8 , 101 as u8 , 114 as u8 , 32 as u8 , 108 as u8 , 101 as u8 , 110 as u8 , 103 as u8 , 116 as u8 , 104 as u8 , 0 as u8 , ] )  } , { ( ( ( ( ::std::mem::size_of::<[ u8 ; 76]>() as usize ) ) . wrapping_div ( ( ::std::mem::size_of::<u8>() as usize ) ) as usize ) ) . wrapping_sub ( 1_usize )  } , { Ptr::<u16 >::from_string_literal(&[97 as u16 , 32 as u16 , 118 as u16 , 101 as u16 , 114 as u16 , 121 as u16 , 32 as u16 , 108 as u16 , 111 as u16 , 110 as u16 , 103 as u16 , 32 as u16 , 115 as u16 , 116 as u16 , 114 as u16 , 105 as u16 , 110 as u16 , 103 as u16 , 32 as u16 , 116 as u16 , 104 as u16 , 97 as u16 , 116 as u16 , 32 as u16 , 101 as u16 , 120 as u16 , 99 as u16 , 101 as u16 , 101 as u16 , 100 as u16 , 115 as u16 , 32 as u16 , 116 as u16 , 104 as u16 , 101 as u16 , 32 as u16 , 115 as u16 , 109 as u16 , 97 as u16 , 108 as u16 , 108 as u16 , 32 as u16 , 115 as u16 , 116 as u16 , 114 as u16 , 105 as u16 , 110 as u16 , 103 as u16 , 32 as u16 , 111 as u16 , 112 as u16 , 116 as u16 , 105 as u16 , 109 as u16 , 105 as u16 , 122 as u16 , 97 as u16 , 116 as u16 , 105 as u16 , 111 as u16 , 110 as u16 , 32 as u16 , 98 as u16 , 117 as u16 , 102 as u16 , 102 as u16 , 101 as u16 , 114 as u16 , 32 as u16 , 108 as u16 , 101 as u16 , 110 as u16 , 103 as u16 , 116 as u16 , 104 as u16 , 0 as u16 , ] )  } , { ( ( ( ( ::std::mem::size_of::<[ u16 ; 76]>() as usize ) ) . wrapping_div ( ( ::std::mem::size_of::<u16>() as usize ) ) as usize ) ) . wrapping_sub ( 1_usize )  } , { Ptr::<u32 >::from_string_literal(&[97 as u32 , 32 as u32 , 118 as u32 , 101 as u32 , 114 as u32 , 121 as u32 , 32 as u32 , 108 as u32 , 111 as u32 , 110 as u32 , 103 as u32 , 32 as u32 , 115 as u32 , 116 as u32 , 114 as u32 , 105 as u32 , 110 as u32 , 103 as u32 , 32 as u32 , 116 as u32 , 104 as u32 , 97 as u32 , 116 as u32 , 32 as u32 , 101 as u32 , 120 as u32 , 99 as u32 , 101 as u32 , 101 as u32 , 100 as u32 , 115 as u32 , 32 as u32 , 116 as u32 , 104 as u32 , 101 as u32 , 32 as u32 , 115 as u32 , 109 as u32 , 97 as u32 , 108 as u32 , 108 as u32 , 32 as u32 , 115 as u32 , 116 as u32 , 114 as u32 , 105 as u32 , 110 as u32 , 103 as u32 , 32 as u32 , 111 as u32 , 112 as u32 , 116 as u32 , 105 as u32 , 109 as u32 , 105 as u32 , 122 as u32 , 97 as u32 , 116 as u32 , 105 as u32 , 111 as u32 , 110 as u32 , 32 as u32 , 98 as u32 , 117 as u32 , 102 as u32 , 102 as u32 , 101 as u32 , 114 as u32 , 32 as u32 , 108 as u32 , 101 as u32 , 110 as u32 , 103 as u32 , 116 as u32 , 104 as u32 , 0 as u32 , ] )  } , { ( ( ( ( ::std::mem::size_of::<[ u32 ; 76]>() as usize ) ) . wrapping_div ( ( ::std::mem::size_of::<u32>() as usize ) ) as usize ) ) . wrapping_sub ( 1_usize )  } , { Ptr::<i8 >::from_string_literal(b"a very long string that exceeds the small string optimization buffer length" )  } , { ( ( ::std::mem::size_of::<[ i8 ; 76]>() as usize ) ) . wrapping_sub ( 1_usize )  } , )  )).as_pointer() , Ptr::<i8>::null()  , )
    });
    {
        ({ set_default_resource_7(({ null_memory_resource_8() })) });
        let mr1: Value<std_pmr_monotonic_buffer_resource> =
            Rc::new(RefCell::new(std_pmr_monotonic_buffer_resource::new_2({
                ({ new_delete_resource_11() })
            })));
        let ss : Value<std_basic_ostringstream_char__std_char_traits_char___std_pmr_polymorphic_allocator_char__ > = Rc::new(RefCell::new({ let __tmp_4 : Value<Vec<i8  > > = Rc::new(RefCell::new({let mut __bytes = initial .to_c_bytes();
    __bytes.push(0_i8);
    __bytes} )); std_basic_ostringstream_char__std_char_traits_char___std_pmr_polymorphic_allocator_char__ :: new_1 ( { __tmp_4.as_pointer() } , None , )  } )) ;
        assert!(
            (({
                ({ (*({ (*ss.borrow()).rdbuf() }).upgrade().deref()).get_allocator() }).resource()
            }) == (mr1.as_pointer()))
        );
        let copied: Value<Vec<i8>> = Rc::new(RefCell::new(({ (*ss.borrow()).str_16() })));
        assert!((({ ({ (*copied.borrow()).get_allocator() }).resource() }) == (mr1.as_pointer())));
        assert!(
            (({
                ({ (*({ (*ss.borrow()).rdbuf() }).upgrade().deref()).get_allocator() }).resource()
            }) == (mr1.as_pointer()))
        );
        assert!(
            ({
                initial.with_c_str(|__s| {
                    (*copied.borrow())[..(*copied.borrow()).len().saturating_sub(1)] == *__s
                })
            })
        );
        let moved: Value<Vec<i8>> = Rc::new(RefCell::new(({ (*ss.borrow()).str_18() })));
        assert!((({ ({ (*moved.borrow()).get_allocator() }).resource() }) == (mr1.as_pointer())));
        assert!(
            (({
                ({ (*({ (*ss.borrow()).rdbuf() }).upgrade().deref()).get_allocator() }).resource()
            }) == (mr1.as_pointer()))
        );
        assert!(
            ({
                initial.with_c_str(|__s| {
                    (*moved.borrow())[..(*moved.borrow()).len().saturating_sub(1)] == *__s
                })
            })
        );
    }
}
pub fn test_no_foreign_allocations_12() {
    let mut initial: Ptr<i32> = ({
        MultiStringTypeImpl :: as_ptr_3 ( &Rc::new(RefCell::new(MultiStringType :: new ( { Ptr::<i32 >::from_string_literal(&[97 as i32 , 32 as i32 , 118 as i32 , 101 as i32 , 114 as i32 , 121 as i32 , 32 as i32 , 108 as i32 , 111 as i32 , 110 as i32 , 103 as i32 , 32 as i32 , 115 as i32 , 116 as i32 , 114 as i32 , 105 as i32 , 110 as i32 , 103 as i32 , 32 as i32 , 116 as i32 , 104 as i32 , 97 as i32 , 116 as i32 , 32 as i32 , 101 as i32 , 120 as i32 , 99 as i32 , 101 as i32 , 101 as i32 , 100 as i32 , 115 as i32 , 32 as i32 , 116 as i32 , 104 as i32 , 101 as i32 , 32 as i32 , 115 as i32 , 109 as i32 , 97 as i32 , 108 as i32 , 108 as i32 , 32 as i32 , 115 as i32 , 116 as i32 , 114 as i32 , 105 as i32 , 110 as i32 , 103 as i32 , 32 as i32 , 111 as i32 , 112 as i32 , 116 as i32 , 105 as i32 , 109 as i32 , 105 as i32 , 122 as i32 , 97 as i32 , 116 as i32 , 105 as i32 , 111 as i32 , 110 as i32 , 32 as i32 , 98 as i32 , 117 as i32 , 102 as i32 , 102 as i32 , 101 as i32 , 114 as i32 , 32 as i32 , 108 as i32 , 101 as i32 , 110 as i32 , 103 as i32 , 116 as i32 , 104 as i32 , 0 as i32 , ] )  } , { ( ( ( ( ::std::mem::size_of::<[ i32 ; 76]>() as usize ) ) . wrapping_div ( ( ::std::mem::size_of::<i32>() as usize ) ) as usize ) ) . wrapping_sub ( 1_usize )  } , { Ptr::<u8 >::from_string_literal(&[97 as u8 , 32 as u8 , 118 as u8 , 101 as u8 , 114 as u8 , 121 as u8 , 32 as u8 , 108 as u8 , 111 as u8 , 110 as u8 , 103 as u8 , 32 as u8 , 115 as u8 , 116 as u8 , 114 as u8 , 105 as u8 , 110 as u8 , 103 as u8 , 32 as u8 , 116 as u8 , 104 as u8 , 97 as u8 , 116 as u8 , 32 as u8 , 101 as u8 , 120 as u8 , 99 as u8 , 101 as u8 , 101 as u8 , 100 as u8 , 115 as u8 , 32 as u8 , 116 as u8 , 104 as u8 , 101 as u8 , 32 as u8 , 115 as u8 , 109 as u8 , 97 as u8 , 108 as u8 , 108 as u8 , 32 as u8 , 115 as u8 , 116 as u8 , 114 as u8 , 105 as u8 , 110 as u8 , 103 as u8 , 32 as u8 , 111 as u8 , 112 as u8 , 116 as u8 , 105 as u8 , 109 as u8 , 105 as u8 , 122 as u8 , 97 as u8 , 116 as u8 , 105 as u8 , 111 as u8 , 110 as u8 , 32 as u8 , 98 as u8 , 117 as u8 , 102 as u8 , 102 as u8 , 101 as u8 , 114 as u8 , 32 as u8 , 108 as u8 , 101 as u8 , 110 as u8 , 103 as u8 , 116 as u8 , 104 as u8 , 0 as u8 , ] )  } , { ( ( ( ( ::std::mem::size_of::<[ u8 ; 76]>() as usize ) ) . wrapping_div ( ( ::std::mem::size_of::<u8>() as usize ) ) as usize ) ) . wrapping_sub ( 1_usize )  } , { Ptr::<u16 >::from_string_literal(&[97 as u16 , 32 as u16 , 118 as u16 , 101 as u16 , 114 as u16 , 121 as u16 , 32 as u16 , 108 as u16 , 111 as u16 , 110 as u16 , 103 as u16 , 32 as u16 , 115 as u16 , 116 as u16 , 114 as u16 , 105 as u16 , 110 as u16 , 103 as u16 , 32 as u16 , 116 as u16 , 104 as u16 , 97 as u16 , 116 as u16 , 32 as u16 , 101 as u16 , 120 as u16 , 99 as u16 , 101 as u16 , 101 as u16 , 100 as u16 , 115 as u16 , 32 as u16 , 116 as u16 , 104 as u16 , 101 as u16 , 32 as u16 , 115 as u16 , 109 as u16 , 97 as u16 , 108 as u16 , 108 as u16 , 32 as u16 , 115 as u16 , 116 as u16 , 114 as u16 , 105 as u16 , 110 as u16 , 103 as u16 , 32 as u16 , 111 as u16 , 112 as u16 , 116 as u16 , 105 as u16 , 109 as u16 , 105 as u16 , 122 as u16 , 97 as u16 , 116 as u16 , 105 as u16 , 111 as u16 , 110 as u16 , 32 as u16 , 98 as u16 , 117 as u16 , 102 as u16 , 102 as u16 , 101 as u16 , 114 as u16 , 32 as u16 , 108 as u16 , 101 as u16 , 110 as u16 , 103 as u16 , 116 as u16 , 104 as u16 , 0 as u16 , ] )  } , { ( ( ( ( ::std::mem::size_of::<[ u16 ; 76]>() as usize ) ) . wrapping_div ( ( ::std::mem::size_of::<u16>() as usize ) ) as usize ) ) . wrapping_sub ( 1_usize )  } , { Ptr::<u32 >::from_string_literal(&[97 as u32 , 32 as u32 , 118 as u32 , 101 as u32 , 114 as u32 , 121 as u32 , 32 as u32 , 108 as u32 , 111 as u32 , 110 as u32 , 103 as u32 , 32 as u32 , 115 as u32 , 116 as u32 , 114 as u32 , 105 as u32 , 110 as u32 , 103 as u32 , 32 as u32 , 116 as u32 , 104 as u32 , 97 as u32 , 116 as u32 , 32 as u32 , 101 as u32 , 120 as u32 , 99 as u32 , 101 as u32 , 101 as u32 , 100 as u32 , 115 as u32 , 32 as u32 , 116 as u32 , 104 as u32 , 101 as u32 , 32 as u32 , 115 as u32 , 109 as u32 , 97 as u32 , 108 as u32 , 108 as u32 , 32 as u32 , 115 as u32 , 116 as u32 , 114 as u32 , 105 as u32 , 110 as u32 , 103 as u32 , 32 as u32 , 111 as u32 , 112 as u32 , 116 as u32 , 105 as u32 , 109 as u32 , 105 as u32 , 122 as u32 , 97 as u32 , 116 as u32 , 105 as u32 , 111 as u32 , 110 as u32 , 32 as u32 , 98 as u32 , 117 as u32 , 102 as u32 , 102 as u32 , 101 as u32 , 114 as u32 , 32 as u32 , 108 as u32 , 101 as u32 , 110 as u32 , 103 as u32 , 116 as u32 , 104 as u32 , 0 as u32 , ] )  } , { ( ( ( ( ::std::mem::size_of::<[ u32 ; 76]>() as usize ) ) . wrapping_div ( ( ::std::mem::size_of::<u32>() as usize ) ) as usize ) ) . wrapping_sub ( 1_usize )  } , { Ptr::<i8 >::from_string_literal(b"a very long string that exceeds the small string optimization buffer length" )  } , { ( ( ::std::mem::size_of::<[ i8 ; 76]>() as usize ) ) . wrapping_sub ( 1_usize )  } , )  )).as_pointer() , Ptr::<i32>::null()  , )
    });
    {
        ({ set_default_resource_7(({ null_memory_resource_8() })) });
        let mr1: Value<std_pmr_monotonic_buffer_resource> =
            Rc::new(RefCell::new(std_pmr_monotonic_buffer_resource::new_2({
                ({ new_delete_resource_11() })
            })));
        let ss : Value<std_basic_ostringstream_wchar_t__std_char_traits_wchar_t___std_pmr_polymorphic_allocator_wchar_t__ > = Rc::new(RefCell::new({ let __tmp_5 : Value<Vec<i32  > > = Rc::new(RefCell::new({let mut __bytes = initial .to_c_bytes();
    __bytes.push(0_i32);
    __bytes} )); std_basic_ostringstream_wchar_t__std_char_traits_wchar_t___std_pmr_polymorphic_allocator_wchar_t__ :: new_1 ( { __tmp_5.as_pointer() } , None , )  } )) ;
        assert!(
            (({
                ({ (*({ (*ss.borrow()).rdbuf() }).upgrade().deref()).get_allocator() }).resource()
            }) == (mr1.as_pointer()))
        );
        let copied: Value<Vec<i32>> = Rc::new(RefCell::new(({ (*ss.borrow()).str_16() })));
        assert!((({ ({ (*copied.borrow()).get_allocator() }).resource() }) == (mr1.as_pointer())));
        assert!(
            (({
                ({ (*({ (*ss.borrow()).rdbuf() }).upgrade().deref()).get_allocator() }).resource()
            }) == (mr1.as_pointer()))
        );
        assert!(
            ({
                initial.with_c_str(|__s| {
                    (*copied.borrow())[..(*copied.borrow()).len().saturating_sub(1)] == *__s
                })
            })
        );
        let moved: Value<Vec<i32>> = Rc::new(RefCell::new(({ (*ss.borrow()).str_18() })));
        assert!((({ ({ (*moved.borrow()).get_allocator() }).resource() }) == (mr1.as_pointer())));
        assert!(
            (({
                ({ (*({ (*ss.borrow()).rdbuf() }).upgrade().deref()).get_allocator() }).resource()
            }) == (mr1.as_pointer()))
        );
        assert!(
            ({
                initial.with_c_str(|__s| {
                    (*moved.borrow())[..(*moved.borrow()).len().saturating_sub(1)] == *__s
                })
            })
        );
    }
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({ test_soccc_behavior_4() });
    ({ test_allocation_is_pilfered_6() });
    ({ test_no_foreign_allocations_10() });
    ({ test_soccc_behavior_5() });
    ({ test_allocation_is_pilfered_9() });
    ({ test_no_foreign_allocations_12() });
    return 0;
}
pub trait MultiStringTypeImpl {
    fn as_ptr_2(&self, _a0: Ptr<i8>) -> Ptr<i8>;
    fn as_ptr_3(&self, _a0: Ptr<i32>) -> Ptr<i32>;
    fn as_ptr_4(&self, _a0: Ptr<u8>) -> Ptr<u8>;
    fn as_ptr_5(&self, _a0: Ptr<u16>) -> Ptr<u16>;
    fn as_ptr_6(&self, _a0: Ptr<u32>) -> Ptr<u32>;
    fn length_7(&self, _a0: Ptr<i8>) -> usize;
    fn length_8(&self, _a0: Ptr<i32>) -> usize;
    fn length_9(&self, _a0: Ptr<u8>) -> usize;
    fn length_10(&self, _a0: Ptr<u16>) -> usize;
    fn length_11(&self, _a0: Ptr<u32>) -> usize;
    fn to_pconsti8(&self) -> Ptr<i8>;
    fn to_pconsti32(&self) -> Ptr<i32>;
    fn to_pconstu8(&self) -> Ptr<u8>;
    fn to_pconstu16(&self) -> Ptr<u16>;
    fn to_pconstu32(&self) -> Ptr<u32>;
}
impl MultiStringTypeImpl for Ptr<MultiStringType> {
    fn as_ptr_2(&self, mut _a0: Ptr<i8>) -> Ptr<i8> {
        return (*self).with(|__s| __s.s_.clone());
    }
    fn as_ptr_3(&self, mut _a0: Ptr<i32>) -> Ptr<i32> {
        return (*self).with(|__s| __s.w_.clone());
    }
    fn as_ptr_4(&self, mut _a0: Ptr<u8>) -> Ptr<u8> {
        return (*self).with(|__s| __s.u8_.clone());
    }
    fn as_ptr_5(&self, mut _a0: Ptr<u16>) -> Ptr<u16> {
        return (*self).with(|__s| __s.u16_.clone());
    }
    fn as_ptr_6(&self, mut _a0: Ptr<u32>) -> Ptr<u32> {
        return (*self).with(|__s| __s.u32_.clone());
    }
    fn length_7(&self, mut _a0: Ptr<i8>) -> usize {
        return (*self).with(|__s| __s.sn_);
    }
    fn length_8(&self, mut _a0: Ptr<i32>) -> usize {
        return (*self).with(|__s| __s.wn_);
    }
    fn length_9(&self, mut _a0: Ptr<u8>) -> usize {
        return (*self).with(|__s| __s.u8n_);
    }
    fn length_10(&self, mut _a0: Ptr<u16>) -> usize {
        return (*self).with(|__s| __s.u16n_);
    }
    fn length_11(&self, mut _a0: Ptr<u32>) -> usize {
        return (*self).with(|__s| __s.u32n_);
    }
    fn to_pconsti8(&self) -> Ptr<i8> {
        return (*self).with(|__s| __s.s_.clone());
    }
    fn to_pconsti32(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.w_.clone());
    }
    fn to_pconstu8(&self) -> Ptr<u8> {
        return (*self).with(|__s| __s.u8_.clone());
    }
    fn to_pconstu16(&self) -> Ptr<u16> {
        return (*self).with(|__s| __s.u16_.clone());
    }
    fn to_pconstu32(&self) -> Ptr<u32> {
        return (*self).with(|__s| __s.u32_.clone());
    }
}
pub trait SocccAllocator_char_Impl {
    fn allocate(&self, n: usize) -> Ptr<i8>;
    fn deallocate(&self, p: Ptr<i8>, n: usize);
    fn select_on_container_copy_construction(&self) -> SocccAllocator_char_;
    fn operator_eq(&self, _a0: Ptr<SocccAllocator_char_>) -> bool;
}
impl SocccAllocator_char_Impl for Ptr<SocccAllocator_char_> {
    fn allocate(&self, mut n: usize) -> Ptr<i8> {
        return ({ <std_allocator_char_>::default().allocate(n) });
    }
    fn deallocate(&self, mut p: Ptr<i8>, mut n: usize) {
        ({ <std_allocator_char_>::default().deallocate((p).clone(), n) });
    }
    fn select_on_container_copy_construction(&self) -> SocccAllocator_char_ {
        return SocccAllocator_char_::new({ ((*self).with(|__s| __s.count_) + 1) });
    }
    fn operator_eq(&self, _a0: Ptr<SocccAllocator_char_>) -> bool {
        return true;
    }
}
pub trait SocccAllocator_wchar_t_Impl {
    fn allocate(&self, n: usize) -> Ptr<i32>;
    fn deallocate(&self, p: Ptr<i32>, n: usize);
    fn select_on_container_copy_construction(&self) -> SocccAllocator_wchar_t_;
    fn operator_eq(&self, _a0: Ptr<SocccAllocator_wchar_t_>) -> bool;
}
impl SocccAllocator_wchar_t_Impl for Ptr<SocccAllocator_wchar_t_> {
    fn allocate(&self, mut n: usize) -> Ptr<i32> {
        return ({ <std_allocator_wchar_t_>::default().allocate(n) });
    }
    fn deallocate(&self, mut p: Ptr<i32>, mut n: usize) {
        ({ <std_allocator_wchar_t_>::default().deallocate((p).clone(), n) });
    }
    fn select_on_container_copy_construction(&self) -> SocccAllocator_wchar_t_ {
        return SocccAllocator_wchar_t_::new({ ((*self).with(|__s| __s.count_) + 1) });
    }
    fn operator_eq(&self, _a0: Ptr<SocccAllocator_wchar_t_>) -> bool {
        return true;
    }
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
pub trait test_allocator_statisticsImpl {
    fn clear(&self);
    fn clear_ctor_counters(&self);
}
impl test_allocator_statisticsImpl for Ptr<test_allocator_statistics> {
    fn clear(&self) {
        assert!(
            ((*self).with(|__s| __s.count) == 0)
                && (!(Ptr::<i8>::from_string_literal(b"clearing leaking allocator data?"))
                    .is_null())
        );
        field!((*self), count).write(0);
        field!((*self), time_to_throw).write(0);
        field!((*self), alloc_count).write(0);
        field!((*self), allocated_size).write(0);
        field!((*self), construct_count).write(0);
        field!((*self), destroy_count).write(0);
        field!((*self), throw_after).write(2147483647);
        ({ test_allocator_statisticsImpl::clear_ctor_counters(self) });
    }
    fn clear_ctor_counters(&self) {
        field!((*self), copied).write(0);
        field!((*self), moved).write(0);
        field!((*self), converted).write(0);
    }
}
pub trait test_allocator_void_Impl {
    fn destructor(&self);
    fn get_id(&self) -> i32;
    fn get_data(&self) -> i32;
    fn copy_assign(&self, _a0: Ptr<test_allocator_void_>) -> Ptr<test_allocator_void_>;
}
impl test_allocator_void_Impl for Ptr<test_allocator_void_> {
    fn destructor(&self) {
        field!((*self), data_).write(destructed_value_0.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_0.with(|rc| *rc.borrow()));
    }
    fn get_id(&self) -> i32 {
        return (*self).with(|__s| __s.id_);
    }
    fn get_data(&self) -> i32 {
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
}
