#include <atomic>
#include <chrono>
#include <shared_mutex>
#include <utility>

template <typename T1> using t1 = std::shared_lock<T1>;

template <typename T1> using t2 = typename std::shared_lock<T1>::mutex_type;

template <typename T1> std::shared_lock<T1> f1() {
  return std::shared_lock<T1>();
}

template <typename T1> std::shared_lock<T1> f2(T1 &a0) {
  return std::shared_lock<T1>(a0);
}

template <typename T1> std::shared_lock<T1> f3(T1 &a0, std::defer_lock_t a1) {
  return std::shared_lock<T1>(a0, std::move(a1));
}

template <typename T1> std::shared_lock<T1> f4(T1 &a0, std::try_to_lock_t a1) {
  return std::shared_lock<T1>(a0, std::move(a1));
}

template <typename T1> std::shared_lock<T1> f5(T1 &a0, std::adopt_lock_t a1) {
  return std::shared_lock<T1>(a0, std::move(a1));
}

template <typename T1> std::shared_lock<T1> f6(std::shared_lock<T1> &&a0) {
  return std::shared_lock<T1>(std::move(a0));
}

template <typename T1>
std::shared_lock<T1> &f7(std::shared_lock<T1> &a0, std::shared_lock<T1> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> void f8(std::shared_lock<T1> &a0) { return a0.lock(); }

template <typename T1> bool f9(std::shared_lock<T1> &a0) {
  return a0.try_lock();
}

template <typename T1> void f10(std::shared_lock<T1> &a0) {
  return a0.unlock();
}

template <typename T1>
void f11(std::shared_lock<T1> &a0, std::shared_lock<T1> &a1) {
  return a0.swap(a1);
}

template <typename T1> T1 *f12(std::shared_lock<T1> &a0) {
  return a0.release();
}

template <typename T1> bool f13(const std::shared_lock<T1> &a0) {
  return a0.owns_lock();
}

template <typename T1> bool f14(const std::shared_lock<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1> T1 *f15(const std::shared_lock<T1> &a0) {
  return a0.mutex();
}

template <typename T1, typename T2, typename T3>
std::shared_lock<T1> f16(T1 &a0, const std::chrono::time_point<T2, T3> &a1) {
  return std::shared_lock<T1>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::shared_lock<T1> f17(T1 &a0, const std::chrono::duration<T2, T3> &a1) {
  return std::shared_lock<T1>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f18(std::shared_lock<T1> &a0, const std::chrono::duration<T2, T3> &a1) {
  return a0.template try_lock_for<T2, T3>(a1);
}

template <typename T1, typename T2, typename T3>
bool f19(std::shared_lock<T1> &a0, const std::chrono::time_point<T2, T3> &a1) {
  return a0.template try_lock_until<T2, T3>(a1);
}
