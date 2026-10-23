#include <condition_variable>
#include <memory>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1> using t1 = std::weak_ptr<T1>;

template <typename T1> std::weak_ptr<T1> f1() { return std::weak_ptr<T1>(); }

template <typename T1> std::weak_ptr<T1> f2(const std::weak_ptr<T1> &a0) {
  return std::weak_ptr<T1>(a0);
}

template <typename T1> std::weak_ptr<T1> f3(std::weak_ptr<T1> &&a0) {
  return std::weak_ptr<T1>(std::move(a0));
}

template <typename T1>
std::weak_ptr<T1> &f4(std::weak_ptr<T1> &a0, const std::weak_ptr<T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::weak_ptr<T1> &f5(std::weak_ptr<T1> &a0, std::weak_ptr<T1> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> std::shared_ptr<T1> f6(const std::weak_ptr<T1> &a0) {
  return a0.lock();
}

template <typename T1> long f7(const std::weak_ptr<T1> &a0) {
  return a0.use_count();
}

template <typename T1> bool f8(const std::weak_ptr<T1> &a0) {
  return a0.expired();
}

template <typename T1> void f9(std::weak_ptr<T1> &a0) { return a0.reset(); }
