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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_const_int__0_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int_ref__0_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_const_int_ref__0_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int_refref__0_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__1_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_const_int__1_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int_ref__1_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_const_int_ref__1_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int_refref__1_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__2_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_const_int__2_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int_ref__2_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_const_int_ref__2_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int_refref__2_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__3_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_const_int__3_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int_ref__3_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_const_int_ref__3_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int_refref__3_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__4_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_const_int__4_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int_ref__4_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_const_int_ref__4_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int_refref__4_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__5_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__6_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__7_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__8_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__9_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__10_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__11_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__12_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__13_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__14_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__15_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__16_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__17_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__18_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__19_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__20_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__21_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__22_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__23_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__24_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__25_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__26_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__27_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__28_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__29_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__30_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__31_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__32_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__33_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__34_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__35_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__36_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__37_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__38_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__39_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__40_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__41_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__42_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__43_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__44_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__45_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__46_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__47_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__48_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__49_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__50_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__51_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__52_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__53_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__54_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__55_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__56_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__57_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__58_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__59_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__60_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__61_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__62_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__63_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__64_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__65_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__66_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__67_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__68_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__69_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__70_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__71_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__72_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__73_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__74_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__75_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__76_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__77_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__78_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__79_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__80_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__81_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__82_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__83_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__84_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__85_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__86_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__87_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__88_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__89_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__90_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__91_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__92_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__93_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__94_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__95_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__96_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__97_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__98_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__99_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__100_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__101_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__102_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__103_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__104_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__105_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__106_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__107_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__108_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__109_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__110_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__111_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__112_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__113_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__114_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__115_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__116_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__117_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__118_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__119_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__120_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__121_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__122_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__123_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__124_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__125_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__126_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__127_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__128_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__129_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__130_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__131_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__132_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__133_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__134_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__135_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__136_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__137_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__138_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__139_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__140_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__141_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__142_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__143_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__144_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__145_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__146_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__147_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__148_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__149_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__150_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__151_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__152_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__153_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__154_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__155_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__156_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__157_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__158_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__159_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__160_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__161_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__162_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__163_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__164_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__165_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__166_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__167_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__168_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__169_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__170_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__171_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__172_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__173_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__174_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__175_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__176_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__177_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__178_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__179_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__180_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__181_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__182_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__183_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__184_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__185_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__186_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__187_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__188_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__189_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__190_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__191_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__192_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__193_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__194_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__195_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__196_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__197_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__198_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__199_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__200_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__201_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__202_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__203_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__204_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__205_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__206_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__207_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__208_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__209_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__210_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__211_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__212_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__213_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__214_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__215_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__216_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__217_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__218_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__219_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__220_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__221_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__222_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__223_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__224_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__225_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__226_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__227_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__228_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__229_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__230_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__231_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__232_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__233_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__234_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__235_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__236_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__237_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__238_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__239_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__240_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__241_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__242_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__243_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__244_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__245_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__246_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__247_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__248_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__249_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__250_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__251_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__252_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__253_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__254_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_imp_int__255_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct make_function_int__std_integer_sequence_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct make_function_int_ref__std_integer_sequence_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct make_function_const_int_ref__std_integer_sequence_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct make_function_int_refref__std_integer_sequence_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct make_function_int__std_integer_sequence_unsigned_long__0__1__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct make_function_int_ref__std_integer_sequence_unsigned_long__0__1__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct make_function_const_int_ref__std_integer_sequence_unsigned_long__0__1__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct make_function_int_refref__std_integer_sequence_unsigned_long__0__1__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct make_function_int__std_integer_sequence_unsigned_long__0__1__2__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct make_function_int_ref__std_integer_sequence_unsigned_long__0__1__2__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct make_function_const_int_ref__std_integer_sequence_unsigned_long__0__1__2__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct make_function_int_refref__std_integer_sequence_unsigned_long__0__1__2__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct make_function_int__std_integer_sequence_unsigned_long__0__1__2__3__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct make_function_int_ref__std_integer_sequence_unsigned_long__0__1__2__3__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct make_function_const_int_ref__std_integer_sequence_unsigned_long__0__1__2__3__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct make_function_int_refref__std_integer_sequence_unsigned_long__0__1__2__3__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct make_function_int__std_integer_sequence_unsigned_long__0__1__2__3__4__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct make_function_int_ref__std_integer_sequence_unsigned_long__0__1__2__3__4__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct make_function_const_int_ref__std_integer_sequence_unsigned_long__0__1__2__3__4__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct make_function_int_refref__std_integer_sequence_unsigned_long__0__1__2__3__4__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct make_function_int__std_integer_sequence_unsigned_long__0__1__2__3__4__5__6__7__8__9__10__11__12__13__14__15__16__17__18__19__20__21__22__23__24__25__26__27__28__29__30__31__32__33__34__35__36__37__38__39__40__41__42__43__44__45__46__47__48__49__50__51__52__53__54__55__56__57__58__59__60__61__62__63__64__65__66__67__68__69__70__71__72__73__74__75__76__77__78__79__80__81__82__83__84__85__86__87__88__89__90__91__92__93__94__95__96__97__98__99__100__101__102__103__104__105__106__107__108__109__110__111__112__113__114__115__116__117__118__119__120__121__122__123__124__125__126__127__128__129__130__131__132__133__134__135__136__137__138__139__140__141__142__143__144__145__146__147__148__149__150__151__152__153__154__155__156__157__158__159__160__161__162__163__164__165__166__167__168__169__170__171__172__173__174__175__176__177__178__179__180__181__182__183__184__185__186__187__188__189__190__191__192__193__194__195__196__197__198__199__200__201__202__203__204__205__206__207__208__209__210__211__212__213__214__215__216__217__218__219__220__221__222__223__224__225__226__227__228__229__230__231__232__233__234__235__236__237__238__239__240__241__242__243__244__245__246__247__248__249__250__251__252__253__254__255__
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct make_tuple_imp_int__std_integer_sequence_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct make_tuple_imp_const_int__std_integer_sequence_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct make_tuple_imp_int__std_integer_sequence_unsigned_long__0__1__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct make_tuple_imp_const_int__std_integer_sequence_unsigned_long__0__1__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct make_tuple_imp_int__std_integer_sequence_unsigned_long__0__1__2__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct make_tuple_imp_const_int__std_integer_sequence_unsigned_long__0__1__2__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct make_tuple_imp_int__std_integer_sequence_unsigned_long__0__1__2__3__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct make_tuple_imp_const_int__std_integer_sequence_unsigned_long__0__1__2__3__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct make_tuple_imp_int__std_integer_sequence_unsigned_long__0__1__2__3__4__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct make_tuple_imp_const_int__std_integer_sequence_unsigned_long__0__1__2__3__4__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct make_tuple_imp_int__std_integer_sequence_unsigned_long__0__1__2__3__4__5__6__7__8__9__10__11__12__13__14__15__16__17__18__19__20__21__22__23__24__25__26__27__28__29__30__31__32__33__34__35__36__37__38__39__40__41__42__43__44__45__46__47__48__49__50__51__52__53__54__55__56__57__58__59__60__61__62__63__64__65__66__67__68__69__70__71__72__73__74__75__76__77__78__79__80__81__82__83__84__85__86__87__88__89__90__91__92__93__94__95__96__97__98__99__100__101__102__103__104__105__106__107__108__109__110__111__112__113__114__115__116__117__118__119__120__121__122__123__124__125__126__127__128__129__130__131__132__133__134__135__136__137__138__139__140__141__142__143__144__145__146__147__148__149__150__151__152__153__154__155__156__157__158__159__160__161__162__163__164__165__166__167__168__169__170__171__172__173__174__175__176__177__178__179__180__181__182__183__184__185__186__187__188__189__190__191__192__193__194__195__196__197__198__199__200__201__202__203__204__205__206__207__208__209__210__211__212__213__214__215__216__217__218__219__220__221__222__223__224__225__226__227__228__229__230__231__232__233__234__235__236__237__238__239__240__241__242__243__244__245__246__247__248__249__250__251__252__253__254__255__
{}
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
    let mut val_fn: FnPtr<fn(i32) -> bool> = (FnPtr::<fn(i32) -> bool>::new(test_apply_fn_0));
    let mut ref_fn: FnPtr<fn(Ptr<i32>) -> bool> =
        (FnPtr::<fn(Ptr<i32>) -> bool>::new(test_apply_fn_1));
    let mut cref_fn: FnPtr<fn(Ptr<i32>) -> bool> =
        (FnPtr::<fn(Ptr<i32>) -> bool>::new(test_apply_fn_2));
    let mut rref_fn: FnPtr<fn(Ptr<i32>) -> bool> =
        (FnPtr::<fn(Ptr<i32>) -> bool>::new(test_apply_fn_3));
    {
        let a: Value<Vec<i32>> = Rc::new(RefCell::new(
            std::array::from_fn::<_, 1, _>(|_| 0_i32).to_vec(),
        ));
        assert!((unimplemented!() as bool));
        assert!((unimplemented!() as bool));
        assert!((unimplemented!() as bool));
        assert!((unimplemented!() as bool));
    }
    {
        let a: Value<Vec<i32>> = Rc::new(RefCell::new(
            std::array::from_fn::<_, 1, _>(|_| 0_i32).to_vec(),
        ));
        assert!((unimplemented!() as bool));
        assert!((unimplemented!() as bool));
    }
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() as bool));
        assert!((unimplemented!() as bool));
        assert!((unimplemented!() as bool));
        assert!((unimplemented!() as bool));
    }
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() as bool));
        assert!((unimplemented!() as bool));
    }
}
pub fn test_all_22() {
    let mut val_fn: FnPtr<fn(i32, i32) -> bool> =
        (FnPtr::<fn(i32, i32) -> bool>::new(test_apply_fn_4));
    let mut ref_fn: FnPtr<fn(Ptr<i32>, Ptr<i32>) -> bool> =
        (FnPtr::<fn(Ptr<i32>, Ptr<i32>) -> bool>::new(test_apply_fn_5));
    let mut cref_fn: FnPtr<fn(Ptr<i32>, Ptr<i32>) -> bool> =
        (FnPtr::<fn(Ptr<i32>, Ptr<i32>) -> bool>::new(test_apply_fn_6));
    let mut rref_fn: FnPtr<fn(Ptr<i32>, Ptr<i32>) -> bool> =
        (FnPtr::<fn(Ptr<i32>, Ptr<i32>) -> bool>::new(test_apply_fn_7));
    {
        let a: Value<Vec<i32>> = Rc::new(RefCell::new(
            std::array::from_fn::<_, 2, _>(|_| 0_i32).to_vec(),
        ));
        assert!((unimplemented!() as bool));
        assert!((unimplemented!() as bool));
        assert!((unimplemented!() as bool));
        assert!((unimplemented!() as bool));
    }
    {
        let a: Value<Vec<i32>> = Rc::new(RefCell::new(
            std::array::from_fn::<_, 2, _>(|_| 0_i32).to_vec(),
        ));
        assert!((unimplemented!() as bool));
        assert!((unimplemented!() as bool));
    }
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() as bool));
        assert!((unimplemented!() as bool));
        assert!((unimplemented!() as bool));
        assert!((unimplemented!() as bool));
    }
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() as bool));
        assert!((unimplemented!() as bool));
    }
}
pub fn test_all_23() {
    let mut val_fn: FnPtr<fn(i32, i32, i32) -> bool> =
        (FnPtr::<fn(i32, i32, i32) -> bool>::new(test_apply_fn_8));
    let mut ref_fn: FnPtr<fn(Ptr<i32>, Ptr<i32>, Ptr<i32>) -> bool> =
        (FnPtr::<fn(Ptr<i32>, Ptr<i32>, Ptr<i32>) -> bool>::new(test_apply_fn_9));
    let mut cref_fn: FnPtr<fn(Ptr<i32>, Ptr<i32>, Ptr<i32>) -> bool> =
        (FnPtr::<fn(Ptr<i32>, Ptr<i32>, Ptr<i32>) -> bool>::new(test_apply_fn_10));
    let mut rref_fn: FnPtr<fn(Ptr<i32>, Ptr<i32>, Ptr<i32>) -> bool> =
        (FnPtr::<fn(Ptr<i32>, Ptr<i32>, Ptr<i32>) -> bool>::new(test_apply_fn_11));
    {
        let a: Value<Vec<i32>> = Rc::new(RefCell::new(
            std::array::from_fn::<_, 3, _>(|_| 0_i32).to_vec(),
        ));
        assert!((unimplemented!() as bool));
        assert!((unimplemented!() as bool));
        assert!((unimplemented!() as bool));
        assert!((unimplemented!() as bool));
    }
    {
        let a: Value<Vec<i32>> = Rc::new(RefCell::new(
            std::array::from_fn::<_, 3, _>(|_| 0_i32).to_vec(),
        ));
        assert!((unimplemented!() as bool));
        assert!((unimplemented!() as bool));
    }
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() as bool));
        assert!((unimplemented!() as bool));
        assert!((unimplemented!() as bool));
        assert!((unimplemented!() as bool));
    }
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() as bool));
        assert!((unimplemented!() as bool));
    }
}
pub fn test_all_24() {
    let mut val_fn: FnPtr<fn(i32, i32, i32, i32) -> bool> =
        (FnPtr::<fn(i32, i32, i32, i32) -> bool>::new(test_apply_fn_12));
    let mut ref_fn: FnPtr<fn(Ptr<i32>, Ptr<i32>, Ptr<i32>, Ptr<i32>) -> bool> =
        (FnPtr::<fn(Ptr<i32>, Ptr<i32>, Ptr<i32>, Ptr<i32>) -> bool>::new(test_apply_fn_13));
    let mut cref_fn: FnPtr<fn(Ptr<i32>, Ptr<i32>, Ptr<i32>, Ptr<i32>) -> bool> =
        (FnPtr::<fn(Ptr<i32>, Ptr<i32>, Ptr<i32>, Ptr<i32>) -> bool>::new(test_apply_fn_14));
    let mut rref_fn: FnPtr<fn(Ptr<i32>, Ptr<i32>, Ptr<i32>, Ptr<i32>) -> bool> =
        (FnPtr::<fn(Ptr<i32>, Ptr<i32>, Ptr<i32>, Ptr<i32>) -> bool>::new(test_apply_fn_15));
    {
        let a: Value<Vec<i32>> = Rc::new(RefCell::new(
            std::array::from_fn::<_, 4, _>(|_| 0_i32).to_vec(),
        ));
        assert!((unimplemented!() as bool));
        assert!((unimplemented!() as bool));
        assert!((unimplemented!() as bool));
        assert!((unimplemented!() as bool));
    }
    {
        let a: Value<Vec<i32>> = Rc::new(RefCell::new(
            std::array::from_fn::<_, 4, _>(|_| 0_i32).to_vec(),
        ));
        assert!((unimplemented!() as bool));
        assert!((unimplemented!() as bool));
    }
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() as bool));
        assert!((unimplemented!() as bool));
        assert!((unimplemented!() as bool));
        assert!((unimplemented!() as bool));
    }
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() as bool));
        assert!((unimplemented!() as bool));
    }
}
pub fn test_all_25() {
    let mut val_fn: FnPtr<fn(i32, i32, i32, i32, i32) -> bool> =
        (FnPtr::<fn(i32, i32, i32, i32, i32) -> bool>::new(test_apply_fn_16));
    let mut ref_fn: FnPtr<fn(Ptr<i32>, Ptr<i32>, Ptr<i32>, Ptr<i32>, Ptr<i32>) -> bool> =
        (FnPtr::<fn(Ptr<i32>, Ptr<i32>, Ptr<i32>, Ptr<i32>, Ptr<i32>) -> bool>::new(
            test_apply_fn_17,
        ));
    let mut cref_fn: FnPtr<fn(Ptr<i32>, Ptr<i32>, Ptr<i32>, Ptr<i32>, Ptr<i32>) -> bool> =
        (FnPtr::<fn(Ptr<i32>, Ptr<i32>, Ptr<i32>, Ptr<i32>, Ptr<i32>) -> bool>::new(
            test_apply_fn_18,
        ));
    let mut rref_fn: FnPtr<fn(Ptr<i32>, Ptr<i32>, Ptr<i32>, Ptr<i32>, Ptr<i32>) -> bool> =
        (FnPtr::<fn(Ptr<i32>, Ptr<i32>, Ptr<i32>, Ptr<i32>, Ptr<i32>) -> bool>::new(
            test_apply_fn_19,
        ));
    {
        let a: Value<Vec<i32>> = Rc::new(RefCell::new(
            std::array::from_fn::<_, 5, _>(|_| 0_i32).to_vec(),
        ));
        assert!((unimplemented!() as bool));
        assert!((unimplemented!() as bool));
        assert!((unimplemented!() as bool));
        assert!((unimplemented!() as bool));
    }
    {
        let a: Value<Vec<i32>> = Rc::new(RefCell::new(
            std::array::from_fn::<_, 5, _>(|_| 0_i32).to_vec(),
        ));
        assert!((unimplemented!() as bool));
        assert!((unimplemented!() as bool));
    }
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() as bool));
        assert!((unimplemented!() as bool));
        assert!((unimplemented!() as bool));
        assert!((unimplemented!() as bool));
    }
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() as bool));
        assert!((unimplemented!() as bool));
    }
}
pub fn test_one_26() {
    let mut val_fn: FnPtr<
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
    > = (FnPtr::<
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
    >::new(test_apply_fn_20));
    {
        let a: Value<Vec<i32>> = Rc::new(RefCell::new(
            std::array::from_fn::<_, 256, _>(|_| 0_i32).to_vec(),
        ));
        assert!((unimplemented!() as bool));
    }
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() as bool));
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
    ({ test_all_22() });
    ({ test_all_23() });
    ({ test_all_24() });
    ({ test_all_25() });
    ({ test_one_26() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
