#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <condition_variable>
#include <utility>

template <typename T1, typename T2, typename... T3>
auto f1(const T2 &a0, T3 &&...a1)
    -> decltype(std::allocate_shared<T1, T2, T3...>(a0,
                                                    std::forward<T3>(a1)...)) {
  return std::allocate_shared<T1, T2, T3...>(a0, std::forward<T3>(a1)...);
}

template <typename T1, typename T2>
std::shared_ptr<T1> f2(const std::shared_ptr<T2> &a0) {
  return std::const_pointer_cast<T1, T2>(a0);
}

template <typename T1, typename T2>
std::shared_ptr<T1> f3(const std::shared_ptr<T2> &a0) {
  return std::dynamic_pointer_cast<T1, T2>(a0);
}

template <typename T1, typename T2> T1 *f4(const std::shared_ptr<T2> &a0) {
  return std::get_deleter<T1, T2>(a0);
}

template <typename T1, typename... T2>
auto f5(T2 &&...a0)
    -> decltype(std::make_shared<T1, T2...>(std::forward<T2>(a0)...)) {
  return std::make_shared<T1, T2...>(std::forward<T2>(a0)...);
}

template <typename T1>
bool f6(const std::shared_ptr<T1> &a0, std::nullptr_t a1) {
  return a0 == a1;
}

template <typename T1, typename T2>
bool f7(const std::shared_ptr<T1> &a0, const std::shared_ptr<T2> &a1) {
  return a0 == a1;
}

template <typename T1> using t1 = std::shared_ptr<T1>;

template <typename T1> using t2 = typename std::shared_ptr<T1>::element_type;

template <typename T1> std::shared_ptr<T1> f8() {
  return std::shared_ptr<T1>();
}

template <typename T1> std::shared_ptr<T1> f9(const std::shared_ptr<T1> &a0) {
  return std::shared_ptr<T1>(a0);
}

template <typename T1> std::shared_ptr<T1> f10(std::shared_ptr<T1> &&a0) {
  return std::shared_ptr<T1>(std::move(a0));
}

template <typename T1> std::shared_ptr<T1> f11(std::nullptr_t a0) {
  return std::shared_ptr<T1>(a0);
}

template <typename T1>
std::shared_ptr<T1> &f12(std::shared_ptr<T1> &a0,
                         const std::shared_ptr<T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::shared_ptr<T1> &f13(std::shared_ptr<T1> &a0, std::shared_ptr<T1> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2> std::shared_ptr<T1> f14(T2 *a0) {
  return std::shared_ptr<T1>(a0);
}

template <typename T1, typename T2, typename T3>
std::shared_ptr<T1> f15(T2 *a0, T3 a1) {
  return std::shared_ptr<T1>(a0, a1);
}

template <typename T1, typename T2>
std::shared_ptr<T1> f16(std::nullptr_t a0, T2 a1) {
  return std::shared_ptr<T1>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::shared_ptr<T1> f17(T2 *a0, T3 a1, T4 a2) {
  return std::shared_ptr<T1>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
std::shared_ptr<T1> f18(std::nullptr_t a0, T2 a1, T3 a2) {
  return std::shared_ptr<T1>(a0, a1, a2);
}

template <typename T1, typename T2>
std::shared_ptr<T1> f19(const std::shared_ptr<T2> &a0,
                        typename std::shared_ptr<T1>::element_type *a1) {
  return std::shared_ptr<T1>(a0, a1);
}

template <typename T1, typename T2>
std::shared_ptr<T1> f20(const std::shared_ptr<T2> &a0) {
  return std::shared_ptr<T1>(a0);
}

template <typename T1, typename T2>
std::shared_ptr<T1> f21(std::shared_ptr<T2> &&a0) {
  return std::shared_ptr<T1>(std::move(a0));
}

template <typename T1, typename T2>
std::shared_ptr<T1> f22(const std::weak_ptr<T2> &a0) {
  return std::shared_ptr<T1>(a0);
}

template <typename T1, typename T2>
std::shared_ptr<T1> f23(std::auto_ptr<T2> &&a0) {
  return std::shared_ptr<T1>(std::move(a0));
}

template <typename T1, typename T2, typename T3>
std::shared_ptr<T1> f24(std::unique_ptr<T2, T3> &&a0) {
  return std::shared_ptr<T1>(std::move(a0));
}

template <typename T1, typename T2>
std::shared_ptr<T1> &f25(std::shared_ptr<T1> &a0,
                         const std::shared_ptr<T2> &a1) {
  return a0.template operator= <T2>(a1);
}

template <typename T1, typename T2>
std::shared_ptr<T1> &f26(std::shared_ptr<T1> &a0, std::auto_ptr<T2> &&a1) {
  return a0.template operator= <T2>(std::move(a1));
}

template <typename T1, typename T2>
std::shared_ptr<T1> &f27(std::shared_ptr<T1> &a0, std::shared_ptr<T2> &&a1) {
  return a0.template operator= <T2>(std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::shared_ptr<T1> &f28(std::shared_ptr<T1> &a0,
                         std::unique_ptr<T2, T3> &&a1) {
  return a0.template operator= <T2, T3>(std::move(a1));
}

template <typename T1> void f29(std::shared_ptr<T1> &a0) { return a0.reset(); }

template <typename T1>
typename std::shared_ptr<T1>::element_type *f30(const std::shared_ptr<T1> &a0) {
  return a0.get();
}

template <typename T1> bool f31(const std::shared_ptr<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1> bool f32(const std::shared_ptr<T1> &a0) {
  return a0.unique();
}

template <typename T1> long f33(const std::shared_ptr<T1> &a0) {
  return a0.use_count();
}

template <typename T1>
void f34(std::shared_ptr<T1> &a0, std::shared_ptr<T1> &a1) {
  return a0.swap(a1);
}

template <typename T1>
typename std::shared_ptr<T1>::element_type &f35(const std::shared_ptr<T1> &a0) {
  return a0.operator*();
}

template <typename T1>
typename std::shared_ptr<T1>::element_type *f36(const std::shared_ptr<T1> &a0) {
  return a0.operator->();
}

template <typename T1, typename T2>
std::shared_ptr<T1> f37(const std::shared_ptr<T2> &a0) {
  return std::static_pointer_cast<T1, T2>(a0);
}

template <typename T1>
void f38(std::shared_ptr<T1> &a0, std::shared_ptr<T1> &a1) {
  return std::swap<T1>(a0, a1);
}

template <typename T1> void f39(std::weak_ptr<T1> &a0, std::weak_ptr<T1> &a1) {
  return std::swap<T1>(a0, a1);
}
