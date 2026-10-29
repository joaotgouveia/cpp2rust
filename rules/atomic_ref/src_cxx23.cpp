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
          ((std::is_floating_point_v<T1>) == false)
{
  return static_cast<T1>(a0);
}

template <typename T1>
bool f4(const std::atomic_ref<T1> &a0)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == false) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return a0.is_lock_free();
}

template <typename T1>
void f5(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == false) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return a0.store(std::move(a1), a2);
}

template <typename T1>
T1 f6(const std::atomic_ref<T1> &a0, std::memory_order a1)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == false) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return a0.load(a1);
}

template <typename T1>
T1 f7(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == false) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return a0.exchange(std::move(a1), a2);
}

template <typename T1>
bool f8(const std::atomic_ref<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
        std::memory_order a4)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == false) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return a0.compare_exchange_weak(a1, std::move(a2), a3, a4);
}

template <typename T1>
bool f9(const std::atomic_ref<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
        std::memory_order a4)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == false) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return a0.compare_exchange_strong(a1, std::move(a2), a3, a4);
}

template <typename T1>
bool f10(const std::atomic_ref<T1> &a0, T1 &a1, T1 a2, std::memory_order a3)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == false) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return a0.compare_exchange_weak(a1, std::move(a2), a3);
}

template <typename T1>
bool f11(const std::atomic_ref<T1> &a0, T1 &a1, T1 a2, std::memory_order a3)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == false) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return a0.compare_exchange_strong(a1, std::move(a2), a3);
}

template <typename T1>
void f12(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == false) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return a0.wait(std::move(a1), a2);
}

template <typename T1>
void f13(const std::atomic_ref<T1> &a0)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == false) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return a0.notify_one();
}

template <typename T1>
void f14(const std::atomic_ref<T1> &a0)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == false) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return a0.notify_all();
}

template <typename T1>
T1 f15(const std::atomic_ref<T1> &a0)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == true) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return static_cast<T1>(a0);
}

template <typename T1>
bool f16(const std::atomic_ref<T1> &a0)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == true) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return a0.is_lock_free();
}

template <typename T1>
void f17(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == true) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return a0.store(std::move(a1), a2);
}

template <typename T1>
T1 f18(const std::atomic_ref<T1> &a0, std::memory_order a1)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == true) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return a0.load(a1);
}

template <typename T1>
T1 f19(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == true) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return a0.exchange(std::move(a1), a2);
}

template <typename T1>
bool f20(const std::atomic_ref<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
         std::memory_order a4)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == true) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return a0.compare_exchange_weak(a1, std::move(a2), a3, a4);
}

template <typename T1>
bool f21(const std::atomic_ref<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
         std::memory_order a4)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == true) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return a0.compare_exchange_strong(a1, std::move(a2), a3, a4);
}

template <typename T1>
bool f22(const std::atomic_ref<T1> &a0, T1 &a1, T1 a2, std::memory_order a3)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == true) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return a0.compare_exchange_weak(a1, std::move(a2), a3);
}

template <typename T1>
bool f23(const std::atomic_ref<T1> &a0, T1 &a1, T1 a2, std::memory_order a3)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == true) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return a0.compare_exchange_strong(a1, std::move(a2), a3);
}

template <typename T1>
void f24(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == true) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return a0.wait(std::move(a1), a2);
}

template <typename T1>
void f25(const std::atomic_ref<T1> &a0)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == true) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return a0.notify_one();
}

template <typename T1>
void f26(const std::atomic_ref<T1> &a0)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == true) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return a0.notify_all();
}

template <typename T1>
T1 f27(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == true) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return a0.fetch_add(std::move(a1), a2);
}

template <typename T1>
T1 f28(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == true) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return a0.fetch_sub(std::move(a1), a2);
}

template <typename T1>
T1 f29(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == true) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return a0.fetch_and(std::move(a1), a2);
}

template <typename T1>
T1 f30(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == true) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return a0.fetch_or(std::move(a1), a2);
}

template <typename T1>
T1 f31(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == true) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return a0.fetch_xor(std::move(a1), a2);
}

template <typename T1>
T1 f32(const std::atomic_ref<T1> &a0, int a1)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == true) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return a0.operator++(a1);
}

template <typename T1>
T1 f33(const std::atomic_ref<T1> &a0, int a1)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == true) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return a0.operator--(a1);
}

template <typename T1>
T1 f34(const std::atomic_ref<T1> &a0)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == true) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return a0.operator++();
}

