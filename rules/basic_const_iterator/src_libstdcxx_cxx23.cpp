#include <algorithm>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1> using t1 = std::basic_const_iterator<T1>;

template <typename T1>
using t2 = typename std::basic_const_iterator<T1>::iterator_concept;

template <typename T1>
using t3 = typename std::basic_const_iterator<T1>::value_type;

template <typename T1>
using t4 = typename std::basic_const_iterator<T1>::difference_type;

template <typename T1> std::basic_const_iterator<T1> f1() {
  return std::basic_const_iterator<T1>();
}

template <typename T1> std::basic_const_iterator<T1> f2(T1 a0) {
  return std::basic_const_iterator<T1>(a0);
}

template <typename T1> const T1 &f3(const std::basic_const_iterator<T1> &a0) {
  return a0.base();
}

template <typename T1> T1 f4(std::basic_const_iterator<T1> &&a0) {
  return std::move(a0).base();
}

template <typename T1>
std::iter_const_reference_t<T1> f5(const std::basic_const_iterator<T1> &a0) {
  return a0.operator*();
}

template <typename T1> const auto *f6(const std::basic_const_iterator<T1> &a0) {
  return a0.operator->();
}

template <typename T1>
std::basic_const_iterator<T1> &f7(std::basic_const_iterator<T1> &a0) {
  return a0.operator++();
}

template <typename T1> void f8(std::basic_const_iterator<T1> &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1>
std::basic_const_iterator<T1> f9(std::basic_const_iterator<T1> &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1>
std::basic_const_iterator<T1> &f10(std::basic_const_iterator<T1> &a0) {
  return a0.operator--();
}

template <typename T1>
std::basic_const_iterator<T1> f11(std::basic_const_iterator<T1> &a0, int a1) {
  return a0.operator--(a1);
}

template <typename T1>
std::basic_const_iterator<T1> &
f12(std::basic_const_iterator<T1> &a0,
    typename std::basic_const_iterator<T1>::difference_type a1) {
  return a0.operator+=(a1);
}

template <typename T1>
std::basic_const_iterator<T1> &
f13(std::basic_const_iterator<T1> &a0,
    typename std::basic_const_iterator<T1>::difference_type a1) {
  return a0.operator-=(a1);
}

template <typename T1>
std::iter_const_reference_t<T1>
f14(const std::basic_const_iterator<T1> &a0,
    typename std::basic_const_iterator<T1>::difference_type a1) {
  return a0.operator[](a1);
}

template <typename T1>
bool f15(const std::basic_const_iterator<T1> &a0,
         const std::basic_const_iterator<T1> &a1) {
  return a0.operator<(a1);
}

template <typename T1>
bool f16(const std::basic_const_iterator<T1> &a0,
         const std::basic_const_iterator<T1> &a1) {
  return a0.operator>(a1);
}

template <typename T1>
bool f17(const std::basic_const_iterator<T1> &a0,
         const std::basic_const_iterator<T1> &a1) {
  return a0.operator<=(a1);
}

template <typename T1>
bool f18(const std::basic_const_iterator<T1> &a0,
         const std::basic_const_iterator<T1> &a1) {
  return a0.operator>=(a1);
}

template <typename T1>
auto f19(const std::basic_const_iterator<T1> &a0,
         const std::basic_const_iterator<T1> &a1) {
  return a0.operator<=>(a1);
}

template <typename T1, typename T2>
std::basic_const_iterator<T1> f20(std::basic_const_iterator<T2> a0) {
  return std::basic_const_iterator<T1>(a0);
}

template <typename T1, typename T2> std::basic_const_iterator<T1> f21(T2 &&a0) {
  return std::basic_const_iterator<T1>(std::forward<T2>(a0));
}

template <typename T1, typename T2>
bool f22(const std::basic_const_iterator<T1> &a0, const T2 &a1) {
  return a0.template operator== <T2>(a1);
}

template <typename T1, typename T2>
T2 f23(const std::basic_const_iterator<T1> &a0) {
  return static_cast<T2>(a0);
}

template <typename T1, typename T2> T2 f24(std::basic_const_iterator<T1> &&a0) {
  return static_cast<T2>(std::move(a0));
}

template <typename T1, typename T2>
bool f25(const std::basic_const_iterator<T1> &a0, const T2 &a1) {
  return a0.template operator< <T2>(a1);
}

template <typename T1, typename T2>
bool f26(const std::basic_const_iterator<T1> &a0, const T2 &a1) {
  return a0.template operator><T2>(a1);
}

template <typename T1, typename T2>
bool f27(const std::basic_const_iterator<T1> &a0, const T2 &a1) {
  return a0.template operator<= <T2>(a1);
}

template <typename T1, typename T2>
bool f28(const std::basic_const_iterator<T1> &a0, const T2 &a1) {
  return a0.template operator>= <T2>(a1);
}

template <typename T1, typename T2>
auto f29(const std::basic_const_iterator<T1> &a0, const T2 &a1) {
  return a0.template operator<=> <T2>(a1);
}

template <typename T1, typename T2>
typename std::basic_const_iterator<T1>::difference_type
f30(const std::basic_const_iterator<T1> &a0, const T2 &a1) {
  return a0.template operator- <T2>(a1);
}

template <typename T1>
std::basic_const_iterator<T1>
f31(const std::basic_const_iterator<T1> &a0,
    typename std::basic_const_iterator<T1>::difference_type a1) {
  return a0 - a1;
}

template <typename T1>
std::basic_const_iterator<T1>
f32(typename std::basic_const_iterator<T1>::difference_type a0,
    const std::basic_const_iterator<T1> &a1) {
  return a0 + a1;
}

template <typename T1>
std::basic_const_iterator<T1>
f33(const std::basic_const_iterator<T1> &a0,
    typename std::basic_const_iterator<T1>::difference_type a1) {
  return a0 + a1;
}
