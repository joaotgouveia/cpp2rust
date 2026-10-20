#include "../common/init.h"
#include "../common/traits.h"
#include <future>
#include <utility>

template <typename T1> using t1 = std::shared_future<T1>;

template <typename T1> std::shared_future<T1> f1() {
  return std::shared_future<T1>();
}

template <typename T1>
std::shared_future<T1> f2(const std::shared_future<T1> &a0) {
  return std::shared_future<T1>(a0);
}

template <typename T1> std::shared_future<T1> f3(std::future<T1> &&a0) {
  return std::shared_future<T1>(std::move(a0));
}

template <typename T1> std::shared_future<T1> f4(std::shared_future<T1> &&a0) {
  return std::shared_future<T1>(std::move(a0));
}

template <typename T1>
std::shared_future<T1> &f5(std::shared_future<T1> &a0,
                           const std::shared_future<T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::shared_future<T1> &f6(std::shared_future<T1> &a0,
                           std::shared_future<T1> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> const T1 &f7(const std::shared_future<T1> &a0) {
  return a0.get();
}

template <typename T1> bool f8(const std::shared_future<T1> &a0) {
  return a0.valid();
}

template <typename T1> void f9(const std::shared_future<T1> &a0) {
  return a0.wait();
}
