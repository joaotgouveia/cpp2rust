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

template <typename T1, typename T2>
void f19(std::unique_ptr<T1, T2> &a0, std::unique_ptr<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}

template <typename T1, typename T2> using t1 = std::unique_ptr<T1[], T2>;

template <typename T1, typename T2>
using t2 = typename std::unique_ptr<T1[], T2>::pointer;

template <typename T1, typename T2>
using t3 = typename std::unique_ptr<T1[], T2>::element_type;

template <typename T1, typename T2>
using t4 = typename std::unique_ptr<T1[], T2>::deleter_type;

template <typename T1, typename T2>
std::unique_ptr<T1[], T2> f20(std::unique_ptr<T1[], T2> &&a0) {
  return std::unique_ptr<T1[], T2>(std::move(a0));
}

template <typename T1, typename T2>
std::unique_ptr<T1[], T2> &f21(std::unique_ptr<T1[], T2> &a0,
                               std::unique_ptr<T1[], T2> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
std::unique_ptr<T1[], T2> &f22(std::unique_ptr<T1[], T2> &a0,
                               std::nullptr_t a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
typename std::unique_ptr<T1[], T2>::pointer
f23(const std::unique_ptr<T1[], T2> &a0) {
  return a0.get();
}

template <typename T1, typename T2>
typename std::unique_ptr<T1[], T2>::deleter_type &
f24(std::unique_ptr<T1[], T2> &a0) {
  return a0.get_deleter();
}

template <typename T1, typename T2>
const typename std::unique_ptr<T1[], T2>::deleter_type &
f25(const std::unique_ptr<T1[], T2> &a0) {
  return a0.get_deleter();
}

template <typename T1, typename T2>
bool f26(const std::unique_ptr<T1[], T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
typename std::unique_ptr<T1[], T2>::pointer f27(std::unique_ptr<T1[], T2> &a0) {
  return a0.release();
}

template <typename T1, typename T2>
void f28(std::unique_ptr<T1[], T2> &a0, std::nullptr_t a1) {
  return a0.reset(a1);
}

template <typename T1, typename T2>
void f29(std::unique_ptr<T1[], T2> &a0, std::unique_ptr<T1[], T2> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2> std::unique_ptr<T1[], T2> f30() {
  return std::unique_ptr<T1[], T2>();
}

template <typename T1, typename T2, typename T3>
std::unique_ptr<T1[], T2> f31(T3 a0) {
  return std::unique_ptr<T1[], T2>(a0);
}

template <typename T1, typename T2, typename T3>
std::unique_ptr<T1[], T2>
f32(T3 a0, const typename std::unique_ptr<T1[], T2>::deleter_type &a1) {
  return std::unique_ptr<T1[], T2>(a0, a1);
}

template <typename T1, typename T2>
std::unique_ptr<T1[], T2> f33(std::nullptr_t a0) {
  return std::unique_ptr<T1[], T2>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::unique_ptr<T1[], T2> f34(std::unique_ptr<T3, T4> &&a0) {
  return std::unique_ptr<T1[], T2>(std::move(a0));
}

template <typename T1, typename T2, typename T3, typename T4>
std::unique_ptr<T1[], T2> &f35(std::unique_ptr<T1[], T2> &a0,
                               std::unique_ptr<T3, T4> &&a1) {
  return a0.template operator= <T3, T4>(std::move(a1));
}

template <typename T1, typename T2, typename T3>
void f36(std::unique_ptr<T1[], T2> &a0, T3 a1) {
  return a0.template reset<T3>(a1);
}

template <typename T1, typename T2> using t5 = std::unique_ptr<T1, T2>;

template <typename T1, typename T2>
using t6 = typename std::unique_ptr<T1, T2>::pointer;

template <typename T1, typename T2>
using t7 = typename std::unique_ptr<T1, T2>::element_type;

template <typename T1, typename T2>
using t8 = typename std::unique_ptr<T1, T2>::deleter_type;

template <typename T1, typename T2>
std::unique_ptr<T1, T2> f37(std::unique_ptr<T1, T2> &&a0) {
  return std::unique_ptr<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::unique_ptr<T1, T2> &f38(std::unique_ptr<T1, T2> &a0,
                             std::unique_ptr<T1, T2> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
std::unique_ptr<T1, T2> &f39(std::unique_ptr<T1, T2> &a0, std::nullptr_t a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
typename std::unique_ptr<T1, T2>::pointer
f40(const std::unique_ptr<T1, T2> &a0) {
  return a0.operator->();
}

template <typename T1, typename T2>
typename std::unique_ptr<T1, T2>::pointer
f41(const std::unique_ptr<T1, T2> &a0) {
  return a0.get();
}

template <typename T1, typename T2>
typename std::unique_ptr<T1, T2>::deleter_type &
f42(std::unique_ptr<T1, T2> &a0) {
  return a0.get_deleter();
}

template <typename T1, typename T2>
const typename std::unique_ptr<T1, T2>::deleter_type &
f43(const std::unique_ptr<T1, T2> &a0) {
  return a0.get_deleter();
}

template <typename T1, typename T2>
bool f44(const std::unique_ptr<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
typename std::unique_ptr<T1, T2>::pointer f45(std::unique_ptr<T1, T2> &a0) {
  return a0.release();
}

template <typename T1, typename T2>
void f46(std::unique_ptr<T1, T2> &a0,
         typename std::unique_ptr<T1, T2>::pointer a1) {
  return a0.reset(a1);
}

template <typename T1, typename T2>
void f47(std::unique_ptr<T1, T2> &a0, std::unique_ptr<T1, T2> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2> std::unique_ptr<T1, T2> f48() {
  return std::unique_ptr<T1, T2>();
}

template <typename T1, typename T2>
std::unique_ptr<T1, T2> f49(typename std::unique_ptr<T1, T2>::pointer a0) {
  return std::unique_ptr<T1, T2>(a0);
}

template <typename T1, typename T2>
std::unique_ptr<T1, T2>
f50(typename std::unique_ptr<T1, T2>::pointer a0,
    const typename std::unique_ptr<T1, T2>::deleter_type &a1) {
  return std::unique_ptr<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::unique_ptr<T1, T2> f51(std::nullptr_t a0) {
  return std::unique_ptr<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::unique_ptr<T1, T2> f52(std::unique_ptr<T3, T4> &&a0) {
  return std::unique_ptr<T1, T2>(std::move(a0));
}

template <typename T1, typename T2, typename T3, typename T4>
std::unique_ptr<T1, T2> &f53(std::unique_ptr<T1, T2> &a0,
                             std::unique_ptr<T3, T4> &&a1) {
  return a0.template operator= <T3, T4>(std::move(a1));
}
