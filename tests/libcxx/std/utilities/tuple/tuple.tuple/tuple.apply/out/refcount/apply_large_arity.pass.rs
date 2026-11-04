extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__0_ {}
impl CppMove for always_imp_int__0_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_const_int__0_ {}
impl CppMove for always_imp_const_int__0_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int_ref__0_ {}
impl CppMove for always_imp_int_ref__0_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_const_int_ref__0_ {}
impl CppMove for always_imp_const_int_ref__0_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int_refref__0_ {}
impl CppMove for always_imp_int_refref__0_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__1_ {}
impl CppMove for always_imp_int__1_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_const_int__1_ {}
impl CppMove for always_imp_const_int__1_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int_ref__1_ {}
impl CppMove for always_imp_int_ref__1_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_const_int_ref__1_ {}
impl CppMove for always_imp_const_int_ref__1_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int_refref__1_ {}
impl CppMove for always_imp_int_refref__1_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__2_ {}
impl CppMove for always_imp_int__2_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_const_int__2_ {}
impl CppMove for always_imp_const_int__2_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int_ref__2_ {}
impl CppMove for always_imp_int_ref__2_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_const_int_ref__2_ {}
impl CppMove for always_imp_const_int_ref__2_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int_refref__2_ {}
impl CppMove for always_imp_int_refref__2_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__3_ {}
impl CppMove for always_imp_int__3_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_const_int__3_ {}
impl CppMove for always_imp_const_int__3_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int_ref__3_ {}
impl CppMove for always_imp_int_ref__3_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_const_int_ref__3_ {}
impl CppMove for always_imp_const_int_ref__3_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int_refref__3_ {}
impl CppMove for always_imp_int_refref__3_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__4_ {}
impl CppMove for always_imp_int__4_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_const_int__4_ {}
impl CppMove for always_imp_const_int__4_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int_ref__4_ {}
impl CppMove for always_imp_int_ref__4_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_const_int_ref__4_ {}
impl CppMove for always_imp_const_int_ref__4_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int_refref__4_ {}
impl CppMove for always_imp_int_refref__4_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__5_ {}
impl CppMove for always_imp_int__5_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__6_ {}
impl CppMove for always_imp_int__6_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__7_ {}
impl CppMove for always_imp_int__7_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__8_ {}
impl CppMove for always_imp_int__8_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__9_ {}
impl CppMove for always_imp_int__9_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__10_ {}
impl CppMove for always_imp_int__10_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__11_ {}
impl CppMove for always_imp_int__11_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__12_ {}
impl CppMove for always_imp_int__12_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__13_ {}
impl CppMove for always_imp_int__13_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__14_ {}
impl CppMove for always_imp_int__14_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__15_ {}
impl CppMove for always_imp_int__15_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__16_ {}
impl CppMove for always_imp_int__16_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__17_ {}
impl CppMove for always_imp_int__17_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__18_ {}
impl CppMove for always_imp_int__18_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__19_ {}
impl CppMove for always_imp_int__19_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__20_ {}
impl CppMove for always_imp_int__20_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__21_ {}
impl CppMove for always_imp_int__21_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__22_ {}
impl CppMove for always_imp_int__22_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__23_ {}
impl CppMove for always_imp_int__23_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__24_ {}
impl CppMove for always_imp_int__24_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__25_ {}
impl CppMove for always_imp_int__25_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__26_ {}
impl CppMove for always_imp_int__26_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__27_ {}
impl CppMove for always_imp_int__27_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__28_ {}
impl CppMove for always_imp_int__28_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__29_ {}
impl CppMove for always_imp_int__29_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__30_ {}
impl CppMove for always_imp_int__30_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__31_ {}
impl CppMove for always_imp_int__31_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__32_ {}
impl CppMove for always_imp_int__32_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__33_ {}
impl CppMove for always_imp_int__33_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__34_ {}
impl CppMove for always_imp_int__34_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__35_ {}
impl CppMove for always_imp_int__35_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__36_ {}
impl CppMove for always_imp_int__36_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__37_ {}
impl CppMove for always_imp_int__37_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__38_ {}
impl CppMove for always_imp_int__38_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__39_ {}
impl CppMove for always_imp_int__39_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__40_ {}
impl CppMove for always_imp_int__40_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__41_ {}
impl CppMove for always_imp_int__41_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__42_ {}
impl CppMove for always_imp_int__42_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__43_ {}
impl CppMove for always_imp_int__43_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__44_ {}
impl CppMove for always_imp_int__44_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__45_ {}
impl CppMove for always_imp_int__45_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__46_ {}
impl CppMove for always_imp_int__46_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__47_ {}
impl CppMove for always_imp_int__47_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__48_ {}
impl CppMove for always_imp_int__48_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__49_ {}
impl CppMove for always_imp_int__49_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__50_ {}
impl CppMove for always_imp_int__50_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__51_ {}
impl CppMove for always_imp_int__51_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__52_ {}
impl CppMove for always_imp_int__52_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__53_ {}
impl CppMove for always_imp_int__53_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__54_ {}
impl CppMove for always_imp_int__54_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__55_ {}
impl CppMove for always_imp_int__55_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__56_ {}
impl CppMove for always_imp_int__56_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__57_ {}
impl CppMove for always_imp_int__57_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__58_ {}
impl CppMove for always_imp_int__58_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__59_ {}
impl CppMove for always_imp_int__59_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__60_ {}
impl CppMove for always_imp_int__60_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__61_ {}
impl CppMove for always_imp_int__61_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__62_ {}
impl CppMove for always_imp_int__62_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__63_ {}
impl CppMove for always_imp_int__63_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__64_ {}
impl CppMove for always_imp_int__64_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__65_ {}
impl CppMove for always_imp_int__65_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__66_ {}
impl CppMove for always_imp_int__66_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__67_ {}
impl CppMove for always_imp_int__67_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__68_ {}
impl CppMove for always_imp_int__68_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__69_ {}
impl CppMove for always_imp_int__69_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__70_ {}
impl CppMove for always_imp_int__70_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__71_ {}
impl CppMove for always_imp_int__71_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__72_ {}
impl CppMove for always_imp_int__72_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__73_ {}
impl CppMove for always_imp_int__73_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__74_ {}
impl CppMove for always_imp_int__74_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__75_ {}
impl CppMove for always_imp_int__75_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__76_ {}
impl CppMove for always_imp_int__76_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__77_ {}
impl CppMove for always_imp_int__77_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__78_ {}
impl CppMove for always_imp_int__78_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__79_ {}
impl CppMove for always_imp_int__79_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__80_ {}
impl CppMove for always_imp_int__80_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__81_ {}
impl CppMove for always_imp_int__81_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__82_ {}
impl CppMove for always_imp_int__82_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__83_ {}
impl CppMove for always_imp_int__83_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__84_ {}
impl CppMove for always_imp_int__84_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__85_ {}
impl CppMove for always_imp_int__85_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__86_ {}
impl CppMove for always_imp_int__86_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__87_ {}
impl CppMove for always_imp_int__87_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__88_ {}
impl CppMove for always_imp_int__88_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__89_ {}
impl CppMove for always_imp_int__89_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__90_ {}
impl CppMove for always_imp_int__90_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__91_ {}
impl CppMove for always_imp_int__91_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__92_ {}
impl CppMove for always_imp_int__92_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__93_ {}
impl CppMove for always_imp_int__93_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__94_ {}
impl CppMove for always_imp_int__94_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__95_ {}
impl CppMove for always_imp_int__95_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__96_ {}
impl CppMove for always_imp_int__96_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__97_ {}
impl CppMove for always_imp_int__97_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__98_ {}
impl CppMove for always_imp_int__98_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__99_ {}
impl CppMove for always_imp_int__99_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__100_ {}
impl CppMove for always_imp_int__100_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__101_ {}
impl CppMove for always_imp_int__101_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__102_ {}
impl CppMove for always_imp_int__102_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__103_ {}
impl CppMove for always_imp_int__103_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__104_ {}
impl CppMove for always_imp_int__104_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__105_ {}
impl CppMove for always_imp_int__105_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__106_ {}
impl CppMove for always_imp_int__106_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__107_ {}
impl CppMove for always_imp_int__107_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__108_ {}
impl CppMove for always_imp_int__108_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__109_ {}
impl CppMove for always_imp_int__109_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__110_ {}
impl CppMove for always_imp_int__110_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__111_ {}
impl CppMove for always_imp_int__111_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__112_ {}
impl CppMove for always_imp_int__112_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__113_ {}
impl CppMove for always_imp_int__113_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__114_ {}
impl CppMove for always_imp_int__114_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__115_ {}
impl CppMove for always_imp_int__115_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__116_ {}
impl CppMove for always_imp_int__116_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__117_ {}
impl CppMove for always_imp_int__117_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__118_ {}
impl CppMove for always_imp_int__118_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__119_ {}
impl CppMove for always_imp_int__119_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__120_ {}
impl CppMove for always_imp_int__120_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__121_ {}
impl CppMove for always_imp_int__121_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__122_ {}
impl CppMove for always_imp_int__122_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__123_ {}
impl CppMove for always_imp_int__123_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__124_ {}
impl CppMove for always_imp_int__124_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__125_ {}
impl CppMove for always_imp_int__125_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__126_ {}
impl CppMove for always_imp_int__126_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__127_ {}
impl CppMove for always_imp_int__127_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__128_ {}
impl CppMove for always_imp_int__128_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__129_ {}
impl CppMove for always_imp_int__129_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__130_ {}
impl CppMove for always_imp_int__130_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__131_ {}
impl CppMove for always_imp_int__131_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__132_ {}
impl CppMove for always_imp_int__132_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__133_ {}
impl CppMove for always_imp_int__133_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__134_ {}
impl CppMove for always_imp_int__134_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__135_ {}
impl CppMove for always_imp_int__135_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__136_ {}
impl CppMove for always_imp_int__136_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__137_ {}
impl CppMove for always_imp_int__137_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__138_ {}
impl CppMove for always_imp_int__138_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__139_ {}
impl CppMove for always_imp_int__139_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__140_ {}
impl CppMove for always_imp_int__140_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__141_ {}
impl CppMove for always_imp_int__141_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__142_ {}
impl CppMove for always_imp_int__142_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__143_ {}
impl CppMove for always_imp_int__143_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__144_ {}
impl CppMove for always_imp_int__144_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__145_ {}
impl CppMove for always_imp_int__145_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__146_ {}
impl CppMove for always_imp_int__146_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__147_ {}
impl CppMove for always_imp_int__147_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__148_ {}
impl CppMove for always_imp_int__148_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__149_ {}
impl CppMove for always_imp_int__149_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__150_ {}
impl CppMove for always_imp_int__150_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__151_ {}
impl CppMove for always_imp_int__151_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__152_ {}
impl CppMove for always_imp_int__152_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__153_ {}
impl CppMove for always_imp_int__153_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__154_ {}
impl CppMove for always_imp_int__154_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__155_ {}
impl CppMove for always_imp_int__155_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__156_ {}
impl CppMove for always_imp_int__156_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__157_ {}
impl CppMove for always_imp_int__157_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__158_ {}
impl CppMove for always_imp_int__158_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__159_ {}
impl CppMove for always_imp_int__159_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__160_ {}
impl CppMove for always_imp_int__160_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__161_ {}
impl CppMove for always_imp_int__161_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__162_ {}
impl CppMove for always_imp_int__162_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__163_ {}
impl CppMove for always_imp_int__163_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__164_ {}
impl CppMove for always_imp_int__164_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__165_ {}
impl CppMove for always_imp_int__165_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__166_ {}
impl CppMove for always_imp_int__166_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__167_ {}
impl CppMove for always_imp_int__167_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__168_ {}
impl CppMove for always_imp_int__168_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__169_ {}
impl CppMove for always_imp_int__169_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__170_ {}
impl CppMove for always_imp_int__170_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__171_ {}
impl CppMove for always_imp_int__171_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__172_ {}
impl CppMove for always_imp_int__172_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__173_ {}
impl CppMove for always_imp_int__173_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__174_ {}
impl CppMove for always_imp_int__174_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__175_ {}
impl CppMove for always_imp_int__175_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__176_ {}
impl CppMove for always_imp_int__176_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__177_ {}
impl CppMove for always_imp_int__177_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__178_ {}
impl CppMove for always_imp_int__178_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__179_ {}
impl CppMove for always_imp_int__179_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__180_ {}
impl CppMove for always_imp_int__180_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__181_ {}
impl CppMove for always_imp_int__181_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__182_ {}
impl CppMove for always_imp_int__182_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__183_ {}
impl CppMove for always_imp_int__183_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__184_ {}
impl CppMove for always_imp_int__184_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__185_ {}
impl CppMove for always_imp_int__185_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__186_ {}
impl CppMove for always_imp_int__186_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__187_ {}
impl CppMove for always_imp_int__187_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__188_ {}
impl CppMove for always_imp_int__188_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__189_ {}
impl CppMove for always_imp_int__189_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__190_ {}
impl CppMove for always_imp_int__190_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__191_ {}
impl CppMove for always_imp_int__191_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__192_ {}
impl CppMove for always_imp_int__192_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__193_ {}
impl CppMove for always_imp_int__193_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__194_ {}
impl CppMove for always_imp_int__194_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__195_ {}
impl CppMove for always_imp_int__195_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__196_ {}
impl CppMove for always_imp_int__196_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__197_ {}
impl CppMove for always_imp_int__197_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__198_ {}
impl CppMove for always_imp_int__198_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__199_ {}
impl CppMove for always_imp_int__199_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__200_ {}
impl CppMove for always_imp_int__200_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__201_ {}
impl CppMove for always_imp_int__201_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__202_ {}
impl CppMove for always_imp_int__202_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__203_ {}
impl CppMove for always_imp_int__203_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__204_ {}
impl CppMove for always_imp_int__204_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__205_ {}
impl CppMove for always_imp_int__205_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__206_ {}
impl CppMove for always_imp_int__206_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__207_ {}
impl CppMove for always_imp_int__207_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__208_ {}
impl CppMove for always_imp_int__208_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__209_ {}
impl CppMove for always_imp_int__209_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__210_ {}
impl CppMove for always_imp_int__210_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__211_ {}
impl CppMove for always_imp_int__211_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__212_ {}
impl CppMove for always_imp_int__212_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__213_ {}
impl CppMove for always_imp_int__213_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__214_ {}
impl CppMove for always_imp_int__214_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__215_ {}
impl CppMove for always_imp_int__215_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__216_ {}
impl CppMove for always_imp_int__216_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__217_ {}
impl CppMove for always_imp_int__217_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__218_ {}
impl CppMove for always_imp_int__218_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__219_ {}
impl CppMove for always_imp_int__219_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__220_ {}
impl CppMove for always_imp_int__220_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__221_ {}
impl CppMove for always_imp_int__221_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__222_ {}
impl CppMove for always_imp_int__222_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__223_ {}
impl CppMove for always_imp_int__223_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__224_ {}
impl CppMove for always_imp_int__224_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__225_ {}
impl CppMove for always_imp_int__225_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__226_ {}
impl CppMove for always_imp_int__226_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__227_ {}
impl CppMove for always_imp_int__227_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__228_ {}
impl CppMove for always_imp_int__228_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__229_ {}
impl CppMove for always_imp_int__229_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__230_ {}
impl CppMove for always_imp_int__230_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__231_ {}
impl CppMove for always_imp_int__231_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__232_ {}
impl CppMove for always_imp_int__232_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__233_ {}
impl CppMove for always_imp_int__233_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__234_ {}
impl CppMove for always_imp_int__234_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__235_ {}
impl CppMove for always_imp_int__235_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__236_ {}
impl CppMove for always_imp_int__236_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__237_ {}
impl CppMove for always_imp_int__237_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__238_ {}
impl CppMove for always_imp_int__238_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__239_ {}
impl CppMove for always_imp_int__239_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__240_ {}
impl CppMove for always_imp_int__240_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__241_ {}
impl CppMove for always_imp_int__241_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__242_ {}
impl CppMove for always_imp_int__242_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__243_ {}
impl CppMove for always_imp_int__243_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__244_ {}
impl CppMove for always_imp_int__244_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__245_ {}
impl CppMove for always_imp_int__245_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__246_ {}
impl CppMove for always_imp_int__246_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__247_ {}
impl CppMove for always_imp_int__247_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__248_ {}
impl CppMove for always_imp_int__248_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__249_ {}
impl CppMove for always_imp_int__249_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__250_ {}
impl CppMove for always_imp_int__250_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__251_ {}
impl CppMove for always_imp_int__251_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__252_ {}
impl CppMove for always_imp_int__252_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__253_ {}
impl CppMove for always_imp_int__253_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__254_ {}
impl CppMove for always_imp_int__254_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__255_ {}
impl CppMove for always_imp_int__255_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_apply_fn_0(mut _a0: i32) -> bool {
    return true;
}
pub fn test_apply_fn_1(_a0: Ptr<i32>) -> bool {
    return true;
}
pub fn test_apply_fn_2(_a0: Ptr<i32>) -> bool {
    return true;
}
pub fn test_apply_fn_3(_a0: Ptr<i32>) -> bool {
    return true;
}
pub fn test_apply_fn_4(mut _a0: i32, mut _a1: i32) -> bool {
    return true;
}
pub fn test_apply_fn_5(_a0: Ptr<i32>, _a1: Ptr<i32>) -> bool {
    return true;
}
pub fn test_apply_fn_6(_a0: Ptr<i32>, _a1: Ptr<i32>) -> bool {
    return true;
}
pub fn test_apply_fn_7(_a0: Ptr<i32>, _a1: Ptr<i32>) -> bool {
    return true;
}
pub fn test_apply_fn_8(mut _a0: i32, mut _a1: i32, mut _a2: i32) -> bool {
    return true;
}
pub fn test_apply_fn_9(_a0: Ptr<i32>, _a1: Ptr<i32>, _a2: Ptr<i32>) -> bool {
    return true;
}
pub fn test_apply_fn_10(_a0: Ptr<i32>, _a1: Ptr<i32>, _a2: Ptr<i32>) -> bool {
    return true;
}
pub fn test_apply_fn_11(_a0: Ptr<i32>, _a1: Ptr<i32>, _a2: Ptr<i32>) -> bool {
    return true;
}
pub fn test_apply_fn_12(mut _a0: i32, mut _a1: i32, mut _a2: i32, mut _a3: i32) -> bool {
    return true;
}
pub fn test_apply_fn_13(_a0: Ptr<i32>, _a1: Ptr<i32>, _a2: Ptr<i32>, _a3: Ptr<i32>) -> bool {
    return true;
}
pub fn test_apply_fn_14(_a0: Ptr<i32>, _a1: Ptr<i32>, _a2: Ptr<i32>, _a3: Ptr<i32>) -> bool {
    return true;
}
pub fn test_apply_fn_15(_a0: Ptr<i32>, _a1: Ptr<i32>, _a2: Ptr<i32>, _a3: Ptr<i32>) -> bool {
    return true;
}
pub fn test_apply_fn_16(
    mut _a0: i32,
    mut _a1: i32,
    mut _a2: i32,
    mut _a3: i32,
    mut _a4: i32,
) -> bool {
    return true;
}
pub fn test_apply_fn_17(
    _a0: Ptr<i32>,
    _a1: Ptr<i32>,
    _a2: Ptr<i32>,
    _a3: Ptr<i32>,
    _a4: Ptr<i32>,
) -> bool {
    return true;
}
pub fn test_apply_fn_18(
    _a0: Ptr<i32>,
    _a1: Ptr<i32>,
    _a2: Ptr<i32>,
    _a3: Ptr<i32>,
    _a4: Ptr<i32>,
) -> bool {
    return true;
}
pub fn test_apply_fn_19(
    _a0: Ptr<i32>,
    _a1: Ptr<i32>,
    _a2: Ptr<i32>,
    _a3: Ptr<i32>,
    _a4: Ptr<i32>,
) -> bool {
    return true;
}
pub fn test_apply_fn_20(
    mut _a0: i32,
    mut _a1: i32,
    mut _a2: i32,
    mut _a3: i32,
    mut _a4: i32,
    mut _a5: i32,
    mut _a6: i32,
    mut _a7: i32,
    mut _a8: i32,
    mut _a9: i32,
    mut _a10: i32,
    mut _a11: i32,
    mut _a12: i32,
    mut _a13: i32,
    mut _a14: i32,
    mut _a15: i32,
    mut _a16: i32,
    mut _a17: i32,
    mut _a18: i32,
    mut _a19: i32,
    mut _a20: i32,
    mut _a21: i32,
    mut _a22: i32,
    mut _a23: i32,
    mut _a24: i32,
    mut _a25: i32,
    mut _a26: i32,
    mut _a27: i32,
    mut _a28: i32,
    mut _a29: i32,
    mut _a30: i32,
    mut _a31: i32,
    mut _a32: i32,
    mut _a33: i32,
    mut _a34: i32,
    mut _a35: i32,
    mut _a36: i32,
    mut _a37: i32,
    mut _a38: i32,
    mut _a39: i32,
    mut _a40: i32,
    mut _a41: i32,
    mut _a42: i32,
    mut _a43: i32,
    mut _a44: i32,
    mut _a45: i32,
    mut _a46: i32,
    mut _a47: i32,
    mut _a48: i32,
    mut _a49: i32,
    mut _a50: i32,
    mut _a51: i32,
    mut _a52: i32,
    mut _a53: i32,
    mut _a54: i32,
    mut _a55: i32,
    mut _a56: i32,
    mut _a57: i32,
    mut _a58: i32,
    mut _a59: i32,
    mut _a60: i32,
    mut _a61: i32,
    mut _a62: i32,
    mut _a63: i32,
    mut _a64: i32,
    mut _a65: i32,
    mut _a66: i32,
    mut _a67: i32,
    mut _a68: i32,
    mut _a69: i32,
    mut _a70: i32,
    mut _a71: i32,
    mut _a72: i32,
    mut _a73: i32,
    mut _a74: i32,
    mut _a75: i32,
    mut _a76: i32,
    mut _a77: i32,
    mut _a78: i32,
    mut _a79: i32,
    mut _a80: i32,
    mut _a81: i32,
    mut _a82: i32,
    mut _a83: i32,
    mut _a84: i32,
    mut _a85: i32,
    mut _a86: i32,
    mut _a87: i32,
    mut _a88: i32,
    mut _a89: i32,
    mut _a90: i32,
    mut _a91: i32,
    mut _a92: i32,
    mut _a93: i32,
    mut _a94: i32,
    mut _a95: i32,
    mut _a96: i32,
    mut _a97: i32,
    mut _a98: i32,
    mut _a99: i32,
    mut _a100: i32,
    mut _a101: i32,
    mut _a102: i32,
    mut _a103: i32,
    mut _a104: i32,
    mut _a105: i32,
    mut _a106: i32,
    mut _a107: i32,
    mut _a108: i32,
    mut _a109: i32,
    mut _a110: i32,
    mut _a111: i32,
    mut _a112: i32,
    mut _a113: i32,
    mut _a114: i32,
    mut _a115: i32,
    mut _a116: i32,
    mut _a117: i32,
    mut _a118: i32,
    mut _a119: i32,
    mut _a120: i32,
    mut _a121: i32,
    mut _a122: i32,
    mut _a123: i32,
    mut _a124: i32,
    mut _a125: i32,
    mut _a126: i32,
    mut _a127: i32,
    mut _a128: i32,
    mut _a129: i32,
    mut _a130: i32,
    mut _a131: i32,
    mut _a132: i32,
    mut _a133: i32,
    mut _a134: i32,
    mut _a135: i32,
    mut _a136: i32,
    mut _a137: i32,
    mut _a138: i32,
    mut _a139: i32,
    mut _a140: i32,
    mut _a141: i32,
    mut _a142: i32,
    mut _a143: i32,
    mut _a144: i32,
    mut _a145: i32,
    mut _a146: i32,
    mut _a147: i32,
    mut _a148: i32,
    mut _a149: i32,
    mut _a150: i32,
    mut _a151: i32,
    mut _a152: i32,
    mut _a153: i32,
    mut _a154: i32,
    mut _a155: i32,
    mut _a156: i32,
    mut _a157: i32,
    mut _a158: i32,
    mut _a159: i32,
    mut _a160: i32,
    mut _a161: i32,
    mut _a162: i32,
    mut _a163: i32,
    mut _a164: i32,
    mut _a165: i32,
    mut _a166: i32,
    mut _a167: i32,
    mut _a168: i32,
    mut _a169: i32,
    mut _a170: i32,
    mut _a171: i32,
    mut _a172: i32,
    mut _a173: i32,
    mut _a174: i32,
    mut _a175: i32,
    mut _a176: i32,
    mut _a177: i32,
    mut _a178: i32,
    mut _a179: i32,
    mut _a180: i32,
    mut _a181: i32,
    mut _a182: i32,
    mut _a183: i32,
    mut _a184: i32,
    mut _a185: i32,
    mut _a186: i32,
    mut _a187: i32,
    mut _a188: i32,
    mut _a189: i32,
    mut _a190: i32,
    mut _a191: i32,
    mut _a192: i32,
    mut _a193: i32,
    mut _a194: i32,
    mut _a195: i32,
    mut _a196: i32,
    mut _a197: i32,
    mut _a198: i32,
    mut _a199: i32,
    mut _a200: i32,
    mut _a201: i32,
    mut _a202: i32,
    mut _a203: i32,
    mut _a204: i32,
    mut _a205: i32,
    mut _a206: i32,
    mut _a207: i32,
    mut _a208: i32,
    mut _a209: i32,
    mut _a210: i32,
    mut _a211: i32,
    mut _a212: i32,
    mut _a213: i32,
    mut _a214: i32,
    mut _a215: i32,
    mut _a216: i32,
    mut _a217: i32,
    mut _a218: i32,
    mut _a219: i32,
    mut _a220: i32,
    mut _a221: i32,
    mut _a222: i32,
    mut _a223: i32,
    mut _a224: i32,
    mut _a225: i32,
    mut _a226: i32,
    mut _a227: i32,
    mut _a228: i32,
    mut _a229: i32,
    mut _a230: i32,
    mut _a231: i32,
    mut _a232: i32,
    mut _a233: i32,
    mut _a234: i32,
    mut _a235: i32,
    mut _a236: i32,
    mut _a237: i32,
    mut _a238: i32,
    mut _a239: i32,
    mut _a240: i32,
    mut _a241: i32,
    mut _a242: i32,
    mut _a243: i32,
    mut _a244: i32,
    mut _a245: i32,
    mut _a246: i32,
    mut _a247: i32,
    mut _a248: i32,
    mut _a249: i32,
    mut _a250: i32,
    mut _a251: i32,
    mut _a252: i32,
    mut _a253: i32,
    mut _a254: i32,
    mut _a255: i32,
) -> bool {
    return true;
}
pub fn test_all_21() {
    let val_fn: Value<FnPtr<fn(i32) -> bool>> = Rc::new(RefCell::new(
        (FnPtr::<fn(i32) -> bool>::new(test_apply_fn_0)),
    ));
    let ref_fn: Value<FnPtr<fn(Ptr<i32>) -> bool>> = Rc::new(RefCell::new(
        (FnPtr::<fn(Ptr<i32>) -> bool>::new(test_apply_fn_1)),
    ));
    let cref_fn: Value<FnPtr<fn(Ptr<i32>) -> bool>> = Rc::new(RefCell::new(
        (FnPtr::<fn(Ptr<i32>) -> bool>::new(test_apply_fn_2)),
    ));
    let rref_fn: Value<FnPtr<fn(Ptr<i32>) -> bool>> = Rc::new(RefCell::new(
        (FnPtr::<fn(Ptr<i32>) -> bool>::new(test_apply_fn_3)),
    ));
    {
        let a: Value<std_array_int__1_> = Rc::new(RefCell::new(<std_array_int__1_>::default()));
        ({
            cpp2rust_assert_fail_22((({ apply_23(val_fn.as_pointer(), a.as_pointer()) }) as bool))
        });
        ({
            cpp2rust_assert_fail_22((({ apply_24(ref_fn.as_pointer(), a.as_pointer()) }) as bool))
        });
        ({
            cpp2rust_assert_fail_22((({ apply_25(cref_fn.as_pointer(), a.as_pointer()) }) as bool))
        });
        ({
            cpp2rust_assert_fail_22((({ apply_26(rref_fn.as_pointer(), a.as_pointer()) }) as bool))
        });
    }
    {
        let a: Value<std_array_const_int__1_> =
            Rc::new(RefCell::new(<std_array_const_int__1_>::default()));
        ({
            cpp2rust_assert_fail_22((({ apply_27(val_fn.as_pointer(), a.as_pointer()) }) as bool))
        });
        ({
            cpp2rust_assert_fail_22((({ apply_28(cref_fn.as_pointer(), a.as_pointer()) }) as bool))
        });
    }
    {
        let a: Value<std_tuple_int_> = Rc::new(RefCell::new(<std_tuple_int_>::default()));
        ({
            cpp2rust_assert_fail_22((({ apply_29(val_fn.as_pointer(), a.as_pointer()) }) as bool))
        });
        ({
            cpp2rust_assert_fail_22((({ apply_30(ref_fn.as_pointer(), a.as_pointer()) }) as bool))
        });
        ({
            cpp2rust_assert_fail_22((({ apply_31(cref_fn.as_pointer(), a.as_pointer()) }) as bool))
        });
        ({
            cpp2rust_assert_fail_22((({ apply_32(rref_fn.as_pointer(), a.as_pointer()) }) as bool))
        });
    }
    {
        let a: Value<std_tuple_const_int_> =
            Rc::new(RefCell::new(<std_tuple_const_int_>::default()));
        ({
            cpp2rust_assert_fail_22((({ apply_33(val_fn.as_pointer(), a.as_pointer()) }) as bool))
        });
        ({
            cpp2rust_assert_fail_22((({ apply_34(cref_fn.as_pointer(), a.as_pointer()) }) as bool))
        });
    }
}
pub fn test_all_35() {
    let val_fn: Value<FnPtr<fn(i32, i32) -> bool>> = Rc::new(RefCell::new(
        (FnPtr::<fn(i32, i32) -> bool>::new(test_apply_fn_4)),
    ));
    let ref_fn: Value<FnPtr<fn(Ptr<i32>, Ptr<i32>) -> bool>> = Rc::new(RefCell::new(
        (FnPtr::<fn(Ptr<i32>, Ptr<i32>) -> bool>::new(test_apply_fn_5)),
    ));
    let cref_fn: Value<FnPtr<fn(Ptr<i32>, Ptr<i32>) -> bool>> = Rc::new(RefCell::new(
        (FnPtr::<fn(Ptr<i32>, Ptr<i32>) -> bool>::new(test_apply_fn_6)),
    ));
    let rref_fn: Value<FnPtr<fn(Ptr<i32>, Ptr<i32>) -> bool>> = Rc::new(RefCell::new(
        (FnPtr::<fn(Ptr<i32>, Ptr<i32>) -> bool>::new(test_apply_fn_7)),
    ));
    {
        let a: Value<std_array_int__2_> = Rc::new(RefCell::new(<std_array_int__2_>::default()));
        ({
            cpp2rust_assert_fail_22((({ apply_36(val_fn.as_pointer(), a.as_pointer()) }) as bool))
        });
        ({
            cpp2rust_assert_fail_22((({ apply_37(ref_fn.as_pointer(), a.as_pointer()) }) as bool))
        });
        ({
            cpp2rust_assert_fail_22((({ apply_38(cref_fn.as_pointer(), a.as_pointer()) }) as bool))
        });
        ({
            cpp2rust_assert_fail_22((({ apply_39(rref_fn.as_pointer(), a.as_pointer()) }) as bool))
        });
    }
    {
        let a: Value<std_array_const_int__2_> =
            Rc::new(RefCell::new(<std_array_const_int__2_>::default()));
        ({
            cpp2rust_assert_fail_22((({ apply_40(val_fn.as_pointer(), a.as_pointer()) }) as bool))
        });
        ({
            cpp2rust_assert_fail_22((({ apply_41(cref_fn.as_pointer(), a.as_pointer()) }) as bool))
        });
    }
    {
        let a: Value<std_tuple_int__int_> = Rc::new(RefCell::new(<std_tuple_int__int_>::default()));
        ({
            cpp2rust_assert_fail_22((({ apply_42(val_fn.as_pointer(), a.as_pointer()) }) as bool))
        });
        ({
            cpp2rust_assert_fail_22((({ apply_43(ref_fn.as_pointer(), a.as_pointer()) }) as bool))
        });
        ({
            cpp2rust_assert_fail_22((({ apply_44(cref_fn.as_pointer(), a.as_pointer()) }) as bool))
        });
        ({
            cpp2rust_assert_fail_22((({ apply_45(rref_fn.as_pointer(), a.as_pointer()) }) as bool))
        });
    }
    {
        let a: Value<std_tuple_const_int__const_int_> =
            Rc::new(RefCell::new(<std_tuple_const_int__const_int_>::default()));
        ({
            cpp2rust_assert_fail_22((({ apply_46(val_fn.as_pointer(), a.as_pointer()) }) as bool))
        });
        ({
            cpp2rust_assert_fail_22((({ apply_47(cref_fn.as_pointer(), a.as_pointer()) }) as bool))
        });
    }
}
pub fn test_all_48() {
    let val_fn: Value<FnPtr<fn(i32, i32, i32) -> bool>> = Rc::new(RefCell::new(
        (FnPtr::<fn(i32, i32, i32) -> bool>::new(test_apply_fn_8)),
    ));
    let ref_fn: Value<FnPtr<fn(Ptr<i32>, Ptr<i32>, Ptr<i32>) -> bool>> = Rc::new(RefCell::new(
        (FnPtr::<fn(Ptr<i32>, Ptr<i32>, Ptr<i32>) -> bool>::new(test_apply_fn_9)),
    ));
    let cref_fn: Value<FnPtr<fn(Ptr<i32>, Ptr<i32>, Ptr<i32>) -> bool>> = Rc::new(RefCell::new(
        (FnPtr::<fn(Ptr<i32>, Ptr<i32>, Ptr<i32>) -> bool>::new(test_apply_fn_10)),
    ));
    let rref_fn: Value<FnPtr<fn(Ptr<i32>, Ptr<i32>, Ptr<i32>) -> bool>> = Rc::new(RefCell::new(
        (FnPtr::<fn(Ptr<i32>, Ptr<i32>, Ptr<i32>) -> bool>::new(test_apply_fn_11)),
    ));
    {
        let a: Value<std_array_int__3_> = Rc::new(RefCell::new(<std_array_int__3_>::default()));
        ({
            cpp2rust_assert_fail_22((({ apply_49(val_fn.as_pointer(), a.as_pointer()) }) as bool))
        });
        ({
            cpp2rust_assert_fail_22((({ apply_50(ref_fn.as_pointer(), a.as_pointer()) }) as bool))
        });
        ({
            cpp2rust_assert_fail_22((({ apply_51(cref_fn.as_pointer(), a.as_pointer()) }) as bool))
        });
        ({
            cpp2rust_assert_fail_22((({ apply_52(rref_fn.as_pointer(), a.as_pointer()) }) as bool))
        });
    }
    {
        let a: Value<std_array_const_int__3_> =
            Rc::new(RefCell::new(<std_array_const_int__3_>::default()));
        ({
            cpp2rust_assert_fail_22((({ apply_53(val_fn.as_pointer(), a.as_pointer()) }) as bool))
        });
        ({
            cpp2rust_assert_fail_22((({ apply_54(cref_fn.as_pointer(), a.as_pointer()) }) as bool))
        });
    }
    {
        let a: Value<std_tuple_int__int__int_> =
            Rc::new(RefCell::new(<std_tuple_int__int__int_>::default()));
        ({
            cpp2rust_assert_fail_22((({ apply_55(val_fn.as_pointer(), a.as_pointer()) }) as bool))
        });
        ({
            cpp2rust_assert_fail_22((({ apply_56(ref_fn.as_pointer(), a.as_pointer()) }) as bool))
        });
        ({
            cpp2rust_assert_fail_22((({ apply_57(cref_fn.as_pointer(), a.as_pointer()) }) as bool))
        });
        ({
            cpp2rust_assert_fail_22((({ apply_58(rref_fn.as_pointer(), a.as_pointer()) }) as bool))
        });
    }
    {
        let a: Value<std_tuple_const_int__const_int__const_int_> = Rc::new(RefCell::new(
            <std_tuple_const_int__const_int__const_int_>::default(),
        ));
        ({
            cpp2rust_assert_fail_22((({ apply_59(val_fn.as_pointer(), a.as_pointer()) }) as bool))
        });
        ({
            cpp2rust_assert_fail_22((({ apply_60(cref_fn.as_pointer(), a.as_pointer()) }) as bool))
        });
    }
}
pub fn test_all_61() {
    let val_fn: Value<FnPtr<fn(i32, i32, i32, i32) -> bool>> = Rc::new(RefCell::new(
        (FnPtr::<fn(i32, i32, i32, i32) -> bool>::new(test_apply_fn_12)),
    ));
    let ref_fn: Value<FnPtr<fn(Ptr<i32>, Ptr<i32>, Ptr<i32>, Ptr<i32>) -> bool>> =
        Rc::new(RefCell::new(
            (FnPtr::<fn(Ptr<i32>, Ptr<i32>, Ptr<i32>, Ptr<i32>) -> bool>::new(test_apply_fn_13)),
        ));
    let cref_fn: Value<FnPtr<fn(Ptr<i32>, Ptr<i32>, Ptr<i32>, Ptr<i32>) -> bool>> =
        Rc::new(RefCell::new(
            (FnPtr::<fn(Ptr<i32>, Ptr<i32>, Ptr<i32>, Ptr<i32>) -> bool>::new(test_apply_fn_14)),
        ));
    let rref_fn: Value<FnPtr<fn(Ptr<i32>, Ptr<i32>, Ptr<i32>, Ptr<i32>) -> bool>> =
        Rc::new(RefCell::new(
            (FnPtr::<fn(Ptr<i32>, Ptr<i32>, Ptr<i32>, Ptr<i32>) -> bool>::new(test_apply_fn_15)),
        ));
    {
        let a: Value<std_array_int__4_> = Rc::new(RefCell::new(<std_array_int__4_>::default()));
        ({
            cpp2rust_assert_fail_22((({ apply_62(val_fn.as_pointer(), a.as_pointer()) }) as bool))
        });
        ({
            cpp2rust_assert_fail_22((({ apply_63(ref_fn.as_pointer(), a.as_pointer()) }) as bool))
        });
        ({
            cpp2rust_assert_fail_22((({ apply_64(cref_fn.as_pointer(), a.as_pointer()) }) as bool))
        });
        ({
            cpp2rust_assert_fail_22((({ apply_65(rref_fn.as_pointer(), a.as_pointer()) }) as bool))
        });
    }
    {
        let a: Value<std_array_const_int__4_> =
            Rc::new(RefCell::new(<std_array_const_int__4_>::default()));
        ({
            cpp2rust_assert_fail_22((({ apply_66(val_fn.as_pointer(), a.as_pointer()) }) as bool))
        });
        ({
            cpp2rust_assert_fail_22((({ apply_67(cref_fn.as_pointer(), a.as_pointer()) }) as bool))
        });
    }
    {
        let a: Value<std_tuple_int__int__int__int_> =
            Rc::new(RefCell::new(<std_tuple_int__int__int__int_>::default()));
        ({
            cpp2rust_assert_fail_22((({ apply_68(val_fn.as_pointer(), a.as_pointer()) }) as bool))
        });
        ({
            cpp2rust_assert_fail_22((({ apply_69(ref_fn.as_pointer(), a.as_pointer()) }) as bool))
        });
        ({
            cpp2rust_assert_fail_22((({ apply_70(cref_fn.as_pointer(), a.as_pointer()) }) as bool))
        });
        ({
            cpp2rust_assert_fail_22((({ apply_71(rref_fn.as_pointer(), a.as_pointer()) }) as bool))
        });
    }
    {
        let a: Value<std_tuple_const_int__const_int__const_int__const_int_> = Rc::new(
            RefCell::new(<std_tuple_const_int__const_int__const_int__const_int_>::default()),
        );
        ({
            cpp2rust_assert_fail_22((({ apply_72(val_fn.as_pointer(), a.as_pointer()) }) as bool))
        });
        ({
            cpp2rust_assert_fail_22((({ apply_73(cref_fn.as_pointer(), a.as_pointer()) }) as bool))
        });
    }
}
pub fn test_all_74() {
    let val_fn: Value<FnPtr<fn(i32, i32, i32, i32, i32) -> bool>> = Rc::new(RefCell::new(
        (FnPtr::<fn(i32, i32, i32, i32, i32) -> bool>::new(test_apply_fn_16)),
    ));
    let ref_fn: Value<FnPtr<fn(Ptr<i32>, Ptr<i32>, Ptr<i32>, Ptr<i32>, Ptr<i32>) -> bool>> =
        Rc::new(RefCell::new(
            (FnPtr::<fn(Ptr<i32>, Ptr<i32>, Ptr<i32>, Ptr<i32>, Ptr<i32>) -> bool>::new(
                test_apply_fn_17,
            )),
        ));
    let cref_fn: Value<FnPtr<fn(Ptr<i32>, Ptr<i32>, Ptr<i32>, Ptr<i32>, Ptr<i32>) -> bool>> =
        Rc::new(RefCell::new(
            (FnPtr::<fn(Ptr<i32>, Ptr<i32>, Ptr<i32>, Ptr<i32>, Ptr<i32>) -> bool>::new(
                test_apply_fn_18,
            )),
        ));
    let rref_fn: Value<FnPtr<fn(Ptr<i32>, Ptr<i32>, Ptr<i32>, Ptr<i32>, Ptr<i32>) -> bool>> =
        Rc::new(RefCell::new(
            (FnPtr::<fn(Ptr<i32>, Ptr<i32>, Ptr<i32>, Ptr<i32>, Ptr<i32>) -> bool>::new(
                test_apply_fn_19,
            )),
        ));
    {
        let a: Value<std_array_int__5_> = Rc::new(RefCell::new(<std_array_int__5_>::default()));
        ({
            cpp2rust_assert_fail_22((({ apply_75(val_fn.as_pointer(), a.as_pointer()) }) as bool))
        });
        ({
            cpp2rust_assert_fail_22((({ apply_76(ref_fn.as_pointer(), a.as_pointer()) }) as bool))
        });
        ({
            cpp2rust_assert_fail_22((({ apply_77(cref_fn.as_pointer(), a.as_pointer()) }) as bool))
        });
        ({
            cpp2rust_assert_fail_22((({ apply_78(rref_fn.as_pointer(), a.as_pointer()) }) as bool))
        });
    }
    {
        let a: Value<std_array_const_int__5_> =
            Rc::new(RefCell::new(<std_array_const_int__5_>::default()));
        ({
            cpp2rust_assert_fail_22((({ apply_79(val_fn.as_pointer(), a.as_pointer()) }) as bool))
        });
        ({
            cpp2rust_assert_fail_22((({ apply_80(cref_fn.as_pointer(), a.as_pointer()) }) as bool))
        });
    }
    {
        let a: Value<std_tuple_int__int__int__int__int_> =
            Rc::new(RefCell::new(<std_tuple_int__int__int__int__int_>::default()));
        ({
            cpp2rust_assert_fail_22((({ apply_81(val_fn.as_pointer(), a.as_pointer()) }) as bool))
        });
        ({
            cpp2rust_assert_fail_22((({ apply_82(ref_fn.as_pointer(), a.as_pointer()) }) as bool))
        });
        ({
            cpp2rust_assert_fail_22((({ apply_83(cref_fn.as_pointer(), a.as_pointer()) }) as bool))
        });
        ({
            cpp2rust_assert_fail_22((({ apply_84(rref_fn.as_pointer(), a.as_pointer()) }) as bool))
        });
    }
    {
        let a: Value<std_tuple_const_int__const_int__const_int__const_int__const_int_> =
            Rc::new(RefCell::new(
                <std_tuple_const_int__const_int__const_int__const_int__const_int_>::default(),
            ));
        ({
            cpp2rust_assert_fail_22((({ apply_85(val_fn.as_pointer(), a.as_pointer()) }) as bool))
        });
        ({
            cpp2rust_assert_fail_22((({ apply_86(cref_fn.as_pointer(), a.as_pointer()) }) as bool))
        });
    }
}
pub fn test_one_87() {
    let val_fn: Value<
        FnPtr<
            fn(
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
            ) -> bool,
        >,
    > = Rc::new(RefCell::new(
        (FnPtr::<
            fn(
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
            ) -> bool,
        >::new(test_apply_fn_20)),
    ));
    {
        let a: Value<std_array_int__256_> = Rc::new(RefCell::new(<std_array_int__256_>::default()));
        ({
            cpp2rust_assert_fail_22((({ apply_88(val_fn.as_pointer(), a.as_pointer()) }) as bool))
        });
    }
    {
        let a : Value<std_tuple_int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int_ > = Rc::new(RefCell::new(<std_tuple_int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int_  >::default() )) ;
        ({
            cpp2rust_assert_fail_22((({ apply_89(val_fn.as_pointer(), a.as_pointer()) }) as bool))
        });
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
    ({ test_all_21() });
    ({ test_all_35() });
    ({ test_all_48() });
    ({ test_all_61() });
    ({ test_all_74() });
    ({ test_one_87() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
