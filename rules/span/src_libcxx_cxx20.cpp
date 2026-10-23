#include <array>
#include <cstddef>
#include <span>
#include <utility>

template <typename T1, std::size_t T2> auto f35(std::span<T1, T2> a0) {
  return std::as_bytes<T1, T2>(a0);
}

template <typename T1, std::size_t T2> auto f36(std::span<T1, T2> a0) {
  return std::as_writable_bytes<T1, T2>(a0);
}

template <typename T1> using t12 = std::span<T1, std::dynamic_extent>;

template <typename T1>
using t13 = typename std::span<T1, std::dynamic_extent>::element_type;

template <typename T1>
using t14 = typename std::span<T1, std::dynamic_extent>::value_type;

template <typename T1>
using t15 = typename std::span<T1, std::dynamic_extent>::size_type;

template <typename T1>
using t16 = typename std::span<T1, std::dynamic_extent>::difference_type;

template <typename T1>
using t17 = typename std::span<T1, std::dynamic_extent>::pointer;

template <typename T1>
using t18 = typename std::span<T1, std::dynamic_extent>::const_pointer;

template <typename T1>
using t19 = typename std::span<T1, std::dynamic_extent>::reference;

template <typename T1>
using t20 = typename std::span<T1, std::dynamic_extent>::const_reference;

template <typename T1> std::span<T1, std::dynamic_extent> f37() {
  return std::span<T1, std::dynamic_extent>();
}

template <typename T1>
std::span<T1, std::dynamic_extent>
f38(const std::span<T1, std::dynamic_extent> &a0) {
  return std::span<T1, std::dynamic_extent>(a0);
}