template <typename T1>
T1 f35(const std::atomic_ref<T1> &a0)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == true) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return a0.operator--();
}

template <typename T1>
T1 f36(const std::atomic_ref<T1> &a0, T1 a1)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == true) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return a0.operator+=(std::move(a1));
}

template <typename T1>
T1 f37(const std::atomic_ref<T1> &a0, T1 a1)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == true) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return a0.operator-=(std::move(a1));
}

template <typename T1>
T1 f38(const std::atomic_ref<T1> &a0, T1 a1)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == true) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return a0.operator&=(std::move(a1));
}

template <typename T1>
T1 f39(const std::atomic_ref<T1> &a0, T1 a1)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == true) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return a0.operator|=(std::move(a1));
}

template <typename T1>
T1 f40(const std::atomic_ref<T1> &a0, T1 a1)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == true) &&
          ((std::is_floating_point_v<T1>) == false)
{
  return a0.operator^=(std::move(a1));
}

template <typename T1>
T1 f41(const std::atomic_ref<T1> &a0)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == false) &&
          ((std::is_floating_point_v<T1>) == true)
{
  return static_cast<T1>(a0);
}

template <typename T1>
bool f42(const std::atomic_ref<T1> &a0)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == false) &&
          ((std::is_floating_point_v<T1>) == true)
{
  return a0.is_lock_free();
}

template <typename T1>
void f43(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == false) &&
          ((std::is_floating_point_v<T1>) == true)
{
  return a0.store(std::move(a1), a2);
}

template <typename T1>
T1 f44(const std::atomic_ref<T1> &a0, std::memory_order a1)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == false) &&
          ((std::is_floating_point_v<T1>) == true)
{
  return a0.load(a1);
}

template <typename T1>
T1 f45(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == false) &&
          ((std::is_floating_point_v<T1>) == true)
{
  return a0.exchange(std::move(a1), a2);
}

template <typename T1>
bool f46(const std::atomic_ref<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
         std::memory_order a4)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == false) &&
          ((std::is_floating_point_v<T1>) == true)
{
  return a0.compare_exchange_weak(a1, std::move(a2), a3, a4);
}

template <typename T1>
bool f47(const std::atomic_ref<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
         std::memory_order a4)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == false) &&
          ((std::is_floating_point_v<T1>) == true)
{
  return a0.compare_exchange_strong(a1, std::move(a2), a3, a4);
}

template <typename T1>
bool f48(const std::atomic_ref<T1> &a0, T1 &a1, T1 a2, std::memory_order a3)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == false) &&
          ((std::is_floating_point_v<T1>) == true)
{
  return a0.compare_exchange_weak(a1, std::move(a2), a3);
}

template <typename T1>
bool f49(const std::atomic_ref<T1> &a0, T1 &a1, T1 a2, std::memory_order a3)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == false) &&
          ((std::is_floating_point_v<T1>) == true)
{
  return a0.compare_exchange_strong(a1, std::move(a2), a3);
}

template <typename T1>
void f50(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == false) &&
          ((std::is_floating_point_v<T1>) == true)
{
  return a0.wait(std::move(a1), a2);
}

template <typename T1>
void f51(const std::atomic_ref<T1> &a0)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == false) &&
          ((std::is_floating_point_v<T1>) == true)
{
  return a0.notify_one();
}

template <typename T1>
void f52(const std::atomic_ref<T1> &a0)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == false) &&
          ((std::is_floating_point_v<T1>) == true)
{
  return a0.notify_all();
}

template <typename T1>
T1 f53(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == false) &&
          ((std::is_floating_point_v<T1>) == true)
{
  return a0.fetch_add(std::move(a1), a2);
}

template <typename T1>
T1 f54(const std::atomic_ref<T1> &a0, T1 a1, std::memory_order a2)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == false) &&
          ((std::is_floating_point_v<T1>) == true)
{
  return a0.fetch_sub(std::move(a1), a2);
}

template <typename T1>
T1 f55(const std::atomic_ref<T1> &a0, T1 a1)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == false) &&
          ((std::is_floating_point_v<T1>) == true)
{
  return a0.operator+=(std::move(a1));
}

template <typename T1>
T1 f56(const std::atomic_ref<T1> &a0, T1 a1)
  requires((std::is_integral_v<T1> && !std::is_same_v<T1, bool>) == false) &&
          ((std::is_floating_point_v<T1>) == true)
{
  return a0.operator-=(std::move(a1));
}
