#include <atomic>
#include <chrono>
#include <condition_variable>
#include <utility>

template <typename T1>
void f1(std::unique_lock<T1> &a0, std::unique_lock<T1> &a1) {
  return std::swap<T1>(a0, a1);
}

template <typename T1> using t1 = std::unique_lock<T1>;

template <typename T1> using t2 = typename std::unique_lock<T1>::mutex_type;

template <typename T1> std::unique_lock<T1> f2() {
  return std::unique_lock<T1>();
}

template <typename T1> std::unique_lock<T1> f3(T1 &a0) {
  return std::unique_lock<T1>(a0);
}

template <typename T1> std::unique_lock<T1> f4(T1 &a0, std::defer_lock_t a1) {
  return std::unique_lock<T1>(a0, std::move(a1));
}

template <typename T1> std::unique_lock<T1> f5(T1 &a0, std::try_to_lock_t a1) {
  return std::unique_lock<T1>(a0, std::move(a1));
}

template <typename T1> std::unique_lock<T1> f6(T1 &a0, std::adopt_lock_t a1) {
  return std::unique_lock<T1>(a0, std::move(a1));
}

template <typename T1> std::unique_lock<T1> f7(std::unique_lock<T1> &&a0) {
  return std::unique_lock<T1>(std::move(a0));
}

template <typename T1>
std::unique_lock<T1> &f8(std::unique_lock<T1> &a0, std::unique_lock<T1> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> void f9(std::unique_lock<T1> &a0) { return a0.lock(); }

template <typename T1> bool f10(std::unique_lock<T1> &a0) {
  return a0.try_lock();
}

template <typename T1> void f11(std::unique_lock<T1> &a0) {
  return a0.unlock();
}

template <typename T1>
void f12(std::unique_lock<T1> &a0, std::unique_lock<T1> &a1) {
  return a0.swap(a1);
}

template <typename T1> T1 *f13(std::unique_lock<T1> &a0) {
  return a0.release();
}

template <typename T1> bool f14(const std::unique_lock<T1> &a0) {
  return a0.owns_lock();
}

template <typename T1> bool f15(const std::unique_lock<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1> T1 *f16(const std::unique_lock<T1> &a0) {
  return a0.mutex();
}

template <typename T1, typename T2, typename T3>
std::unique_lock<T1> f17(T1 &a0, const std::chrono::time_point<T2, T3> &a1) {
  return std::unique_lock<T1>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::unique_lock<T1> f18(T1 &a0, const std::chrono::duration<T2, T3> &a1) {
  return std::unique_lock<T1>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f19(std::unique_lock<T1> &a0, const std::chrono::time_point<T2, T3> &a1) {
  return a0.template try_lock_until<T2, T3>(a1);
}

template <typename T1, typename T2, typename T3>
bool f20(std::unique_lock<T1> &a0, const std::chrono::duration<T2, T3> &a1) {
  return a0.template try_lock_for<T2, T3>(a1);
}
