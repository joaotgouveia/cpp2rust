#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <condition_variable>
#include <cstddef>
#include <memory>
#include <utility>

template <typename T1, typename T2>
bool f5(const std::unique_ptr<T1, T2> &a0, std::nullptr_t a1) {
  return a0 < a1;
}

template <typename T1, typename T2>
bool f6(std::nullptr_t a0, const std::unique_ptr<T1, T2> &a1) {
  return a0 < a1;
}

template <typename T1, typename T2, typename T3, typename T4>
bool f7(const std::unique_ptr<T1, T2> &a0, const std::unique_ptr<T3, T4> &a1) {
  return a0 < a1;
}

template <typename T1, typename T2>
bool f8(const std::unique_ptr<T1, T2> &a0, std::nullptr_t a1) {
  return a0 <= a1;
}

template <typename T1, typename T2>
bool f9(std::nullptr_t a0, const std::unique_ptr<T1, T2> &a1) {
  return a0 <= a1;
}

template <typename T1, typename T2, typename T3, typename T4>
bool f10(const std::unique_ptr<T1, T2> &a0, const std::unique_ptr<T3, T4> &a1) {
  return a0 <= a1;
}

template <typename T1, typename T2>
bool f11(const std::unique_ptr<T1, T2> &a0, std::nullptr_t a1) {
  return a0 == a1;
}

template <typename T1, typename T2, typename T3, typename T4>
bool f12(const std::unique_ptr<T1, T2> &a0, const std::unique_ptr<T3, T4> &a1) {
  return a0 == a1;
}

template <typename T1, typename T2>
bool f13(const std::unique_ptr<T1, T2> &a0, std::nullptr_t a1) {
  return a0 > a1;
}

template <typename T1, typename T2>
bool f14(std::nullptr_t a0, const std::unique_ptr<T1, T2> &a1) {
  return a0 > a1;
}

template <typename T1, typename T2, typename T3, typename T4>
bool f15(const std::unique_ptr<T1, T2> &a0, const std::unique_ptr<T3, T4> &a1) {
  return a0 > a1;
}

template <typename T1, typename T2>
bool f16(const std::unique_ptr<T1, T2> &a0, std::nullptr_t a1) {
  return a0 >= a1;
}

template <typename T1, typename T2>
bool f17(std::nullptr_t a0, const std::unique_ptr<T1, T2> &a1) {
  return a0 >= a1;
}

template <typename T1, typename T2, typename T3, typename T4>
bool f18(const std::unique_ptr<T1, T2> &a0, const std::unique_ptr<T3, T4> &a1) {
  return a0 >= a1;
}

template <typename T1, typename T2> using t1 = std::unique_ptr<T1[], T2>;

template <typename T1, typename T2>
using t2 = typename std::unique_ptr<T1[], T2>::pointer;

template <typename T1, typename T2>
using t3 = typename std::unique_ptr<T1[], T2>::element_type;

template <typename T1, typename T2>
using t4 = typename std::unique_ptr<T1[], T2>::deleter_type;

template <typename T1, typename T2>
std::unique_ptr<T1[], T2> f19(std::unique_ptr<T1[], T2> &&a0) {
  return std::unique_ptr<T1[], T2>(std::move(a0));
}

template <typename T1, typename T2>
std::unique_ptr<T1[], T2> &f20(std::unique_ptr<T1[], T2> &a0,
                               std::unique_ptr<T1[], T2> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
std::unique_ptr<T1[], T2> &f21(std::unique_ptr<T1[], T2> &a0,
                               std::nullptr_t a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
typename std::unique_ptr<T1[], T2>::pointer
f22(const std::unique_ptr<T1[], T2> &a0) {
  return a0.get();
}

template <typename T1, typename T2>
typename std::unique_ptr<T1[], T2>::deleter_type &
f23(std::unique_ptr<T1[], T2> &a0) {
  return a0.get_deleter();
}

template <typename T1, typename T2>
const typename std::unique_ptr<T1[], T2>::deleter_type &
f24(const std::unique_ptr<T1[], T2> &a0) {
  return a0.get_deleter();
}

template <typename T1, typename T2>
bool f25(const std::unique_ptr<T1[], T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
typename std::unique_ptr<T1[], T2>::pointer f26(std::unique_ptr<T1[], T2> &a0) {
  return a0.release();
}

template <typename T1, typename T2>
void f27(std::unique_ptr<T1[], T2> &a0, std::nullptr_t a1) {
  return a0.reset(a1);
}

template <typename T1, typename T2>
void f28(std::unique_ptr<T1[], T2> &a0, std::unique_ptr<T1[], T2> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2> using t5 = std::unique_ptr<T1, T2>;

template <typename T1, typename T2>
using t6 = typename std::unique_ptr<T1, T2>::pointer;

template <typename T1, typename T2>
using t7 = typename std::unique_ptr<T1, T2>::element_type;

template <typename T1, typename T2>
using t8 = typename std::unique_ptr<T1, T2>::deleter_type;

template <typename T1, typename T2>
std::unique_ptr<T1, T2> f29(std::unique_ptr<T1, T2> &&a0) {
  return std::unique_ptr<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::unique_ptr<T1, T2> &f30(std::unique_ptr<T1, T2> &a0,
                             std::unique_ptr<T1, T2> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
std::unique_ptr<T1, T2> &f31(std::unique_ptr<T1, T2> &a0, std::nullptr_t a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
typename std::unique_ptr<T1, T2>::pointer
f32(const std::unique_ptr<T1, T2> &a0) {
  return a0.operator->();
}

template <typename T1, typename T2>
typename std::unique_ptr<T1, T2>::pointer
f33(const std::unique_ptr<T1, T2> &a0) {
  return a0.get();
}

template <typename T1, typename T2>
typename std::unique_ptr<T1, T2>::deleter_type &
f34(std::unique_ptr<T1, T2> &a0) {
  return a0.get_deleter();
}

template <typename T1, typename T2>
const typename std::unique_ptr<T1, T2>::deleter_type &
f35(const std::unique_ptr<T1, T2> &a0) {
  return a0.get_deleter();
}

template <typename T1, typename T2>
bool f36(const std::unique_ptr<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
typename std::unique_ptr<T1, T2>::pointer f37(std::unique_ptr<T1, T2> &a0) {
  return a0.release();
}

template <typename T1, typename T2>
void f38(std::unique_ptr<T1, T2> &a0,
         typename std::unique_ptr<T1, T2>::pointer a1) {
  return a0.reset(a1);
}

template <typename T1, typename T2>
void f39(std::unique_ptr<T1, T2> &a0, std::unique_ptr<T1, T2> &a1) {
  return a0.swap(a1);
}
