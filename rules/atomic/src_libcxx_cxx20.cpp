#include <atomic>
#include <cstddef>
#include <utility>

template <typename T1>
T1 *f422(std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.fetch_and(a1, a2);
}

template <typename T1>
T1 *f423(std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.fetch_or(a1, a2);
}

template <typename T1>
T1 *f424(std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.fetch_xor(a1, a2);
}

template <typename T1>
T1 *f425(std::atomic<T1 *> &a0, T1 *a1)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.operator&=(a1);
}

template <typename T1>
T1 *f426(std::atomic<T1 *> &a0, T1 *a1)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.operator|=(a1);
}

template <typename T1>
T1 *f427(std::atomic<T1 *> &a0, T1 *a1)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.operator^=(a1);
}

template <typename T1>
bool f428(const std::atomic<T1 *> &a0)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.is_lock_free();
}

template <typename T1>
void f429(std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.store(a1, a2);
}

template <typename T1>
T1 *f430(const std::atomic<T1 *> &a0, std::memory_order a1)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.load(a1);
}

template <typename T1>
T1 *f431(const std::atomic<T1 *> &a0)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return static_cast<T1 *>(a0);
}

template <typename T1>
T1 *f432(std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.exchange(a1, a2);
}

template <typename T1>
bool f433(std::atomic<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3,
          std::memory_order a4)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

template <typename T1>
bool f434(std::atomic<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3,
          std::memory_order a4)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

template <typename T1>
bool f435(std::atomic<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.compare_exchange_weak(a1, a2, a3);
}

template <typename T1>
bool f436(std::atomic<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.compare_exchange_strong(a1, a2, a3);
}

template <typename T1>
void f437(const std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.wait(a1, a2);
}

template <typename T1>
void f438(std::atomic<T1 *> &a0)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.notify_one();
}

template <typename T1>
void f439(std::atomic<T1 *> &a0)
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
std::atomic<T1> f440()
  requires(std::is_floating_point_v<T1>)
{
  return {};
}

template <typename T1>
std::atomic<T1> f441(T1 a0)
  requires(std::is_floating_point_v<T1>)
{
  return {a0};
}

template <typename T1>
T1 f442(std::atomic<T1> &a0, T1 a1)
  requires(std::is_floating_point_v<T1>)
{
  return a0.operator=(a1);
}

template <typename T1>
T1 f443(std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_floating_point_v<T1>)
{
  return a0.fetch_add(a1, a2);
}

template <typename T1>
T1 f444(std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_floating_point_v<T1>)
{
  return a0.fetch_sub(a1, a2);
}

template <typename T1>
T1 f445(std::atomic<T1> &a0, T1 a1)
  requires(std::is_floating_point_v<T1>)
{
  return a0.operator+=(a1);
}

template <typename T1>
T1 f446(std::atomic<T1> &a0, T1 a1)
  requires(std::is_floating_point_v<T1>)
{
  return a0.operator-=(a1);
}

template <typename T1>
bool f447(const std::atomic<T1> &a0)
  requires(std::is_floating_point_v<T1>)
{
  return a0.is_lock_free();
}

template <typename T1>
void f448(std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_floating_point_v<T1>)
{
  return a0.store(a1, a2);
}

template <typename T1>
T1 f449(const std::atomic<T1> &a0, std::memory_order a1)
  requires(std::is_floating_point_v<T1>)
{
  return a0.load(a1);
}

template <typename T1>
T1 f450(const std::atomic<T1> &a0)
  requires(std::is_floating_point_v<T1>)
{
  return static_cast<T1>(a0);
}

template <typename T1>
T1 f451(std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_floating_point_v<T1>)
{
  return a0.exchange(a1, a2);
}

