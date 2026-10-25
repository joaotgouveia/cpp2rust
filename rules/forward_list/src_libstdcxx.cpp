#include <algorithm>
#include <forward_list>
#include <utility>

template <typename T1> using t13 = std::_Fwd_list_const_iterator<T1>;

template <typename T1>
using t14 = typename std::_Fwd_list_const_iterator<T1>::iterator;

template <typename T1>
using t15 = typename std::_Fwd_list_const_iterator<T1>::value_type;

template <typename T1>
using t16 = typename std::_Fwd_list_const_iterator<T1>::pointer;

template <typename T1>
using t17 = typename std::_Fwd_list_const_iterator<T1>::reference;

template <typename T1>
using t18 = typename std::_Fwd_list_const_iterator<T1>::difference_type;

template <typename T1>
using t19 = typename std::_Fwd_list_const_iterator<T1>::iterator_category;

template <typename T1> std::_Fwd_list_const_iterator<T1> f65() {
  return std::_Fwd_list_const_iterator<T1>();
}

template <typename T1>
std::_Fwd_list_const_iterator<T1> f66(const std::_Fwd_list_iterator<T1> &a0) {
  return std::_Fwd_list_const_iterator<T1>(a0);
}

template <typename T1>
const T1 &f67(const std::_Fwd_list_const_iterator<T1> &a0) {
  return a0.operator*();
}

template <typename T1>
const T1 *f68(const std::_Fwd_list_const_iterator<T1> &a0) {
  return a0.operator->();
}

template <typename T1>
std::_Fwd_list_const_iterator<T1> &f69(std::_Fwd_list_const_iterator<T1> &a0) {
  return a0.operator++();
}

template <typename T1>
std::_Fwd_list_const_iterator<T1> f70(std::_Fwd_list_const_iterator<T1> &a0,
                                      int a1) {
  return a0.operator++(a1);
}

template <typename T1>
bool f71(const std::_Fwd_list_const_iterator<T1> &a0,
         const std::_Fwd_list_const_iterator<T1> &a1) {
  return operator==(a0, a1);
}

template <typename T1> using t20 = std::_Fwd_list_iterator<T1>;

template <typename T1>
using t21 = typename std::_Fwd_list_iterator<T1>::value_type;

template <typename T1>
using t22 = typename std::_Fwd_list_iterator<T1>::pointer;

template <typename T1>
using t23 = typename std::_Fwd_list_iterator<T1>::reference;

template <typename T1>
using t24 = typename std::_Fwd_list_iterator<T1>::difference_type;

template <typename T1>
using t25 = typename std::_Fwd_list_iterator<T1>::iterator_category;

template <typename T1> std::_Fwd_list_iterator<T1> f72() {
  return std::_Fwd_list_iterator<T1>();
}

template <typename T1> T1 &f73(const std::_Fwd_list_iterator<T1> &a0) {
  return a0.operator*();
}

template <typename T1> T1 *f74(const std::_Fwd_list_iterator<T1> &a0) {
  return a0.operator->();
}

template <typename T1>
std::_Fwd_list_iterator<T1> &f75(std::_Fwd_list_iterator<T1> &a0) {
  return a0.operator++();
}

