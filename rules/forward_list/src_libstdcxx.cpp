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

template <typename T1>
using t20 = typename std::_Fwd_list_const_iterator<T1>::iterator::value_type;

template <typename T1>
using t21 = typename std::_Fwd_list_const_iterator<T1>::iterator::pointer;

template <typename T1>
using t22 = typename std::_Fwd_list_const_iterator<T1>::iterator::reference;

template <typename T1>
using t23 =
    typename std::_Fwd_list_const_iterator<T1>::iterator::difference_type;

template <typename T1>
using t24 =
    typename std::_Fwd_list_const_iterator<T1>::iterator::iterator_category;

template <typename T1>
T1 &f72(const typename std::_Fwd_list_const_iterator<T1>::iterator &a0) {
  return a0.operator*();
}

template <typename T1>
T1 *f73(const typename std::_Fwd_list_const_iterator<T1>::iterator &a0) {
  return a0.operator->();
}

template <typename T1> using t25 = std::_Fwd_list_iterator<T1>;

template <typename T1>
using t26 = typename std::_Fwd_list_iterator<T1>::value_type;

template <typename T1>
using t27 = typename std::_Fwd_list_iterator<T1>::pointer;

template <typename T1>
using t28 = typename std::_Fwd_list_iterator<T1>::reference;

template <typename T1>
using t29 = typename std::_Fwd_list_iterator<T1>::difference_type;

template <typename T1>
using t30 = typename std::_Fwd_list_iterator<T1>::iterator_category;

template <typename T1> std::_Fwd_list_iterator<T1> f74() {
  return std::_Fwd_list_iterator<T1>();
}

template <typename T1> T1 &f75(const std::_Fwd_list_iterator<T1> &a0) {
  return a0.operator*();
}

template <typename T1> T1 *f76(const std::_Fwd_list_iterator<T1> &a0) {
  return a0.operator->();
}

template <typename T1>
std::_Fwd_list_iterator<T1> &f77(std::_Fwd_list_iterator<T1> &a0) {
  return a0.operator++();
}