template <typename T1>
std::span<T1, std::dynamic_extent> &
f39(std::span<T1, std::dynamic_extent> &a0,
    const std::span<T1, std::dynamic_extent> &a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::span<T1, std::dynamic_extent>
f40(const std::span<T1, std::dynamic_extent> &a0, std::size_t a1) {
  return a0.first(a1);
}

template <typename T1>
std::span<T1, std::dynamic_extent>
f41(const std::span<T1, std::dynamic_extent> &a0, std::size_t a1) {
  return a0.last(a1);
}

template <typename T1>
std::span<T1, std::dynamic_extent>
f42(const std::span<T1, std::dynamic_extent> &a0, std::size_t a1,
    std::size_t a2) {
  return a0.subspan(a1, a2);
}

template <typename T1>
std::size_t f43(const std::span<T1, std::dynamic_extent> &a0) {
  return a0.size();
}

template <typename T1>
std::size_t f44(const std::span<T1, std::dynamic_extent> &a0) {
  return a0.size_bytes();
}

template <typename T1> bool f45(const std::span<T1, std::dynamic_extent> &a0) {
  return a0.empty();
}

template <typename T1>
T1 &f46(const std::span<T1, std::dynamic_extent> &a0, std::size_t a1) {
  return a0.operator[](a1);
}

template <typename T1> T1 &f47(const std::span<T1, std::dynamic_extent> &a0) {
  return a0.front();
}

template <typename T1> T1 &f48(const std::span<T1, std::dynamic_extent> &a0) {
  return a0.back();
}

template <typename T1> T1 *f49(const std::span<T1, std::dynamic_extent> &a0) {
  return a0.data();
}

template <typename T1>
typename std::span<T1, std::dynamic_extent>::iterator
f50(const std::span<T1, std::dynamic_extent> &a0) {
  return a0.begin();
}

template <typename T1>
typename std::span<T1, std::dynamic_extent>::iterator
f51(const std::span<T1, std::dynamic_extent> &a0) {
  return a0.end();
}

template <typename T1>
typename std::span<T1, std::dynamic_extent>::reverse_iterator
f52(const std::span<T1, std::dynamic_extent> &a0) {
  return a0.rbegin();
}

template <typename T1>
typename std::span<T1, std::dynamic_extent>::reverse_iterator
f53(const std::span<T1, std::dynamic_extent> &a0) {
  return a0.rend();
}

template <typename T1, typename T2>
std::span<T1, std::dynamic_extent> f54(T2 a0, std::size_t a1) {
  return std::span<T1, std::dynamic_extent>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::span<T1, std::dynamic_extent> f55(T2 a0, T3 a1) {
  return std::span<T1, std::dynamic_extent>(a0, a1);
}

template <typename T1, std::size_t T2>
std::span<T1, std::dynamic_extent>
f56(typename std::type_identity<T1>::type (&a0)[T2]) {
  return std::span<T1, std::dynamic_extent>(a0);
}

template <typename T1, typename T2, std::size_t T3>
std::span<T1, std::dynamic_extent> f57(std::array<T2, T3> &a0) {
  return std::span<T1, std::dynamic_extent>(a0);
}

template <typename T1, typename T2, std::size_t T3>
std::span<T1, std::dynamic_extent> f58(const std::array<T2, T3> &a0) {
  return std::span<T1, std::dynamic_extent>(a0);
}

template <typename T1, typename T2>
std::span<T1, std::dynamic_extent> f59(T2 &&a0) {
  return std::span<T1, std::dynamic_extent>(std::forward<T2>(a0));
}

template <typename T1, typename T2, std::size_t T3>
std::span<T1, std::dynamic_extent> f60(const std::span<T2, T3> &a0) {
  return std::span<T1, std::dynamic_extent>(a0);
}

template <typename T1, std::size_t T2>
std::span<T1, T2> f61(const std::span<T1, std::dynamic_extent> &a0) {
  return a0.template first<T2>();
}

template <typename T1, std::size_t T2>
std::span<T1, T2> f62(const std::span<T1, std::dynamic_extent> &a0) {
  return a0.template last<T2>();
}

template <typename T1, std::size_t T2, std::size_t T3 = std::dynamic_extent>
auto f63(const std::span<T1, std::dynamic_extent> &a0) {
  return a0.template subspan<T2>();
}

template <typename T1>
using t21 = typename std::span<T1, std::dynamic_extent>::iterator::value_type;

template <typename T1>
using t22 =
    typename std::span<T1, std::dynamic_extent>::iterator::difference_type;

template <typename T1>
using t23 = typename std::span<T1, std::dynamic_extent>::iterator::pointer;

template <typename T1>
using t24 = typename std::span<T1, std::dynamic_extent>::iterator::reference;

template <typename T1>
using t25 =
    typename std::span<T1, std::dynamic_extent>::iterator::iterator_category;

template <typename T1>
using t26 =
    typename std::span<T1, std::dynamic_extent>::iterator::iterator_concept;

template <typename T1> std::size_t f64() {
  return std::span<T1, std::dynamic_extent>::extent;
}

template <typename T1, std::size_t T2>
std::span<T1, T2> f65(typename std::type_identity<T1>::type (&a0)[T2]) {
  return std::span<T1, T2>(a0);
}

template <typename T1, std::size_t T2, typename T3>
std::span<T1, T2> f66(std::array<T3, T2> &a0) {
  return std::span<T1, T2>(a0);
}

template <typename T1, std::size_t T2, typename T3>
std::span<T1, T2> f67(const std::array<T3, T2> &a0) {
  return std::span<T1, T2>(a0);
}

template <typename T1, std::size_t T2, typename T3>
std::span<T1, T2> f68(const std::span<T3, T2> &a0) {
  return std::span<T1, T2>(a0);
}

template <typename T1, std::size_t T2, typename T3>
std::span<T1, T2> f69(const std::span<T3, std::dynamic_extent> &a0) {
  return std::span<T1, T2>(a0);
}

template <typename T1, std::size_t T2>
using t27 = typename std::span<T1, T2>::iterator::value_type;

template <typename T1, std::size_t T2>
using t28 = typename std::span<T1, T2>::iterator::difference_type;

template <typename T1, std::size_t T2>
using t29 = typename std::span<T1, T2>::iterator::pointer;

template <typename T1, std::size_t T2>
using t30 = typename std::span<T1, T2>::iterator::reference;

template <typename T1, std::size_t T2>
using t31 = typename std::span<T1, T2>::iterator::iterator_category;

template <typename T1, std::size_t T2>
using t32 = typename std::span<T1, T2>::iterator::iterator_concept;
