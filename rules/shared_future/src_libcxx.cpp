#include <chrono>
#include <future>
#include <utility>

template <typename T1>
void f10(std::shared_future<T1> &a0, std::shared_future<T1> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2, typename T3>
std::future_status f11(const std::shared_future<T1> &a0,
                       const std::chrono::duration<T2, T3> &a1) {
  return a0.template wait_for<T2, T3>(a1);
}

template <typename T1, typename T2, typename T3>
std::future_status f12(const std::shared_future<T1> &a0,
                       const std::chrono::time_point<T2, T3> &a1) {
  return a0.template wait_until<T2, T3>(a1);
}
