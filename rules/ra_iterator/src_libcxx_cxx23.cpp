#include <flat_set>
#include <iterator>
#include <utility>

template <typename T1, typename T2> using t1 = std::__ra_iterator<T1, T2>;

template <typename T1, typename T2>
using t2 = typename std::__ra_iterator<T1, T2>::iterator_concept;

template <typename T1, typename T2>
using t3 = typename std::__ra_iterator<T1, T2>::iterator_category;

template <typename T1, typename T2>
using t4 = typename std::__ra_iterator<T1, T2>::value_type;

template <typename T1, typename T2>
using t5 = typename std::__ra_iterator<T1, T2>::difference_type;

template <typename T1, typename T2>
std::__ra_iterator<T1, T2> f1()
  requires(std::is_default_constructible_v<T2>)
{
  return std::__ra_iterator<T1, T2>();
}

template <typename T1, typename T2> std::__ra_iterator<T1, T2> f2(T2 a0) {
  return std::__ra_iterator<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
decltype(auto) f3(const std::__ra_iterator<T1, T2> &a0) {
  return a0.operator*();
}

template <typename T1, typename T2>
decltype(auto) f4(const std::__ra_iterator<T1, T2> &a0) {
  return a0.operator->();
}

template <typename T1, typename T2>
std::__ra_iterator<T1, T2> &f5(std::__ra_iterator<T1, T2> &a0) {
  return a0.operator++();
}

template <typename T1, typename T2>
std::__ra_iterator<T1, T2> f6(std::__ra_iterator<T1, T2> &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1, typename T2>
std::__ra_iterator<T1, T2> &f7(std::__ra_iterator<T1, T2> &a0) {
  return a0.operator--();
}

template <typename T1, typename T2>
std::__ra_iterator<T1, T2> f8(std::__ra_iterator<T1, T2> &a0, int a1) {
  return a0.operator--(a1);
}

template <typename T1, typename T2>
std::__ra_iterator<T1, T2> &f9(std::__ra_iterator<T1, T2> &a0,
                               std::iter_difference_t<T2> a1) {
  return a0.operator+=(std::forward<std::iter_difference_t<T2>>(a1));
}

template <typename T1, typename T2>
std::__ra_iterator<T1, T2> &f10(std::__ra_iterator<T1, T2> &a0,
                                std::iter_difference_t<T2> a1) {
  return a0.operator-=(std::forward<std::iter_difference_t<T2>>(a1));
}

template <typename T1, typename T2>
decltype(auto) f11(const std::__ra_iterator<T1, T2> &a0,
                   std::iter_difference_t<T2> a1) {
  return a0.operator[](std::forward<std::iter_difference_t<T2>>(a1));
}

template <typename T1, typename T2>
std::iter_difference_t<T2> f12(const std::__ra_iterator<T1, T2> &a0,
                               const std::__ra_iterator<T1, T2> &a1) {
  return operator-(a0, a1);
}

template <typename T1, typename T2>
std::__ra_iterator<T1, T2> f13(const std::__ra_iterator<T1, T2> &a0,
                               std::iter_difference_t<T2> a1) {
  return operator-(a0, std::forward<std::iter_difference_t<T2>>(a1));
}

template <typename T1, typename T2>
std::__ra_iterator<T1, T2> f14(std::iter_difference_t<T2> a0,
                               const std::__ra_iterator<T1, T2> &a1) {
  return operator+(std::forward<std::iter_difference_t<T2>>(a0), a1);
}

template <typename T1, typename T2>
std::__ra_iterator<T1, T2> f15(const std::__ra_iterator<T1, T2> &a0,
                               std::iter_difference_t<T2> a1) {
  return operator+(a0, std::forward<std::iter_difference_t<T2>>(a1));
}

template <typename T1, typename T2>
auto f16(const std::__ra_iterator<T1, T2> &a0,
         const std::__ra_iterator<T1, T2> &a1) {
  return operator<=>(a0, a1);
}

template <typename T1, typename T2>
bool f17(const std::__ra_iterator<T1, T2> &a0,
         const std::__ra_iterator<T1, T2> &a1) {
  return operator>=(a0, a1);
}

template <typename T1, typename T2>
bool f18(const std::__ra_iterator<T1, T2> &a0,
         const std::__ra_iterator<T1, T2> &a1) {
  return operator<=(a0, a1);
}

template <typename T1, typename T2>
bool f19(const std::__ra_iterator<T1, T2> &a0,
         const std::__ra_iterator<T1, T2> &a1) {
  return operator>(a0, a1);
}

template <typename T1, typename T2>
bool f20(const std::__ra_iterator<T1, T2> &a0,
         const std::__ra_iterator<T1, T2> &a1) {
  return operator<(a0, a1);
}

template <typename T1, typename T2>
bool f21(const std::__ra_iterator<T1, T2> &a0,
         const std::__ra_iterator<T1, T2> &a1) {
  return operator==(a0, a1);
}