template <typename T1>
std::_Fwd_list_iterator<T1> f76(std::_Fwd_list_iterator<T1> &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1>
bool f77(const std::_Fwd_list_iterator<T1> &a0,
         const std::_Fwd_list_iterator<T1> &a1) {
  return operator==(a0, a1);
}

template <bool T1, typename T2> using t26 = std::__fwdlist::_Iterator<T1, T2>;

template <bool T1, typename T2>
using t27 = typename std::__fwdlist::_Iterator<T1, T2>::value_type;

template <bool T1, typename T2>
using t28 = typename std::__fwdlist::_Iterator<T1, T2>::difference_type;

template <bool T1, typename T2>
using t29 = typename std::__fwdlist::_Iterator<T1, T2>::iterator_category;

template <bool T1, typename T2>
using t30 = typename std::__fwdlist::_Iterator<T1, T2>::pointer;

template <bool T1, typename T2>
using t31 = typename std::__fwdlist::_Iterator<T1, T2>::reference;

template <bool T1, typename T2> std::__fwdlist::_Iterator<T1, T2> f78() {
  return std::__fwdlist::_Iterator<T1, T2>();
}

template <bool T1, typename T2>
std::__fwdlist::_Iterator<T1, T2>
f79(const std::__fwdlist::_Iterator<T1, T2> &a0) {
  return std::__fwdlist::_Iterator<T1, T2>(a0);
}

template <bool T1, typename T2>
std::__fwdlist::_Iterator<T1, T2> &
f80(std::__fwdlist::_Iterator<T1, T2> &a0,
    const std::__fwdlist::_Iterator<T1, T2> &a1) {
  return a0.operator=(a1);
}

template <bool T1, typename T2>
std::__fwdlist::_Iterator<T1, T2>
f81(typename std::__fwdlist::_Node_base<std::__ptr_rebind<T2, void>>::_Base_ptr
        a0) {
  return std::__fwdlist::_Iterator<T1, T2>(
      std::forward<typename std::__fwdlist::_Node_base<
          std::__ptr_rebind<T2, void>>::_Base_ptr>(a0));
}

template <bool T1, typename T2>
auto f82(const std::__fwdlist::_Iterator<T1, T2> &a0)
    -> decltype(a0.operator*()) {
  return a0.operator*();
}

template <bool T1, typename T2>
auto f83(const std::__fwdlist::_Iterator<T1, T2> &a0)
    -> decltype(a0.operator->()) {
  return a0.operator->();
}

template <bool T1, typename T2>
std::__fwdlist::_Iterator<T1, T2> &f84(std::__fwdlist::_Iterator<T1, T2> &a0) {
  return a0.operator++();
}

template <bool T1, typename T2>
std::__fwdlist::_Iterator<T1, T2> f85(std::__fwdlist::_Iterator<T1, T2> &a0,
                                      int a1) {
  return a0.operator++(a1);
}

template <bool T1, typename T2>
bool f86(const std::__fwdlist::_Iterator<T1, T2> &a0,
         const std::__fwdlist::_Iterator<T1, T2> &a1) {
  return operator==(a0, a1);
}

template <typename T1, typename T2>
std::forward_list<T1, T2> f87(std::size_t a0, const T2 &a1) {
  return std::forward_list<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::forward_list<T1, T2> f88(std::size_t a0, const T1 &a1, const T2 &a2) {
  return std::forward_list<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2>
void f89(std::forward_list<T1, T2> &a0, std::size_t a1, const T1 &a2) {
  return a0.assign(a1, a2);
}

template <typename T1, typename T2>
std::size_t f90(const std::forward_list<T1, T2> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::iterator
f91(std::forward_list<T1, T2> &a0,
    typename std::forward_list<T1, T2>::const_iterator a1, std::size_t a2,
    const T1 &a3) {
  return a0.insert_after(
      std::forward<typename std::forward_list<T1, T2>::const_iterator>(a1), a2,
      a3);
}

template <typename T1, typename T2>
void f92(std::forward_list<T1, T2> &a0, std::size_t a1) {
  return a0.resize(a1);
}

template <typename T1, typename T2>
void f93(std::forward_list<T1, T2> &a0, std::size_t a1, const T1 &a2) {
  return a0.resize(a1, a2);
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::_RequireInputIter<T3>>
std::forward_list<T1, T2> f94(T3 a0, T3 a1, const T2 &a2) {
  return std::forward_list<T1, T2>(std::move(a0), std::move(a1), a2);
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::_RequireInputIter<T3>>
void f95(std::forward_list<T1, T2> &a0, T3 a1, T3 a2) {
  return a0.template assign<T3>(std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::_RequireInputIter<T3>>
typename std::forward_list<T1, T2>::iterator
f96(std::forward_list<T1, T2> &a0,
    typename std::forward_list<T1, T2>::const_iterator a1, T3 a2, T3 a3) {
  return a0.template insert_after<T3>(
      std::forward<typename std::forward_list<T1, T2>::const_iterator>(a1),
      std::move(a2), std::move(a3));
}
