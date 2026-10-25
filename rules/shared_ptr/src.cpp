#include <algorithm>
#include <atomic>
#include <condition_variable>
#include <cstddef>
#include <memory>
#include <ostream>
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
  return std::operator== <T1>(a0, a1);
}

template <typename T1, typename T2>
bool f5(const std::shared_ptr<T1> &a0, const std::shared_ptr<T2> &a1) {
  return std::operator== <T1, T2>(a0, a1);
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

template <typename T1, typename T2>
std::shared_ptr<T1> f12(std::nullptr_t a0, T2 a1) {
  return std::shared_ptr<T1>(a0, std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::shared_ptr<T1> f13(std::nullptr_t a0, T2 a1, T3 a2) {
  return std::shared_ptr<T1>(a0, std::move(a1), std::move(a2));
}

template <typename T1> void f14(std::shared_ptr<T1> &a0) { return a0.reset(); }

template <typename T1> bool f15(const std::shared_ptr<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1> long f16(const std::shared_ptr<T1> &a0) {
  return a0.use_count();
}

template <typename T1>
void f17(std::shared_ptr<T1> &a0, std::shared_ptr<T1> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2>
std::shared_ptr<T1> f18(const std::shared_ptr<T2> &a0) {
  return std::static_pointer_cast<T1, T2>(a0);
}

template <typename T1>
void f19(std::shared_ptr<T1> &a0, std::shared_ptr<T1> &a1) {
  return std::swap<T1>(a0, a1);
}

template <typename T1> void f20(std::weak_ptr<T1> &a0, std::weak_ptr<T1> &a1) {
  return std::swap<T1>(a0, a1);
}
