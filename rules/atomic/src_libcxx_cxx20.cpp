#include <atomic>
#include <cstddef>
#include <utility>

template <typename T1>
void f599(const volatile std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

template <typename T1> void f600(volatile std::atomic<T1 *> &a0) {
  return a0.notify_one();
}

template <typename T1> void f601(volatile std::atomic<T1 *> &a0) {
  return a0.notify_all();
}

template <typename T1>
T1 *f602(volatile std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.fetch_and(a1, a2);
}

template <typename T1>
T1 *f603(std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.fetch_and(a1, a2);
}

template <typename T1>
T1 *f604(volatile std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.fetch_or(a1, a2);
}

template <typename T1>
T1 *f605(std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.fetch_or(a1, a2);
}

template <typename T1>
T1 *f606(volatile std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.fetch_xor(a1, a2);
}

template <typename T1>
T1 *f607(std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.fetch_xor(a1, a2);
}

template <typename T1>
T1 *f608(volatile std::atomic<T1 *> &a0, T1 *a1)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.operator&=(a1);
}

template <typename T1>
T1 *f609(std::atomic<T1 *> &a0, T1 *a1)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.operator&=(a1);
}

template <typename T1>
T1 *f610(volatile std::atomic<T1 *> &a0, T1 *a1)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.operator|=(a1);
}

template <typename T1>
T1 *f611(std::atomic<T1 *> &a0, T1 *a1)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.operator|=(a1);
}

template <typename T1>
T1 *f612(volatile std::atomic<T1 *> &a0, T1 *a1)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.operator^=(a1);
}

template <typename T1>
T1 *f613(std::atomic<T1 *> &a0, T1 *a1)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.operator^=(a1);
}

template <typename T1>
bool f614(const volatile std::atomic<T1 *> &a0)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.is_lock_free();
}

template <typename T1>
bool f615(const std::atomic<T1 *> &a0)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.is_lock_free();
}

template <typename T1>
void f616(volatile std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.store(a1, a2);
}

template <typename T1>
void f617(std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.store(a1, a2);
}

template <typename T1>
T1 *f618(const volatile std::atomic<T1 *> &a0, std::memory_order a1)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.load(a1);
}

template <typename T1>
T1 *f619(const std::atomic<T1 *> &a0, std::memory_order a1)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.load(a1);
}

template <typename T1>
T1 *f620(const volatile std::atomic<T1 *> &a0)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return static_cast<T1 *>(a0);
}

template <typename T1>
T1 *f621(const std::atomic<T1 *> &a0)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return static_cast<T1 *>(a0);
}

template <typename T1>
T1 *f622(volatile std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.exchange(a1, a2);
}

template <typename T1>
T1 *f623(std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.exchange(a1, a2);
}

template <typename T1>
bool f624(volatile std::atomic<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3,
          std::memory_order a4)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

template <typename T1>
bool f625(std::atomic<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3,
          std::memory_order a4)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

template <typename T1>
bool f626(volatile std::atomic<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3,
          std::memory_order a4)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

template <typename T1>
bool f627(std::atomic<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3,
          std::memory_order a4)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

template <typename T1>
bool f628(volatile std::atomic<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.compare_exchange_weak(a1, a2, a3);
}

template <typename T1>
bool f629(std::atomic<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.compare_exchange_weak(a1, a2, a3);
}

template <typename T1>
bool f630(volatile std::atomic<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.compare_exchange_strong(a1, a2, a3);
}

template <typename T1>
bool f631(std::atomic<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.compare_exchange_strong(a1, a2, a3);
}

template <typename T1>
void f632(const volatile std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.wait(a1, a2);
}

template <typename T1>
void f633(const std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.wait(a1, a2);
}

template <typename T1>
void f634(volatile std::atomic<T1 *> &a0)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.notify_one();
}

template <typename T1>
void f635(std::atomic<T1 *> &a0)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.notify_one();
}

template <typename T1>
void f636(volatile std::atomic<T1 *> &a0)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.notify_all();
}

template <typename T1>
void f637(std::atomic<T1 *> &a0)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.notify_all();
}

template <typename T1>
  requires(std::is_floating_point_v<T1>)
using t78 = std::atomic<T1>;

template <typename T1> using t79 = typename std::atomic<T1>::value_type;

