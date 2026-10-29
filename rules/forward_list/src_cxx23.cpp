#include <algorithm>
#include <forward_list>
#include <utility>

template <typename T1> using t1 = std::_Fwd_list_const_iterator<T1>;

template <typename T1>
using t2 = typename std::_Fwd_list_const_iterator<T1>::iterator;

template <typename T1>
using t3 = typename std::_Fwd_list_const_iterator<T1>::value_type;

template <typename T1>
using t4 = typename std::_Fwd_list_const_iterator<T1>::pointer;

template <typename T1>
using t5 = typename std::_Fwd_list_const_iterator<T1>::reference;

template <typename T1>
using t6 = typename std::_Fwd_list_const_iterator<T1>::difference_type;

template <typename T1>
using t7 = typename std::_Fwd_list_const_iterator<T1>::iterator_category;

template <typename T1> std::_Fwd_list_const_iterator<T1> f1() {
  return std::_Fwd_list_const_iterator<T1>();
}

template <typename T1>
std::_Fwd_list_const_iterator<T1> f2(const std::_Fwd_list_iterator<T1> &a0) {
  return std::_Fwd_list_const_iterator<T1>(a0);
}

template <typename T1>
const T1 &f3(const std::_Fwd_list_const_iterator<T1> &a0) {
  return a0.operator*();
}

template <typename T1>
const T1 *f4(const std::_Fwd_list_const_iterator<T1> &a0) {
  return a0.operator->();
}

template <typename T1>
std::_Fwd_list_const_iterator<T1> &f5(std::_Fwd_list_const_iterator<T1> &a0) {
  return a0.operator++();
}

template <typename T1>
std::_Fwd_list_const_iterator<T1> f6(std::_Fwd_list_const_iterator<T1> &a0,
                                     int a1) {
  return a0.operator++(a1);
}

template <typename T1>
bool f7(const std::_Fwd_list_const_iterator<T1> &a0,
        const std::_Fwd_list_const_iterator<T1> &a1) {
  return operator==(a0, a1);
}

template <typename T1> using t8 = std::_Fwd_list_iterator<T1>;

template <typename T1>
using t9 = typename std::_Fwd_list_iterator<T1>::value_type;

template <typename T1>
using t10 = typename std::_Fwd_list_iterator<T1>::pointer;

template <typename T1>
using t11 = typename std::_Fwd_list_iterator<T1>::reference;

template <typename T1>
using t12 = typename std::_Fwd_list_iterator<T1>::difference_type;

template <typename T1>
using t13 = typename std::_Fwd_list_iterator<T1>::iterator_category;

template <typename T1> std::_Fwd_list_iterator<T1> f8() {
  return std::_Fwd_list_iterator<T1>();
}

template <typename T1> T1 &f9(const std::_Fwd_list_iterator<T1> &a0) {
  return a0.operator*();
}

template <typename T1> T1 *f10(const std::_Fwd_list_iterator<T1> &a0) {
  return a0.operator->();
}

template <typename T1>
std::_Fwd_list_iterator<T1> &f11(std::_Fwd_list_iterator<T1> &a0) {
  return a0.operator++();
}

