#include <atomic>
#include <cstddef>
#include <utility>

template <typename T1> using t2 = std::atomic_ref<T1 *>;

template <typename T1>
using t3 = typename std::atomic_ref<T1 *>::difference_type;

template <typename T1> std::atomic_ref<T1 *> f3(T1 *&a0) {
  return std::atomic_ref<T1 *>(a0);
}

template <typename T1> T1 *f4(const std::atomic_ref<T1 *> &a0, T1 *a1) {
  return a0.operator=(a1);
}

template <typename T1>
T1 *f5(const std::atomic_ref<T1 *> &a0, std::ptrdiff_t a1,
       std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

template <typename T1>
T1 *f6(const std::atomic_ref<T1 *> &a0, std::ptrdiff_t a1,
       std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

template <typename T1> T1 *f7(const std::atomic_ref<T1 *> &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1> T1 *f8(const std::atomic_ref<T1 *> &a0, int a1) {
  return a0.operator--(a1);
}

template <typename T1> T1 *f9(const std::atomic_ref<T1 *> &a0) {
  return a0.operator++();
}

template <typename T1> T1 *f10(const std::atomic_ref<T1 *> &a0) {
  return a0.operator--();
}

template <typename T1>
T1 *f11(const std::atomic_ref<T1 *> &a0, std::ptrdiff_t a1) {
  return a0.operator+=(a1);
}

template <typename T1>
T1 *f12(const std::atomic_ref<T1 *> &a0, std::ptrdiff_t a1) {
  return a0.operator-=(a1);
}

template <typename T1> bool f13(const std::atomic_ref<T1 *> &a0) {
  return a0.is_lock_free();
}

template <typename T1>
void f14(const std::atomic_ref<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

template <typename T1>
T1 *f15(const std::atomic_ref<T1 *> &a0, std::memory_order a1) {
  return a0.load(a1);
}

template <typename T1> T1 *f16(const std::atomic_ref<T1 *> &a0) {
  return static_cast<T1 *>(a0);
}

template <typename T1>
T1 *f17(const std::atomic_ref<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

template <typename T1>
bool f18(const std::atomic_ref<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3,
         std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

template <typename T1>
bool f19(const std::atomic_ref<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3,
         std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

template <typename T1>
bool f20(const std::atomic_ref<T1 *> &a0, T1 *&a1, T1 *a2,
         std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

template <typename T1>
bool f21(const std::atomic_ref<T1 *> &a0, T1 *&a1, T1 *a2,
         std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

template <typename T1>
void f22(const std::atomic_ref<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

template <typename T1> void f23(const std::atomic_ref<T1 *> &a0) {
  return a0.notify_one();
}

template <typename T1> void f24(const std::atomic_ref<T1 *> &a0) {
  return a0.notify_all();
}

template <typename T1>
  requires((std::integral<T1> && !std::same_as<bool, T1>))
using t4 = std::atomic_ref<T1>;

template <typename T1> using t5 = typename std::atomic_ref<T1>::difference_type;

template <typename T1>
std::atomic_ref<T1> f25(T1 &a0)
  requires((std::integral<T1> && !std::same_as<bool, T1>))
{
  return std::atomic_ref<T1>(a0);
}

template <typename T1>
std::atomic_ref<T1> f26(const std::atomic_ref<T1> &a0)
  requires((std::integral<T1> && !std::same_as<bool, T1>))
{
  return std::atomic_ref<T1>(a0);
}

template <typename T1>
T1 f27(const std::atomic_ref<T1> &a0, T1 a1)
  requires((std::integral<T1> && !std::same_as<bool, T1>))
{
  return a0.operator=(a1);
}

template <typename T1>
T1 f28(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2)
  requires((std::integral<T1> && !std::same_as<bool, T1>))
{
  return a0.fetch_add(a1, a2);
}

template <typename T1>
T1 f29(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2)
  requires((std::integral<T1> && !std::same_as<bool, T1>))
{
  return a0.fetch_sub(a1, a2);
}

template <typename T1>
T1 f30(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2)
  requires((std::integral<T1> && !std::same_as<bool, T1>))
{
  return a0.fetch_and(a1, a2);
}

template <typename T1>
T1 f31(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2)
  requires((std::integral<T1> && !std::same_as<bool, T1>))
{
  return a0.fetch_or(a1, a2);
}

template <typename T1>
T1 f32(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2)
  requires((std::integral<T1> && !std::same_as<bool, T1>))
{
  return a0.fetch_xor(a1, a2);
}

template <typename T1>
T1 f33(const std::atomic_ref<T1> &a0, int a1)
  requires((std::integral<T1> && !std::same_as<bool, T1>))
{
  return a0.operator++(a1);
}

template <typename T1>
T1 f34(const std::atomic_ref<T1> &a0, int a1)
  requires((std::integral<T1> && !std::same_as<bool, T1>))
{
  return a0.operator--(a1);
}

template <typename T1>
T1 f35(const std::atomic_ref<T1> &a0)
  requires((std::integral<T1> && !std::same_as<bool, T1>))
{
  return a0.operator++();
}

template <typename T1>
T1 f36(const std::atomic_ref<T1> &a0)
  requires((std::integral<T1> && !std::same_as<bool, T1>))
{
  return a0.operator--();
}

template <typename T1>
T1 f37(const std::atomic_ref<T1> &a0, T1 a1)
  requires((std::integral<T1> && !std::same_as<bool, T1>))
{
  return a0.operator+=(a1);
}

template <typename T1>
T1 f38(const std::atomic_ref<T1> &a0, T1 a1)
  requires((std::integral<T1> && !std::same_as<bool, T1>))
{
  return a0.operator-=(a1);
}

template <typename T1>
T1 f39(const std::atomic_ref<T1> &a0, T1 a1)
  requires((std::integral<T1> && !std::same_as<bool, T1>))
{
  return a0.operator&=(a1);
}

template <typename T1>
T1 f40(const std::atomic_ref<T1> &a0, T1 a1)
  requires((std::integral<T1> && !std::same_as<bool, T1>))
{
  return a0.operator|=(a1);
}

template <typename T1>
T1 f41(const std::atomic_ref<T1> &a0, T1 a1)
  requires((std::integral<T1> && !std::same_as<bool, T1>))
{
  return a0.operator^=(a1);
}

template <typename T1>
bool f42(const std::atomic_ref<T1> &a0)
  requires((std::integral<T1> && !std::same_as<bool, T1>))
{
  return a0.is_lock_free();
}

template <typename T1>
void f43(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2)
  requires((std::integral<T1> && !std::same_as<bool, T1>))
{
  return a0.store(a1, a2);
}

template <typename T1>
T1 f44(const std::atomic_ref<T1> &a0, std::memory_order a1)
  requires((std::integral<T1> && !std::same_as<bool, T1>))
{
  return a0.load(a1);
}

template <typename T1>
T1 f45(const std::atomic_ref<T1> &a0)
  requires((std::integral<T1> && !std::same_as<bool, T1>))
{
  return static_cast<T1>(a0);
}

template <typename T1>
T1 f46(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2)
  requires((std::integral<T1> && !std::same_as<bool, T1>))
{
  return a0.exchange(a1, a2);
}

template <typename T1>
bool f47(const std::atomic_ref<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
         std::memory_order a4)
  requires((std::integral<T1> && !std::same_as<bool, T1>))
{
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

template <typename T1>
bool f48(const std::atomic_ref<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
         std::memory_order a4)
  requires((std::integral<T1> && !std::same_as<bool, T1>))
{
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

template <typename T1>
bool f49(const std::atomic_ref<T1> &a0, T1 &a1, T1 a2, std::memory_order a3)
  requires((std::integral<T1> && !std::same_as<bool, T1>))
{
  return a0.compare_exchange_weak(a1, a2, a3);
}

template <typename T1>
bool f50(const std::atomic_ref<T1> &a0, T1 &a1, T1 a2, std::memory_order a3)
  requires((std::integral<T1> && !std::same_as<bool, T1>))
{
  return a0.compare_exchange_strong(a1, a2, a3);
}

template <typename T1>
void f51(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2)
  requires((std::integral<T1> && !std::same_as<bool, T1>))
{
  return a0.wait(a1, a2);
}

template <typename T1>
void f52(const std::atomic_ref<T1> &a0)
  requires((std::integral<T1> && !std::same_as<bool, T1>))
{
  return a0.notify_one();
}

template <typename T1>
void f53(const std::atomic_ref<T1> &a0)
  requires((std::integral<T1> && !std::same_as<bool, T1>))
{
  return a0.notify_all();
}

template <typename T1>
  requires(std::floating_point<T1>)
using t6 = std::atomic_ref<T1>;

template <typename T1>
std::atomic_ref<T1> f54(T1 &a0)
  requires(std::floating_point<T1>)
{
  return std::atomic_ref<T1>(a0);
}

template <typename T1>
std::atomic_ref<T1> f55(const std::atomic_ref<T1> &a0)
  requires(std::floating_point<T1>)
{
  return std::atomic_ref<T1>(a0);
}

template <typename T1>
T1 f56(const std::atomic_ref<T1> &a0, T1 a1)
  requires(std::floating_point<T1>)
{
  return a0.operator=(a1);
}

template <typename T1>
T1 f57(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::floating_point<T1>)
{
  return a0.fetch_add(a1, a2);
}

template <typename T1>
T1 f58(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::floating_point<T1>)
{
  return a0.fetch_sub(a1, a2);
}

template <typename T1>
T1 f59(const std::atomic_ref<T1> &a0, T1 a1)
  requires(std::floating_point<T1>)
{
  return a0.operator+=(a1);
}

template <typename T1>
T1 f60(const std::atomic_ref<T1> &a0, T1 a1)
  requires(std::floating_point<T1>)
{
  return a0.operator-=(a1);
}

template <typename T1>
bool f61(const std::atomic_ref<T1> &a0)
  requires(std::floating_point<T1>)
{
  return a0.is_lock_free();
}

template <typename T1>
void f62(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::floating_point<T1>)
{
  return a0.store(a1, a2);
}

template <typename T1>
T1 f63(const std::atomic_ref<T1> &a0, std::memory_order a1)
  requires(std::floating_point<T1>)
{
  return a0.load(a1);
}

template <typename T1>
T1 f64(const std::atomic_ref<T1> &a0)
  requires(std::floating_point<T1>)
{
  return static_cast<T1>(a0);
}

template <typename T1>
T1 f65(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::floating_point<T1>)
{
  return a0.exchange(a1, a2);
}

template <typename T1>
bool f66(const std::atomic_ref<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
         std::memory_order a4)
  requires(std::floating_point<T1>)
{
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

template <typename T1>
bool f67(const std::atomic_ref<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
         std::memory_order a4)
  requires(std::floating_point<T1>)
{
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

template <typename T1>
bool f68(const std::atomic_ref<T1> &a0, T1 &a1, T1 a2, std::memory_order a3)
  requires(std::floating_point<T1>)
{
  return a0.compare_exchange_weak(a1, a2, a3);
}

template <typename T1>
bool f69(const std::atomic_ref<T1> &a0, T1 &a1, T1 a2, std::memory_order a3)
  requires(std::floating_point<T1>)
{
  return a0.compare_exchange_strong(a1, a2, a3);
}

template <typename T1>
void f70(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::floating_point<T1>)
{
  return a0.wait(a1, a2);
}

template <typename T1>
void f71(const std::atomic_ref<T1> &a0)
  requires(std::floating_point<T1>)
{
  return a0.notify_one();
}

template <typename T1>
void f72(const std::atomic_ref<T1> &a0)
  requires(std::floating_point<T1>)
{
  return a0.notify_all();
}

template <typename T1> T1 f73(const std::atomic_ref<T1> &a0, T1 a1) {
  return a0.operator=(a1);
}

template <typename T1> bool f74(const std::atomic_ref<T1> &a0) {
  return a0.is_lock_free();
}

template <typename T1>
void f75(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

template <typename T1>
T1 f76(const std::atomic_ref<T1> &a0, std::memory_order a1) {
  return a0.load(a1);
}

template <typename T1> T1 f77(const std::atomic_ref<T1> &a0) {
  return static_cast<T1>(a0);
}

template <typename T1>
T1 f78(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

template <typename T1>
bool f79(const std::atomic_ref<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
         std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

template <typename T1>
bool f80(const std::atomic_ref<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
         std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

template <typename T1>
bool f81(const std::atomic_ref<T1> &a0, T1 &a1, T1 a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

template <typename T1>
bool f82(const std::atomic_ref<T1> &a0, T1 &a1, T1 a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

template <typename T1>
void f83(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

template <typename T1> void f84(const std::atomic_ref<T1> &a0) {
  return a0.notify_one();
}

template <typename T1> void f85(const std::atomic_ref<T1> &a0) {
  return a0.notify_all();
}