template <typename T1> using t80 = typename std::atomic<T1>::difference_type;

template <typename T1>
std::atomic<T1> f638()
  requires(std::is_floating_point_v<T1>)
{
  return {};
}

template <typename T1>
std::atomic<T1> f639(T1 a0)
  requires(std::is_floating_point_v<T1>)
{
  return {a0};
}

template <typename T1>
T1 f640(volatile std::atomic<T1> &a0, T1 a1)
  requires(std::atomic<T1>::__base::is_always_lock_free) &&
          (std::is_floating_point_v<T1>)
{
  return a0.operator=(a1);
}

template <typename T1>
T1 f641(std::atomic<T1> &a0, T1 a1)
  requires(std::is_floating_point_v<T1>)
{
  return a0.operator=(a1);
}

template <typename T1>
T1 f642(volatile std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::atomic<T1>::__base::is_always_lock_free) &&
          (std::is_floating_point_v<T1>)
{
  return a0.fetch_add(a1, a2);
}

template <typename T1>
T1 f643(std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_floating_point_v<T1>)
{
  return a0.fetch_add(a1, a2);
}

template <typename T1>
T1 f644(volatile std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::atomic<T1>::__base::is_always_lock_free) &&
          (std::is_floating_point_v<T1>)
{
  return a0.fetch_sub(a1, a2);
}

template <typename T1>
T1 f645(std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_floating_point_v<T1>)
{
  return a0.fetch_sub(a1, a2);
}

template <typename T1>
T1 f646(volatile std::atomic<T1> &a0, T1 a1)
  requires(std::atomic<T1>::__base::is_always_lock_free) &&
          (std::is_floating_point_v<T1>)
{
  return a0.operator+=(a1);
}

template <typename T1>
T1 f647(std::atomic<T1> &a0, T1 a1)
  requires(std::is_floating_point_v<T1>)
{
  return a0.operator+=(a1);
}

template <typename T1>
T1 f648(volatile std::atomic<T1> &a0, T1 a1)
  requires(std::atomic<T1>::__base::is_always_lock_free) &&
          (std::is_floating_point_v<T1>)
{
  return a0.operator-=(a1);
}

template <typename T1>
T1 f649(std::atomic<T1> &a0, T1 a1)
  requires(std::is_floating_point_v<T1>)
{
  return a0.operator-=(a1);
}

template <typename T1>
bool f650(const volatile std::atomic<T1> &a0)
  requires(std::is_floating_point_v<T1>)
{
  return a0.is_lock_free();
}

template <typename T1>
bool f651(const std::atomic<T1> &a0)
  requires(std::is_floating_point_v<T1>)
{
  return a0.is_lock_free();
}

template <typename T1>
void f652(volatile std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_floating_point_v<T1>)
{
  return a0.store(a1, a2);
}

template <typename T1>
void f653(std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_floating_point_v<T1>)
{
  return a0.store(a1, a2);
}

template <typename T1>
T1 f654(const volatile std::atomic<T1> &a0, std::memory_order a1)
  requires(std::is_floating_point_v<T1>)
{
  return a0.load(a1);
}

template <typename T1>
T1 f655(const std::atomic<T1> &a0, std::memory_order a1)
  requires(std::is_floating_point_v<T1>)
{
  return a0.load(a1);
}

template <typename T1>
T1 f656(const volatile std::atomic<T1> &a0)
  requires(std::is_floating_point_v<T1>)
{
  return static_cast<T1>(a0);
}

template <typename T1>
T1 f657(const std::atomic<T1> &a0)
  requires(std::is_floating_point_v<T1>)
{
  return static_cast<T1>(a0);
}

template <typename T1>
T1 f658(volatile std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_floating_point_v<T1>)
{
  return a0.exchange(a1, a2);
}

template <typename T1>
T1 f659(std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_floating_point_v<T1>)
{
  return a0.exchange(a1, a2);
}

