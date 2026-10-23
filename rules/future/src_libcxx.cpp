#include <chrono>
#include <future>
#include <system_error>
#include <utility>

template <typename T1> void f38(std::future<T1> &a0, std::future<T1> &a1) {
  return std::swap<T1>(a0, a1);
}

template <typename T1>
void f39(std::shared_future<T1> &a0, std::shared_future<T1> &a1) {
  return std::swap<T1>(a0, a1);
}

template <typename T1> void f40(std::future<T1> &a0, std::future<T1> &a1) {
  return a0.swap(a1);
}

std::launch f41() { return std::launch::any; }
