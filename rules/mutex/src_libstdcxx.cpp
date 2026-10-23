#include <chrono>
#include <ios>
#include <mutex>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename... T2>
void f17(std::once_flag &a0, T1 &&a1, T2 &&...a2) {
  return std::call_once<T1, T2...>(a0, std::forward<T1>(a1),
                                   std::forward<T2>(a2)...);
}

template <typename T1, typename T2, typename... T3>
void f18(T1 &a0, T2 &a1, T3 &...a2) {
  return std::lock<T1, T2, T3...>(a0, a1, a2...);
}

using t5 = std::once_flag;

std::once_flag f19() { return {}; }

__gthread_recursive_mutex_t *f20(std::recursive_mutex &a0) {
  return a0.native_handle();
}

using t6 = std::recursive_timed_mutex::native_handle_type;

__gthread_recursive_mutex_t *f21(std::recursive_timed_mutex &a0) {
  return a0.native_handle();
}

using t7 = std::timed_mutex::native_handle_type;

__gthread_mutex_t *f22(std::timed_mutex &a0) { return a0.native_handle(); }

template <typename T1, typename T2, typename... T3>
int f23(T1 &a0, T2 &a1, T3 &...a2) {
  return std::try_lock<T1, T2, T3...>(a0, a1, a2...);
}