template <typename T1>
bool f660(volatile std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
          std::memory_order a4)
  requires(std::is_floating_point_v<T1>)
{
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

template <typename T1>
bool f661(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
          std::memory_order a4)
  requires(std::is_floating_point_v<T1>)
{
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

template <typename T1>
bool f662(volatile std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
          std::memory_order a4)
  requires(std::is_floating_point_v<T1>)
{
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

template <typename T1>
bool f663(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
          std::memory_order a4)
  requires(std::is_floating_point_v<T1>)
{
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

template <typename T1>
bool f664(volatile std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3)
  requires(std::is_floating_point_v<T1>)
{
  return a0.compare_exchange_weak(a1, a2, a3);
}

template <typename T1>
bool f665(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3)
  requires(std::is_floating_point_v<T1>)
{
  return a0.compare_exchange_weak(a1, a2, a3);
}

template <typename T1>
bool f666(volatile std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3)
  requires(std::is_floating_point_v<T1>)
{
  return a0.compare_exchange_strong(a1, a2, a3);
}

template <typename T1>
bool f667(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3)
  requires(std::is_floating_point_v<T1>)
{
  return a0.compare_exchange_strong(a1, a2, a3);
}

template <typename T1>
void f668(const volatile std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_floating_point_v<T1>)
{
  return a0.wait(a1, a2);
}

template <typename T1>
void f669(const std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_floating_point_v<T1>)
{
  return a0.wait(a1, a2);
}

template <typename T1>
void f670(volatile std::atomic<T1> &a0)
  requires(std::is_floating_point_v<T1>)
{
  return a0.notify_one();
}

template <typename T1>
void f671(std::atomic<T1> &a0)
  requires(std::is_floating_point_v<T1>)
{
  return a0.notify_one();
}

template <typename T1>
void f672(volatile std::atomic<T1> &a0)
  requires(std::is_floating_point_v<T1>)
{
  return a0.notify_all();
}

template <typename T1>
void f673(std::atomic<T1> &a0)
  requires(std::is_floating_point_v<T1>)
{
  return a0.notify_all();
}

template <typename T1>
T1 f674(volatile std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.fetch_and(a1, a2);
}

template <typename T1>
T1 f675(std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.fetch_and(a1, a2);
}

template <typename T1>
T1 f676(volatile std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.fetch_or(a1, a2);
}

template <typename T1>
T1 f677(std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.fetch_or(a1, a2);
}

template <typename T1>
T1 f678(volatile std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.fetch_xor(a1, a2);
}

template <typename T1>
T1 f679(std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.fetch_xor(a1, a2);
}

template <typename T1>
T1 f680(volatile std::atomic<T1> &a0, int a1)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.operator++(a1);
}

template <typename T1>
T1 f681(std::atomic<T1> &a0, int a1)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.operator++(a1);
}

template <typename T1>
T1 f682(volatile std::atomic<T1> &a0, int a1)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.operator--(a1);
}

template <typename T1>
T1 f683(std::atomic<T1> &a0, int a1)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.operator--(a1);
}

template <typename T1>
T1 f684(volatile std::atomic<T1> &a0)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.operator++();
}

template <typename T1>
T1 f685(std::atomic<T1> &a0)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.operator++();
}

template <typename T1>
T1 f686(volatile std::atomic<T1> &a0)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.operator--();
}

template <typename T1>
T1 f687(std::atomic<T1> &a0)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.operator--();
}

template <typename T1>
T1 f688(volatile std::atomic<T1> &a0, T1 a1)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.operator&=(a1);
}

template <typename T1>
T1 f689(std::atomic<T1> &a0, T1 a1)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.operator&=(a1);
}

template <typename T1>
T1 f690(volatile std::atomic<T1> &a0, T1 a1)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.operator|=(a1);
}

template <typename T1>
T1 f691(std::atomic<T1> &a0, T1 a1)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.operator|=(a1);
}

template <typename T1>
T1 f692(volatile std::atomic<T1> &a0, T1 a1)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.operator^=(a1);
}

template <typename T1>
T1 f693(std::atomic<T1> &a0, T1 a1)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.operator^=(a1);
}

template <typename T1>
bool f694(const volatile std::atomic<T1> &a0)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.is_lock_free();
}

template <typename T1>
bool f695(const std::atomic<T1> &a0)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.is_lock_free();
}

template <typename T1>
void f696(volatile std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.store(a1, a2);
}

template <typename T1>
void f697(std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.store(a1, a2);
}

template <typename T1>
T1 f698(const volatile std::atomic<T1> &a0, std::memory_order a1)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.load(a1);
}

