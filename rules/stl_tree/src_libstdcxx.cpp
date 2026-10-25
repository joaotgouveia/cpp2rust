#include <map>
#include <utility>

template <typename T1> using t1 = std::_Rb_tree_const_iterator<T1>;

template <typename T1>
using t2 = typename std::_Rb_tree_const_iterator<T1>::value_type;

template <typename T1>
using t3 = typename std::_Rb_tree_const_iterator<T1>::reference;

template <typename T1>
using t4 = typename std::_Rb_tree_const_iterator<T1>::pointer;

template <typename T1>
using t5 = typename std::_Rb_tree_const_iterator<T1>::iterator;

template <typename T1>
using t6 = typename std::_Rb_tree_const_iterator<T1>::iterator_category;

template <typename T1>
using t7 = typename std::_Rb_tree_const_iterator<T1>::difference_type;

template <typename T1> std::_Rb_tree_const_iterator<T1> f1() {
  return std::_Rb_tree_const_iterator<T1>();
}

template <typename T1>
std::_Rb_tree_const_iterator<T1> f2(const std::_Rb_tree_iterator<T1> &a0) {
  return std::_Rb_tree_const_iterator<T1>(a0);
}

template <typename T1>
const T1 &f3(const std::_Rb_tree_const_iterator<T1> &a0) {
  return a0.operator*();
}

template <typename T1>
const T1 *f4(const std::_Rb_tree_const_iterator<T1> &a0) {
  return a0.operator->();
}

template <typename T1>
std::_Rb_tree_const_iterator<T1> &f5(std::_Rb_tree_const_iterator<T1> &a0) {
  return a0.operator++();
}

template <typename T1>
std::_Rb_tree_const_iterator<T1> f6(std::_Rb_tree_const_iterator<T1> &a0,
                                    int a1) {
  return a0.operator++(a1);
}

template <typename T1>
std::_Rb_tree_const_iterator<T1> &f7(std::_Rb_tree_const_iterator<T1> &a0) {
  return a0.operator--();
}

template <typename T1>
std::_Rb_tree_const_iterator<T1> f8(std::_Rb_tree_const_iterator<T1> &a0,
                                    int a1) {
  return a0.operator--(a1);
}

template <typename T1>
bool f9(const std::_Rb_tree_const_iterator<T1> &a0,
        const std::_Rb_tree_const_iterator<T1> &a1) {
  return operator==(a0, a1);
}

template <typename T1>
using t8 = typename std::_Rb_tree_const_iterator<T1>::iterator::value_type;

template <typename T1>
using t9 = typename std::_Rb_tree_const_iterator<T1>::iterator::reference;

template <typename T1>
using t10 = typename std::_Rb_tree_const_iterator<T1>::iterator::pointer;

template <typename T1>
using t11 =
    typename std::_Rb_tree_const_iterator<T1>::iterator::iterator_category;

template <typename T1>
using t12 =
    typename std::_Rb_tree_const_iterator<T1>::iterator::difference_type;

template <typename T1>
T1 &f10(const typename std::_Rb_tree_const_iterator<T1>::iterator &a0) {
  return a0.operator*();
}

template <typename T1>
T1 *f11(const typename std::_Rb_tree_const_iterator<T1>::iterator &a0) {
  return a0.operator->();
}

template <typename T1> using t13 = std::_Rb_tree_iterator<T1>;

template <typename T1>
using t14 = typename std::_Rb_tree_iterator<T1>::value_type;

template <typename T1>
using t15 = typename std::_Rb_tree_iterator<T1>::reference;

template <typename T1> using t16 = typename std::_Rb_tree_iterator<T1>::pointer;

template <typename T1>
using t17 = typename std::_Rb_tree_iterator<T1>::iterator_category;

template <typename T1>
using t18 = typename std::_Rb_tree_iterator<T1>::difference_type;

template <typename T1> std::_Rb_tree_iterator<T1> f12() {
  return std::_Rb_tree_iterator<T1>();
}

template <typename T1> T1 &f13(const std::_Rb_tree_iterator<T1> &a0) {
  return a0.operator*();
}