template <typename T1>
std::_Fwd_list_iterator<T1> f78(std::_Fwd_list_iterator<T1> &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1>
bool f79(const std::_Fwd_list_iterator<T1> &a0,
         const std::_Fwd_list_iterator<T1> &a1) {
  return operator==(a0, a1);
}

template <bool T1, typename T2> using t31 = std::__fwdlist::_Iterator<T1, T2>;

template <bool T1, typename T2>
using t32 = typename std::__fwdlist::_Iterator<T1, T2>::value_type;

template <bool T1, typename T2>
using t33 = typename std::__fwdlist::_Iterator<T1, T2>::difference_type;

template <bool T1, typename T2>
using t34 = typename std::__fwdlist::_Iterator<T1, T2>::iterator_category;

template <bool T1, typename T2>
using t35 = typename std::__fwdlist::_Iterator<T1, T2>::pointer;

template <bool T1, typename T2>
using t36 = typename std::__fwdlist::_Iterator<T1, T2>::reference;

template <bool T1, typename T2> std::__fwdlist::_Iterator<T1, T2> f80() {
  return std::__fwdlist::_Iterator<T1, T2>();
}

template <bool T1, typename T2>
std::__fwdlist::_Iterator<T1, T2>
f81(const std::__fwdlist::_Iterator<T1, T2> &a0) {
  return std::__fwdlist::_Iterator<T1, T2>(a0);
}

template <bool T1, typename T2>
std::__fwdlist::_Iterator<T1, T2> &
f82(std::__fwdlist::_Iterator<T1, T2> &a0,
    const std::__fwdlist::_Iterator<T1, T2> &a1) {
  return a0.operator=(a1);
}

template <bool T1, typename T2>
std::__fwdlist::_Iterator<T1, T2>
f83(typename std::__fwdlist::_Node_base<std::__ptr_rebind<T2, void>>::_Base_ptr
        a0) {
  return std::__fwdlist::_Iterator<T1, T2>(std::move(a0));
}

template <bool T1, typename T2>
auto f84(const std::__fwdlist::_Iterator<T1, T2> &a0)
    -> decltype(a0.operator*()) {
  return a0.operator*();
}

template <bool T1, typename T2>
auto f85(const std::__fwdlist::_Iterator<T1, T2> &a0)
    -> decltype(a0.operator->()) {
  return a0.operator->();
}

template <bool T1, typename T2>
std::__fwdlist::_Iterator<T1, T2> &f86(std::__fwdlist::_Iterator<T1, T2> &a0) {
  return a0.operator++();
}

template <bool T1, typename T2>
std::__fwdlist::_Iterator<T1, T2> f87(std::__fwdlist::_Iterator<T1, T2> &a0,
                                      int a1) {
  return a0.operator++(a1);
}

template <bool T1, typename T2>
bool f88(const std::__fwdlist::_Iterator<T1, T2> &a0,
         const std::__fwdlist::_Iterator<T1, T2> &a1) {
  return operator==(a0, a1);
}

template <typename T1, typename T2>
std::forward_list<T1, T2> f89(std::size_t a0, const T2 &a1) {
  return std::forward_list<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::forward_list<T1, T2> f90(std::size_t a0, const T1 &a1, const T2 &a2) {
  return std::forward_list<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2>
void f91(std::forward_list<T1, T2> &a0, std::size_t a1, const T1 &a2) {
  return a0.assign(a1, a2);
}

template <typename T1, typename T2>
std::size_t f92(const std::forward_list<T1, T2> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::iterator
f93(std::forward_list<T1, T2> &a0,
    typename std::forward_list<T1, T2>::const_iterator a1, std::size_t a2,
    const T1 &a3) {
  return a0.insert_after(std::move(a1), a2, a3);
}

template <typename T1, typename T2>
void f94(std::forward_list<T1, T2> &a0, std::size_t a1) {
  return a0.resize(a1);
}

template <typename T1, typename T2>
void f95(std::forward_list<T1, T2> &a0, std::size_t a1, const T1 &a2) {
  return a0.resize(a1, a2);
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::_RequireInputIter<T3>>
std::forward_list<T1, T2> f96(T3 a0, T3 a1, const T2 &a2) {
  return std::forward_list<T1, T2>(std::move(a0), std::move(a1), a2);
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::_RequireInputIter<T3>>
void f97(std::forward_list<T1, T2> &a0, T3 a1, T3 a2) {
  return a0.template assign<T3>(std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::_RequireInputIter<T3>>
typename std::forward_list<T1, T2>::iterator
f98(std::forward_list<T1, T2> &a0,
    typename std::forward_list<T1, T2>::const_iterator a1, T3 a2, T3 a3) {
  return a0.template insert_after<T3>(std::move(a1), std::move(a2),
                                      std::move(a3));
}

template <typename T1, typename T2>
using t37 = typename std::forward_list<T1, T2>::iterator::value_type;

template <typename T1, typename T2>
using t38 = typename std::forward_list<T1, T2>::iterator::difference_type;

template <typename T1, typename T2>
using t39 = typename std::forward_list<T1, T2>::iterator::iterator_category;

template <typename T1, typename T2>
using t40 = typename std::forward_list<T1, T2>::iterator::pointer;

template <typename T1, typename T2>
using t41 = typename std::forward_list<T1, T2>::iterator::reference;

template <typename T1, typename T2>
using t42 = typename std::forward_list<T1, T2>::const_iterator::value_type;

template <typename T1, typename T2>
using t43 = typename std::forward_list<T1, T2>::const_iterator::difference_type;

template <typename T1, typename T2>
using t44 =
    typename std::forward_list<T1, T2>::const_iterator::iterator_category;

template <typename T1, typename T2>
using t45 = typename std::forward_list<T1, T2>::const_iterator::pointer;

template <typename T1, typename T2>
using t46 = typename std::forward_list<T1, T2>::const_iterator::reference;
