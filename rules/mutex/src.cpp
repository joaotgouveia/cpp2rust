#include "../common/init.h"
#include "../common/traits.h"
#include <chrono>
#include <ios>
#include <mutex>
#include <utility>

template <typename T1, typename... T2>
void f1(std::once_flag &a0, T1 &&a1, T2 &&...a2) {
  return std::call_once<T1, T2...>(a0, std::forward<T1>(a1),
                                   std::forward<T2>(a2)...);
}

template <typename T1, typename T2, typename... T3>
void f2(T1 &a0, T2 &a1, T3 &...a2) {
  return std::lock<T1, T2, T3...>(a0, a1, a2...);
}

using t1 = std::once_flag;

std::once_flag f3() { return {}; }

using t2 = std::recursive_mutex;

using t3 = std::recursive_mutex::native_handle_type;

std::recursive_mutex f4() { return {}; }

void f5(std::recursive_mutex &a0) { return a0.lock(); }

bool f6(std::recursive_mutex &a0) { return a0.try_lock(); }

void f7(std::recursive_mutex &a0) { return a0.unlock(); }

__gthread_recursive_mutex_t *f8(std::recursive_mutex &a0) {
  return a0.native_handle();
}

using t4 = std::recursive_timed_mutex;

using t5 = std::recursive_timed_mutex::native_handle_type;

std::recursive_timed_mutex f9() { return {}; }

void f10(std::recursive_timed_mutex &a0) { return a0.lock(); }

bool f11(std::recursive_timed_mutex &a0) { return a0.try_lock(); }

void f12(std::recursive_timed_mutex &a0) { return a0.unlock(); }

__gthread_recursive_mutex_t *f13(std::recursive_timed_mutex &a0) {
  return a0.native_handle();
}

template <typename T1, typename T2>
bool f14(std::recursive_timed_mutex &a0,
         const std::chrono::duration<T1, T2> &a1) {
  return a0.template try_lock_for<T1, T2>(a1);
}

template <typename T1, typename T2>
bool f15(std::recursive_timed_mutex &a0,
         const std::chrono::time_point<T1, T2> &a1) {
  return a0.template try_lock_until<T1, T2>(a1);
}

using t6 = std::timed_mutex;

using t7 = std::timed_mutex::native_handle_type;

std::timed_mutex f16() { return {}; }

void f17(std::timed_mutex &a0) { return a0.lock(); }

bool f18(std::timed_mutex &a0) { return a0.try_lock(); }

void f19(std::timed_mutex &a0) { return a0.unlock(); }

__gthread_mutex_t *f20(std::timed_mutex &a0) { return a0.native_handle(); }

template <typename T1, typename T2>
bool f21(std::timed_mutex &a0, const std::chrono::duration<T1, T2> &a1) {
  return a0.template try_lock_for<T1, T2>(a1);
}

template <typename T1, typename T2>
bool f22(std::timed_mutex &a0, const std::chrono::time_point<T1, T2> &a1) {
  return a0.template try_lock_until<T1, T2>(a1);
}

template <typename T1, typename T2, typename... T3>
int f23(T1 &a0, T2 &a1, T3 &...a2) {
  return std::try_lock<T1, T2, T3...>(a0, a1, a2...);
}
