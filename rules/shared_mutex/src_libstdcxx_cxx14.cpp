#include "../common/init.h"
#include "../common/traits.h"
#include <chrono>
#include <shared_mutex>
#include <utility>

template <typename T1>
bool f20(std::shared_timed_mutex &a0,
         const std::chrono::time_point<std::chrono::system_clock, T1> &a1) {
  return a0.template try_lock_until<T1>(a1);
}

template <typename T1>
bool f21(std::shared_timed_mutex &a0,
         const std::chrono::time_point<std::chrono::steady_clock, T1> &a1) {
  return a0.template try_lock_until<T1>(a1);
}

template <typename T1>
bool f22(std::shared_timed_mutex &a0,
         const std::chrono::time_point<std::chrono::system_clock, T1> &a1) {
  return a0.template try_lock_shared_until<T1>(a1);
}

template <typename T1>
bool f23(std::shared_timed_mutex &a0,
         const std::chrono::time_point<std::chrono::steady_clock, T1> &a1) {
  return a0.template try_lock_shared_until<T1>(a1);
}
