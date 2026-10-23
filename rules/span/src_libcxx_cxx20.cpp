#include <array>
#include <span>
#include <utility>

template <typename T1, std::size_t T2> auto f34(std::span<T1, T2> a0) {
  return std::as_bytes<T1, T2>(a0);
}

template <typename T1, std::size_t T2> auto f35(std::span<T1, T2> a0) {
  return std::as_writable_bytes<T1, T2>(a0);
}

template <typename T1> using t18 = std::span<T1, std::dynamic_extent>;

template <typename T1>
using t19 = typename std::span<T1, std::dynamic_extent>::element_type;

template <typename T1>
using t20 = typename std::span<T1, std::dynamic_extent>::value_type;

template <typename T1>
using t21 = typename std::span<T1, std::dynamic_extent>::size_type;

template <typename T1>
using t22 = typename std::span<T1, std::dynamic_extent>::difference_type;

template <typename T1>
using t23 = typename std::span<T1, std::dynamic_extent>::pointer;

template <typename T1>
using t24 = typename std::span<T1, std::dynamic_extent>::const_pointer;

template <typename T1>
using t25 = typename std::span<T1, std::dynamic_extent>::reference;

template <typename T1>
using t26 = typename std::span<T1, std::dynamic_extent>::const_reference;

template <typename T1> std::span<T1, std::dynamic_extent> f36() {
  return std::span<T1, std::dynamic_extent>();
}

template <typename T1>
std::span<T1, std::dynamic_extent>
f37(const std::span<T1, std::dynamic_extent> &a0) {
  return std::span<T1, std::dynamic_extent>(a0);
}

