#include <forward_list>
#include <initializer_list>
#include <utility>

template <typename T1> using t32 = std::__forward_list_const_iterator<T1>;

template <typename T1>
using t33 = typename std::__forward_list_const_iterator<T1>::iterator_category;

template <typename T1>
using t34 = typename std::__forward_list_const_iterator<T1>::value_type;

template <typename T1>
using t35 = typename std::__forward_list_const_iterator<T1>::reference;

template <typename T1>
using t36 = typename std::__forward_list_const_iterator<T1>::difference_type;

template <typename T1>
using t37 = typename std::__forward_list_const_iterator<T1>::pointer;

template <typename T1> std::__forward_list_const_iterator<T1> f110() {
  return std::__forward_list_const_iterator<T1>();
}

template <typename T1>
const typename std::__forward_list_const_iterator<T1>::value_type &
f111(const std::__forward_list_const_iterator<T1> &a0) {
  return a0.operator*();
}

template <typename T1>
typename std::__forward_list_const_iterator<T1>::pointer
f112(const std::__forward_list_const_iterator<T1> &a0) {
  return a0.operator->();
}

template <typename T1>
std::__forward_list_const_iterator<T1> &
f113(std::__forward_list_const_iterator<T1> &a0) {
  return a0.operator++();
}

template <typename T1>
std::__forward_list_const_iterator<T1>
f114(std::__forward_list_const_iterator<T1> &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1>
bool f115(const std::__forward_list_const_iterator<T1> &a0,
          const std::__forward_list_const_iterator<T1> &a1) {
  return operator!=(a0, a1);
}

template <typename T1>
bool f116(const std::__forward_list_const_iterator<T1> &a0,
          const std::__forward_list_const_iterator<T1> &a1) {
  return operator==(a0, a1);
}

template <typename T1> using t38 = std::__forward_list_iterator<T1>;

template <typename T1>
using t39 = typename std::__forward_list_iterator<T1>::iterator_category;

template <typename T1>
using t40 = typename std::__forward_list_iterator<T1>::value_type;

template <typename T1>
using t41 = typename std::__forward_list_iterator<T1>::reference;

template <typename T1>
using t42 = typename std::__forward_list_iterator<T1>::difference_type;

template <typename T1>
using t43 = typename std::__forward_list_iterator<T1>::pointer;

template <typename T1> std::__forward_list_iterator<T1> f117() {
  return std::__forward_list_iterator<T1>();
}

template <typename T1>
typename std::__forward_list_iterator<T1>::value_type &
f118(const std::__forward_list_iterator<T1> &a0) {
  return a0.operator*();
}

template <typename T1>
typename std::__forward_list_iterator<T1>::pointer
f119(const std::__forward_list_iterator<T1> &a0) {
  return a0.operator->();
}

template <typename T1>
std::__forward_list_iterator<T1> &f120(std::__forward_list_iterator<T1> &a0) {
  return a0.operator++();
}

template <typename T1>
std::__forward_list_iterator<T1> f121(std::__forward_list_iterator<T1> &a0,
                                      int a1) {
  return a0.operator++(a1);
}

template <typename T1>
bool f122(const std::__forward_list_iterator<T1> &a0,
          const std::__forward_list_iterator<T1> &a1) {
  return operator!=(a0, a1);
}

template <typename T1>
bool f123(const std::__forward_list_iterator<T1> &a0,
          const std::__forward_list_iterator<T1> &a1) {
  return operator==(a0, a1);
}

template <typename T1, typename T2>
std::forward_list<T1, T2>
f124(typename std::forward_list<T1, T2>::size_type a0) {
  return std::forward_list<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::forward_list<T1, T2> f125(typename std::forward_list<T1, T2>::size_type a0,
                               const T2 &a1) {
  return std::forward_list<T1, T2>(std::move(a0), a1);
}

template <typename T1, typename T2>
std::forward_list<T1, T2> f126(typename std::forward_list<T1, T2>::size_type a0,
                               const T1 &a1) {
  return std::forward_list<T1, T2>(std::move(a0), a1);
}

template <typename T1, typename T2>
std::forward_list<T1, T2> f127(std::initializer_list<T1> a0) {
  return std::forward_list<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
void f128(std::forward_list<T1, T2> &a0,
          typename std::forward_list<T1, T2>::size_type a1, const T1 &a2) {
  return a0.assign(std::move(a1), a2);
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::size_type
f129(const std::forward_list<T1, T2> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::iterator
f130(std::forward_list<T1, T2> &a0,
     typename std::forward_list<T1, T2>::const_iterator a1,
     typename std::forward_list<T1, T2>::size_type a2, const T1 &a3) {
  return a0.insert_after(std::move(a1), std::move(a2), a3);
}

template <typename T1, typename T2>
void f131(std::forward_list<T1, T2> &a0,
          typename std::forward_list<T1, T2>::size_type a1) {
  return a0.resize(std::move(a1));
}

template <typename T1, typename T2>
void f132(std::forward_list<T1, T2> &a0,
          typename std::forward_list<T1, T2>::size_type a1, const T1 &a2) {
  return a0.resize(std::move(a1), a2);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_allocator_v<T2>, int> T3 = 0>
std::forward_list<T1, T2> f133(typename std::forward_list<T1, T2>::size_type a0,
                               const T1 &a1, const T2 &a2) {
  return std::forward_list<T1, T2>(std::move(a0), a1, a2);
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<std::__has_input_iterator_category<T3>::value, int>
              T4 = 0>
std::forward_list<T1, T2> f134(T3 a0, T3 a1) {
  return std::forward_list<T1, T2>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<std::__has_input_iterator_category<T3>::value, int>
              T4 = 0>
std::forward_list<T1, T2> f135(T3 a0, T3 a1, const T2 &a2) {
  return std::forward_list<T1, T2>(std::move(a0), std::move(a1), a2);
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<std::__has_input_iterator_category<T3>::value, int>
              T4 = 0>
void f136(std::forward_list<T1, T2> &a0, T3 a1, T3 a2) {
  return a0.template assign<T3>(std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<std::__has_input_iterator_category<T3>::value, int>
              T4 = 0>
typename std::forward_list<T1, T2>::iterator
f137(std::forward_list<T1, T2> &a0,
     typename std::forward_list<T1, T2>::const_iterator a1, T3 a2, T3 a3) {
  return a0.template insert_after<T3>(std::move(a1), std::move(a2),
                                      std::move(a3));
}
