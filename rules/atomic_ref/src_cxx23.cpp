#include <algorithm>
#include <atomic>
#include <utility>

template <typename T1> using t1 = std::atomic_ref<T1>;

template <typename T1> std::atomic_ref<T1> f1(T1 &a0) {
  return std::atomic_ref<T1>(a0);
}

template <typename T1> std::atomic_ref<T1> f2(const std::atomic_ref<T1> &a0) {
  return std::atomic_ref<T1>(a0);
}

template <typename T1>
T1 f3(const std::atomic_ref<T1> &a0)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == false) &&
          ((std::is_floating_point_v<T1>) == false) &&
          (!std::is_pointer<T1>::value)
{
  return static_cast<T1>(a0);
}

template <typename T1>
bool f4(const std::atomic_ref<T1> &a0)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == false) &&
          ((std::is_floating_point_v<T1>) == false) &&
          (!std::is_pointer<T1>::value)
{
  return a0.is_lock_free();
}

template <typename T1>
void f5(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == false) &&
          ((std::is_floating_point_v<T1>) == false) &&
          (!std::is_pointer<T1>::value)
{
  return a0.store(std::move(a1), a2);
}

template <typename T1>
T1 f6(const std::atomic_ref<T1> &a0, std::memory_order a1)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == false) &&
          ((std::is_floating_point_v<T1>) == false) &&
          (!std::is_pointer<T1>::value)
{
  return a0.load(a1);
}

template <typename T1>
T1 f7(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == false) &&
          ((std::is_floating_point_v<T1>) == false) &&
          (!std::is_pointer<T1>::value)
{
  return a0.exchange(std::move(a1), a2);
}

template <typename T1>
bool f8(const std::atomic_ref<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
        std::memory_order a4)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == false) &&
          ((std::is_floating_point_v<T1>) == false) &&
          (!std::is_pointer<T1>::value)
{
  return a0.compare_exchange_weak(a1, std::move(a2), a3, a4);
}

template <typename T1>
bool f9(const std::atomic_ref<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
        std::memory_order a4)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == false) &&
          ((std::is_floating_point_v<T1>) == false) &&
          (!std::is_pointer<T1>::value)
{
  return a0.compare_exchange_strong(a1, std::move(a2), a3, a4);
}

template <typename T1>
bool f10(const std::atomic_ref<T1> &a0, T1 &a1, T1 a2, std::memory_order a3)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == false) &&
          ((std::is_floating_point_v<T1>) == false) &&
          (!std::is_pointer<T1>::value)
{
  return a0.compare_exchange_weak(a1, std::move(a2), a3);
}

template <typename T1>
bool f11(const std::atomic_ref<T1> &a0, T1 &a1, T1 a2, std::memory_order a3)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == false) &&
          ((std::is_floating_point_v<T1>) == false) &&
          (!std::is_pointer<T1>::value)
{
  return a0.compare_exchange_strong(a1, std::move(a2), a3);
}

template <typename T1>
void f12(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == false) &&
          ((std::is_floating_point_v<T1>) == false) &&
          (!std::is_pointer<T1>::value)
{
  return a0.wait(std::move(a1), a2);
}

template <typename T1>
void f13(const std::atomic_ref<T1> &a0)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == false) &&
          ((std::is_floating_point_v<T1>) == false) &&
          (!std::is_pointer<T1>::value)
{
  return a0.notify_one();
}

template <typename T1>
void f14(const std::atomic_ref<T1> &a0)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == false) &&
          ((std::is_floating_point_v<T1>) == false) &&
          (!std::is_pointer<T1>::value)
{
  return a0.notify_all();
}

template <typename T1>
bool f15()
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == false) &&
          ((std::is_floating_point_v<T1>) == false) &&
          (!std::is_pointer<T1>::value)
{
  return std::atomic_ref<T1>::is_always_lock_free;
}

template <typename T1>
std::size_t f16()
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == false) &&
          ((std::is_floating_point_v<T1>) == false) &&
          (!std::is_pointer<T1>::value)
{
  return std::atomic_ref<T1>::required_alignment;
}

template <typename T1>
T1 f17(const std::atomic_ref<T1> &a0)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == true) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return static_cast<T1>(a0);
}

template <typename T1>
bool f18(const std::atomic_ref<T1> &a0)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == true) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return a0.is_lock_free();
}

template <typename T1>
void f19(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == true) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return a0.store(std::move(a1), a2);
}

template <typename T1>
T1 f20(const std::atomic_ref<T1> &a0, std::memory_order a1)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == true) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return a0.load(a1);
}

template <typename T1>
T1 f21(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == true) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return a0.exchange(std::move(a1), a2);
}

template <typename T1>
bool f22(const std::atomic_ref<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
         std::memory_order a4)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == true) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return a0.compare_exchange_weak(a1, std::move(a2), a3, a4);
}

