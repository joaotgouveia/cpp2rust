#include <chrono>
#include <shared_mutex>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

using t1 = std::shared_timed_mutex;

std::shared_timed_mutex f1() { return {}; }

void f2(std::shared_timed_mutex &a0) { return a0.lock(); }

bool f3(std::shared_timed_mutex &a0) { return a0.try_lock(); }

void f4(std::shared_timed_mutex &a0) { return a0.unlock(); }

void f5(std::shared_timed_mutex &a0) { return a0.lock_shared(); }

bool f6(std::shared_timed_mutex &a0) { return a0.try_lock_shared(); }

void f7(std::shared_timed_mutex &a0) { return a0.unlock_shared(); }

template <typename T1, typename T2>
bool f8(std::shared_timed_mutex &a0, const std::chrono::duration<T1, T2> &a1) {
  return a0.template try_lock_for<T1, T2>(a1);
}

template <typename T1, typename T2>
bool f9(std::shared_timed_mutex &a0, const std::chrono::duration<T1, T2> &a1) {
  return a0.template try_lock_shared_for<T1, T2>(a1);
}

template <typename T1, typename T2>
bool f10(std::shared_timed_mutex &a0,
         const std::chrono::time_point<T1, T2> &a1) {
  return a0.template try_lock_until<T1, T2>(a1);
}

template <typename T1, typename T2>
bool f11(std::shared_timed_mutex &a0,
         const std::chrono::time_point<T1, T2> &a1) {
  return a0.template try_lock_shared_until<T1, T2>(a1);
}

template <typename T1>
void f12(std::shared_lock<T1> &a0, std::shared_lock<T1> &a1) {
  return std::swap<T1>(a0, a1);
}
