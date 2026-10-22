#include "../common/init.h"
#include "../common/traits.h"
#include <array>
#include <utility>

template <typename T1, std::size_t T2> using t1 = std::array<T1, T2>;

template <typename T1, std::size_t T2>
using t2 = typename std::array<T1, T2>::value_type;

template <typename T1, std::size_t T2>
using t3 = typename std::array<T1, T2>::pointer;

template <typename T1, std::size_t T2>
using t4 = typename std::array<T1, T2>::const_pointer;

template <typename T1, std::size_t T2>
using t5 = typename std::array<T1, T2>::reference;

template <typename T1, std::size_t T2>
using t6 = typename std::array<T1, T2>::const_reference;

template <typename T1, std::size_t T2>
using t7 = typename std::array<T1, T2>::iterator;

template <typename T1, std::size_t T2>
using t8 = typename std::array<T1, T2>::const_iterator;

template <typename T1, std::size_t T2>
using t9 = typename std::array<T1, T2>::size_type;

template <typename T1, std::size_t T2>
using t10 = typename std::array<T1, T2>::difference_type;

template <typename T1, std::size_t T2>
using t11 = typename std::array<T1, T2>::reverse_iterator;

template <typename T1, std::size_t T2>
using t12 = typename std::array<T1, T2>::const_reverse_iterator;

template <typename T1, std::size_t T2>
void f1(std::array<T1, T2> &a0,
        const typename std::array<T1, T2>::value_type &a1) {
  return a0.fill(a1);
}

template <typename T1, std::size_t T2>
void f2(std::array<T1, T2> &a0, std::array<T1, T2> &a1) {
  return a0.swap(a1);
}

template <typename T1, std::size_t T2>
typename std::array<T1, T2>::value_type *f3(std::array<T1, T2> &a0) {
  return a0.begin();
}

template <typename T1, std::size_t T2>
const typename std::array<T1, T2>::value_type *
f4(const std::array<T1, T2> &a0) {
  return a0.begin();
}

template <typename T1, std::size_t T2>
typename std::array<T1, T2>::value_type *f5(std::array<T1, T2> &a0) {
  return a0.end();
}

template <typename T1, std::size_t T2>
const typename std::array<T1, T2>::value_type *
f6(const std::array<T1, T2> &a0) {
  return a0.end();
}

template <typename T1, std::size_t T2>
typename std::array<T1, T2>::reverse_iterator f7(std::array<T1, T2> &a0) {
  return a0.rbegin();
}

template <typename T1, std::size_t T2>
typename std::array<T1, T2>::const_reverse_iterator
f8(const std::array<T1, T2> &a0) {
  return a0.rbegin();
}

template <typename T1, std::size_t T2>
typename std::array<T1, T2>::reverse_iterator f9(std::array<T1, T2> &a0) {
  return a0.rend();
}

template <typename T1, std::size_t T2>
typename std::array<T1, T2>::const_reverse_iterator
f10(const std::array<T1, T2> &a0) {
  return a0.rend();
}

template <typename T1, std::size_t T2>
const typename std::array<T1, T2>::value_type *
f11(const std::array<T1, T2> &a0) {
  return a0.cbegin();
}

template <typename T1, std::size_t T2>
const typename std::array<T1, T2>::value_type *
f12(const std::array<T1, T2> &a0) {
  return a0.cend();
}

template <typename T1, std::size_t T2>
typename std::array<T1, T2>::const_reverse_iterator
f13(const std::array<T1, T2> &a0) {
  return a0.crbegin();
}

template <typename T1, std::size_t T2>
typename std::array<T1, T2>::const_reverse_iterator
f14(const std::array<T1, T2> &a0) {
  return a0.crend();
}

template <typename T1, std::size_t T2>
typename std::array<T1, T2>::size_type f15(const std::array<T1, T2> &a0) {
  return a0.size();
}

template <typename T1, std::size_t T2>
typename std::array<T1, T2>::size_type f16(const std::array<T1, T2> &a0) {
  return a0.max_size();
}

template <typename T1, std::size_t T2> bool f17(const std::array<T1, T2> &a0) {
  return a0.empty();
}

template <typename T1, std::size_t T2>
typename std::array<T1, T2>::value_type &
f18(std::array<T1, T2> &a0, typename std::array<T1, T2>::size_type a1) {
  return a0.operator[](a1);
}

template <typename T1, std::size_t T2>
const typename std::array<T1, T2>::value_type &
f19(const std::array<T1, T2> &a0, typename std::array<T1, T2>::size_type a1) {
  return a0.operator[](a1);
}

template <typename T1, std::size_t T2>
typename std::array<T1, T2>::value_type &
f20(std::array<T1, T2> &a0, typename std::array<T1, T2>::size_type a1) {
  return a0.at(a1);
}

template <typename T1, std::size_t T2>
const typename std::array<T1, T2>::value_type &
f21(const std::array<T1, T2> &a0, typename std::array<T1, T2>::size_type a1) {
  return a0.at(a1);
}

template <typename T1, std::size_t T2>
typename std::array<T1, T2>::value_type &f22(std::array<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, std::size_t T2>
const typename std::array<T1, T2>::value_type &
f23(const std::array<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, std::size_t T2>
typename std::array<T1, T2>::value_type &f24(std::array<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, std::size_t T2>
const typename std::array<T1, T2>::value_type &
f25(const std::array<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, std::size_t T2>
typename std::array<T1, T2>::value_type *f26(std::array<T1, T2> &a0) {
  return a0.data();
}

template <typename T1, std::size_t T2>
const typename std::array<T1, T2>::value_type *
f27(const std::array<T1, T2> &a0) {
  return a0.data();
}

template <typename T1, std::size_t T2>
using t13 = typename std::array<T1, T2>::reverse_iterator::iterator_type;

template <typename T1, std::size_t T2>
using t14 = typename std::array<T1, T2>::reverse_iterator::pointer;

template <typename T1, std::size_t T2>
using t15 = typename std::array<T1, T2>::reverse_iterator::difference_type;

template <typename T1, std::size_t T2>
using t16 = typename std::array<T1, T2>::reverse_iterator::reference;

template <typename T1, std::size_t T2>
using t17 = typename std::array<T1, T2>::const_reverse_iterator::iterator_type;

template <typename T1, std::size_t T2>
using t18 = typename std::array<T1, T2>::const_reverse_iterator::pointer;

template <typename T1, std::size_t T2>
using t19 =
    typename std::array<T1, T2>::const_reverse_iterator::difference_type;

template <typename T1, std::size_t T2>
using t20 = typename std::array<T1, T2>::const_reverse_iterator::reference;

template <std::size_t T1, typename T2, std::size_t T3>
const T2 &&f28(const std::array<T2, T3> &&a0) {
  return std::get<T1, T2, T3>(std::move(a0));
}

template <std::size_t T1, typename T2, std::size_t T3>
T2 &&f29(std::array<T2, T3> &&a0) {
  return std::get<T1, T2, T3>(std::move(a0));
}

template <std::size_t T1, typename T2, std::size_t T3>
const T2 &f30(const std::array<T2, T3> &a0) {
  return std::get<T1, T2, T3>(a0);
}

template <std::size_t T1, typename T2, std::size_t T3>
T2 &f31(std::array<T2, T3> &a0) {
  return std::get<T1, T2, T3>(a0);
}

template <typename T1, std::size_t T2>
bool f32(const std::array<T1, T2> &a0, const std::array<T1, T2> &a1) {
  return a0 == a1;
}
