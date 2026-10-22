#include "../common/init.h"
#include "../common/traits.h"
#include <chrono>
#include <future>
#include <system_error>
#include <utility>

template <typename T1> void f36(std::future<T1> &a0, std::future<T1> &a1) {
  return std::swap<T1>(a0, a1);
}

template <typename T1>
void f37(std::shared_future<T1> &a0, std::shared_future<T1> &a1) {
  return std::swap<T1>(a0, a1);
}

template <typename T1> void f38(std::future<T1> &a0, std::future<T1> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2, typename T3>
std::future_status f39(const std::future<T1> &a0,
                       const std::chrono::duration<T2, T3> &a1) {
  return a0.template wait_for<T2, T3>(a1);
}

template <typename T1, typename T2, typename T3>
std::future_status f40(const std::future<T1> &a0,
                       const std::chrono::time_point<T2, T3> &a1) {
  return a0.template wait_until<T2, T3>(a1);
}
