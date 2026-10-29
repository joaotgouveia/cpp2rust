#include <chrono>
#include <shared_mutex>
#include <utility>

using t1 = std::shared_mutex;

using t2 = std::shared_mutex::native_handle_type;

std::shared_mutex f1() { return {}; }

void f2(std::shared_mutex &a0) { return a0.lock(); }

bool f3(std::shared_mutex &a0) { return a0.try_lock(); }

void f4(std::shared_mutex &a0) { return a0.unlock(); }

void f5(std::shared_mutex &a0) { return a0.lock_shared(); }

bool f6(std::shared_mutex &a0) { return a0.try_lock_shared(); }

void f7(std::shared_mutex &a0) { return a0.unlock_shared(); }

void *f8(std::shared_mutex &a0) { return a0.native_handle(); }

using t3 = std::shared_timed_mutex;

std::shared_timed_mutex f9() { return {}; }

void f10(std::shared_timed_mutex &a0) { return a0.lock(); }

bool f11(std::shared_timed_mutex &a0) { return a0.try_lock(); }

void f12(std::shared_timed_mutex &a0) { return a0.unlock(); }

void f13(std::shared_timed_mutex &a0) { return a0.lock_shared(); }

bool f14(std::shared_timed_mutex &a0) { return a0.try_lock_shared(); }

void f15(std::shared_timed_mutex &a0) { return a0.unlock_shared(); }

template <typename T1, typename T2>
bool f16(std::shared_timed_mutex &a0, const std::chrono::duration<T1, T2> &a1) {
  return a0.template try_lock_for<T1, T2>(a1);
}

template <typename T1, typename T2>
bool f17(std::shared_timed_mutex &a0, const std::chrono::duration<T1, T2> &a1) {
  return a0.template try_lock_shared_for<T1, T2>(a1);
}

template <typename T1>
bool f18(std::shared_timed_mutex &a0,
         const std::chrono::time_point<std::chrono::system_clock, T1> &a1) {
  return a0.template try_lock_until<T1>(a1);
}

template <typename T1>
bool f19(std::shared_timed_mutex &a0,
         const std::chrono::time_point<std::chrono::steady_clock, T1> &a1) {
  return a0.template try_lock_until<T1>(a1);
}

template <typename T1, typename T2>
bool f20(std::shared_timed_mutex &a0,
         const std::chrono::time_point<T1, T2> &a1) {
  return a0.template try_lock_until<T1, T2>(a1);
}

template <typename T1>
bool f21(std::shared_timed_mutex &a0,
         const std::chrono::time_point<std::chrono::system_clock, T1> &a1) {
  return a0.template try_lock_shared_until<T1>(a1);
}

template <typename T1>
bool f22(std::shared_timed_mutex &a0,
         const std::chrono::time_point<std::chrono::steady_clock, T1> &a1) {
  return a0.template try_lock_shared_until<T1>(a1);
}

template <typename T1, typename T2>
bool f23(std::shared_timed_mutex &a0,
         const std::chrono::time_point<T1, T2> &a1) {
  return a0.template try_lock_shared_until<T1, T2>(a1);
}

template <typename T1>
void f24(std::shared_lock<T1> &a0, std::shared_lock<T1> &a1) {
  return std::swap(a0, a1);
}