template <typename T1>
bool f452(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
          std::memory_order a4)
  requires(std::is_floating_point_v<T1>)
{
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

template <typename T1>
bool f453(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
          std::memory_order a4)
  requires(std::is_floating_point_v<T1>)
{
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

template <typename T1>
bool f454(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3)
  requires(std::is_floating_point_v<T1>)
{
  return a0.compare_exchange_weak(a1, a2, a3);
}

template <typename T1>
bool f455(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3)
  requires(std::is_floating_point_v<T1>)
{
  return a0.compare_exchange_strong(a1, a2, a3);
}

template <typename T1>
void f456(const std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_floating_point_v<T1>)
{
  return a0.wait(a1, a2);
}

template <typename T1>
void f457(std::atomic<T1> &a0)
  requires(std::is_floating_point_v<T1>)
{
  return a0.notify_one();
}

template <typename T1>
void f458(std::atomic<T1> &a0)
  requires(std::is_floating_point_v<T1>)
{
  return a0.notify_all();
}

template <typename T1>
T1 f459(std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.fetch_and(a1, a2);
}

template <typename T1>
T1 f460(std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.fetch_or(a1, a2);
}

template <typename T1>
T1 f461(std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.fetch_xor(a1, a2);
}

template <typename T1>
T1 f462(std::atomic<T1> &a0, int a1)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.operator++(a1);
}

template <typename T1>
T1 f463(std::atomic<T1> &a0, int a1)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.operator--(a1);
}

template <typename T1>
T1 f464(std::atomic<T1> &a0)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.operator++();
}

template <typename T1>
T1 f465(std::atomic<T1> &a0)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.operator--();
}

template <typename T1>
T1 f466(std::atomic<T1> &a0, T1 a1)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.operator&=(a1);
}

template <typename T1>
T1 f467(std::atomic<T1> &a0, T1 a1)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.operator|=(a1);
}

template <typename T1>
T1 f468(std::atomic<T1> &a0, T1 a1)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.operator^=(a1);
}

template <typename T1>
bool f469(const std::atomic<T1> &a0)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.is_lock_free();
}

template <typename T1>
void f470(std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.store(a1, a2);
}

template <typename T1>
T1 f471(const std::atomic<T1> &a0, std::memory_order a1)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.load(a1);
}

template <typename T1>
T1 f472(const std::atomic<T1> &a0)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return static_cast<T1>(a0);
}

template <typename T1>
T1 f473(std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.exchange(a1, a2);
}

template <typename T1>
bool f474(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
          std::memory_order a4)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

template <typename T1>
bool f475(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
          std::memory_order a4)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

template <typename T1>
bool f476(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.compare_exchange_weak(a1, a2, a3);
}

template <typename T1>
bool f477(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.compare_exchange_strong(a1, a2, a3);
}

template <typename T1>
void f478(const std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.wait(a1, a2);
}

template <typename T1>
void f479(std::atomic<T1> &a0)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.notify_one();
}

template <typename T1>
void f480(std::atomic<T1> &a0)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.notify_all();
}

template <typename T1>
auto f481(std::atomic<T1> &a0, int a1)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.operator++(a1);
}

template <typename T1>
auto f482(std::atomic<T1> &a0, int a1)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.operator--(a1);
}

template <typename T1>
auto f483(std::atomic<T1> &a0)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.operator++();
}

template <typename T1>
auto f484(std::atomic<T1> &a0)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.operator--();
}

template <typename T1>
bool f485(const std::atomic<T1> &a0)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.is_lock_free();
}

template <typename T1>
auto f486(const std::atomic<T1> &a0, std::memory_order a1)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.load(a1);
}

template <typename T1>
void f487(std::atomic<T1> &a0)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.notify_one();
}

template <typename T1>
void f488(std::atomic<T1> &a0)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.notify_all();
}

template <typename T1> void f489(volatile std::atomic<T1> *a0) {
  return std::atomic_notify_all<T1>(a0);
}

template <typename T1> void f490(volatile std::atomic<T1> *a0) {
  return std::atomic_notify_one<T1>(a0);
}

template <typename T1>
void f491(const volatile std::atomic<T1> *a0,
          typename std::atomic<T1>::value_type a1) {
  return std::atomic_wait<T1>(a0, a1);
}

template <typename T1>
void f492(const volatile std::atomic<T1> *a0,
          typename std::atomic<T1>::value_type a1, std::memory_order a2) {
  return std::atomic_wait_explicit<T1>(a0, a1, a2);
}
