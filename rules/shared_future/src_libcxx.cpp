#include <chrono>
#include <future>
#include <utility>

template <typename T1>
void f12(std::shared_future<T1> &a0, std::shared_future<T1> &a1) {
  return a0.swap(a1);
}
