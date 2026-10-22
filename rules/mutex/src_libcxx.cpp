#include "../common/init.h"
#include "../common/traits.h"
#include <atomic>
#include <mutex>
#include <utility>

template <typename T1, typename T2> void f24(T1 &a0, T2 &a1) {
  return std::lock<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename... T4>
void f25(T1 &a0, T2 &a1, T3 &a2, T4 &...a3) {
  return std::lock<T1, T2, T3, T4...>(a0, a1, a2, a3...);
}

using t8 = std::mutex;

using t9 = std::mutex::native_handle_type;

std::mutex f26() { return {}; }

void f27(std::mutex &a0) { return a0.lock(); }

bool f28(std::mutex &a0) { return a0.try_lock(); }

void f29(std::mutex &a0) { return a0.unlock(); }

std::__libcpp_mutex_t *f30(std::mutex &a0) { return a0.native_handle(); }

std::__libcpp_recursive_mutex_t *f31(std::recursive_mutex &a0) {
  return a0.native_handle();
}

template <typename T1, typename T2> int f32(T1 &a0, T2 &a1) {
  return std::try_lock<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename... T4>
int f33(T1 &a0, T2 &a1, T3 &a2, T4 &...a3) {
  return std::try_lock<T1, T2, T3, T4...>(a0, a1, a2, a3...);
}
