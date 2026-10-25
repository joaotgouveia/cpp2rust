#include <forward_list>
#include <initializer_list>
#include <utility>

template <typename T1> using t47 = std::__forward_list_const_iterator<T1>;

template <typename T1>
using t48 = typename std::__forward_list_const_iterator<T1>::iterator_category;

template <typename T1>
using t49 = typename std::__forward_list_const_iterator<T1>::value_type;

template <typename T1>
using t50 = typename std::__forward_list_const_iterator<T1>::reference;

template <typename T1>
using t51 = typename std::__forward_list_const_iterator<T1>::difference_type;

template <typename T1>
using t52 = typename std::__forward_list_const_iterator<T1>::pointer;

template <typename T1> std::__forward_list_const_iterator<T1> f112() {
  return std::__forward_list_const_iterator<T1>();
}

template <typename T1>
const typename std::__forward_list_const_iterator<T1>::value_type &
f113(const std::__forward_list_const_iterator<T1> &a0) {
  return a0.operator*();
}

template <typename T1>
typename std::__forward_list_const_iterator<T1>::pointer
f114(const std::__forward_list_const_iterator<T1> &a0) {
  return a0.operator->();
}

template <typename T1>
std::__forward_list_const_iterator<T1> &
f115(std::__forward_list_const_iterator<T1> &a0) {
  return a0.operator++();
}

template <typename T1>
std::__forward_list_const_iterator<T1>
f116(std::__forward_list_const_iterator<T1> &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1>
bool f117(const std::__forward_list_const_iterator<T1> &a0,
          const std::__forward_list_const_iterator<T1> &a1) {
  return operator!=(a0, a1);
}

template <typename T1>
bool f118(const std::__forward_list_const_iterator<T1> &a0,
          const std::__forward_list_const_iterator<T1> &a1) {
  return operator==(a0, a1);
}

template <typename T1> using t53 = std::__forward_list_iterator<T1>;

template <typename T1>
using t54 = typename std::__forward_list_iterator<T1>::iterator_category;

template <typename T1>
using t55 = typename std::__forward_list_iterator<T1>::value_type;

template <typename T1>
using t56 = typename std::__forward_list_iterator<T1>::reference;

template <typename T1>
using t57 = typename std::__forward_list_iterator<T1>::difference_type;

template <typename T1>
using t58 = typename std::__forward_list_iterator<T1>::pointer;

template <typename T1> std::__forward_list_iterator<T1> f119() {
  return std::__forward_list_iterator<T1>();
}

template <typename T1>
typename std::__forward_list_iterator<T1>::value_type &
f120(const std::__forward_list_iterator<T1> &a0) {
  return a0.operator*();
}

template <typename T1>
typename std::__forward_list_iterator<T1>::pointer
f121(const std::__forward_list_iterator<T1> &a0) {
  return a0.operator->();
}

template <typename T1>
std::__forward_list_iterator<T1> &f122(std::__forward_list_iterator<T1> &a0) {
  return a0.operator++();
}

template <typename T1>
std::__forward_list_iterator<T1> f123(std::__forward_list_iterator<T1> &a0,
                                      int a1) {
  return a0.operator++(a1);
}

template <typename T1>
bool f124(const std::__forward_list_iterator<T1> &a0,
          const std::__forward_list_iterator<T1> &a1) {
  return operator!=(a0, a1);
}

template <typename T1>
bool f125(const std::__forward_list_iterator<T1> &a0,
          const std::__forward_list_iterator<T1> &a1) {
  return operator==(a0, a1);
}

template <typename T1, typename T2>
std::forward_list<T1, T2>
f126(typename std::forward_list<T1, T2>::size_type a0) {
  return std::forward_list<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::forward_list<T1, T2> f127(typename std::forward_list<T1, T2>::size_type a0,
                               const T2 &a1) {
  return std::forward_list<T1, T2>(std::move(a0), a1);
}

template <typename T1, typename T2>
std::forward_list<T1, T2> f128(typename std::forward_list<T1, T2>::size_type a0,
                               const T1 &a1) {
  return std::forward_list<T1, T2>(std::move(a0), a1);
}

template <typename T1, typename T2>
std::forward_list<T1, T2> f129(std::initializer_list<T1> a0) {
  return std::forward_list<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
void f130(std::forward_list<T1, T2> &a0,
          typename std::forward_list<T1, T2>::size_type a1, const T1 &a2) {
  return a0.assign(std::move(a1), a2);
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::size_type
f131(const std::forward_list<T1, T2> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::iterator
f132(std::forward_list<T1, T2> &a0,
     typename std::forward_list<T1, T2>::const_iterator a1,
     typename std::forward_list<T1, T2>::size_type a2, const T1 &a3) {
  return a0.insert_after(std::move(a1), std::move(a2), a3);
}

template <typename T1, typename T2>
void f133(std::forward_list<T1, T2> &a0,
          typename std::forward_list<T1, T2>::size_type a1) {
  return a0.resize(std::move(a1));
}

template <typename T1, typename T2>
void f134(std::forward_list<T1, T2> &a0,
          typename std::forward_list<T1, T2>::size_type a1, const T1 &a2) {
  return a0.resize(std::move(a1), a2);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_allocator_v<T2>, int> T3 = 0>
std::forward_list<T1, T2> f135(typename std::forward_list<T1, T2>::size_type a0,
                               const T1 &a1, const T2 &a2) {
  return std::forward_list<T1, T2>(std::move(a0), a1, a2);
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<std::__has_input_iterator_category<T3>::value, int>
              T4 = 0>
std::forward_list<T1, T2> f136(T3 a0, T3 a1) {
  return std::forward_list<T1, T2>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<std::__has_input_iterator_category<T3>::value, int>
              T4 = 0>
std::forward_list<T1, T2> f137(T3 a0, T3 a1, const T2 &a2) {
  return std::forward_list<T1, T2>(std::move(a0), std::move(a1), a2);
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<std::__has_input_iterator_category<T3>::value, int>
              T4 = 0>
void f138(std::forward_list<T1, T2> &a0, T3 a1, T3 a2) {
  return a0.template assign<T3>(std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<std::__has_input_iterator_category<T3>::value, int>
              T4 = 0>
typename std::forward_list<T1, T2>::iterator
f139(std::forward_list<T1, T2> &a0,
     typename std::forward_list<T1, T2>::const_iterator a1, T3 a2, T3 a3) {
  return a0.template insert_after<T3>(std::move(a1), std::move(a2),
                                      std::move(a3));
}
