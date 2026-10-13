extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
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
#[byte_size(56)]
pub struct ifaddrs {
    #[offset(0)]
    #[byte_size(8)]
    pub ifa_next: Ptr<ifaddrs>,
    #[offset(8)]
    #[byte_size(8)]
    pub ifa_name: Ptr<i8>,
    #[offset(16)]
    pub ifa_flags: u32,
    #[offset(24)]
    #[byte_size(8)]
    pub ifa_addr: Ptr<sockaddr>,
    #[offset(32)]
    #[byte_size(8)]
    pub ifa_netmask: Ptr<sockaddr>,
    #[offset(40)]
    #[byte_size(8)]
    pub ifa_ifu: anon_1,
    #[offset(48)]
    #[byte_size(8)]
    pub ifa_data: AnyPtr,
}
impl Clone for ifaddrs {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for ifaddrs {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct anon_1 {
    #[offset(0)]
    #[byte_size(8)]
    pub ifu_broadaddr: Ptr<sockaddr>,
    #[offset(0)]
    #[byte_size(8)]
    pub ifu_dstaddr: Ptr<sockaddr>,
}
impl Clone for anon_1 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_1 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct if_nameindex {
    #[offset(0)]
    pub if_index: u32,
    #[offset(8)]
    #[byte_size(8)]
    pub if_name: Ptr<i8>,
}
impl Clone for if_nameindex {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for if_nameindex {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(48)]
pub struct ifaddr {
    #[offset(0)]
    #[byte_size(16)]
    pub ifa_addr: sockaddr,
    #[offset(16)]
    #[byte_size(16)]
    pub ifa_ifu: anon_2,
    #[offset(32)]
    #[byte_size(8)]
    pub ifa_ifp: Ptr<iface>,
    #[offset(40)]
    #[byte_size(8)]
    pub ifa_next: Ptr<ifaddr>,
}
impl Clone for ifaddr {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for ifaddr {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct anon_2 {
    #[offset(0)]
    #[byte_size(16)]
    pub ifu_broadaddr: sockaddr,
    #[offset(0)]
    #[byte_size(16)]
    pub ifu_dstaddr: sockaddr,
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
#[byte_size(24)]
pub struct ifmap {
    #[offset(0)]
    pub mem_start: u64,
    #[offset(8)]
    pub mem_end: u64,
    #[offset(16)]
    pub base_addr: u16,
    #[offset(18)]
    pub irq: u8,
    #[offset(19)]
    pub dma: u8,
    #[offset(20)]
    pub port: u8,
}
impl Clone for ifmap {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for ifmap {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(40)]
pub struct ifreq {
    #[offset(0)]
    #[byte_size(16)]
    pub ifr_ifrn: anon_3,
    #[offset(16)]
    #[byte_size(24)]
    pub ifr_ifru: anon_4,
}
impl Clone for ifreq {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for ifreq {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct anon_3 {
    #[offset(0)]
    #[byte_size(16)]
    pub ifrn_name: Value<Box<[i8]>>,
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
#[byte_size(24)]
pub struct anon_4 {
    #[offset(0)]
    #[byte_size(16)]
    pub ifru_addr: sockaddr,
    #[offset(0)]
    #[byte_size(16)]
    pub ifru_dstaddr: sockaddr,
    #[offset(0)]
    #[byte_size(16)]
    pub ifru_broadaddr: sockaddr,
    #[offset(0)]
    #[byte_size(16)]
    pub ifru_netmask: sockaddr,
    #[offset(0)]
    #[byte_size(16)]
    pub ifru_hwaddr: sockaddr,
    #[offset(0)]
    pub ifru_flags: i16,
    #[offset(0)]
    pub ifru_ivalue: i32,
    #[offset(0)]
    pub ifru_mtu: i32,
    #[offset(0)]
    #[byte_size(24)]
    pub ifru_map: ifmap,
    #[offset(0)]
    #[byte_size(16)]
    pub ifru_slave: Value<Box<[i8]>>,
    #[offset(0)]
    #[byte_size(16)]
    pub ifru_newname: Value<Box<[i8]>>,
    #[offset(0)]
    #[byte_size(8)]
    pub ifru_data: Ptr<i8>,
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
#[byte_size(16)]
pub struct ifconf {
    #[offset(0)]
    pub ifc_len: i32,
    #[offset(8)]
    #[byte_size(8)]
    pub ifc_ifcu: anon_5,
}
impl Clone for ifconf {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for ifconf {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct anon_5 {
    #[offset(0)]
    #[byte_size(8)]
    pub ifcu_buf: Ptr<i8>,
    #[offset(0)]
    #[byte_size(8)]
    pub ifcu_req: Ptr<ifreq>,
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct in6_addr {
    #[offset(0)]
    #[byte_size(16)]
    pub __in6_u: anon_6,
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
pub struct anon_6 {
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
impl Clone for anon_6 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_6 {
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
#[byte_size(232)]
pub struct __locale_struct {
    #[offset(0)]
    #[byte_size(104)]
    pub __locales: Value<Box<[Ptr<__locale_data>]>>,
    #[offset(104)]
    #[byte_size(8)]
    pub __ctype_b: Ptr<u16>,
    #[offset(112)]
    #[byte_size(8)]
    pub __ctype_tolower: Ptr<i32>,
    #[offset(120)]
    #[byte_size(8)]
    pub __ctype_toupper: Ptr<i32>,
    #[offset(128)]
    #[byte_size(104)]
    pub __names: Value<Box<[Ptr<i8>]>>,
}
impl Clone for __locale_struct {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __locale_struct {
    fn default() -> Self {
        unimplemented!()
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let list: Value<Ptr<ifaddrs>> = Rc::new(RefCell::new(Ptr::<ifaddrs>::null()));
    {
        &(4usize);
        {
            if (((({ getifaddrs_7((list.as_pointer())) }) == 0) as i32) != 0) {
            } else {
                ({
                    __assert_fail_8(
                        Ptr::<i8>::from_string_literal(b"getifaddrs(&list) == 0"),
                        Ptr::<i8>::from_string_literal(b"ifaddrs.c"),
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
            if (((!((*list.borrow()).is_null())) as i32) != 0) {
            } else {
                ({
                    __assert_fail_8(
                        Ptr::<i8>::from_string_literal(b"list != NULL"),
                        Ptr::<i8>::from_string_literal(b"ifaddrs.c"),
                        12_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    let mut found_loopback: i32 = 0;
    let mut ifa: Ptr<ifaddrs> = Ptr::<ifaddrs>::null();
    ifa = (*list.borrow()).clone();
    'loop_: while (((!((ifa).is_null())) as i32) != 0) {
        {
            &(4usize);
            {
                if (((!((ifa.with(|__s| __s.ifa_name.clone())).is_null())) as i32) != 0) {
                } else {
                    ({
                        __assert_fail_8(
                            Ptr::<i8>::from_string_literal(b"ifa->ifa_name != NULL"),
                            Ptr::<i8>::from_string_literal(b"ifaddrs.c"),
                            16_u32,
                            Ptr::<i8>::from_string_literal(b"int main(void)"),
                        )
                    });
                }
            }
        };
        if ((((ifa.with(|__s| __s.ifa_addr.clone())).is_null()) as i32) != 0) {
            ifa = { ifa.with(|__s| __s.ifa_next.clone()) };
            continue 'loop_;
        }
        if ((((ifa
            .with(|__s| __s.ifa_addr.clone())
            .with(|__s| __s.sa_family) as i32)
            != 2) as i32)
            != 0)
        {
            ifa = { ifa.with(|__s| __s.ifa_next.clone()) };
            continue 'loop_;
        }
        let mut sin: Ptr<sockaddr_in> = ifa
            .with(|__s| __s.ifa_addr.clone())
            .reinterpret_cast::<sockaddr_in>();
        let lo_be: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([127_u8, 0_u8, 0_u8, 1_u8])));
        if (((({
            memcmp_9(
                ((field_ptr!(sin, sin_addr)) as Ptr<in_addr>).to_any(),
                ((lo_be.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any(),
                4_usize,
            )
        }) == 0) as i32)
            != 0)
        {
            found_loopback = 1;
            {
                &(4usize);
                {
                    if (((ifa.with(|__s| __s.ifa_flags) != 0_u32) as i32) != 0) {
                    } else {
                        ({
                            __assert_fail_8(
                                Ptr::<i8>::from_string_literal(b"ifa->ifa_flags != 0"),
                                Ptr::<i8>::from_string_literal(b"ifaddrs.c"),
                                27_u32,
                                Ptr::<i8>::from_string_literal(b"int main(void)"),
                            )
                        });
                    }
                }
            };
            {
                &(4usize);
                {
                    if (((!((ifa.with(|__s| __s.ifa_netmask.clone())).is_null())) as i32) != 0) {
                    } else {
                        ({
                            __assert_fail_8(
                                Ptr::<i8>::from_string_literal(b"ifa->ifa_netmask != NULL"),
                                Ptr::<i8>::from_string_literal(b"ifaddrs.c"),
                                28_u32,
                                Ptr::<i8>::from_string_literal(b"int main(void)"),
                            )
                        });
                    }
                }
            };
            let mut mask: Ptr<sockaddr_in> = ifa
                .with(|__s| __s.ifa_netmask.clone())
                .reinterpret_cast::<sockaddr_in>();
            let mask_be: Value<Box<[u8]>> =
                Rc::new(RefCell::new(Box::new([255_u8, 0_u8, 0_u8, 0_u8])));
            {
                &(4usize);
                {
                    if (((({
                        memcmp_9(
                            ((field_ptr!(mask, sin_addr)) as Ptr<in_addr>).to_any(),
                            ((mask_be.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any(),
                            4_usize,
                        )
                    }) == 0) as i32)
                        != 0)
                    {
                    } else {
                        ({
                            __assert_fail_8(
                                Ptr::<i8>::from_string_literal(
                                    b"memcmp(&mask->sin_addr, mask_be, 4) == 0",
                                ),
                                Ptr::<i8>::from_string_literal(b"ifaddrs.c"),
                                31_u32,
                                Ptr::<i8>::from_string_literal(b"int main(void)"),
                            )
                        });
                    }
                }
            };
            {
                &(4usize);
                {
                    if (((({ if_nametoindex_10(ifa.with(|__s| __s.ifa_name.clone())) }) > 0_u32)
                        as i32)
                        != 0)
                    {
                    } else {
                        ({
                            __assert_fail_8(
                                Ptr::<i8>::from_string_literal(
                                    b"if_nametoindex(ifa->ifa_name) > 0",
                                ),
                                Ptr::<i8>::from_string_literal(b"ifaddrs.c"),
                                32_u32,
                                Ptr::<i8>::from_string_literal(b"int main(void)"),
                            )
                        });
                    }
                }
            };
        }
        ifa = { ifa.with(|__s| __s.ifa_next.clone()) };
    }
    {
        &(4usize);
        {
            if (found_loopback != 0) {
            } else {
                ({
                    __assert_fail_8(
                        Ptr::<i8>::from_string_literal(b"found_loopback"),
                        Ptr::<i8>::from_string_literal(b"ifaddrs.c"),
                        35_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    ({ freeifaddrs_11((*list.borrow()).clone()) });
    {
        &(4usize);
        {
            if (((({ if_nametoindex_10(Ptr::<i8>::from_string_literal(b"cpp2rust_no_such_if")) })
                == 0_u32) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_8(
                        Ptr::<i8>::from_string_literal(
                            b"if_nametoindex(\"cpp2rust_no_such_if\") == 0",
                        ),
                        Ptr::<i8>::from_string_literal(b"ifaddrs.c"),
                        38_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct __locale_data;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct iface;
pub fn getifaddrs_7(mut __ifap: Ptr<Ptr<ifaddrs>>) -> i32 {
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
pub fn memcmp_9(mut __s1: AnyPtr, mut __s2: AnyPtr, mut __n: usize) -> i32 {
    unimplemented!()
}
pub fn if_nametoindex_10(mut __ifname: Ptr<i8>) -> u32 {
    unimplemented!()
}
pub fn freeifaddrs_11(mut __ifa: Ptr<ifaddrs>) {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {}
