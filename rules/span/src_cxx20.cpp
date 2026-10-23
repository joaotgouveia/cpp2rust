#include <algorithm>
#include <array>
#include <cstddef>
#include <span>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, std::size_t T2> using t1 = std::span<T1, T2>;

template <typename T1, std::size_t T2>
using t2 = typename std::span<T1, T2>::element_type;

template <typename T1, std::size_t T2>
using t3 = typename std::span<T1, T2>::value_type;

template <typename T1, std::size_t T2>
using t4 = typename std::span<T1, T2>::size_type;

template <typename T1, std::size_t T2>
using t5 = typename std::span<T1, T2>::difference_type;

template <typename T1, std::size_t T2>
using t6 = typename std::span<T1, T2>::pointer;

template <typename T1, std::size_t T2>
using t7 = typename std::span<T1, T2>::const_pointer;

template <typename T1, std::size_t T2>
using t8 = typename std::span<T1, T2>::reference;

template <typename T1, std::size_t T2>
using t9 = typename std::span<T1, T2>::const_reference;

template <typename T1, std::size_t T2>
std::span<T1, T2> f1(const std::span<T1, T2> &a0) {
  return std::span<T1, T2>(a0);
}

template <typename T1, std::size_t T2>
std::span<T1, T2> &f2(std::span<T1, T2> &a0, const std::span<T1, T2> &a1) {
  return a0.operator=(a1);
}

template <typename T1, std::size_t T2>
typename std::span<T1, T2>::size_type f3(const std::span<T1, T2> &a0) {
  return a0.size();
}

template <typename T1, std::size_t T2>
typename std::span<T1, T2>::size_type f4(const std::span<T1, T2> &a0) {
  return a0.size_bytes();
}

template <typename T1, std::size_t T2> bool f5(const std::span<T1, T2> &a0) {
  return a0.empty();
}

template <typename T1, std::size_t T2> T1 *f6(const std::span<T1, T2> &a0) {
  return a0.data();
}

template <typename T1, std::size_t T2>
typename std::span<T1, T2>::iterator f7(const std::span<T1, T2> &a0) {
  return a0.begin();
}

template <typename T1, std::size_t T2>
typename std::span<T1, T2>::iterator f8(const std::span<T1, T2> &a0) {
  return a0.end();
}

template <typename T1, std::size_t T2>
typename std::span<T1, T2>::reverse_iterator f9(const std::span<T1, T2> &a0) {
  return a0.rbegin();
}

template <typename T1, std::size_t T2>
typename std::span<T1, T2>::reverse_iterator f10(const std::span<T1, T2> &a0) {
  return a0.rend();
}

template <typename T1, std::size_t T2>
std::span<typename std::span<T1, T2>::element_type, std::dynamic_extent>
f11(const std::span<T1, T2> &a0, typename std::span<T1, T2>::size_type a1) {
  return a0.first(a1);
}

template <typename T1, std::size_t T2>
std::span<typename std::span<T1, T2>::element_type, std::dynamic_extent>
f12(const std::span<T1, T2> &a0, typename std::span<T1, T2>::size_type a1) {
  return a0.last(a1);
}

template <typename T1, std::size_t T2>
std::span<typename std::span<T1, T2>::element_type, std::dynamic_extent>
f13(const std::span<T1, T2> &a0, typename std::span<T1, T2>::size_type a1,
    typename std::span<T1, T2>::size_type a2) {
  return a0.subspan(a1, a2);
}

template <typename T1, std::size_t T2, typename T3>
std::span<T1, T2> f14(T3 a0, typename std::span<T1, T2>::size_type a1) {
  return std::span<T1, T2>(a0, a1);
}

template <typename T1, std::size_t T2, typename T3, typename T4>
std::span<T1, T2> f15(T3 a0, T4 a1) {
  return std::span<T1, T2>(a0, a1);
}

template <typename T1, std::size_t T2, typename T3>
std::span<T1, T2> f16(T3 &&a0) {
  return std::span<T1, T2>(std::forward<T3>(a0));
}

template <typename T1, std::size_t T2, std::size_t T3>
std::span<typename std::span<T1, T2>::element_type, T3>
f17(const std::span<T1, T2> &a0) {
  return a0.template first<T3>();
}

template <typename T1, std::size_t T2, std::size_t T3>
std::span<typename std::span<T1, T2>::element_type, T3>
f18(const std::span<T1, T2> &a0) {
  return a0.template last<T3>();
}

template <typename T1, std::size_t T2>
using t10 = typename std::span<T1, T2>::reverse_iterator::pointer;

template <typename T1, std::size_t T2>
using t11 = typename std::span<T1, T2>::reverse_iterator::iterator_concept;

template <typename T1, std::size_t T2>
using t12 = typename std::span<T1, T2>::reverse_iterator::iterator_category;

template <typename T1, std::size_t T2>
using t13 = typename std::span<T1, T2>::reverse_iterator::value_type;

template <typename T1, std::size_t T2>
using t14 = typename std::span<T1, T2>::reverse_iterator::difference_type;

template <typename T1, std::size_t T2>
using t15 = typename std::span<T1, T2>::reverse_iterator::reference;