template <typename T1>
bool f23(const std::atomic_ref<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
         std::memory_order a4)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == true) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return a0.compare_exchange_strong(a1, std::move(a2), a3, a4);
}

template <typename T1>
bool f24(const std::atomic_ref<T1> &a0, T1 &a1, T1 a2, std::memory_order a3)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == true) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return a0.compare_exchange_weak(a1, std::move(a2), a3);
}

template <typename T1>
bool f25(const std::atomic_ref<T1> &a0, T1 &a1, T1 a2, std::memory_order a3)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == true) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return a0.compare_exchange_strong(a1, std::move(a2), a3);
}

template <typename T1>
void f26(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == true) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return a0.wait(std::move(a1), a2);
}

template <typename T1>
void f27(const std::atomic_ref<T1> &a0)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == true) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return a0.notify_one();
}

template <typename T1>
void f28(const std::atomic_ref<T1> &a0)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == true) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return a0.notify_all();
}

template <typename T1>
T1 f29(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == true) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return a0.fetch_add(std::move(a1), a2);
}

template <typename T1>
T1 f30(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == true) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return a0.fetch_sub(std::move(a1), a2);
}

template <typename T1>
T1 f31(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == true) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return a0.fetch_and(std::move(a1), a2);
}

template <typename T1>
T1 f32(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == true) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return a0.fetch_or(std::move(a1), a2);
}

template <typename T1>
T1 f33(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == true) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return a0.fetch_xor(std::move(a1), a2);
}

template <typename T1>
T1 f34(const std::atomic_ref<T1> &a0, int a1)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == true) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return a0.operator++(a1);
}

template <typename T1>
T1 f35(const std::atomic_ref<T1> &a0, int a1)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == true) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return a0.operator--(a1);
}

template <typename T1>
T1 f36(const std::atomic_ref<T1> &a0)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == true) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return a0.operator++();
}

template <typename T1>
T1 f37(const std::atomic_ref<T1> &a0)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == true) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return a0.operator--();
}

template <typename T1>
T1 f38(const std::atomic_ref<T1> &a0, T1 a1)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == true) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return a0.operator+=(std::move(a1));
}

template <typename T1>
T1 f39(const std::atomic_ref<T1> &a0, T1 a1)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == true) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return a0.operator-=(std::move(a1));
}

template <typename T1>
T1 f40(const std::atomic_ref<T1> &a0, T1 a1)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == true) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return a0.operator&=(std::move(a1));
}

template <typename T1>
T1 f41(const std::atomic_ref<T1> &a0, T1 a1)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == true) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return a0.operator|=(std::move(a1));
}

template <typename T1>
T1 f42(const std::atomic_ref<T1> &a0, T1 a1)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == true) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return a0.operator^=(std::move(a1));
}

template <typename T1>
bool f43()
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == true) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return std::atomic_ref<T1>::is_always_lock_free;
}

template <typename T1>
std::size_t f44()
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == true) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return std::atomic_ref<T1>::required_alignment;
}

template <typename T1>
T1 f45(const std::atomic_ref<T1> &a0)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == false) &&
          ((std::is_floating_point_v<T1>) == true)
{
  return static_cast<T1>(a0);
}

template <typename T1>
bool f46(const std::atomic_ref<T1> &a0)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == false) &&
          ((std::is_floating_point_v<T1>) == true)
{
  return a0.is_lock_free();
}

template <typename T1>
void f47(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == false) &&
          ((std::is_floating_point_v<T1>) == true)
{
  return a0.store(std::move(a1), a2);
}

template <typename T1>
T1 f48(const std::atomic_ref<T1> &a0, std::memory_order a1)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == false) &&
          ((std::is_floating_point_v<T1>) == true)
{
  return a0.load(a1);
}

template <typename T1>
T1 f49(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == false) &&
          ((std::is_floating_point_v<T1>) == true)
{
  return a0.exchange(std::move(a1), a2);
}

template <typename T1>
bool f50(const std::atomic_ref<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
         std::memory_order a4)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == false) &&
          ((std::is_floating_point_v<T1>) == true)
{
  return a0.compare_exchange_weak(a1, std::move(a2), a3, a4);
}

template <typename T1>
bool f51(const std::atomic_ref<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
         std::memory_order a4)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == false) &&
          ((std::is_floating_point_v<T1>) == true)
{
  return a0.compare_exchange_strong(a1, std::move(a2), a3, a4);
}

template <typename T1>
bool f52(const std::atomic_ref<T1> &a0, T1 &a1, T1 a2, std::memory_order a3)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == false) &&
          ((std::is_floating_point_v<T1>) == true)
{
  return a0.compare_exchange_weak(a1, std::move(a2), a3);
}

