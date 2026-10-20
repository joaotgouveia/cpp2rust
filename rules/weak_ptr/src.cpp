#include "../common/init.h"
#include "../common/traits.h"
#include <condition_variable>
#include <utility>

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

template <typename T1, typename T2>
std::weak_ptr<T1> f7(const std::shared_ptr<T2> &a0) {
  return std::weak_ptr<T1>(a0);
}

template <typename T1, typename T2>
std::weak_ptr<T1> f8(const std::weak_ptr<T2> &a0) {
  return std::weak_ptr<T1>(a0);
}

template <typename T1, typename T2>
std::weak_ptr<T1> f9(std::weak_ptr<T2> &&a0) {
  return std::weak_ptr<T1>(std::move(a0));
}

template <typename T1, typename T2>
std::weak_ptr<T1> &f10(std::weak_ptr<T1> &a0, const std::weak_ptr<T2> &a1) {
  return a0.template operator= <T2>(a1);
}

template <typename T1, typename T2>
std::weak_ptr<T1> &f11(std::weak_ptr<T1> &a0, const std::shared_ptr<T2> &a1) {
  return a0.template operator= <T2>(a1);
}

template <typename T1, typename T2>
std::weak_ptr<T1> &f12(std::weak_ptr<T1> &a0, std::weak_ptr<T2> &&a1) {
  return a0.template operator= <T2>(std::move(a1));
}

template <typename T1> long f13(const std::weak_ptr<T1> &a0) {
  return a0.use_count();
}

template <typename T1> bool f14(const std::weak_ptr<T1> &a0) {
  return a0.expired();
}

template <typename T1> void f15(std::weak_ptr<T1> &a0) { return a0.reset(); }
