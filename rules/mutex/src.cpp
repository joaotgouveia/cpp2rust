#include <atomic>
#include <chrono>
#include <mutex>
#include <stdlib.h>
#include <utility>

using t1 = std::recursive_mutex;

using t2 = std::recursive_mutex::native_handle_type;

std::recursive_mutex f1() { return {}; }

void f2(std::recursive_mutex &a0) { return a0.lock(); }

bool f3(std::recursive_mutex &a0) { return a0.try_lock(); }

void f4(std::recursive_mutex &a0) { return a0.unlock(); }

pthread_mutex_t *f5(std::recursive_mutex &a0) { return a0.native_handle(); }

using t3 = std::recursive_timed_mutex;

std::recursive_timed_mutex f6() { return {}; }

void f7(std::recursive_timed_mutex &a0) { return a0.lock(); }

bool f8(std::recursive_timed_mutex &a0) { return a0.try_lock(); }

void f9(std::recursive_timed_mutex &a0) { return a0.unlock(); }

template <typename T1, typename T2>
bool f10(std::recursive_timed_mutex &a0,
         const std::chrono::duration<T1, T2> &a1) {
  return a0.template try_lock_for<T1, T2>(a1);
}

template <typename T1, typename T2>
bool f11(std::recursive_timed_mutex &a0,
         const std::chrono::time_point<T1, T2> &a1) {
  return a0.template try_lock_until<T1, T2>(a1);
}

using t4 = std::timed_mutex;

std::timed_mutex f12() { return {}; }

void f13(std::timed_mutex &a0) { return a0.lock(); }

bool f14(std::timed_mutex &a0) { return a0.try_lock(); }

void f15(std::timed_mutex &a0) { return a0.unlock(); }

template <typename T1, typename T2>
bool f16(std::timed_mutex &a0, const std::chrono::duration<T1, T2> &a1) {
  return a0.template try_lock_for<T1, T2>(a1);
}

template <typename T1, typename T2>
bool f17(std::timed_mutex &a0, const std::chrono::time_point<T1, T2> &a1) {
  return a0.template try_lock_until<T1, T2>(a1);
}