template <typename T1>
bool f53(const std::atomic_ref<T1> &a0, T1 &a1, T1 a2, std::memory_order a3)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == false) &&
          ((std::is_floating_point_v<T1>) == true)
{
  return a0.compare_exchange_strong(a1, std::move(a2), a3);
}

template <typename T1>
void f54(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == false) &&
          ((std::is_floating_point_v<T1>) == true)
{
  return a0.wait(std::move(a1), a2);
}

template <typename T1>
void f55(const std::atomic_ref<T1> &a0)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == false) &&
          ((std::is_floating_point_v<T1>) == true)
{
  return a0.notify_one();
}

template <typename T1>
void f56(const std::atomic_ref<T1> &a0)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == false) &&
          ((std::is_floating_point_v<T1>) == true)
{
  return a0.notify_all();
}

template <typename T1>
T1 f57(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == false) &&
          ((std::is_floating_point_v<T1>) == true)
{
  return a0.fetch_add(std::move(a1), a2);
}

template <typename T1>
T1 f58(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == false) &&
          ((std::is_floating_point_v<T1>) == true)
{
  return a0.fetch_sub(std::move(a1), a2);
}

template <typename T1>
T1 f59(const std::atomic_ref<T1> &a0, T1 a1)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == false) &&
          ((std::is_floating_point_v<T1>) == true)
{
  return a0.operator+=(std::move(a1));
}

template <typename T1>
T1 f60(const std::atomic_ref<T1> &a0, T1 a1)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == false) &&
          ((std::is_floating_point_v<T1>) == true)
{
  return a0.operator-=(std::move(a1));
}

template <typename T1>
bool f61()
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == false) &&
          ((std::is_floating_point_v<T1>) == true)
{
  return std::atomic_ref<T1>::is_always_lock_free;
}

template <typename T1>
std::size_t f62()
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == false) &&
          ((std::is_floating_point_v<T1>) == true)
{
  return std::atomic_ref<T1>::required_alignment;
}

template <typename T1> using t2 = std::atomic_ref<T1 *>;

template <typename T1> std::atomic_ref<T1 *> f63(T1 *&a0) {
  return std::atomic_ref<T1 *>(a0);
}

template <typename T1>
std::atomic_ref<T1 *> f64(const std::atomic_ref<T1 *> &a0) {
  return std::atomic_ref<T1 *>(a0);
}

template <typename T1> T1 *f65(const std::atomic_ref<T1 *> &a0) {
  return static_cast<T1 *>(a0);
}

template <typename T1> bool f66(const std::atomic_ref<T1 *> &a0) {
  return a0.is_lock_free();
}

template <typename T1>
void f67(const std::atomic_ref<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

template <typename T1>
T1 *f68(const std::atomic_ref<T1 *> &a0, std::memory_order a1) {
  return a0.load(a1);
}

template <typename T1>
T1 *f69(const std::atomic_ref<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

template <typename T1>
bool f70(const std::atomic_ref<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3,
         std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

template <typename T1>
bool f71(const std::atomic_ref<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3,
         std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

template <typename T1>
bool f72(const std::atomic_ref<T1 *> &a0, T1 *&a1, T1 *a2,
         std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

template <typename T1>
bool f73(const std::atomic_ref<T1 *> &a0, T1 *&a1, T1 *a2,
         std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

template <typename T1>
void f74(const std::atomic_ref<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

template <typename T1> void f75(const std::atomic_ref<T1 *> &a0) {
  return a0.notify_one();
}

template <typename T1> void f76(const std::atomic_ref<T1 *> &a0) {
  return a0.notify_all();
}

template <typename T1>
T1 *f77(const std::atomic_ref<T1 *> &a0, std::ptrdiff_t a1,
        std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

template <typename T1>
T1 *f78(const std::atomic_ref<T1 *> &a0, std::ptrdiff_t a1,
        std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

template <typename T1> T1 *f79(const std::atomic_ref<T1 *> &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1> T1 *f80(const std::atomic_ref<T1 *> &a0, int a1) {
  return a0.operator--(a1);
}

template <typename T1> T1 *f81(const std::atomic_ref<T1 *> &a0) {
  return a0.operator++();
}

template <typename T1> T1 *f82(const std::atomic_ref<T1 *> &a0) {
  return a0.operator--();
}

template <typename T1>
T1 *f83(const std::atomic_ref<T1 *> &a0, std::ptrdiff_t a1) {
  return a0.operator+=(a1);
}

template <typename T1>
T1 *f84(const std::atomic_ref<T1 *> &a0, std::ptrdiff_t a1) {
  return a0.operator-=(a1);
}

template <typename T1> bool f85() {
  return std::atomic_ref<T1 *>::is_always_lock_free;
}

template <typename T1> std::size_t f86() {
  return std::atomic_ref<T1 *>::required_alignment;
}
