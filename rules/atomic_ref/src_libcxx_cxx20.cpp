#include <atomic>
#include <cstddef>
#include <utility>

template <typename T1> using t2 = std::atomic_ref<T1 *>;

template <typename T1>
using t3 = typename std::atomic_ref<T1 *>::difference_type;

template <typename T1> std::atomic_ref<T1 *> f57(T1 *&a0) {
  return std::atomic_ref<T1 *>(a0);
}

template <typename T1> T1 *f58(const std::atomic_ref<T1 *> &a0, T1 *a1) {
  return a0.operator=(a1);
}

template <typename T1>
T1 *f59(const std::atomic_ref<T1 *> &a0, std::ptrdiff_t a1,
        std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

template <typename T1>
T1 *f60(const std::atomic_ref<T1 *> &a0, std::ptrdiff_t a1,
        std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

template <typename T1> T1 *f61(const std::atomic_ref<T1 *> &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1> T1 *f62(const std::atomic_ref<T1 *> &a0, int a1) {
  return a0.operator--(a1);
}

template <typename T1> T1 *f63(const std::atomic_ref<T1 *> &a0) {
  return a0.operator++();
}

template <typename T1> T1 *f64(const std::atomic_ref<T1 *> &a0) {
  return a0.operator--();
}

template <typename T1>
T1 *f65(const std::atomic_ref<T1 *> &a0, std::ptrdiff_t a1) {
  return a0.operator+=(a1);
}

template <typename T1>
T1 *f66(const std::atomic_ref<T1 *> &a0, std::ptrdiff_t a1) {
  return a0.operator-=(a1);
}

template <typename T1> bool f67(const std::atomic_ref<T1 *> &a0) {
  return a0.is_lock_free();
}

template <typename T1>
void f68(const std::atomic_ref<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

template <typename T1>
T1 *f69(const std::atomic_ref<T1 *> &a0, std::memory_order a1) {
  return a0.load(a1);
}

template <typename T1> T1 *f70(const std::atomic_ref<T1 *> &a0) {
  return static_cast<T1 *>(a0);
}

template <typename T1>
T1 *f71(const std::atomic_ref<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

template <typename T1>
bool f72(const std::atomic_ref<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3,
         std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

template <typename T1>
bool f73(const std::atomic_ref<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3,
         std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

template <typename T1>
bool f74(const std::atomic_ref<T1 *> &a0, T1 *&a1, T1 *a2,
         std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

template <typename T1>
bool f75(const std::atomic_ref<T1 *> &a0, T1 *&a1, T1 *a2,
         std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

template <typename T1>
void f76(const std::atomic_ref<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

template <typename T1> void f77(const std::atomic_ref<T1 *> &a0) {
  return a0.notify_one();
}

template <typename T1> void f78(const std::atomic_ref<T1 *> &a0) {
  return a0.notify_all();
}

template <typename T1>
  requires((std::integral<T1> && !std::same_as<bool, T1>))
using t4 = std::atomic_ref<T1>;

template <typename T1> using t5 = typename std::atomic_ref<T1>::difference_type;

template <typename T1>
std::atomic_ref<T1> f79(T1 &a0)
  requires((std::integral<T1> && !std::same_as<bool, T1>))
{
  return std::atomic_ref<T1>(a0);
}

template <typename T1>
std::atomic_ref<T1> f80(const std::atomic_ref<T1> &a0)
  requires((std::integral<T1> && !std::same_as<bool, T1>))
{
  return std::atomic_ref<T1>(a0);
}

template <typename T1>
T1 f81(const std::atomic_ref<T1> &a0, T1 a1)
  requires((std::integral<T1> && !std::same_as<bool, T1>))
{
  return a0.operator=(std::move(a1));
}

template <typename T1>
T1 f82(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2)
  requires((std::integral<T1> && !std::same_as<bool, T1>))
{
  return a0.fetch_add(std::move(a1), a2);
}

template <typename T1>
T1 f83(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2)
  requires((std::integral<T1> && !std::same_as<bool, T1>))
{
  return a0.fetch_sub(std::move(a1), a2);
}

template <typename T1>
T1 f84(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2)
  requires((std::integral<T1> && !std::same_as<bool, T1>))
{
  return a0.fetch_and(std::move(a1), a2);
}

template <typename T1>
T1 f85(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2)
  requires((std::integral<T1> && !std::same_as<bool, T1>))
{
  return a0.fetch_or(std::move(a1), a2);
}

template <typename T1>
T1 f86(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2)
  requires((std::integral<T1> && !std::same_as<bool, T1>))
{
  return a0.fetch_xor(std::move(a1), a2);
}

template <typename T1>
T1 f87(const std::atomic_ref<T1> &a0, int a1)
  requires((std::integral<T1> && !std::same_as<bool, T1>))
{
  return a0.operator++(a1);
}

template <typename T1>
T1 f88(const std::atomic_ref<T1> &a0, int a1)
  requires((std::integral<T1> && !std::same_as<bool, T1>))
{
  return a0.operator--(a1);
}

template <typename T1>
T1 f89(const std::atomic_ref<T1> &a0)
  requires((std::integral<T1> && !std::same_as<bool, T1>))
{
  return a0.operator++();
}

template <typename T1>
T1 f90(const std::atomic_ref<T1> &a0)
  requires((std::integral<T1> && !std::same_as<bool, T1>))
{
  return a0.operator--();
}

template <typename T1>
T1 f91(const std::atomic_ref<T1> &a0, T1 a1)
  requires((std::integral<T1> && !std::same_as<bool, T1>))
{
  return a0.operator+=(std::move(a1));
}

template <typename T1>
T1 f92(const std::atomic_ref<T1> &a0, T1 a1)
  requires((std::integral<T1> && !std::same_as<bool, T1>))
{
  return a0.operator-=(std::move(a1));
}

template <typename T1>
T1 f93(const std::atomic_ref<T1> &a0, T1 a1)
  requires((std::integral<T1> && !std::same_as<bool, T1>))
{
  return a0.operator&=(std::move(a1));
}

template <typename T1>
T1 f94(const std::atomic_ref<T1> &a0, T1 a1)
  requires((std::integral<T1> && !std::same_as<bool, T1>))
{
  return a0.operator|=(std::move(a1));
}

template <typename T1>
T1 f95(const std::atomic_ref<T1> &a0, T1 a1)
  requires((std::integral<T1> && !std::same_as<bool, T1>))
{
  return a0.operator^=(std::move(a1));
}

template <typename T1>
bool f96(const std::atomic_ref<T1> &a0)
  requires((std::integral<T1> && !std::same_as<bool, T1>))
{
  return a0.is_lock_free();
}

template <typename T1>
void f97(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2)
  requires((std::integral<T1> && !std::same_as<bool, T1>))
{
  return a0.store(std::move(a1), a2);
}

template <typename T1>
T1 f98(const std::atomic_ref<T1> &a0, std::memory_order a1)
  requires((std::integral<T1> && !std::same_as<bool, T1>))
{
  return a0.load(a1);
}

template <typename T1>
T1 f99(const std::atomic_ref<T1> &a0)
  requires((std::integral<T1> && !std::same_as<bool, T1>))
{
  return static_cast<T1>(a0);
}

template <typename T1>
T1 f100(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2)
  requires((std::integral<T1> && !std::same_as<bool, T1>))
{
  return a0.exchange(std::move(a1), a2);
}

template <typename T1>
bool f101(const std::atomic_ref<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
          std::memory_order a4)
  requires((std::integral<T1> && !std::same_as<bool, T1>))
{
  return a0.compare_exchange_weak(a1, std::move(a2), a3, a4);
}

template <typename T1>
bool f102(const std::atomic_ref<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
          std::memory_order a4)
  requires((std::integral<T1> && !std::same_as<bool, T1>))
{
  return a0.compare_exchange_strong(a1, std::move(a2), a3, a4);
}

template <typename T1>
bool f103(const std::atomic_ref<T1> &a0, T1 &a1, T1 a2, std::memory_order a3)
  requires((std::integral<T1> && !std::same_as<bool, T1>))
{
  return a0.compare_exchange_weak(a1, std::move(a2), a3);
}

template <typename T1>
bool f104(const std::atomic_ref<T1> &a0, T1 &a1, T1 a2, std::memory_order a3)
  requires((std::integral<T1> && !std::same_as<bool, T1>))
{
  return a0.compare_exchange_strong(a1, std::move(a2), a3);
}

template <typename T1>
void f105(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2)
  requires((std::integral<T1> && !std::same_as<bool, T1>))
{
  return a0.wait(std::move(a1), a2);
}

template <typename T1>
void f106(const std::atomic_ref<T1> &a0)
  requires((std::integral<T1> && !std::same_as<bool, T1>))
{
  return a0.notify_one();
}

template <typename T1>
void f107(const std::atomic_ref<T1> &a0)
  requires((std::integral<T1> && !std::same_as<bool, T1>))
{
  return a0.notify_all();
}

template <typename T1>
  requires(std::floating_point<T1>)
using t6 = std::atomic_ref<T1>;

template <typename T1>
std::atomic_ref<T1> f108(T1 &a0)
  requires(std::floating_point<T1>)
{
  return std::atomic_ref<T1>(a0);
}

template <typename T1>
std::atomic_ref<T1> f109(const std::atomic_ref<T1> &a0)
  requires(std::floating_point<T1>)
{
  return std::atomic_ref<T1>(a0);
}

template <typename T1>
T1 f110(const std::atomic_ref<T1> &a0, T1 a1)
  requires(std::floating_point<T1>)
{
  return a0.operator=(std::move(a1));
}

template <typename T1>
T1 f111(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::floating_point<T1>)
{
  return a0.fetch_add(std::move(a1), a2);
}

template <typename T1>
T1 f112(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::floating_point<T1>)
{
  return a0.fetch_sub(std::move(a1), a2);
}

template <typename T1>
T1 f113(const std::atomic_ref<T1> &a0, T1 a1)
  requires(std::floating_point<T1>)
{
  return a0.operator+=(std::move(a1));
}

template <typename T1>
T1 f114(const std::atomic_ref<T1> &a0, T1 a1)
  requires(std::floating_point<T1>)
{
  return a0.operator-=(std::move(a1));
}

template <typename T1>
bool f115(const std::atomic_ref<T1> &a0)
  requires(std::floating_point<T1>)
{
  return a0.is_lock_free();
}

template <typename T1>
void f116(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::floating_point<T1>)
{
  return a0.store(std::move(a1), a2);
}

template <typename T1>
T1 f117(const std::atomic_ref<T1> &a0, std::memory_order a1)
  requires(std::floating_point<T1>)
{
  return a0.load(a1);
}

template <typename T1>
T1 f118(const std::atomic_ref<T1> &a0)
  requires(std::floating_point<T1>)
{
  return static_cast<T1>(a0);
}

template <typename T1>
T1 f119(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::floating_point<T1>)
{
  return a0.exchange(std::move(a1), a2);
}

template <typename T1>
bool f120(const std::atomic_ref<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
          std::memory_order a4)
  requires(std::floating_point<T1>)
{
  return a0.compare_exchange_weak(a1, std::move(a2), a3, a4);
}

template <typename T1>
bool f121(const std::atomic_ref<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
          std::memory_order a4)
  requires(std::floating_point<T1>)
{
  return a0.compare_exchange_strong(a1, std::move(a2), a3, a4);
}

template <typename T1>
bool f122(const std::atomic_ref<T1> &a0, T1 &a1, T1 a2, std::memory_order a3)
  requires(std::floating_point<T1>)
{
  return a0.compare_exchange_weak(a1, std::move(a2), a3);
}

template <typename T1>
bool f123(const std::atomic_ref<T1> &a0, T1 &a1, T1 a2, std::memory_order a3)
  requires(std::floating_point<T1>)
{
  return a0.compare_exchange_strong(a1, std::move(a2), a3);
}

template <typename T1>
void f124(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::floating_point<T1>)
{
  return a0.wait(std::move(a1), a2);
}

template <typename T1>
void f125(const std::atomic_ref<T1> &a0)
  requires(std::floating_point<T1>)
{
  return a0.notify_one();
}

template <typename T1>
void f126(const std::atomic_ref<T1> &a0)
  requires(std::floating_point<T1>)
{
  return a0.notify_all();
}

template <typename T1> T1 f127(const std::atomic_ref<T1> &a0, T1 a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> bool f128(const std::atomic_ref<T1> &a0) {
  return a0.is_lock_free();
}

template <typename T1>
void f129(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.store(std::move(a1), a2);
}

template <typename T1>
T1 f130(const std::atomic_ref<T1> &a0, std::memory_order a1) {
  return a0.load(a1);
}

template <typename T1> T1 f131(const std::atomic_ref<T1> &a0) {
  return static_cast<T1>(a0);
}

template <typename T1>
T1 f132(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.exchange(std::move(a1), a2);
}

template <typename T1>
bool f133(const std::atomic_ref<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, std::move(a2), a3, a4);
}

template <typename T1>
bool f134(const std::atomic_ref<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, std::move(a2), a3, a4);
}

template <typename T1>
bool f135(const std::atomic_ref<T1> &a0, T1 &a1, T1 a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, std::move(a2), a3);
}

template <typename T1>
bool f136(const std::atomic_ref<T1> &a0, T1 &a1, T1 a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, std::move(a2), a3);
}

template <typename T1>
void f137(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.wait(std::move(a1), a2);
}

template <typename T1> void f138(const std::atomic_ref<T1> &a0) {
  return a0.notify_one();
}

template <typename T1> void f139(const std::atomic_ref<T1> &a0) {
  return a0.notify_all();
}