template <typename T1>
std::_Fwd_list_iterator<T1> f12(std::_Fwd_list_iterator<T1> &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1>
bool f13(const std::_Fwd_list_iterator<T1> &a0,
         const std::_Fwd_list_iterator<T1> &a1) {
  return operator==(a0, a1);
}

template <bool T1, typename T2> using t14 = std::__fwdlist::_Iterator<T1, T2>;

template <bool T1, typename T2>
using t15 = typename std::__fwdlist::_Iterator<T1, T2>::value_type;

template <bool T1, typename T2>
using t16 = typename std::__fwdlist::_Iterator<T1, T2>::difference_type;

template <bool T1, typename T2>
using t17 = typename std::__fwdlist::_Iterator<T1, T2>::iterator_category;

template <bool T1, typename T2>
using t18 = typename std::__fwdlist::_Iterator<T1, T2>::pointer;

template <bool T1, typename T2>
using t19 = typename std::__fwdlist::_Iterator<T1, T2>::reference;

template <bool T1, typename T2> std::__fwdlist::_Iterator<T1, T2> f14() {
  return std::__fwdlist::_Iterator<T1, T2>();
}

template <bool T1, typename T2>
std::__fwdlist::_Iterator<T1, T2>
f15(const std::__fwdlist::_Iterator<T1, T2> &a0) {
  return std::__fwdlist::_Iterator<T1, T2>(a0);
}

template <bool T1, typename T2>
std::__fwdlist::_Iterator<T1, T2> &
f16(std::__fwdlist::_Iterator<T1, T2> &a0,
    const std::__fwdlist::_Iterator<T1, T2> &a1) {
  return a0.operator=(a1);
}

template <bool T1, typename T2>
std::__fwdlist::_Iterator<T1, T2>
f17(const std::__fwdlist::_Iterator<false, T2> &a0)
  requires(T1)
{
  return std::__fwdlist::_Iterator<T1, T2>(a0);
}

template <bool T1, typename T2>
std::__fwdlist::_Iterator<T1, T2>
f18(typename std::__fwdlist::_Node_base<std::__ptr_rebind<T2, void>>::_Base_ptr
        a0) {
  return std::__fwdlist::_Iterator<T1, T2>(
      std::forward<typename std::__fwdlist::_Node_base<
          std::__ptr_rebind<T2, void>>::_Base_ptr>(a0));
}

template <bool T1, typename T2>
auto f19(const std::__fwdlist::_Iterator<T1, T2> &a0) {
  return a0.operator*();
}

template <bool T1, typename T2>
auto f20(const std::__fwdlist::_Iterator<T1, T2> &a0) {
  return a0.operator->();
}

template <bool T1, typename T2>
std::__fwdlist::_Iterator<T1, T2> &f21(std::__fwdlist::_Iterator<T1, T2> &a0) {
  return a0.operator++();
}

template <bool T1, typename T2>
std::__fwdlist::_Iterator<T1, T2> f22(std::__fwdlist::_Iterator<T1, T2> &a0,
                                      int a1) {
  return a0.operator++(a1);
}

template <bool T1, typename T2>
bool f23(const std::__fwdlist::_Iterator<T1, T2> &a0,
         const std::__fwdlist::_Iterator<T1, T2> &a1) {
  return operator==(a0, a1);
}

template <typename T1, typename T2, typename T3>
typename std::forward_list<T1, T2>::size_type f24(std::forward_list<T1, T2> &a0,
                                                  const T3 &a1) {
  return std::erase<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
typename std::forward_list<T1, T2>::size_type f25(std::forward_list<T1, T2> &a0,
                                                  T3 a1) {
  return std::erase_if<T1, T2, T3>(a0, std::move(a1));
}

template <typename T1, typename T2> using t20 = std::forward_list<T1, T2>;

template <typename T1, typename T2>
using t21 = typename std::forward_list<T1, T2>::value_type;

template <typename T1, typename T2>
using t22 = typename std::forward_list<T1, T2>::pointer;

template <typename T1, typename T2>
using t23 = typename std::forward_list<T1, T2>::const_pointer;

template <typename T1, typename T2>
using t24 = typename std::forward_list<T1, T2>::reference;

template <typename T1, typename T2>
using t25 = typename std::forward_list<T1, T2>::const_reference;

template <typename T1, typename T2>
using t26 = typename std::forward_list<T1, T2>::iterator;

template <typename T1, typename T2>
using t27 = typename std::forward_list<T1, T2>::const_iterator;

template <typename T1, typename T2>
using t28 = typename std::forward_list<T1, T2>::size_type;

template <typename T1, typename T2>
using t29 = typename std::forward_list<T1, T2>::difference_type;

template <typename T1, typename T2>
using t30 = typename std::forward_list<T1, T2>::allocator_type;

template <typename T1, typename T2> std::forward_list<T1, T2> f26() {
  return std::forward_list<T1, T2>();
}

template <typename T1, typename T2>
std::forward_list<T1, T2> f27(const T2 &a0) {
  return std::forward_list<T1, T2>(a0);
}

template <typename T1, typename T2>
std::forward_list<T1, T2> f28(const std::forward_list<T1, T2> &a0,
                              const T2 &a1) {
  return std::forward_list<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::forward_list<T1, T2> f29(std::forward_list<T1, T2> &&a0, const T2 &a1) {
  return std::forward_list<T1, T2>(std::move(a0), a1);
}

template <typename T1, typename T2>
std::forward_list<T1, T2> f30(std::size_t a0, const T2 &a1) {
  return std::forward_list<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::forward_list<T1, T2> f31(std::size_t a0, const T1 &a1, const T2 &a2) {
  return std::forward_list<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2>
std::forward_list<T1, T2> f32(const std::forward_list<T1, T2> &a0) {
  return std::forward_list<T1, T2>(a0);
}

template <typename T1, typename T2>
std::forward_list<T1, T2> f33(std::forward_list<T1, T2> &&a0) {
  return std::forward_list<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::forward_list<T1, T2> f34(std::initializer_list<T1> a0, const T2 &a1) {
  return std::forward_list<T1, T2>(std::forward<std::initializer_list<T1>>(a0),
                                   a1);
}

template <typename T1, typename T2>
std::forward_list<T1, T2> &f35(std::forward_list<T1, T2> &a0,
                               const std::forward_list<T1, T2> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
std::forward_list<T1, T2> &f36(std::forward_list<T1, T2> &a0,
                               std::forward_list<T1, T2> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
std::forward_list<T1, T2> &f37(std::forward_list<T1, T2> &a0,
                               std::initializer_list<T1> a1) {
  return a0.operator=(std::forward<std::initializer_list<T1>>(a1));
}

template <typename T1, typename T2>
void f38(std::forward_list<T1, T2> &a0, std::size_t a1, const T1 &a2) {
  return a0.assign(a1, a2);
}

template <typename T1, typename T2>
void f39(std::forward_list<T1, T2> &a0, std::initializer_list<T1> a1) {
  return a0.assign(std::forward<std::initializer_list<T1>>(a1));
}

template <typename T1, typename T2>
T2 f40(const std::forward_list<T1, T2> &a0) {
  return a0.get_allocator();
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::iterator
f41(std::forward_list<T1, T2> &a0) {
  return a0.before_begin();
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::const_iterator
f42(const std::forward_list<T1, T2> &a0) {
  return a0.before_begin();
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::iterator
f43(std::forward_list<T1, T2> &a0) {
  return a0.begin();
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::const_iterator
f44(const std::forward_list<T1, T2> &a0) {
  return a0.begin();
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::iterator
f45(std::forward_list<T1, T2> &a0) {
  return a0.end();
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::const_iterator
f46(const std::forward_list<T1, T2> &a0) {
  return a0.end();
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::const_iterator
f47(const std::forward_list<T1, T2> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::const_iterator
f48(const std::forward_list<T1, T2> &a0) {
  return a0.cbefore_begin();
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::const_iterator
f49(const std::forward_list<T1, T2> &a0) {
  return a0.cend();
}

template <typename T1, typename T2>
bool f50(const std::forward_list<T1, T2> &a0) {
  return a0.empty();
}

template <typename T1, typename T2>
std::size_t f51(const std::forward_list<T1, T2> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2> T1 &f52(std::forward_list<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
const T1 &f53(const std::forward_list<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
void f54(std::forward_list<T1, T2> &a0, const T1 &a1) {
  return a0.push_front(a1);
}

template <typename T1, typename T2>
void f55(std::forward_list<T1, T2> &a0, T1 &&a1) {
  return a0.push_front(std::move(a1));
}

template <typename T1, typename T2> void f56(std::forward_list<T1, T2> &a0) {
  return a0.pop_front();
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::iterator
f57(std::forward_list<T1, T2> &a0,
    typename std::forward_list<T1, T2>::const_iterator a1, const T1 &a2) {
  return a0.insert_after(
      std::forward<typename std::forward_list<T1, T2>::const_iterator>(a1), a2);
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::iterator
f58(std::forward_list<T1, T2> &a0,
    typename std::forward_list<T1, T2>::const_iterator a1, T1 &&a2) {
  return a0.insert_after(
      std::forward<typename std::forward_list<T1, T2>::const_iterator>(a1),
      std::move(a2));
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::iterator
f59(std::forward_list<T1, T2> &a0,
    typename std::forward_list<T1, T2>::const_iterator a1, std::size_t a2,
    const T1 &a3) {
  return a0.insert_after(
      std::forward<typename std::forward_list<T1, T2>::const_iterator>(a1), a2,
      a3);
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::iterator
f60(std::forward_list<T1, T2> &a0,
    typename std::forward_list<T1, T2>::const_iterator a1,
    std::initializer_list<T1> a2) {
  return a0.insert_after(
      std::forward<typename std::forward_list<T1, T2>::const_iterator>(a1),
      std::forward<std::initializer_list<T1>>(a2));
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::iterator
f61(std::forward_list<T1, T2> &a0,
    typename std::forward_list<T1, T2>::const_iterator a1) {
  return a0.erase_after(
      std::forward<typename std::forward_list<T1, T2>::const_iterator>(a1));
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::iterator
f62(std::forward_list<T1, T2> &a0,
    typename std::forward_list<T1, T2>::const_iterator a1,
    typename std::forward_list<T1, T2>::const_iterator a2) {
  return a0.erase_after(
      std::forward<typename std::forward_list<T1, T2>::const_iterator>(a1),
      std::forward<typename std::forward_list<T1, T2>::const_iterator>(a2));
}

template <typename T1, typename T2>
void f63(std::forward_list<T1, T2> &a0, std::forward_list<T1, T2> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2>
void f64(std::forward_list<T1, T2> &a0, std::size_t a1) {
  return a0.resize(a1);
}

template <typename T1, typename T2>
void f65(std::forward_list<T1, T2> &a0, std::size_t a1, const T1 &a2) {
  return a0.resize(a1, a2);
}

template <typename T1, typename T2> void f66(std::forward_list<T1, T2> &a0) {
  return a0.clear();
}

template <typename T1, typename T2>
void f67(std::forward_list<T1, T2> &a0,
         typename std::forward_list<T1, T2>::const_iterator a1,
         std::forward_list<T1, T2> &&a2) {
  return a0.splice_after(
      std::forward<typename std::forward_list<T1, T2>::const_iterator>(a1),
      std::move(a2));
}

template <typename T1, typename T2>
void f68(std::forward_list<T1, T2> &a0,
         typename std::forward_list<T1, T2>::const_iterator a1,
         std::forward_list<T1, T2> &a2) {
  return a0.splice_after(
      std::forward<typename std::forward_list<T1, T2>::const_iterator>(a1), a2);
}

template <typename T1, typename T2>
void f69(std::forward_list<T1, T2> &a0,
         typename std::forward_list<T1, T2>::const_iterator a1,
         std::forward_list<T1, T2> &&a2,
         typename std::forward_list<T1, T2>::const_iterator a3) {
  return a0.splice_after(
      std::forward<typename std::forward_list<T1, T2>::const_iterator>(a1),
      std::move(a2),
      std::forward<typename std::forward_list<T1, T2>::const_iterator>(a3));
}

template <typename T1, typename T2>
void f70(std::forward_list<T1, T2> &a0,
         typename std::forward_list<T1, T2>::const_iterator a1,
         std::forward_list<T1, T2> &a2,
         typename std::forward_list<T1, T2>::const_iterator a3) {
  return a0.splice_after(
      std::forward<typename std::forward_list<T1, T2>::const_iterator>(a1), a2,
      std::forward<typename std::forward_list<T1, T2>::const_iterator>(a3));
}

template <typename T1, typename T2>
void f71(std::forward_list<T1, T2> &a0,
         typename std::forward_list<T1, T2>::const_iterator a1,
         std::forward_list<T1, T2> &&a2,
         typename std::forward_list<T1, T2>::const_iterator a3,
         typename std::forward_list<T1, T2>::const_iterator a4) {
  return a0.splice_after(
      std::forward<typename std::forward_list<T1, T2>::const_iterator>(a1),
      std::move(a2),
      std::forward<typename std::forward_list<T1, T2>::const_iterator>(a3),
      std::forward<typename std::forward_list<T1, T2>::const_iterator>(a4));
}

template <typename T1, typename T2>
void f72(std::forward_list<T1, T2> &a0,
         typename std::forward_list<T1, T2>::const_iterator a1,
         std::forward_list<T1, T2> &a2,
         typename std::forward_list<T1, T2>::const_iterator a3,
         typename std::forward_list<T1, T2>::const_iterator a4) {
  return a0.splice_after(
      std::forward<typename std::forward_list<T1, T2>::const_iterator>(a1), a2,
      std::forward<typename std::forward_list<T1, T2>::const_iterator>(a3),
      std::forward<typename std::forward_list<T1, T2>::const_iterator>(a4));
}

template <typename T1, typename T2>
std::size_t f73(std::forward_list<T1, T2> &a0, const T1 &a1) {
  return a0.remove(a1);
}

template <typename T1, typename T2>
std::size_t f74(std::forward_list<T1, T2> &a0) {
  return a0.unique();
}

template <typename T1, typename T2>
void f75(std::forward_list<T1, T2> &a0, std::forward_list<T1, T2> &&a1) {
  return a0.merge(std::move(a1));
}

template <typename T1, typename T2>
void f76(std::forward_list<T1, T2> &a0, std::forward_list<T1, T2> &a1) {
  return a0.merge(a1);
}

template <typename T1, typename T2> void f77(std::forward_list<T1, T2> &a0) {
  return a0.sort();
}

template <typename T1, typename T2> void f78(std::forward_list<T1, T2> &a0) {
  return a0.reverse();
}

template <typename T1, typename T2, typename T3,
          typename T4 = typename std::enable_if<
              std::is_convertible<
                  typename std::iterator_traits<T3>::iterator_category,
                  std::input_iterator_tag>::value,
              void>::type>
std::forward_list<T1, T2> f79(T3 a0, T3 a1, const T2 &a2) {
  return std::forward_list<T1, T2>(std::move(a0), std::move(a1), a2);
}

template <typename T1, typename T2, typename T3>
std::forward_list<T1, T2> f80(std::from_range_t a0, T3 &&a1, const T2 &a2) {
  return std::forward_list<T1, T2>(std::move(a0), std::forward<T3>(a1), a2);
}

template <typename T1, typename T2, typename T3,
          typename T4 = typename std::enable_if<
              std::is_convertible<
                  typename std::iterator_traits<T3>::iterator_category,
                  std::input_iterator_tag>::value,
              void>::type>
void f81(std::forward_list<T1, T2> &a0, T3 a1, T3 a2) {
  return a0.template assign<T3>(std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3>
void f82(std::forward_list<T1, T2> &a0, T3 &&a1) {
  return a0.template assign_range<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename... T3>
T1 &f83(std::forward_list<T1, T2> &a0, T3 &&...a1) {
  return a0.template emplace_front<T3...>(std::forward<T3>(a1)...);
}

template <typename T1, typename T2, typename T3>
void f84(std::forward_list<T1, T2> &a0, T3 &&a1) {
  return a0.template prepend_range<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename... T3>
typename std::forward_list<T1, T2>::iterator
f85(std::forward_list<T1, T2> &a0,
    typename std::forward_list<T1, T2>::const_iterator a1, T3 &&...a2) {
  return a0.template emplace_after<T3...>(
      std::forward<typename std::forward_list<T1, T2>::const_iterator>(a1),
      std::forward<T3>(a2)...);
}

template <typename T1, typename T2, typename T3,
          typename T4 = typename std::enable_if<
              std::is_convertible<
                  typename std::iterator_traits<T3>::iterator_category,
                  std::input_iterator_tag>::value,
              void>::type>
typename std::forward_list<T1, T2>::iterator
f86(std::forward_list<T1, T2> &a0,
    typename std::forward_list<T1, T2>::const_iterator a1, T3 a2, T3 a3) {
  return a0.template insert_after<T3>(
      std::forward<typename std::forward_list<T1, T2>::const_iterator>(a1),
      std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3>
typename std::forward_list<T1, T2>::iterator
f87(std::forward_list<T1, T2> &a0,
    typename std::forward_list<T1, T2>::const_iterator a1, T3 &&a2) {
  return a0.template insert_range_after<T3>(
      std::forward<typename std::forward_list<T1, T2>::const_iterator>(a1),
      std::forward<T3>(a2));
}

template <typename T1, typename T2, typename T3>
std::size_t f88(std::forward_list<T1, T2> &a0, T3 a1) {
  return a0.template remove_if<T3>(std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::size_t f89(std::forward_list<T1, T2> &a0, T3 a1) {
  return a0.template unique<T3>(std::move(a1));
}

template <typename T1, typename T2, typename T3>
void f90(std::forward_list<T1, T2> &a0, std::forward_list<T1, T2> &&a1, T3 a2) {
  return a0.template merge<T3>(std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3>
void f91(std::forward_list<T1, T2> &a0, std::forward_list<T1, T2> &a1, T3 a2) {
  return a0.template merge<T3>(a1, std::move(a2));
}

template <typename T1, typename T2, typename T3>
void f92(std::forward_list<T1, T2> &a0, T3 a1) {
  return a0.template sort<T3>(std::move(a1));
}

template <typename T1, typename T2>
auto f93(const std::forward_list<T1, T2> &a0,
         const std::forward_list<T1, T2> &a1) {
  return std::operator<=> <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f94(const std::forward_list<T1, T2> &a0,
         const std::forward_list<T1, T2> &a1) {
  return std::operator== <T1, T2>(a0, a1);
}

template <typename T1> using t31 = std::pmr::forward_list<T1>;

template <typename T1, typename T2>
void f95(std::forward_list<T1, T2> &a0, std::forward_list<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}
