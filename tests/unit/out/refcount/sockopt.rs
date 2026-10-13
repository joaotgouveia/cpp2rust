extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct __fsid_t {
    #[offset(0)]
    #[byte_size(8)]
    pub __val: Value<Box<[i32]>>,
}
impl Clone for __fsid_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __fsid_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct iovec {
    #[offset(0)]
    #[byte_size(8)]
    pub iov_base: AnyPtr,
    #[offset(8)]
    pub iov_len: usize,
}
impl Clone for iovec {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iovec {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(128)]
pub struct __sigset_t {
    #[offset(0)]
    #[byte_size(128)]
    pub __val: Value<Box<[u64]>>,
}
impl Clone for __sigset_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __sigset_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct timeval {
    #[offset(0)]
    pub tv_sec: i64,
    #[offset(8)]
    pub tv_usec: i64,
}
impl Clone for timeval {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for timeval {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct timespec {
    #[offset(0)]
    pub tv_sec: i64,
    #[offset(8)]
    pub tv_nsec: i64,
}
impl Clone for timespec {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for timespec {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(128)]
pub struct fd_set {
    #[offset(0)]
    #[byte_size(128)]
    pub __fds_bits: Value<Box<[i64]>>,
}
impl Clone for fd_set {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for fd_set {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct __atomic_wide_counter {
    #[offset(0)]
    pub __value64: u64,
    #[offset(0)]
    #[byte_size(8)]
    pub __value32: anon_0,
}
impl Clone for __atomic_wide_counter {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __atomic_wide_counter {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct anon_0 {
    #[offset(0)]
    pub __low: u32,
    #[offset(4)]
    pub __high: u32,
}
impl Clone for anon_0 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_0 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct __pthread_internal_list {
    #[offset(0)]
    #[byte_size(8)]
    pub __prev: Ptr<__pthread_internal_list>,
    #[offset(8)]
    #[byte_size(8)]
    pub __next: Ptr<__pthread_internal_list>,
}
impl Clone for __pthread_internal_list {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pthread_internal_list {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct __pthread_internal_slist {
    #[offset(0)]
    #[byte_size(8)]
    pub __next: Ptr<__pthread_internal_slist>,
}
impl Clone for __pthread_internal_slist {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pthread_internal_slist {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(40)]
pub struct __pthread_mutex_s {
    #[offset(0)]
    pub __lock: i32,
    #[offset(4)]
    pub __count: u32,
    #[offset(8)]
    pub __owner: i32,
    #[offset(12)]
    pub __nusers: u32,
    #[offset(16)]
    pub __kind: i32,
    #[offset(20)]
    pub __spins: i16,
    #[offset(22)]
    pub __elision: i16,
    #[offset(24)]
    #[byte_size(16)]
    pub __list: __pthread_internal_list,
}
impl Clone for __pthread_mutex_s {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pthread_mutex_s {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(56)]
pub struct __pthread_rwlock_arch_t {
    #[offset(0)]
    pub __readers: u32,
    #[offset(4)]
    pub __writers: u32,
    #[offset(8)]
    pub __wrphase_futex: u32,
    #[offset(12)]
    pub __writers_futex: u32,
    #[offset(16)]
    pub __pad3: u32,
    #[offset(20)]
    pub __pad4: u32,
    #[offset(24)]
    pub __cur_writer: i32,
    #[offset(28)]
    pub __shared: i32,
    #[offset(32)]
    pub __rwelision: i8,
    #[offset(33)]
    #[byte_size(7)]
    pub __pad1: Value<Box<[u8]>>,
    #[offset(40)]
    pub __pad2: u64,
    #[offset(48)]
    pub __flags: u32,
}
impl Clone for __pthread_rwlock_arch_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pthread_rwlock_arch_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(48)]
pub struct __pthread_cond_s {
    #[offset(0)]
    #[byte_size(8)]
    pub __wseq: __atomic_wide_counter,
    #[offset(8)]
    #[byte_size(8)]
    pub __g1_start: __atomic_wide_counter,
    #[offset(16)]
    #[byte_size(8)]
    pub __g_refs: Value<Box<[u32]>>,
    #[offset(24)]
    #[byte_size(8)]
    pub __g_size: Value<Box<[u32]>>,
    #[offset(32)]
    pub __g1_orig_size: u32,
    #[offset(36)]
    pub __wrefs: u32,
    #[offset(40)]
    #[byte_size(8)]
    pub __g_signals: Value<Box<[u32]>>,
}
impl Clone for __pthread_cond_s {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pthread_cond_s {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(4)]
pub struct __once_flag {
    #[offset(0)]
    pub __data: i32,
}
impl Clone for __once_flag {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __once_flag {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(4)]
pub struct pthread_mutexattr_t {
    #[offset(0)]
    #[byte_size(4)]
    pub __size: Value<Box<[i8]>>,
    #[offset(0)]
    pub __align: i32,
}
impl Clone for pthread_mutexattr_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_mutexattr_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(4)]
pub struct pthread_condattr_t {
    #[offset(0)]
    #[byte_size(4)]
    pub __size: Value<Box<[i8]>>,
    #[offset(0)]
    pub __align: i32,
}
impl Clone for pthread_condattr_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_condattr_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(56)]
pub struct pthread_attr_t {
    #[offset(0)]
    #[byte_size(56)]
    pub __size: Value<Box<[i8]>>,
    #[offset(0)]
    pub __align: i64,
}
impl Clone for pthread_attr_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_attr_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(40)]
pub struct pthread_mutex_t {
    #[offset(0)]
    #[byte_size(40)]
    pub __data: __pthread_mutex_s,
    #[offset(0)]
    #[byte_size(40)]
    pub __size: Value<Box<[i8]>>,
    #[offset(0)]
    pub __align: i64,
}
impl Clone for pthread_mutex_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_mutex_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(48)]
pub struct pthread_cond_t {
    #[offset(0)]
    #[byte_size(48)]
    pub __data: __pthread_cond_s,
    #[offset(0)]
    #[byte_size(48)]
    pub __size: Value<Box<[i8]>>,
    #[offset(0)]
    pub __align: i64,
}
impl Clone for pthread_cond_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_cond_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(56)]
pub struct pthread_rwlock_t {
    #[offset(0)]
    #[byte_size(56)]
    pub __data: __pthread_rwlock_arch_t,
    #[offset(0)]
    #[byte_size(56)]
    pub __size: Value<Box<[i8]>>,
    #[offset(0)]
    pub __align: i64,
}
impl Clone for pthread_rwlock_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_rwlock_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct pthread_rwlockattr_t {
    #[offset(0)]
    #[byte_size(8)]
    pub __size: Value<Box<[i8]>>,
    #[offset(0)]
    pub __align: i64,
}
impl Clone for pthread_rwlockattr_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_rwlockattr_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(32)]
pub struct pthread_barrier_t {
    #[offset(0)]
    #[byte_size(32)]
    pub __size: Value<Box<[i8]>>,
    #[offset(0)]
    pub __align: i64,
}
impl Clone for pthread_barrier_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_barrier_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(4)]
pub struct pthread_barrierattr_t {
    #[offset(0)]
    #[byte_size(4)]
    pub __size: Value<Box<[i8]>>,
    #[offset(0)]
    pub __align: i32,
}
impl Clone for pthread_barrierattr_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_barrierattr_t {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type __socket_type = u32;
pub const __socket_type_SOCK_STREAM: __socket_type = 1;
pub const __socket_type_SOCK_DGRAM: __socket_type = 2;
pub const __socket_type_SOCK_RAW: __socket_type = 3;
pub const __socket_type_SOCK_RDM: __socket_type = 4;
pub const __socket_type_SOCK_SEQPACKET: __socket_type = 5;
pub const __socket_type_SOCK_DCCP: __socket_type = 6;
pub const __socket_type_SOCK_PACKET: __socket_type = 10;
pub const __socket_type_SOCK_CLOEXEC: __socket_type = 524288;
pub const __socket_type_SOCK_NONBLOCK: __socket_type = 2048;
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct sockaddr {
    #[offset(0)]
    pub sa_family: u16,
    #[offset(2)]
    #[byte_size(14)]
    pub sa_data: Value<Box<[i8]>>,
}
impl Clone for sockaddr {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for sockaddr {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(128)]
pub struct sockaddr_storage {
    #[offset(0)]
    pub ss_family: u16,
    #[offset(2)]
    #[byte_size(118)]
    pub __ss_padding: Value<Box<[i8]>>,
    #[offset(120)]
    pub __ss_align: u64,
}
impl Clone for sockaddr_storage {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for sockaddr_storage {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(56)]
pub struct msghdr {
    #[offset(0)]
    #[byte_size(8)]
    pub msg_name: AnyPtr,
    #[offset(8)]
    pub msg_namelen: u32,
    #[offset(16)]
    #[byte_size(8)]
    pub msg_iov: Ptr<iovec>,
    #[offset(24)]
    pub msg_iovlen: usize,
    #[offset(32)]
    #[byte_size(8)]
    pub msg_control: AnyPtr,
    #[offset(40)]
    pub msg_controllen: usize,
    #[offset(48)]
    pub msg_flags: i32,
}
impl Clone for msghdr {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for msghdr {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct cmsghdr {
    #[offset(0)]
    pub cmsg_len: usize,
    #[offset(8)]
    pub cmsg_level: i32,
    #[offset(12)]
    pub cmsg_type: i32,
    #[offset(16)]
    #[byte_size(0)]
    pub __cmsg_data: Value<Box<[u8]>>,
}
impl Clone for cmsghdr {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for cmsghdr {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(128)]
pub struct __kernel_fd_set {
    #[offset(0)]
    #[byte_size(128)]
    pub fds_bits: Value<Box<[u64]>>,
}
impl Clone for __kernel_fd_set {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __kernel_fd_set {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct __kernel_fsid_t {
    #[offset(0)]
    #[byte_size(8)]
    pub val: Value<Box<[i32]>>,
}
impl Clone for __kernel_fsid_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __kernel_fsid_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct linger {
    #[offset(0)]
    pub l_onoff: i32,
    #[offset(4)]
    pub l_linger: i32,
}
impl Clone for linger {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for linger {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct osockaddr {
    #[offset(0)]
    pub sa_family: u16,
    #[offset(2)]
    #[byte_size(14)]
    pub sa_data: Value<Box<[u8]>>,
}
impl Clone for osockaddr {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for osockaddr {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(4)]
pub struct in_addr {
    #[offset(0)]
    pub s_addr: u32,
}
impl Clone for in_addr {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for in_addr {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(44)]
pub struct ip_opts {
    #[offset(0)]
    #[byte_size(4)]
    pub ip_dst: in_addr,
    #[offset(4)]
    #[byte_size(40)]
    pub ip_opts: Value<Box<[i8]>>,
}
impl Clone for ip_opts {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for ip_opts {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(12)]
pub struct in_pktinfo {
    #[offset(0)]
    pub ipi_ifindex: i32,
    #[offset(4)]
    #[byte_size(4)]
    pub ipi_spec_dst: in_addr,
    #[offset(8)]
    #[byte_size(4)]
    pub ipi_addr: in_addr,
}
impl Clone for in_pktinfo {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for in_pktinfo {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_1 = u32;
pub const anon_1_IPPROTO_IP: anon_1 = 0;
pub const anon_1_IPPROTO_ICMP: anon_1 = 1;
pub const anon_1_IPPROTO_IGMP: anon_1 = 2;
pub const anon_1_IPPROTO_IPIP: anon_1 = 4;
pub const anon_1_IPPROTO_TCP: anon_1 = 6;
pub const anon_1_IPPROTO_EGP: anon_1 = 8;
pub const anon_1_IPPROTO_PUP: anon_1 = 12;
pub const anon_1_IPPROTO_UDP: anon_1 = 17;
pub const anon_1_IPPROTO_IDP: anon_1 = 22;
pub const anon_1_IPPROTO_TP: anon_1 = 29;
pub const anon_1_IPPROTO_DCCP: anon_1 = 33;
pub const anon_1_IPPROTO_IPV6: anon_1 = 41;
pub const anon_1_IPPROTO_RSVP: anon_1 = 46;
pub const anon_1_IPPROTO_GRE: anon_1 = 47;
pub const anon_1_IPPROTO_ESP: anon_1 = 50;
pub const anon_1_IPPROTO_AH: anon_1 = 51;
pub const anon_1_IPPROTO_MTP: anon_1 = 92;
pub const anon_1_IPPROTO_BEETPH: anon_1 = 94;
pub const anon_1_IPPROTO_ENCAP: anon_1 = 98;
pub const anon_1_IPPROTO_PIM: anon_1 = 103;
pub const anon_1_IPPROTO_COMP: anon_1 = 108;
pub const anon_1_IPPROTO_L2TP: anon_1 = 115;
pub const anon_1_IPPROTO_SCTP: anon_1 = 132;
pub const anon_1_IPPROTO_UDPLITE: anon_1 = 136;
pub const anon_1_IPPROTO_MPLS: anon_1 = 137;
pub const anon_1_IPPROTO_ETHERNET: anon_1 = 143;
pub const anon_1_IPPROTO_RAW: anon_1 = 255;
pub const anon_1_IPPROTO_MPTCP: anon_1 = 262;
pub const anon_1_IPPROTO_MAX: anon_1 = 263;
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct in6_addr {
    #[offset(0)]
    #[byte_size(16)]
    pub __in6_u: anon_2,
}
impl Clone for in6_addr {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for in6_addr {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct anon_2 {
    #[offset(0)]
    #[byte_size(16)]
    pub __u6_addr8: Value<Box<[u8]>>,
    #[offset(0)]
    #[byte_size(16)]
    pub __u6_addr16: Value<Box<[u16]>>,
    #[offset(0)]
    #[byte_size(16)]
    pub __u6_addr32: Value<Box<[u32]>>,
}
impl Clone for anon_2 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_2 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct sockaddr_in {
    #[offset(0)]
    pub sin_family: u16,
    #[offset(2)]
    pub sin_port: u16,
    #[offset(4)]
    #[byte_size(4)]
    pub sin_addr: in_addr,
    #[offset(8)]
    #[byte_size(8)]
    pub sin_zero: Value<Box<[u8]>>,
}
impl Clone for sockaddr_in {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for sockaddr_in {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(28)]
pub struct sockaddr_in6 {
    #[offset(0)]
    pub sin6_family: u16,
    #[offset(2)]
    pub sin6_port: u16,
    #[offset(4)]
    pub sin6_flowinfo: u32,
    #[offset(8)]
    #[byte_size(16)]
    pub sin6_addr: in6_addr,
    #[offset(24)]
    pub sin6_scope_id: u32,
}
impl Clone for sockaddr_in6 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for sockaddr_in6 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct ip_mreq {
    #[offset(0)]
    #[byte_size(4)]
    pub imr_multiaddr: in_addr,
    #[offset(4)]
    #[byte_size(4)]
    pub imr_interface: in_addr,
}
impl Clone for ip_mreq {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for ip_mreq {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(12)]
pub struct ip_mreqn {
    #[offset(0)]
    #[byte_size(4)]
    pub imr_multiaddr: in_addr,
    #[offset(4)]
    #[byte_size(4)]
    pub imr_address: in_addr,
    #[offset(8)]
    pub imr_ifindex: i32,
}
impl Clone for ip_mreqn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for ip_mreqn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(12)]
pub struct ip_mreq_source {
    #[offset(0)]
    #[byte_size(4)]
    pub imr_multiaddr: in_addr,
    #[offset(4)]
    #[byte_size(4)]
    pub imr_interface: in_addr,
    #[offset(8)]
    #[byte_size(4)]
    pub imr_sourceaddr: in_addr,
}
impl Clone for ip_mreq_source {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for ip_mreq_source {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(20)]
pub struct ipv6_mreq {
    #[offset(0)]
    #[byte_size(16)]
    pub ipv6mr_multiaddr: in6_addr,
    #[offset(16)]
    pub ipv6mr_interface: u32,
}
impl Clone for ipv6_mreq {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for ipv6_mreq {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(136)]
pub struct group_req {
    #[offset(0)]
    pub gr_interface: u32,
    #[offset(8)]
    #[byte_size(128)]
    pub gr_group: sockaddr_storage,
}
impl Clone for group_req {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for group_req {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(264)]
pub struct group_source_req {
    #[offset(0)]
    pub gsr_interface: u32,
    #[offset(8)]
    #[byte_size(128)]
    pub gsr_group: sockaddr_storage,
    #[offset(136)]
    #[byte_size(128)]
    pub gsr_source: sockaddr_storage,
}
impl Clone for group_source_req {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for group_source_req {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(20)]
pub struct ip_msfilter {
    #[offset(0)]
    #[byte_size(4)]
    pub imsf_multiaddr: in_addr,
    #[offset(4)]
    #[byte_size(4)]
    pub imsf_interface: in_addr,
    #[offset(8)]
    pub imsf_fmode: u32,
    #[offset(12)]
    pub imsf_numsrc: u32,
    #[offset(16)]
    #[byte_size(4)]
    pub imsf_slist: Value<Box<[in_addr]>>,
}
impl Clone for ip_msfilter {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for ip_msfilter {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(272)]
pub struct group_filter {
    #[offset(0)]
    pub gf_interface: u32,
    #[offset(8)]
    #[byte_size(128)]
    pub gf_group: sockaddr_storage,
    #[offset(136)]
    pub gf_fmode: u32,
    #[offset(140)]
    pub gf_numsrc: u32,
    #[offset(144)]
    #[byte_size(128)]
    pub gf_slist: Value<Box<[sockaddr_storage]>>,
}
impl Clone for group_filter {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for group_filter {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(20)]
pub struct tcphdr {
    #[offset(0)]
    #[byte_size(20)]
    pub anon_3: anon_3,
}
impl Clone for tcphdr {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tcphdr {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(20)]
pub struct anon_3 {
    #[offset(0)]
    #[byte_size(20)]
    pub anon_4: anon_4,
    #[offset(0)]
    #[byte_size(20)]
    pub anon_5: anon_5,
}
impl Clone for anon_3 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_3 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(20)]
pub struct anon_4 {
    #[offset(0)]
    pub th_sport: u16,
    #[offset(2)]
    pub th_dport: u16,
    #[offset(4)]
    pub th_seq: u32,
    #[offset(8)]
    pub th_ack: u32,
    #[offset(12)]
    pub th_x2: u8,
    #[offset(12)]
    pub th_off: u8,
    #[offset(13)]
    pub th_flags: u8,
    #[offset(14)]
    pub th_win: u16,
    #[offset(16)]
    pub th_sum: u16,
    #[offset(18)]
    pub th_urp: u16,
}
impl Clone for anon_4 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_4 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(20)]
pub struct anon_5 {
    #[offset(0)]
    pub source: u16,
    #[offset(2)]
    pub dest: u16,
    #[offset(4)]
    pub seq: u32,
    #[offset(8)]
    pub ack_seq: u32,
    #[offset(12)]
    pub res1: u16,
    #[offset(12)]
    pub doff: u16,
    #[offset(13)]
    pub fin: u16,
    #[offset(13)]
    pub syn: u16,
    #[offset(13)]
    pub rst: u16,
    #[offset(13)]
    pub psh: u16,
    #[offset(13)]
    pub ack: u16,
    #[offset(13)]
    pub urg: u16,
    #[offset(13)]
    pub res2: u16,
    #[offset(14)]
    pub window: u16,
    #[offset(16)]
    pub check: u16,
    #[offset(18)]
    pub urg_ptr: u16,
}
impl Clone for anon_5 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_5 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(104)]
pub struct tcp_info {
    #[offset(0)]
    pub tcpi_state: u8,
    #[offset(1)]
    pub tcpi_ca_state: u8,
    #[offset(2)]
    pub tcpi_retransmits: u8,
    #[offset(3)]
    pub tcpi_probes: u8,
    #[offset(4)]
    pub tcpi_backoff: u8,
    #[offset(5)]
    pub tcpi_options: u8,
    #[offset(6)]
    pub tcpi_snd_wscale: u8,
    #[offset(6)]
    pub tcpi_rcv_wscale: u8,
    #[offset(8)]
    pub tcpi_rto: u32,
    #[offset(12)]
    pub tcpi_ato: u32,
    #[offset(16)]
    pub tcpi_snd_mss: u32,
    #[offset(20)]
    pub tcpi_rcv_mss: u32,
    #[offset(24)]
    pub tcpi_unacked: u32,
    #[offset(28)]
    pub tcpi_sacked: u32,
    #[offset(32)]
    pub tcpi_lost: u32,
    #[offset(36)]
    pub tcpi_retrans: u32,
    #[offset(40)]
    pub tcpi_fackets: u32,
    #[offset(44)]
    pub tcpi_last_data_sent: u32,
    #[offset(48)]
    pub tcpi_last_ack_sent: u32,
    #[offset(52)]
    pub tcpi_last_data_recv: u32,
    #[offset(56)]
    pub tcpi_last_ack_recv: u32,
    #[offset(60)]
    pub tcpi_pmtu: u32,
    #[offset(64)]
    pub tcpi_rcv_ssthresh: u32,
    #[offset(68)]
    pub tcpi_rtt: u32,
    #[offset(72)]
    pub tcpi_rttvar: u32,
    #[offset(76)]
    pub tcpi_snd_ssthresh: u32,
    #[offset(80)]
    pub tcpi_snd_cwnd: u32,
    #[offset(84)]
    pub tcpi_advmss: u32,
    #[offset(88)]
    pub tcpi_reordering: u32,
    #[offset(92)]
    pub tcpi_rcv_rtt: u32,
    #[offset(96)]
    pub tcpi_rcv_space: u32,
    #[offset(100)]
    pub tcpi_total_retrans: u32,
}
impl Clone for tcp_info {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tcp_info {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(216)]
pub struct tcp_md5sig {
    #[offset(0)]
    #[byte_size(128)]
    pub tcpm_addr: sockaddr_storage,
    #[offset(128)]
    pub tcpm_flags: u8,
    #[offset(129)]
    pub tcpm_prefixlen: u8,
    #[offset(130)]
    pub tcpm_keylen: u16,
    #[offset(132)]
    pub tcpm_ifindex: i32,
    #[offset(136)]
    #[byte_size(80)]
    pub tcpm_key: Value<Box<[u8]>>,
}
impl Clone for tcp_md5sig {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tcp_md5sig {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct tcp_repair_opt {
    #[offset(0)]
    pub opt_code: u32,
    #[offset(4)]
    pub opt_val: u32,
}
impl Clone for tcp_repair_opt {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tcp_repair_opt {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(544)]
pub struct tcp_cookie_transactions {
    #[offset(0)]
    pub tcpct_flags: u16,
    #[offset(2)]
    pub __tcpct_pad1: u8,
    #[offset(3)]
    pub tcpct_cookie_desired: u8,
    #[offset(4)]
    pub tcpct_s_data_desired: u16,
    #[offset(6)]
    pub tcpct_used: u16,
    #[offset(8)]
    #[byte_size(536)]
    pub tcpct_value: Value<Box<[u8]>>,
}
impl Clone for tcp_cookie_transactions {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tcp_cookie_transactions {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(20)]
pub struct tcp_repair_window {
    #[offset(0)]
    pub snd_wl1: u32,
    #[offset(4)]
    pub snd_wnd: u32,
    #[offset(8)]
    pub max_window: u32,
    #[offset(12)]
    pub rcv_wnd: u32,
    #[offset(16)]
    pub rcv_wup: u32,
}
impl Clone for tcp_repair_window {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tcp_repair_window {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct tcp_zerocopy_receive {
    #[offset(0)]
    pub address: u64,
    #[offset(8)]
    pub length: u32,
    #[offset(12)]
    pub recv_skip_hint: u32,
}
impl Clone for tcp_zerocopy_receive {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tcp_zerocopy_receive {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_6 = u32;
pub const anon_6__SC_ARG_MAX: anon_6 = 0;
pub const anon_6__SC_CHILD_MAX: anon_6 = 1;
pub const anon_6__SC_CLK_TCK: anon_6 = 2;
pub const anon_6__SC_NGROUPS_MAX: anon_6 = 3;
pub const anon_6__SC_OPEN_MAX: anon_6 = 4;
pub const anon_6__SC_STREAM_MAX: anon_6 = 5;
pub const anon_6__SC_TZNAME_MAX: anon_6 = 6;
pub const anon_6__SC_JOB_CONTROL: anon_6 = 7;
pub const anon_6__SC_SAVED_IDS: anon_6 = 8;
pub const anon_6__SC_REALTIME_SIGNALS: anon_6 = 9;
pub const anon_6__SC_PRIORITY_SCHEDULING: anon_6 = 10;
pub const anon_6__SC_TIMERS: anon_6 = 11;
pub const anon_6__SC_ASYNCHRONOUS_IO: anon_6 = 12;
pub const anon_6__SC_PRIORITIZED_IO: anon_6 = 13;
pub const anon_6__SC_SYNCHRONIZED_IO: anon_6 = 14;
pub const anon_6__SC_FSYNC: anon_6 = 15;
pub const anon_6__SC_MAPPED_FILES: anon_6 = 16;
pub const anon_6__SC_MEMLOCK: anon_6 = 17;
pub const anon_6__SC_MEMLOCK_RANGE: anon_6 = 18;
pub const anon_6__SC_MEMORY_PROTECTION: anon_6 = 19;
pub const anon_6__SC_MESSAGE_PASSING: anon_6 = 20;
pub const anon_6__SC_SEMAPHORES: anon_6 = 21;
pub const anon_6__SC_SHARED_MEMORY_OBJECTS: anon_6 = 22;
pub const anon_6__SC_AIO_LISTIO_MAX: anon_6 = 23;
pub const anon_6__SC_AIO_MAX: anon_6 = 24;
pub const anon_6__SC_AIO_PRIO_DELTA_MAX: anon_6 = 25;
pub const anon_6__SC_DELAYTIMER_MAX: anon_6 = 26;
pub const anon_6__SC_MQ_OPEN_MAX: anon_6 = 27;
pub const anon_6__SC_MQ_PRIO_MAX: anon_6 = 28;
pub const anon_6__SC_VERSION: anon_6 = 29;
pub const anon_6__SC_PAGESIZE: anon_6 = 30;
pub const anon_6__SC_RTSIG_MAX: anon_6 = 31;
pub const anon_6__SC_SEM_NSEMS_MAX: anon_6 = 32;
pub const anon_6__SC_SEM_VALUE_MAX: anon_6 = 33;
pub const anon_6__SC_SIGQUEUE_MAX: anon_6 = 34;
pub const anon_6__SC_TIMER_MAX: anon_6 = 35;
pub const anon_6__SC_BC_BASE_MAX: anon_6 = 36;
pub const anon_6__SC_BC_DIM_MAX: anon_6 = 37;
pub const anon_6__SC_BC_SCALE_MAX: anon_6 = 38;
pub const anon_6__SC_BC_STRING_MAX: anon_6 = 39;
pub const anon_6__SC_COLL_WEIGHTS_MAX: anon_6 = 40;
pub const anon_6__SC_EQUIV_CLASS_MAX: anon_6 = 41;
pub const anon_6__SC_EXPR_NEST_MAX: anon_6 = 42;
pub const anon_6__SC_LINE_MAX: anon_6 = 43;
pub const anon_6__SC_RE_DUP_MAX: anon_6 = 44;
pub const anon_6__SC_CHARCLASS_NAME_MAX: anon_6 = 45;
pub const anon_6__SC_2_VERSION: anon_6 = 46;
pub const anon_6__SC_2_C_BIND: anon_6 = 47;
pub const anon_6__SC_2_C_DEV: anon_6 = 48;
pub const anon_6__SC_2_FORT_DEV: anon_6 = 49;
pub const anon_6__SC_2_FORT_RUN: anon_6 = 50;
pub const anon_6__SC_2_SW_DEV: anon_6 = 51;
pub const anon_6__SC_2_LOCALEDEF: anon_6 = 52;
pub const anon_6__SC_PII: anon_6 = 53;
pub const anon_6__SC_PII_XTI: anon_6 = 54;
pub const anon_6__SC_PII_SOCKET: anon_6 = 55;
pub const anon_6__SC_PII_INTERNET: anon_6 = 56;
pub const anon_6__SC_PII_OSI: anon_6 = 57;
pub const anon_6__SC_POLL: anon_6 = 58;
pub const anon_6__SC_SELECT: anon_6 = 59;
pub const anon_6__SC_UIO_MAXIOV: anon_6 = 60;
pub const anon_6__SC_IOV_MAX: anon_6 = 60;
pub const anon_6__SC_PII_INTERNET_STREAM: anon_6 = 61;
pub const anon_6__SC_PII_INTERNET_DGRAM: anon_6 = 62;
pub const anon_6__SC_PII_OSI_COTS: anon_6 = 63;
pub const anon_6__SC_PII_OSI_CLTS: anon_6 = 64;
pub const anon_6__SC_PII_OSI_M: anon_6 = 65;
pub const anon_6__SC_T_IOV_MAX: anon_6 = 66;
pub const anon_6__SC_THREADS: anon_6 = 67;
pub const anon_6__SC_THREAD_SAFE_FUNCTIONS: anon_6 = 68;
pub const anon_6__SC_GETGR_R_SIZE_MAX: anon_6 = 69;
pub const anon_6__SC_GETPW_R_SIZE_MAX: anon_6 = 70;
pub const anon_6__SC_LOGIN_NAME_MAX: anon_6 = 71;
pub const anon_6__SC_TTY_NAME_MAX: anon_6 = 72;
pub const anon_6__SC_THREAD_DESTRUCTOR_ITERATIONS: anon_6 = 73;
pub const anon_6__SC_THREAD_KEYS_MAX: anon_6 = 74;
pub const anon_6__SC_THREAD_STACK_MIN: anon_6 = 75;
pub const anon_6__SC_THREAD_THREADS_MAX: anon_6 = 76;
pub const anon_6__SC_THREAD_ATTR_STACKADDR: anon_6 = 77;
pub const anon_6__SC_THREAD_ATTR_STACKSIZE: anon_6 = 78;
pub const anon_6__SC_THREAD_PRIORITY_SCHEDULING: anon_6 = 79;
pub const anon_6__SC_THREAD_PRIO_INHERIT: anon_6 = 80;
pub const anon_6__SC_THREAD_PRIO_PROTECT: anon_6 = 81;
pub const anon_6__SC_THREAD_PROCESS_SHARED: anon_6 = 82;
pub const anon_6__SC_NPROCESSORS_CONF: anon_6 = 83;
pub const anon_6__SC_NPROCESSORS_ONLN: anon_6 = 84;
pub const anon_6__SC_PHYS_PAGES: anon_6 = 85;
pub const anon_6__SC_AVPHYS_PAGES: anon_6 = 86;
pub const anon_6__SC_ATEXIT_MAX: anon_6 = 87;
pub const anon_6__SC_PASS_MAX: anon_6 = 88;
pub const anon_6__SC_XOPEN_VERSION: anon_6 = 89;
pub const anon_6__SC_XOPEN_XCU_VERSION: anon_6 = 90;
pub const anon_6__SC_XOPEN_UNIX: anon_6 = 91;
pub const anon_6__SC_XOPEN_CRYPT: anon_6 = 92;
pub const anon_6__SC_XOPEN_ENH_I18N: anon_6 = 93;
pub const anon_6__SC_XOPEN_SHM: anon_6 = 94;
pub const anon_6__SC_2_CHAR_TERM: anon_6 = 95;
pub const anon_6__SC_2_C_VERSION: anon_6 = 96;
pub const anon_6__SC_2_UPE: anon_6 = 97;
pub const anon_6__SC_XOPEN_XPG2: anon_6 = 98;
pub const anon_6__SC_XOPEN_XPG3: anon_6 = 99;
pub const anon_6__SC_XOPEN_XPG4: anon_6 = 100;
pub const anon_6__SC_CHAR_BIT: anon_6 = 101;
pub const anon_6__SC_CHAR_MAX: anon_6 = 102;
pub const anon_6__SC_CHAR_MIN: anon_6 = 103;
pub const anon_6__SC_INT_MAX: anon_6 = 104;
pub const anon_6__SC_INT_MIN: anon_6 = 105;
pub const anon_6__SC_LONG_BIT: anon_6 = 106;
pub const anon_6__SC_WORD_BIT: anon_6 = 107;
pub const anon_6__SC_MB_LEN_MAX: anon_6 = 108;
pub const anon_6__SC_NZERO: anon_6 = 109;
pub const anon_6__SC_SSIZE_MAX: anon_6 = 110;
pub const anon_6__SC_SCHAR_MAX: anon_6 = 111;
pub const anon_6__SC_SCHAR_MIN: anon_6 = 112;
pub const anon_6__SC_SHRT_MAX: anon_6 = 113;
pub const anon_6__SC_SHRT_MIN: anon_6 = 114;
pub const anon_6__SC_UCHAR_MAX: anon_6 = 115;
pub const anon_6__SC_UINT_MAX: anon_6 = 116;
pub const anon_6__SC_ULONG_MAX: anon_6 = 117;
pub const anon_6__SC_USHRT_MAX: anon_6 = 118;
pub const anon_6__SC_NL_ARGMAX: anon_6 = 119;
pub const anon_6__SC_NL_LANGMAX: anon_6 = 120;
pub const anon_6__SC_NL_MSGMAX: anon_6 = 121;
pub const anon_6__SC_NL_NMAX: anon_6 = 122;
pub const anon_6__SC_NL_SETMAX: anon_6 = 123;
pub const anon_6__SC_NL_TEXTMAX: anon_6 = 124;
pub const anon_6__SC_XBS5_ILP32_OFF32: anon_6 = 125;
pub const anon_6__SC_XBS5_ILP32_OFFBIG: anon_6 = 126;
pub const anon_6__SC_XBS5_LP64_OFF64: anon_6 = 127;
pub const anon_6__SC_XBS5_LPBIG_OFFBIG: anon_6 = 128;
pub const anon_6__SC_XOPEN_LEGACY: anon_6 = 129;
pub const anon_6__SC_XOPEN_REALTIME: anon_6 = 130;
pub const anon_6__SC_XOPEN_REALTIME_THREADS: anon_6 = 131;
pub const anon_6__SC_ADVISORY_INFO: anon_6 = 132;
pub const anon_6__SC_BARRIERS: anon_6 = 133;
pub const anon_6__SC_BASE: anon_6 = 134;
pub const anon_6__SC_C_LANG_SUPPORT: anon_6 = 135;
pub const anon_6__SC_C_LANG_SUPPORT_R: anon_6 = 136;
pub const anon_6__SC_CLOCK_SELECTION: anon_6 = 137;
pub const anon_6__SC_CPUTIME: anon_6 = 138;
pub const anon_6__SC_THREAD_CPUTIME: anon_6 = 139;
pub const anon_6__SC_DEVICE_IO: anon_6 = 140;
pub const anon_6__SC_DEVICE_SPECIFIC: anon_6 = 141;
pub const anon_6__SC_DEVICE_SPECIFIC_R: anon_6 = 142;
pub const anon_6__SC_FD_MGMT: anon_6 = 143;
pub const anon_6__SC_FIFO: anon_6 = 144;
pub const anon_6__SC_PIPE: anon_6 = 145;
pub const anon_6__SC_FILE_ATTRIBUTES: anon_6 = 146;
pub const anon_6__SC_FILE_LOCKING: anon_6 = 147;
pub const anon_6__SC_FILE_SYSTEM: anon_6 = 148;
pub const anon_6__SC_MONOTONIC_CLOCK: anon_6 = 149;
pub const anon_6__SC_MULTI_PROCESS: anon_6 = 150;
pub const anon_6__SC_SINGLE_PROCESS: anon_6 = 151;
pub const anon_6__SC_NETWORKING: anon_6 = 152;
pub const anon_6__SC_READER_WRITER_LOCKS: anon_6 = 153;
pub const anon_6__SC_SPIN_LOCKS: anon_6 = 154;
pub const anon_6__SC_REGEXP: anon_6 = 155;
pub const anon_6__SC_REGEX_VERSION: anon_6 = 156;
pub const anon_6__SC_SHELL: anon_6 = 157;
pub const anon_6__SC_SIGNALS: anon_6 = 158;
pub const anon_6__SC_SPAWN: anon_6 = 159;
pub const anon_6__SC_SPORADIC_SERVER: anon_6 = 160;
pub const anon_6__SC_THREAD_SPORADIC_SERVER: anon_6 = 161;
pub const anon_6__SC_SYSTEM_DATABASE: anon_6 = 162;
pub const anon_6__SC_SYSTEM_DATABASE_R: anon_6 = 163;
pub const anon_6__SC_TIMEOUTS: anon_6 = 164;
pub const anon_6__SC_TYPED_MEMORY_OBJECTS: anon_6 = 165;
pub const anon_6__SC_USER_GROUPS: anon_6 = 166;
pub const anon_6__SC_USER_GROUPS_R: anon_6 = 167;
pub const anon_6__SC_2_PBS: anon_6 = 168;
pub const anon_6__SC_2_PBS_ACCOUNTING: anon_6 = 169;
pub const anon_6__SC_2_PBS_LOCATE: anon_6 = 170;
pub const anon_6__SC_2_PBS_MESSAGE: anon_6 = 171;
pub const anon_6__SC_2_PBS_TRACK: anon_6 = 172;
pub const anon_6__SC_SYMLOOP_MAX: anon_6 = 173;
pub const anon_6__SC_STREAMS: anon_6 = 174;
pub const anon_6__SC_2_PBS_CHECKPOINT: anon_6 = 175;
pub const anon_6__SC_V6_ILP32_OFF32: anon_6 = 176;
pub const anon_6__SC_V6_ILP32_OFFBIG: anon_6 = 177;
pub const anon_6__SC_V6_LP64_OFF64: anon_6 = 178;
pub const anon_6__SC_V6_LPBIG_OFFBIG: anon_6 = 179;
pub const anon_6__SC_HOST_NAME_MAX: anon_6 = 180;
pub const anon_6__SC_TRACE: anon_6 = 181;
pub const anon_6__SC_TRACE_EVENT_FILTER: anon_6 = 182;
pub const anon_6__SC_TRACE_INHERIT: anon_6 = 183;
pub const anon_6__SC_TRACE_LOG: anon_6 = 184;
pub const anon_6__SC_LEVEL1_ICACHE_SIZE: anon_6 = 185;
pub const anon_6__SC_LEVEL1_ICACHE_ASSOC: anon_6 = 186;
pub const anon_6__SC_LEVEL1_ICACHE_LINESIZE: anon_6 = 187;
pub const anon_6__SC_LEVEL1_DCACHE_SIZE: anon_6 = 188;
pub const anon_6__SC_LEVEL1_DCACHE_ASSOC: anon_6 = 189;
pub const anon_6__SC_LEVEL1_DCACHE_LINESIZE: anon_6 = 190;
pub const anon_6__SC_LEVEL2_CACHE_SIZE: anon_6 = 191;
pub const anon_6__SC_LEVEL2_CACHE_ASSOC: anon_6 = 192;
pub const anon_6__SC_LEVEL2_CACHE_LINESIZE: anon_6 = 193;
pub const anon_6__SC_LEVEL3_CACHE_SIZE: anon_6 = 194;
pub const anon_6__SC_LEVEL3_CACHE_ASSOC: anon_6 = 195;
pub const anon_6__SC_LEVEL3_CACHE_LINESIZE: anon_6 = 196;
pub const anon_6__SC_LEVEL4_CACHE_SIZE: anon_6 = 197;
pub const anon_6__SC_LEVEL4_CACHE_ASSOC: anon_6 = 198;
pub const anon_6__SC_LEVEL4_CACHE_LINESIZE: anon_6 = 199;
pub const anon_6__SC_IPV6: anon_6 = 235;
pub const anon_6__SC_RAW_SOCKETS: anon_6 = 236;
pub const anon_6__SC_V7_ILP32_OFF32: anon_6 = 237;
pub const anon_6__SC_V7_ILP32_OFFBIG: anon_6 = 238;
pub const anon_6__SC_V7_LP64_OFF64: anon_6 = 239;
pub const anon_6__SC_V7_LPBIG_OFFBIG: anon_6 = 240;
pub const anon_6__SC_SS_REPL_MAX: anon_6 = 241;
pub const anon_6__SC_TRACE_EVENT_NAME_MAX: anon_6 = 242;
pub const anon_6__SC_TRACE_NAME_MAX: anon_6 = 243;
pub const anon_6__SC_TRACE_SYS_MAX: anon_6 = 244;
pub const anon_6__SC_TRACE_USER_EVENT_MAX: anon_6 = 245;
pub const anon_6__SC_XOPEN_STREAMS: anon_6 = 246;
pub const anon_6__SC_THREAD_ROBUST_PRIO_INHERIT: anon_6 = 247;
pub const anon_6__SC_THREAD_ROBUST_PRIO_PROTECT: anon_6 = 248;
pub const anon_6__SC_MINSIGSTKSZ: anon_6 = 249;
pub const anon_6__SC_SIGSTKSZ: anon_6 = 250;
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut s: i32 = ({ socket_7(2, (__socket_type_SOCK_STREAM as i32), 0) });
    {
        &(4usize);
        {
            if (((s >= 0) as i32) != 0) {
            } else {
                ({
                    __assert_fail_8(
                        Ptr::<i8>::from_string_literal(b"s >= 0"),
                        Ptr::<i8>::from_string_literal(b"sockopt.c"),
                        9_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    let on: Value<i32> = Rc::new(RefCell::new(1));
    {
        &(4usize);
        {
            if (((({
                let ___optval: AnyPtr = ((on.as_pointer()) as Ptr<i32>).to_any();
                let ___optlen: u32 = (4usize as u32);
                setsockopt_9(s, 1, 9, ___optval, ___optlen)
            }) == 0) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_8(
                        Ptr::<i8>::from_string_literal(
                            b"setsockopt(s, SOL_SOCKET, SO_KEEPALIVE, &on, sizeof(on)) == 0",
                        ),
                        Ptr::<i8>::from_string_literal(b"sockopt.c"),
                        11_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({
                let ___optval: AnyPtr = ((on.as_pointer()) as Ptr<i32>).to_any();
                let ___optlen: u32 = (4usize as u32);
                setsockopt_9(s, (anon_1_IPPROTO_TCP as i32), 1, ___optval, ___optlen)
            }) == 0) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_8(
                        Ptr::<i8>::from_string_literal(
                            b"setsockopt(s, IPPROTO_TCP, TCP_NODELAY, &on, sizeof(on)) == 0",
                        ),
                        Ptr::<i8>::from_string_literal(b"sockopt.c"),
                        12_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    let err: Value<i32> = Rc::new(RefCell::new(-1_i32));
    let len: Value<u32> = Rc::new(RefCell::new((4usize as u32)));
    {
        &(4usize);
        {
            if (((({
                getsockopt_10(
                    s,
                    1,
                    4,
                    ((err.as_pointer()) as Ptr<i32>).to_any(),
                    (len.as_pointer()),
                )
            }) == 0) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_8(
                        Ptr::<i8>::from_string_literal(
                            b"getsockopt(s, SOL_SOCKET, SO_ERROR, &err, &len) == 0",
                        ),
                        Ptr::<i8>::from_string_literal(b"sockopt.c"),
                        15_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if ((((*err.borrow()) == 0) as i32) != 0) {
            } else {
                ({
                    __assert_fail_8(
                        Ptr::<i8>::from_string_literal(b"err == 0"),
                        Ptr::<i8>::from_string_literal(b"sockopt.c"),
                        16_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({ close_11(s) }) == 0) as i32) != 0) {
            } else {
                ({
                    __assert_fail_8(
                        Ptr::<i8>::from_string_literal(b"close(s) == 0"),
                        Ptr::<i8>::from_string_literal(b"sockopt.c"),
                        17_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    return 0;
}
pub fn socket_7(mut __domain: i32, mut __type: i32, mut __protocol: i32) -> i32 {
    unimplemented!()
}
pub fn __assert_fail_8(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn setsockopt_9(
    mut __fd: i32,
    mut __level: i32,
    mut __optname: i32,
    mut __optval: AnyPtr,
    mut __optlen: u32,
) -> i32 {
    unimplemented!()
}
pub fn getsockopt_10(
    mut __fd: i32,
    mut __level: i32,
    mut __optname: i32,
    mut __optval: AnyPtr,
    mut __optlen: Ptr<u32>,
) -> i32 {
    unimplemented!()
}
pub fn close_11(mut __fd: i32) -> i32 {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {}
