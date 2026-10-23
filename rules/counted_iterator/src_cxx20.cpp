#include <algorithm>
#include <iterator>
#include <utility>

template <typename T1> using t1 = std::counted_iterator<T1>;

template <typename T1>
using t2 = typename std::counted_iterator<T1>::iterator_type;

template <typename T1>
using t3 = typename std::counted_iterator<T1>::difference_type;

template <typename T1> std::counted_iterator<T1> f1() {
  return std::counted_iterator<T1>();
}

template <typename T1>
std::counted_iterator<T1> f2(T1 a0, std::iter_difference_t<T1> a1) {
  return std::counted_iterator<T1>(a0, a1);
}

template <typename T1> const T1 &f3(const std::counted_iterator<T1> &a0) {
  return a0.base();
}

template <typename T1> T1 f4(std::counted_iterator<T1> &&a0) {
  return std::move(a0).base();
}

template <typename T1>
std::iter_difference_t<T1> f5(const std::counted_iterator<T1> &a0) {
  return a0.count();
}

template <typename T1> decltype(auto) f6(std::counted_iterator<T1> &a0) {
  return a0.operator*();
}

template <typename T1> decltype(auto) f7(const std::counted_iterator<T1> &a0) {
  return a0.operator*();
}

template <typename T1> auto f8(const std::counted_iterator<T1> &a0) {
  return a0.operator->();
}

template <typename T1>
std::counted_iterator<T1> &f9(std::counted_iterator<T1> &a0) {
  return a0.operator++();
}

template <typename T1>
decltype(auto) f10(std::counted_iterator<T1> &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1>
std::counted_iterator<T1> f11(std::counted_iterator<T1> &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1>
std::counted_iterator<T1> &f12(std::counted_iterator<T1> &a0) {
  return a0.operator--();
}

template <typename T1>
std::counted_iterator<T1> f13(std::counted_iterator<T1> &a0, int a1) {
  return a0.operator--(a1);
}

template <typename T1>
std::counted_iterator<T1> f14(const std::counted_iterator<T1> &a0,
                              std::iter_difference_t<T1> a1) {
  return a0.operator+(a1);
}

template <typename T1>
std::counted_iterator<T1> &f15(std::counted_iterator<T1> &a0,
                               std::iter_difference_t<T1> a1) {
  return a0.operator+=(a1);
}

template <typename T1>
std::counted_iterator<T1> f16(const std::counted_iterator<T1> &a0,
                              std::iter_difference_t<T1> a1) {
  return a0.operator-(a1);
}

template <typename T1>
std::counted_iterator<T1> &f17(std::counted_iterator<T1> &a0,
                               std::iter_difference_t<T1> a1) {
  return a0.operator-=(a1);
}

template <typename T1>
decltype(auto) f18(const std::counted_iterator<T1> &a0,
                   std::iter_difference_t<T1> a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2>
std::counted_iterator<T1> f19(const std::counted_iterator<T2> &a0) {
  return std::counted_iterator<T1>(a0);
}

template <typename T1, typename T2>
std::counted_iterator<T1> &f20(std::counted_iterator<T1> &a0,
                               const std::counted_iterator<T2> &a1) {
  return a0.template operator= <T2>(a1);
}

template <typename T1>
bool f21(const std::counted_iterator<T1> &a0, std::default_sentinel_t a1) {
  return operator==(a0, std::move(a1));
}

template <typename T1>
std::iter_difference_t<T1> f22(std::default_sentinel_t a0,
                               const std::counted_iterator<T1> &a1) {
  return operator-(std::move(a0), a1);
}

template <typename T1>
std::iter_difference_t<T1> f23(const std::counted_iterator<T1> &a0,
                               std::default_sentinel_t a1) {
  return operator-(a0, std::move(a1));
}

template <typename T1>
std::counted_iterator<T1> f24(std::iter_difference_t<T1> a0,
                              const std::counted_iterator<T1> &a1) {
  return operator+(a0, a1);
}
