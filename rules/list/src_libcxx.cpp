#include <initializer_list>
#include <list>
#include <utility>

template <typename T1, typename T2>
bool f92(const std::list<T1, T2> &a0, const std::list<T1, T2> &a1) {
  return std::operator== <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
void f93(std::list<T1, T2> &a0, std::list<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
using t15 = std::__list_const_iterator<T1, T2>;

template <typename T1, typename T2>
using t16 = typename std::__list_const_iterator<T1, T2>::iterator_category;

template <typename T1, typename T2>
using t17 = typename std::__list_const_iterator<T1, T2>::value_type;

template <typename T1, typename T2>
using t18 = typename std::__list_const_iterator<T1, T2>::reference;

template <typename T1, typename T2>
using t19 = typename std::__list_const_iterator<T1, T2>::pointer;

template <typename T1, typename T2>
using t20 = typename std::__list_const_iterator<T1, T2>::difference_type;

template <typename T1, typename T2> std::__list_const_iterator<T1, T2> f94() {
  return std::__list_const_iterator<T1, T2>();
}

template <typename T1, typename T2>
std::__list_const_iterator<T1, T2> f95(const std::__list_iterator<T1, T2> &a0) {
  return std::__list_const_iterator<T1, T2>(a0);
}

template <typename T1, typename T2>
const T1 &f96(const std::__list_const_iterator<T1, T2> &a0) {
  return a0.operator*();
}

template <typename T1, typename T2>
typename std::__list_const_iterator<T1, T2>::pointer
f97(const std::__list_const_iterator<T1, T2> &a0) {
  return a0.operator->();
}

template <typename T1, typename T2>
std::__list_const_iterator<T1, T2> &
f98(std::__list_const_iterator<T1, T2> &a0) {
  return a0.operator++();
}

template <typename T1, typename T2>
std::__list_const_iterator<T1, T2> f99(std::__list_const_iterator<T1, T2> &a0,
                                       int a1) {
  return a0.operator++(a1);
}

template <typename T1, typename T2>
std::__list_const_iterator<T1, T2> &
f100(std::__list_const_iterator<T1, T2> &a0) {
  return a0.operator--();
}

template <typename T1, typename T2>
std::__list_const_iterator<T1, T2> f101(std::__list_const_iterator<T1, T2> &a0,
                                        int a1) {
  return a0.operator--(a1);
}

template <typename T1, typename T2>
bool f102(const std::__list_const_iterator<T1, T2> &a0,
          const std::__list_const_iterator<T1, T2> &a1) {
  return operator!=(a0, a1);
}

template <typename T1, typename T2>
bool f103(const std::__list_const_iterator<T1, T2> &a0,
          const std::__list_const_iterator<T1, T2> &a1) {
  return operator==(a0, a1);
}

template <typename T1, typename T2> using t21 = std::__list_iterator<T1, T2>;

template <typename T1, typename T2>
using t22 = typename std::__list_iterator<T1, T2>::iterator_category;

template <typename T1, typename T2>
using t23 = typename std::__list_iterator<T1, T2>::value_type;

template <typename T1, typename T2>
using t24 = typename std::__list_iterator<T1, T2>::reference;

template <typename T1, typename T2>
using t25 = typename std::__list_iterator<T1, T2>::pointer;

template <typename T1, typename T2>
using t26 = typename std::__list_iterator<T1, T2>::difference_type;

template <typename T1, typename T2> std::__list_iterator<T1, T2> f104() {
  return std::__list_iterator<T1, T2>();
}

template <typename T1, typename T2>
T1 &f105(const std::__list_iterator<T1, T2> &a0) {
  return a0.operator*();
}

template <typename T1, typename T2>
typename std::__list_iterator<T1, T2>::pointer
f106(const std::__list_iterator<T1, T2> &a0) {
  return a0.operator->();
}

template <typename T1, typename T2>
std::__list_iterator<T1, T2> &f107(std::__list_iterator<T1, T2> &a0) {
  return a0.operator++();
}

template <typename T1, typename T2>
std::__list_iterator<T1, T2> f108(std::__list_iterator<T1, T2> &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1, typename T2>
std::__list_iterator<T1, T2> &f109(std::__list_iterator<T1, T2> &a0) {
  return a0.operator--();
}

template <typename T1, typename T2>
std::__list_iterator<T1, T2> f110(std::__list_iterator<T1, T2> &a0, int a1) {
  return a0.operator--(a1);
}

template <typename T1, typename T2>
bool f111(const std::__list_iterator<T1, T2> &a0,
          const std::__list_iterator<T1, T2> &a1) {
  return operator!=(a0, a1);
}

template <typename T1, typename T2>
bool f112(const std::__list_iterator<T1, T2> &a0,
          const std::__list_iterator<T1, T2> &a1) {
  return operator==(a0, a1);
}

template <typename T1, typename T2>
std::list<T1, T2> f113(typename std::list<T1, T2>::size_type a0) {
  return std::list<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::list<T1, T2> f114(typename std::list<T1, T2>::size_type a0, const T2 &a1) {
  return std::list<T1, T2>(std::move(a0), a1);
}

template <typename T1, typename T2>
std::list<T1, T2> f115(typename std::list<T1, T2>::size_type a0, const T1 &a1) {
  return std::list<T1, T2>(std::move(a0), a1);
}

template <typename T1, typename T2>
std::list<T1, T2> f116(std::initializer_list<T1> a0) {
  return std::list<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
void f117(std::list<T1, T2> &a0, typename std::list<T1, T2>::size_type a1,
          const T1 &a2) {
  return a0.assign(std::move(a1), a2);
}

template <typename T1, typename T2>
typename std::list<T1, T2>::size_type f118(const std::list<T1, T2> &a0) {
  return a0.size();
}

template <typename T1, typename T2>
typename std::list<T1, T2>::size_type f119(const std::list<T1, T2> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2> T1 &f120(std::list<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
const T1 &f121(const std::list<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2> T1 &f122(std::list<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
const T1 &f123(const std::list<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
typename std::list<T1, T2>::iterator
f124(std::list<T1, T2> &a0, typename std::list<T1, T2>::const_iterator a1,
     typename std::list<T1, T2>::size_type a2, const T1 &a3) {
  return a0.insert(std::move(a1), std::move(a2), a3);
}

template <typename T1, typename T2>
void f125(std::list<T1, T2> &a0, typename std::list<T1, T2>::size_type a1) {
  return a0.resize(std::move(a1));
}

template <typename T1, typename T2>
void f126(std::list<T1, T2> &a0, typename std::list<T1, T2>::size_type a1,
          const T1 &a2) {
  return a0.resize(std::move(a1), a2);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_allocator_v<T2>, int> T3 = 0>
std::list<T1, T2> f127(typename std::list<T1, T2>::size_type a0, const T1 &a1,
                       const T2 &a2) {
  return std::list<T1, T2>(std::move(a0), a1, a2);
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<std::__has_input_iterator_category<T3>::value, int>
              T4 = 0>
std::list<T1, T2> f128(T3 a0, T3 a1) {
  return std::list<T1, T2>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<std::__has_input_iterator_category<T3>::value, int>
              T4 = 0>
std::list<T1, T2> f129(T3 a0, T3 a1, const T2 &a2) {
  return std::list<T1, T2>(std::move(a0), std::move(a1), a2);
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<std::__has_input_iterator_category<T3>::value, int>
              T4 = 0>
void f130(std::list<T1, T2> &a0, T3 a1, T3 a2) {
  return a0.template assign<T3>(std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<std::__has_input_iterator_category<T3>::value, int>
              T4 = 0>
typename std::list<T1, T2>::iterator
f131(std::list<T1, T2> &a0, typename std::list<T1, T2>::const_iterator a1,
     T3 a2, T3 a3) {
  return a0.template insert<T3>(std::move(a1), std::move(a2), std::move(a3));
}
