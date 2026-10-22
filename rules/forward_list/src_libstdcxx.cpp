#include "../common/init.h"
#include "../common/traits.h"
#include <forward_list>
#include <initializer_list>
#include <utility>

template <typename T1, typename T2>
std::forward_list<T1, T2> f62(const T2 &a0) {
  return std::forward_list<T1, T2>(a0);
}

template <typename T1, typename T2>
std::forward_list<T1, T2> f63(const std::forward_list<T1, T2> &a0,
                              const T2 &a1) {
  return std::forward_list<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::forward_list<T1, T2> f64(std::forward_list<T1, T2> &&a0, const T2 &a1) {
  return std::forward_list<T1, T2>(std::move(a0), a1);
}

template <typename T1, typename T2>
std::forward_list<T1, T2> f65(typename std::forward_list<T1, T2>::size_type a0,
                              const T2 &a1) {
  return std::forward_list<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::forward_list<T1, T2> f66(typename std::forward_list<T1, T2>::size_type a0,
                              const T1 &a1, const T2 &a2) {
  return std::forward_list<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2>
std::forward_list<T1, T2> f67(std::initializer_list<T1> a0, const T2 &a1) {
  return std::forward_list<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::forward_list<T1, T2> &f68(std::forward_list<T1, T2> &a0,
                               std::initializer_list<T1> a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
void f69(std::forward_list<T1, T2> &a0,
         typename std::forward_list<T1, T2>::size_type a1, const T1 &a2) {
  return a0.assign(a1, a2);
}

template <typename T1, typename T2>
void f70(std::forward_list<T1, T2> &a0, std::initializer_list<T1> a1) {
  return a0.assign(a1);
}

template <typename T1, typename T2>
void f71(std::forward_list<T1, T2> &a0, const T1 &a1) {
  return a0.push_front(a1);
}

template <typename T1, typename T2>
void f72(std::forward_list<T1, T2> &a0, T1 &&a1) {
  return a0.push_front(std::move(a1));
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::iterator
f73(std::forward_list<T1, T2> &a0,
    typename std::forward_list<T1, T2>::const_iterator a1, const T1 &a2) {
  return a0.insert_after(a1, a2);
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::iterator
f74(std::forward_list<T1, T2> &a0,
    typename std::forward_list<T1, T2>::const_iterator a1, T1 &&a2) {
  return a0.insert_after(a1, std::move(a2));
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::iterator
f75(std::forward_list<T1, T2> &a0,
    typename std::forward_list<T1, T2>::const_iterator a1,
    typename std::forward_list<T1, T2>::size_type a2, const T1 &a3) {
  return a0.insert_after(a1, a2, a3);
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::iterator
f76(std::forward_list<T1, T2> &a0,
    typename std::forward_list<T1, T2>::const_iterator a1,
    std::initializer_list<T1> a2) {
  return a0.insert_after(a1, a2);
}

template <typename T1, typename T2>
void f77(std::forward_list<T1, T2> &a0, const T1 &a1) {
  return a0.remove(a1);
}

template <typename T1, typename T2, typename T3,
          typename T4 = typename std::enable_if<
              std::is_convertible<
                  typename std::iterator_traits<T3>::iterator_category,
                  std::input_iterator_tag>::value,
              void>::type>
std::forward_list<T1, T2> f78(T3 a0, T3 a1, const T2 &a2) {
  return std::forward_list<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3,
          typename T4 = typename std::enable_if<
              std::is_convertible<
                  typename std::iterator_traits<T3>::iterator_category,
                  std::input_iterator_tag>::value,
              void>::type>
void f79(std::forward_list<T1, T2> &a0, T3 a1, T3 a2) {
  return a0.template assign<T3>(a1, a2);
}

template <typename T1, typename T2, typename T3,
          typename T4 = typename std::enable_if<
              std::is_convertible<
                  typename std::iterator_traits<T3>::iterator_category,
                  std::input_iterator_tag>::value,
              void>::type>
typename std::forward_list<T1, T2>::iterator
f80(std::forward_list<T1, T2> &a0,
    typename std::forward_list<T1, T2>::const_iterator a1, T3 a2, T3 a3) {
  return a0.template insert_after<T3>(a1, a2, a3);
}

template <typename T1, typename T2>
using t11 = typename std::forward_list<T1, T2>::iterator::value_type;

template <typename T1, typename T2>
using t12 = typename std::forward_list<T1, T2>::iterator::difference_type;

template <typename T1, typename T2>
using t13 = typename std::forward_list<T1, T2>::iterator::iterator_category;

template <typename T1, typename T2>
using t14 = typename std::forward_list<T1, T2>::iterator::pointer;

template <typename T1, typename T2>
using t15 = typename std::forward_list<T1, T2>::iterator::reference;

template <typename T1, typename T2>
using t16 = typename std::forward_list<T1, T2>::const_iterator::value_type;

template <typename T1, typename T2>
using t17 = typename std::forward_list<T1, T2>::const_iterator::difference_type;

template <typename T1, typename T2>
using t18 =
    typename std::forward_list<T1, T2>::const_iterator::iterator_category;

template <typename T1, typename T2>
using t19 = typename std::forward_list<T1, T2>::const_iterator::pointer;

template <typename T1, typename T2>
using t20 = typename std::forward_list<T1, T2>::const_iterator::reference;