template <typename T1>
T1 f699(const std::atomic<T1> &a0, std::memory_order a1)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.load(a1);
}

template <typename T1>
T1 f700(const volatile std::atomic<T1> &a0)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return static_cast<T1>(a0);
}

template <typename T1>
T1 f701(const std::atomic<T1> &a0)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return static_cast<T1>(a0);
}

template <typename T1>
T1 f702(volatile std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.exchange(a1, a2);
}

template <typename T1>
T1 f703(std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.exchange(a1, a2);
}

template <typename T1>
bool f704(volatile std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
          std::memory_order a4)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

template <typename T1>
bool f705(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
          std::memory_order a4)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

template <typename T1>
bool f706(volatile std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
          std::memory_order a4)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

template <typename T1>
bool f707(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
          std::memory_order a4)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

template <typename T1>
bool f708(volatile std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.compare_exchange_weak(a1, a2, a3);
}

template <typename T1>
bool f709(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.compare_exchange_weak(a1, a2, a3);
}

template <typename T1>
bool f710(volatile std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.compare_exchange_strong(a1, a2, a3);
}

template <typename T1>
bool f711(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.compare_exchange_strong(a1, a2, a3);
}

template <typename T1>
void f712(const volatile std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.wait(a1, a2);
}

template <typename T1>
void f713(const std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.wait(a1, a2);
}

template <typename T1>
void f714(volatile std::atomic<T1> &a0)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.notify_one();
}

template <typename T1>
void f715(std::atomic<T1> &a0)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.notify_one();
}

template <typename T1>
void f716(volatile std::atomic<T1> &a0)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.notify_all();
}

template <typename T1>
void f717(std::atomic<T1> &a0)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.notify_all();
}

template <typename T1> void f718(volatile std::atomic<T1> &a0) {
  return a0.notify_one();
}

template <typename T1> void f719(volatile std::atomic<T1> &a0) {
  return a0.notify_all();
}

template <typename T1>
auto f720(volatile std::atomic<T1> &a0, int a1)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.operator++(a1);
}

template <typename T1>
auto f721(std::atomic<T1> &a0, int a1)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.operator++(a1);
}

template <typename T1>
auto f722(volatile std::atomic<T1> &a0, int a1)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.operator--(a1);
}

template <typename T1>
auto f723(std::atomic<T1> &a0, int a1)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.operator--(a1);
}

template <typename T1>
auto f724(volatile std::atomic<T1> &a0)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.operator++();
}

template <typename T1>
auto f725(std::atomic<T1> &a0)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.operator++();
}

template <typename T1>
auto f726(volatile std::atomic<T1> &a0)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.operator--();
}

template <typename T1>
auto f727(std::atomic<T1> &a0)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.operator--();
}

template <typename T1>
bool f728(const volatile std::atomic<T1> &a0)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.is_lock_free();
}

template <typename T1>
bool f729(const std::atomic<T1> &a0)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.is_lock_free();
}

template <typename T1>
auto f730(const volatile std::atomic<T1> &a0, std::memory_order a1)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.load(a1);
}

template <typename T1>
auto f731(const std::atomic<T1> &a0, std::memory_order a1)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.load(a1);
}

template <typename T1>
void f732(volatile std::atomic<T1> &a0)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.notify_one();
}

template <typename T1>
void f733(std::atomic<T1> &a0)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.notify_one();
}

template <typename T1>
void f734(volatile std::atomic<T1> &a0)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.notify_all();
}

template <typename T1>
void f735(std::atomic<T1> &a0)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.notify_all();
}

template <typename T1> void f736(volatile std::atomic<T1> *a0) {
  return std::atomic_notify_all<T1>(a0);
}

template <typename T1> void f737(volatile std::atomic<T1> *a0) {
  return std::atomic_notify_one<T1>(a0);
}

template <typename T1>
void f738(const volatile std::atomic<T1> *a0,
          typename std::atomic<T1>::value_type a1) {
  return std::atomic_wait<T1>(a0, a1);
}

template <typename T1>
void f739(const volatile std::atomic<T1> *a0,
          typename std::atomic<T1>::value_type a1, std::memory_order a2) {
  return std::atomic_wait_explicit<T1>(a0, a1, a2);
}