template <typename T1>
std::span<T1, std::dynamic_extent> &
f38(std::span<T1, std::dynamic_extent> &a0,
    const std::span<T1, std::dynamic_extent> &a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::span<T1, std::dynamic_extent>
f39(const std::span<T1, std::dynamic_extent> &a0,
    typename std::span<T1, std::dynamic_extent>::size_type a1) {
  return a0.first(a1);
}

template <typename T1>
std::span<T1, std::dynamic_extent>
f40(const std::span<T1, std::dynamic_extent> &a0,
    typename std::span<T1, std::dynamic_extent>::size_type a1) {
  return a0.last(a1);
}

template <typename T1>
std::span<T1, std::dynamic_extent>
f41(const std::span<T1, std::dynamic_extent> &a0,
    typename std::span<T1, std::dynamic_extent>::size_type a1,
    typename std::span<T1, std::dynamic_extent>::size_type a2) {
  return a0.subspan(a1, a2);
}

template <typename T1>
typename std::span<T1, std::dynamic_extent>::size_type
f42(const std::span<T1, std::dynamic_extent> &a0) {
  return a0.size();
}

template <typename T1>
typename std::span<T1, std::dynamic_extent>::size_type
f43(const std::span<T1, std::dynamic_extent> &a0) {
  return a0.size_bytes();
}

template <typename T1> bool f44(const std::span<T1, std::dynamic_extent> &a0) {
  return a0.empty();
}

template <typename T1>
T1 &f45(const std::span<T1, std::dynamic_extent> &a0,
        typename std::span<T1, std::dynamic_extent>::size_type a1) {
  return a0.operator[](a1);
}

template <typename T1> T1 &f46(const std::span<T1, std::dynamic_extent> &a0) {
  return a0.front();
}

template <typename T1> T1 &f47(const std::span<T1, std::dynamic_extent> &a0) {
  return a0.back();
}

template <typename T1> T1 *f48(const std::span<T1, std::dynamic_extent> &a0) {
  return a0.data();
}

template <typename T1>
typename std::span<T1, std::dynamic_extent>::iterator
f49(const std::span<T1, std::dynamic_extent> &a0) {
  return a0.begin();
}

template <typename T1>
typename std::span<T1, std::dynamic_extent>::iterator
f50(const std::span<T1, std::dynamic_extent> &a0) {
  return a0.end();
}

template <typename T1>
typename std::span<T1, std::dynamic_extent>::reverse_iterator
f51(const std::span<T1, std::dynamic_extent> &a0) {
  return a0.rbegin();
}

template <typename T1>
typename std::span<T1, std::dynamic_extent>::reverse_iterator
f52(const std::span<T1, std::dynamic_extent> &a0) {
  return a0.rend();
}

template <typename T1, typename T2>
std::span<T1, std::dynamic_extent>
f53(T2 a0, typename std::span<T1, std::dynamic_extent>::size_type a1) {
  return std::span<T1, std::dynamic_extent>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::span<T1, std::dynamic_extent> f54(T2 a0, T3 a1) {
  return std::span<T1, std::dynamic_extent>(a0, a1);
}

template <typename T1, std::size_t T2>
std::span<T1, std::dynamic_extent>
f55(typename std::type_identity<T1>::type (&a0)[T2]) {
  return std::span<T1, std::dynamic_extent>(a0);
}

template <typename T1, typename T2, std::size_t T3>
std::span<T1, std::dynamic_extent> f56(std::array<T2, T3> &a0) {
  return std::span<T1, std::dynamic_extent>(a0);
}

template <typename T1, typename T2, std::size_t T3>
std::span<T1, std::dynamic_extent> f57(const std::array<T2, T3> &a0) {
  return std::span<T1, std::dynamic_extent>(a0);
}

template <typename T1, typename T2>
std::span<T1, std::dynamic_extent> f58(T2 &&a0) {
  return std::span<T1, std::dynamic_extent>(std::forward<T2>(a0));
}

template <typename T1, typename T2, std::size_t T3>
std::span<T1, std::dynamic_extent> f59(const std::span<T2, T3> &a0) {
  return std::span<T1, std::dynamic_extent>(a0);
}

template <typename T1, std::size_t T2>
std::span<typename std::span<T1, std::dynamic_extent>::element_type, T2>
f60(const std::span<T1, std::dynamic_extent> &a0) {
  return a0.template first<T2>();
}

template <typename T1, std::size_t T2>
std::span<typename std::span<T1, std::dynamic_extent>::element_type, T2>
f61(const std::span<T1, std::dynamic_extent> &a0) {
  return a0.template last<T2>();
}

template <typename T1>
using t27 = typename std::span<T1, std::dynamic_extent>::iterator::value_type;

template <typename T1>
using t28 =
    typename std::span<T1, std::dynamic_extent>::iterator::difference_type;

template <typename T1>
using t29 = typename std::span<T1, std::dynamic_extent>::iterator::pointer;

template <typename T1>
using t30 = typename std::span<T1, std::dynamic_extent>::iterator::reference;

template <typename T1>
using t31 =
    typename std::span<T1, std::dynamic_extent>::iterator::iterator_category;

template <typename T1>
using t32 =
    typename std::span<T1, std::dynamic_extent>::iterator::iterator_concept;

template <typename T1>
using t33 = typename std::span<
    T1, std::dynamic_extent>::reverse_iterator::iterator_category;

template <typename T1>
using t34 =
    typename std::span<T1, std::dynamic_extent>::reverse_iterator::pointer;

template <typename T1>
using t35 =
    typename std::span<T1,
                       std::dynamic_extent>::reverse_iterator::iterator_concept;

template <typename T1>
using t36 =
    typename std::span<T1, std::dynamic_extent>::reverse_iterator::value_type;

template <typename T1>
using t37 =
    typename std::span<T1,
                       std::dynamic_extent>::reverse_iterator::difference_type;

template <typename T1>
using t38 =
    typename std::span<T1, std::dynamic_extent>::reverse_iterator::reference;

template <typename T1>
typename std::span<T1, std::dynamic_extent>::size_type f62() {
  return std::span<T1, std::dynamic_extent>::extent;
}

template <typename T1, std::size_t T2>
std::span<T1, T2> f63(typename std::type_identity<T1>::type (&a0)[T2]) {
  return std::span<T1, T2>(a0);
}

template <typename T1, std::size_t T2>
T1 &f64(const std::span<T1, T2> &a0, typename std::span<T1, T2>::size_type a1) {
  return a0.operator[](a1);
}

template <typename T1, std::size_t T2> T1 &f65(const std::span<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, std::size_t T2> T1 &f66(const std::span<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, std::size_t T2, typename T3>
std::span<T1, T2> f67(std::array<T3, T2> &a0) {
  return std::span<T1, T2>(a0);
}

template <typename T1, std::size_t T2, typename T3>
std::span<T1, T2> f68(const std::array<T3, T2> &a0) {
  return std::span<T1, T2>(a0);
}

template <typename T1, std::size_t T2, typename T3>
std::span<T1, T2> f69(const std::span<T3, T2> &a0) {
  return std::span<T1, T2>(a0);
}

template <typename T1, std::size_t T2, typename T3>
std::span<T1, T2> f70(const std::span<T3, std::dynamic_extent> &a0) {
  return std::span<T1, T2>(a0);
}

template <typename T1, std::size_t T2>
using t39 = typename std::span<T1, T2>::iterator::value_type;

template <typename T1, std::size_t T2>
using t40 = typename std::span<T1, T2>::iterator::difference_type;

template <typename T1, std::size_t T2>
using t41 = typename std::span<T1, T2>::iterator::pointer;

template <typename T1, std::size_t T2>
using t42 = typename std::span<T1, T2>::iterator::reference;

template <typename T1, std::size_t T2>
using t43 = typename std::span<T1, T2>::iterator::iterator_category;

template <typename T1, std::size_t T2>
using t44 = typename std::span<T1, T2>::iterator::iterator_concept;

template <typename T1, std::size_t T2>
typename std::span<T1, T2>::size_type f71() {
  return std::span<T1, T2>::extent;
}