template <typename T1> T1 *f14(const std::_Rb_tree_iterator<T1> &a0) {
  return a0.operator->();
}

template <typename T1>
std::_Rb_tree_iterator<T1> &f15(std::_Rb_tree_iterator<T1> &a0) {
  return a0.operator++();
}

template <typename T1>
std::_Rb_tree_iterator<T1> f16(std::_Rb_tree_iterator<T1> &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1>
std::_Rb_tree_iterator<T1> &f17(std::_Rb_tree_iterator<T1> &a0) {
  return a0.operator--();
}

template <typename T1>
std::_Rb_tree_iterator<T1> f18(std::_Rb_tree_iterator<T1> &a0, int a1) {
  return a0.operator--(a1);
}

template <typename T1>
bool f19(const std::_Rb_tree_iterator<T1> &a0,
         const std::_Rb_tree_iterator<T1> &a1) {
  return operator==(a0, a1);
}

template <bool T1, typename T2> using t19 = std::__rb_tree::_Iterator<T1, T2>;

template <bool T1, typename T2>
using t20 = typename std::__rb_tree::_Iterator<T1, T2>::value_type;

template <bool T1, typename T2>
using t21 = typename std::__rb_tree::_Iterator<T1, T2>::reference;

template <bool T1, typename T2>
using t22 = typename std::__rb_tree::_Iterator<T1, T2>::pointer;

template <bool T1, typename T2>
using t23 = typename std::__rb_tree::_Iterator<T1, T2>::iterator_category;

template <bool T1, typename T2>
using t24 = typename std::__rb_tree::_Iterator<T1, T2>::difference_type;

template <bool T1, typename T2> std::__rb_tree::_Iterator<T1, T2> f20() {
  return std::__rb_tree::_Iterator<T1, T2>();
}

template <bool T1, typename T2>
std::__rb_tree::_Iterator<T1, T2>
f21(typename std::__rb_tree::_Node_base<std::__ptr_rebind<T2, void>>::_Base_ptr
        a0) {
  return std::__rb_tree::_Iterator<T1, T2>(std::move(a0));
}

template <bool T1, typename T2>
std::__rb_tree::_Iterator<T1, T2>
f22(const std::__rb_tree::_Iterator<T1, T2> &a0) {
  return std::__rb_tree::_Iterator<T1, T2>(a0);
}

template <bool T1, typename T2>
std::__rb_tree::_Iterator<T1, T2> &
f23(std::__rb_tree::_Iterator<T1, T2> &a0,
    const std::__rb_tree::_Iterator<T1, T2> &a1) {
  return a0.operator=(a1);
}

template <bool T1, typename T2>
auto f24(const std::__rb_tree::_Iterator<T1, T2> &a0)
    -> decltype(a0.operator*()) {
  return a0.operator*();
}

template <bool T1, typename T2>
auto f25(const std::__rb_tree::_Iterator<T1, T2> &a0)
    -> decltype(a0.operator->()) {
  return a0.operator->();
}

template <bool T1, typename T2>
std::__rb_tree::_Iterator<T1, T2> &f26(std::__rb_tree::_Iterator<T1, T2> &a0) {
  return a0.operator++();
}

template <bool T1, typename T2>
std::__rb_tree::_Iterator<T1, T2> f27(std::__rb_tree::_Iterator<T1, T2> &a0,
                                      int a1) {
  return a0.operator++(a1);
}

template <bool T1, typename T2>
std::__rb_tree::_Iterator<T1, T2> &f28(std::__rb_tree::_Iterator<T1, T2> &a0) {
  return a0.operator--();
}

template <bool T1, typename T2>
std::__rb_tree::_Iterator<T1, T2> f29(std::__rb_tree::_Iterator<T1, T2> &a0,
                                      int a1) {
  return a0.operator--(a1);
}

template <bool T1, typename T2>
bool f30(const std::__rb_tree::_Iterator<T1, T2> &a0,
         const std::__rb_tree::_Iterator<T1, T2> &a1) {
  return operator==(a0, a1);
}
