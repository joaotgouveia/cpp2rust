#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <atomic>
#include <condition_variable>
#include <cstddef>
#include <memory>
#include <utility>

template <typename T1, typename T2>
std::shared_ptr<T1> f1(const std::shared_ptr<T2> &a0) {
  return std::const_pointer_cast<T1, T2>(a0);
}

template <typename T1, typename T2>
std::shared_ptr<T1> f2(const std::shared_ptr<T2> &a0) {
  return std::dynamic_pointer_cast<T1, T2>(a0);
}

template <typename T1, typename T2> T1 *f3(const std::shared_ptr<T2> &a0) {
  return std::get_deleter<T1, T2>(a0);
}

template <typename T1>
bool f4(const std::shared_ptr<T1> &a0, std::nullptr_t a1) {
  return a0 == a1;
}

template <typename T1, typename T2>
bool f5(const std::shared_ptr<T1> &a0, const std::shared_ptr<T2> &a1) {
  return a0 == a1;
}

template <typename T1> using t1 = std::shared_ptr<T1>;

template <typename T1> using t2 = typename std::shared_ptr<T1>::element_type;

template <typename T1> std::shared_ptr<T1> f6() {
  return std::shared_ptr<T1>();
}

template <typename T1> std::shared_ptr<T1> f7(const std::shared_ptr<T1> &a0) {
  return std::shared_ptr<T1>(a0);
}

template <typename T1> std::shared_ptr<T1> f8(std::shared_ptr<T1> &&a0) {
  return std::shared_ptr<T1>(std::move(a0));
}

template <typename T1> std::shared_ptr<T1> f9(std::nullptr_t a0) {
  return std::shared_ptr<T1>(a0);
}

template <typename T1>
std::shared_ptr<T1> &f10(std::shared_ptr<T1> &a0,
                         const std::shared_ptr<T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::shared_ptr<T1> &f11(std::shared_ptr<T1> &a0, std::shared_ptr<T1> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2> std::shared_ptr<T1> f12(T2 *a0) {
  return std::shared_ptr<T1>(a0);
}

template <typename T1, typename T2, typename T3>
std::shared_ptr<T1> f13(T2 *a0, T3 a1) {
  return std::shared_ptr<T1>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::shared_ptr<T1> f14(T2 *a0, T3 a1, T4 a2) {
  return std::shared_ptr<T1>(a0, a1, a2);
}

template <typename T1, typename T2>
std::shared_ptr<T1> f15(const std::shared_ptr<T2> &a0,
                        typename std::shared_ptr<T1>::element_type *a1) {
  return std::shared_ptr<T1>(a0, a1);
}

template <typename T1, typename T2>
std::shared_ptr<T1> f16(const std::shared_ptr<T2> &a0) {
  return std::shared_ptr<T1>(a0);
}

template <typename T1, typename T2>
std::shared_ptr<T1> f17(std::shared_ptr<T2> &&a0) {
  return std::shared_ptr<T1>(std::move(a0));
}

template <typename T1, typename T2>
std::shared_ptr<T1> f18(const std::weak_ptr<T2> &a0) {
  return std::shared_ptr<T1>(a0);
}

template <typename T1, typename T2, typename T3>
std::shared_ptr<T1> f19(std::unique_ptr<T2, T3> &&a0) {
  return std::shared_ptr<T1>(std::move(a0));
}

template <typename T1, typename T2>
std::shared_ptr<T1> &f20(std::shared_ptr<T1> &a0,
                         const std::shared_ptr<T2> &a1) {
  return a0.template operator= <T2>(a1);
}

template <typename T1, typename T2>
std::shared_ptr<T1> &f21(std::shared_ptr<T1> &a0, std::shared_ptr<T2> &&a1) {
  return a0.template operator= <T2>(std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::shared_ptr<T1> &f22(std::shared_ptr<T1> &a0,
                         std::unique_ptr<T2, T3> &&a1) {
  return a0.template operator= <T2, T3>(std::move(a1));
}

template <typename T1> void f23(std::shared_ptr<T1> &a0) { return a0.reset(); }

template <typename T1>
typename std::shared_ptr<T1>::element_type *f24(const std::shared_ptr<T1> &a0) {
  return a0.get();
}

template <typename T1> bool f25(const std::shared_ptr<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1> long f26(const std::shared_ptr<T1> &a0) {
  return a0.use_count();
}

template <typename T1>
void f27(std::shared_ptr<T1> &a0, std::shared_ptr<T1> &a1) {
  return a0.swap(a1);
}

template <typename T1>
typename std::shared_ptr<T1>::element_type *f28(const std::shared_ptr<T1> &a0) {
  return a0.operator->();
}

template <typename T1, typename T2>
std::shared_ptr<T1> f29(const std::shared_ptr<T2> &a0) {
  return std::static_pointer_cast<T1, T2>(a0);
}

template <typename T1>
void f30(std::shared_ptr<T1> &a0, std::shared_ptr<T1> &a1) {
  return std::swap<T1>(a0, a1);
}

template <typename T1> void f31(std::weak_ptr<T1> &a0, std::weak_ptr<T1> &a1) {
  return std::swap<T1>(a0, a1);
}
