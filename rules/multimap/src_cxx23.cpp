#include <algorithm>
#include <initializer_list>
#include <map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
using t1 = std::multimap<T1, T2, T3, T4>;

template <typename T1, typename T2, typename T3, typename T4>
using t2 = typename std::multimap<T1, T2, T3, T4>::key_type;

template <typename T1, typename T2, typename T3, typename T4>
using t3 = typename std::multimap<T1, T2, T3, T4>::mapped_type;

template <typename T1, typename T2, typename T3, typename T4>
using t4 = typename std::multimap<T1, T2, T3, T4>::value_type;

template <typename T1, typename T2, typename T3, typename T4>
using t5 = typename std::multimap<T1, T2, T3, T4>::key_compare;

template <typename T1, typename T2, typename T3, typename T4>
using t6 = typename std::multimap<T1, T2, T3, T4>::allocator_type;

template <typename T1, typename T2, typename T3, typename T4>
using t7 = typename std::multimap<T1, T2, T3, T4>::value_compare;

template <typename T1, typename T2, typename T3, typename T4>
using t8 = typename std::multimap<T1, T2, T3, T4>::pointer;

template <typename T1, typename T2, typename T3, typename T4>
using t9 = typename std::multimap<T1, T2, T3, T4>::const_pointer;

template <typename T1, typename T2, typename T3, typename T4>
using t10 = typename std::multimap<T1, T2, T3, T4>::reference;

template <typename T1, typename T2, typename T3, typename T4>
using t11 = typename std::multimap<T1, T2, T3, T4>::const_reference;

template <typename T1, typename T2, typename T3, typename T4>
using t12 = typename std::multimap<T1, T2, T3, T4>::iterator;

template <typename T1, typename T2, typename T3, typename T4>
using t13 = typename std::multimap<T1, T2, T3, T4>::const_iterator;

template <typename T1, typename T2, typename T3, typename T4>
using t14 = typename std::multimap<T1, T2, T3, T4>::size_type;

template <typename T1, typename T2, typename T3, typename T4>
using t15 = typename std::multimap<T1, T2, T3, T4>::difference_type;

template <typename T1, typename T2, typename T3, typename T4>
using t16 = typename std::multimap<T1, T2, T3, T4>::reverse_iterator;

template <typename T1, typename T2, typename T3, typename T4>
using t17 = typename std::multimap<T1, T2, T3, T4>::const_reverse_iterator;

template <typename T1, typename T2, typename T3, typename T4>
using t18 = typename std::multimap<T1, T2, T3, T4>::node_type;

template <typename T1, typename T2, typename T3, typename T4>
std::multimap<T1, T2, T3, T4> f1() {
  return std::multimap<T1, T2, T3, T4>();
}

template <typename T1, typename T2, typename T3, typename T4>
std::multimap<T1, T2, T3, T4> f2(const T3 &a0, const T4 &a1) {
  return std::multimap<T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::multimap<T1, T2, T3, T4> f3(const std::multimap<T1, T2, T3, T4> &a0) {
  return std::multimap<T1, T2, T3, T4>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::multimap<T1, T2, T3, T4> f4(std::multimap<T1, T2, T3, T4> &&a0) {
  return std::multimap<T1, T2, T3, T4>(std::move(a0));
}

template <typename T1, typename T2, typename T3, typename T4>
std::multimap<T1, T2, T3, T4>
f5(std::initializer_list<std::pair<const T1, T2>> a0, const T3 &a1,
   const T4 &a2) {
  return std::multimap<T1, T2, T3, T4>(
      std::forward<std::initializer_list<std::pair<const T1, T2>>>(a0), a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::multimap<T1, T2, T3, T4> f6(const T4 &a0) {
  return std::multimap<T1, T2, T3, T4>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::multimap<T1, T2, T3, T4> f7(const std::multimap<T1, T2, T3, T4> &a0,
                                 const T4 &a1) {
  return std::multimap<T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::multimap<T1, T2, T3, T4> f8(std::multimap<T1, T2, T3, T4> &&a0,
                                 const T4 &a1) {
  return std::multimap<T1, T2, T3, T4>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::multimap<T1, T2, T3, T4>
f9(std::initializer_list<std::pair<const T1, T2>> a0, const T4 &a1) {
  return std::multimap<T1, T2, T3, T4>(
      std::forward<std::initializer_list<std::pair<const T1, T2>>>(a0), a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::multimap<T1, T2, T3, T4> &f10(std::multimap<T1, T2, T3, T4> &a0,
                                   const std::multimap<T1, T2, T3, T4> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::multimap<T1, T2, T3, T4> &f11(std::multimap<T1, T2, T3, T4> &a0,
                                   std::multimap<T1, T2, T3, T4> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
std::multimap<T1, T2, T3, T4> &
f12(std::multimap<T1, T2, T3, T4> &a0,
    std::initializer_list<std::pair<const T1, T2>> a1) {
  return a0.operator=(
      std::forward<std::initializer_list<std::pair<const T1, T2>>>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
T4 f13(const std::multimap<T1, T2, T3, T4> &a0) {
  return a0.get_allocator();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::multimap<T1, T2, T3, T4>::iterator
f14(std::multimap<T1, T2, T3, T4> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::multimap<T1, T2, T3, T4>::const_iterator
f15(const std::multimap<T1, T2, T3, T4> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::multimap<T1, T2, T3, T4>::iterator
f16(std::multimap<T1, T2, T3, T4> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::multimap<T1, T2, T3, T4>::const_iterator
f17(const std::multimap<T1, T2, T3, T4> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::multimap<T1, T2, T3, T4>::reverse_iterator
f18(std::multimap<T1, T2, T3, T4> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::multimap<T1, T2, T3, T4>::const_reverse_iterator
f19(const std::multimap<T1, T2, T3, T4> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::multimap<T1, T2, T3, T4>::reverse_iterator
f20(std::multimap<T1, T2, T3, T4> &a0) {
  return a0.rend();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::multimap<T1, T2, T3, T4>::const_reverse_iterator
f21(const std::multimap<T1, T2, T3, T4> &a0) {
  return a0.rend();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::multimap<T1, T2, T3, T4>::const_iterator
f22(const std::multimap<T1, T2, T3, T4> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::multimap<T1, T2, T3, T4>::const_iterator
f23(const std::multimap<T1, T2, T3, T4> &a0) {
  return a0.cend();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::multimap<T1, T2, T3, T4>::const_reverse_iterator
f24(const std::multimap<T1, T2, T3, T4> &a0) {
  return a0.crbegin();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::multimap<T1, T2, T3, T4>::const_reverse_iterator
f25(const std::multimap<T1, T2, T3, T4> &a0) {
  return a0.crend();
}

template <typename T1, typename T2, typename T3, typename T4>
bool f26(const std::multimap<T1, T2, T3, T4> &a0) {
  return a0.empty();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::multimap<T1, T2, T3, T4>::size_type
f27(const std::multimap<T1, T2, T3, T4> &a0) {
  return a0.size();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::multimap<T1, T2, T3, T4>::size_type
f28(const std::multimap<T1, T2, T3, T4> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::multimap<T1, T2, T3, T4>::iterator
f29(std::multimap<T1, T2, T3, T4> &a0, const std::pair<const T1, T2> &a1) {
  return a0.insert(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::multimap<T1, T2, T3, T4>::iterator
f30(std::multimap<T1, T2, T3, T4> &a0, std::pair<const T1, T2> &&a1) {
  return a0.insert(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::multimap<T1, T2, T3, T4>::iterator
f31(std::multimap<T1, T2, T3, T4> &a0,
    typename std::multimap<T1, T2, T3, T4>::const_iterator a1,
    const std::pair<const T1, T2> &a2) {
  return a0.insert(
      std::forward<typename std::multimap<T1, T2, T3, T4>::const_iterator>(a1),
      a2);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::multimap<T1, T2, T3, T4>::iterator
f32(std::multimap<T1, T2, T3, T4> &a0,
    typename std::multimap<T1, T2, T3, T4>::const_iterator a1,
    std::pair<const T1, T2> &&a2) {
  return a0.insert(
      std::forward<typename std::multimap<T1, T2, T3, T4>::const_iterator>(a1),
      std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4>
void f33(std::multimap<T1, T2, T3, T4> &a0,
         std::initializer_list<std::pair<const T1, T2>> a1) {
  return a0.insert(
      std::forward<std::initializer_list<std::pair<const T1, T2>>>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::multimap<T1, T2, T3, T4>::node_type
f34(std::multimap<T1, T2, T3, T4> &a0,
    typename std::multimap<T1, T2, T3, T4>::const_iterator a1) {
  return a0.extract(
      std::forward<typename std::multimap<T1, T2, T3, T4>::const_iterator>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::multimap<T1, T2, T3, T4>::node_type
f35(std::multimap<T1, T2, T3, T4> &a0, const T1 &a1) {
  return a0.extract(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::multimap<T1, T2, T3, T4>::iterator
f36(std::multimap<T1, T2, T3, T4> &a0,
    typename std::multimap<T1, T2, T3, T4>::node_type &&a1) {
  return a0.insert(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::multimap<T1, T2, T3, T4>::iterator
f37(std::multimap<T1, T2, T3, T4> &a0,
    typename std::multimap<T1, T2, T3, T4>::const_iterator a1,
    typename std::multimap<T1, T2, T3, T4>::node_type &&a2) {
  return a0.insert(
      std::forward<typename std::multimap<T1, T2, T3, T4>::const_iterator>(a1),
      std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::multimap<T1, T2, T3, T4>::iterator
f38(std::multimap<T1, T2, T3, T4> &a0,
    typename std::multimap<T1, T2, T3, T4>::const_iterator a1) {
  return a0.erase(
      std::forward<typename std::multimap<T1, T2, T3, T4>::const_iterator>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::multimap<T1, T2, T3, T4>::iterator
f39(std::multimap<T1, T2, T3, T4> &a0,
    typename std::multimap<T1, T2, T3, T4>::iterator a1) {
  return a0.erase(
      std::forward<typename std::multimap<T1, T2, T3, T4>::iterator>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::multimap<T1, T2, T3, T4>::size_type
f40(std::multimap<T1, T2, T3, T4> &a0, const T1 &a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::multimap<T1, T2, T3, T4>::iterator
f41(std::multimap<T1, T2, T3, T4> &a0,
    typename std::multimap<T1, T2, T3, T4>::const_iterator a1,
    typename std::multimap<T1, T2, T3, T4>::const_iterator a2) {
  return a0.erase(
      std::forward<typename std::multimap<T1, T2, T3, T4>::const_iterator>(a1),
      std::forward<typename std::multimap<T1, T2, T3, T4>::const_iterator>(a2));
}

template <typename T1, typename T2, typename T3, typename T4>
void f42(std::multimap<T1, T2, T3, T4> &a0, std::multimap<T1, T2, T3, T4> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
void f43(std::multimap<T1, T2, T3, T4> &a0) {
  return a0.clear();
}

template <typename T1, typename T2, typename T3, typename T4>
T3 f44(const std::multimap<T1, T2, T3, T4> &a0) {
  return a0.key_comp();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::multimap<T1, T2, T3, T4>::value_compare
f45(const std::multimap<T1, T2, T3, T4> &a0) {
  return a0.value_comp();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::multimap<T1, T2, T3, T4>::iterator
f46(std::multimap<T1, T2, T3, T4> &a0, const T1 &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::multimap<T1, T2, T3, T4>::const_iterator
f47(const std::multimap<T1, T2, T3, T4> &a0, const T1 &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::multimap<T1, T2, T3, T4>::size_type
f48(const std::multimap<T1, T2, T3, T4> &a0, const T1 &a1) {
  return a0.count(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f49(const std::multimap<T1, T2, T3, T4> &a0, const T1 &a1) {
  return a0.contains(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::multimap<T1, T2, T3, T4>::iterator
f50(std::multimap<T1, T2, T3, T4> &a0, const T1 &a1) {
  return a0.lower_bound(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::multimap<T1, T2, T3, T4>::const_iterator
f51(const std::multimap<T1, T2, T3, T4> &a0, const T1 &a1) {
  return a0.lower_bound(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::multimap<T1, T2, T3, T4>::iterator
f52(std::multimap<T1, T2, T3, T4> &a0, const T1 &a1) {
  return a0.upper_bound(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::multimap<T1, T2, T3, T4>::const_iterator
f53(const std::multimap<T1, T2, T3, T4> &a0, const T1 &a1) {
  return a0.upper_bound(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<typename std::multimap<T1, T2, T3, T4>::iterator,
          typename std::multimap<T1, T2, T3, T4>::iterator>
f54(std::multimap<T1, T2, T3, T4> &a0, const T1 &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<typename std::multimap<T1, T2, T3, T4>::const_iterator,
          typename std::multimap<T1, T2, T3, T4>::const_iterator>
f55(const std::multimap<T1, T2, T3, T4> &a0, const T1 &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::multimap<T1, T2, T3, T4> f56(T5 a0, T5 a1, const T4 &a2) {
  return std::multimap<T1, T2, T3, T4>(std::move(a0), std::move(a1), a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::multimap<T1, T2, T3, T4> f57(T5 a0, T5 a1) {
  return std::multimap<T1, T2, T3, T4>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::multimap<T1, T2, T3, T4> f58(T5 a0, T5 a1, const T3 &a2, const T4 &a3) {
  return std::multimap<T1, T2, T3, T4>(std::move(a0), std::move(a1), a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::multimap<T1, T2, T3, T4> f59(std::from_range_t a0, T5 &&a1, const T3 &a2,
                                  const T4 &a3) {
  return std::multimap<T1, T2, T3, T4>(std::move(a0), std::forward<T5>(a1), a2,
                                       a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::multimap<T1, T2, T3, T4> f60(std::from_range_t a0, T5 &&a1, const T4 &a2) {
  return std::multimap<T1, T2, T3, T4>(std::move(a0), std::forward<T5>(a1), a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename... T5>
typename std::multimap<T1, T2, T3, T4>::iterator
f61(std::multimap<T1, T2, T3, T4> &a0, T5 &&...a1) {
  return a0.template emplace<T5...>(std::forward<T5>(a1)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename... T5>
typename std::multimap<T1, T2, T3, T4>::iterator
f62(std::multimap<T1, T2, T3, T4> &a0,
    typename std::multimap<T1, T2, T3, T4>::const_iterator a1, T5 &&...a2) {
  return a0.template emplace_hint<T5...>(
      std::forward<typename std::multimap<T1, T2, T3, T4>::const_iterator>(a1),
      std::forward<T5>(a2)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::__enable_if_t<std::is_constructible<std::pair<const T1, T2>, T5>::value,
                   typename std::_Rb_tree<
                       T1, std::pair<const T1, T2>,
                       std::_Select1st<std::pair<const T1, T2>>, T3,
                       typename __gnu_cxx::__alloc_traits<T4>::template rebind<
                           std::pair<const T1, T2>>::other>::iterator>
f63(std::multimap<T1, T2, T3, T4> &a0, T5 &&a1)
  requires(
      !(std::is_same<T5, const typename std::multimap<
                             T1, T2, T3, T4>::value_type &>::value) &&
      !(std::is_same<
          T5, typename std::multimap<T1, T2, T3, T4>::value_type>::value) &&
      !(std::is_same<T5, std::initializer_list<typename std::multimap<
                             T1, T2, T3, T4>::value_type>>::value) &&
      !(std::is_same<T5,
                     typename std::multimap<T1, T2, T3, T4>::node_type>::value))
{
  return a0.template insert<T5>(std::forward<T5>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::__enable_if_t<std::is_constructible<std::pair<const T1, T2>, T5 &&>::value,
                   typename std::_Rb_tree<
                       T1, std::pair<const T1, T2>,
                       std::_Select1st<std::pair<const T1, T2>>, T3,
                       typename __gnu_cxx::__alloc_traits<T4>::template rebind<
                           std::pair<const T1, T2>>::other>::iterator>
f64(std::multimap<T1, T2, T3, T4> &a0,
    typename std::multimap<T1, T2, T3, T4>::const_iterator a1, T5 &&a2)
  requires(
      !(std::is_same<T5, const typename std::multimap<
                             T1, T2, T3, T4>::value_type &>::value) &&
      !(std::is_same<
          T5, typename std::multimap<T1, T2, T3, T4>::value_type>::value) &&
      !(std::is_same<T5,
                     typename std::multimap<T1, T2, T3, T4>::node_type>::value))
{
  return a0.template insert<T5>(
      std::forward<typename std::multimap<T1, T2, T3, T4>::const_iterator>(a1),
      std::forward<T5>(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f65(std::multimap<T1, T2, T3, T4> &a0, T5 a1, T5 a2) {
  return a0.template insert<T5>(std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f66(std::multimap<T1, T2, T3, T4> &a0, T5 &&a1) {
  return a0.template insert_range<T5>(std::forward<T5>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f67(std::multimap<T1, T2, T3, T4> &a0, std::multimap<T1, T2, T5, T4> &a1) {
  return a0.template merge<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f68(std::multimap<T1, T2, T3, T4> &a0,
         std::multimap<T1, T2, T5, T4> &&a1) {
  return a0.template merge<T5>(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f69(std::multimap<T1, T2, T3, T4> &a0, std::map<T1, T2, T5, T4> &a1) {
  return a0.template merge<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f70(std::multimap<T1, T2, T3, T4> &a0, std::map<T1, T2, T5, T4> &&a1) {
  return a0.template merge<T5>(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f71(std::multimap<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(!(std::is_same<
             T5, typename std::multimap<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template find<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f72(const std::multimap<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(!(std::is_same<
             T5, typename std::multimap<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template find<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f73(const std::multimap<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(!(std::is_same<
             T5, typename std::multimap<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template count<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f74(const std::multimap<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(!(std::is_same<
             T5, typename std::multimap<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template contains<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f75(std::multimap<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(!(std::is_same<
             T5, typename std::multimap<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template lower_bound<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f76(const std::multimap<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(!(std::is_same<
             T5, typename std::multimap<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template lower_bound<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f77(std::multimap<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(!(std::is_same<
             T5, typename std::multimap<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template upper_bound<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f78(const std::multimap<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(!(std::is_same<
             T5, typename std::multimap<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template upper_bound<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f79(std::multimap<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(!(std::is_same<
             T5, typename std::multimap<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template equal_range<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f80(const std::multimap<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(!(std::is_same<
             T5, typename std::multimap<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template equal_range<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
using t19 = typename std::multimap<T1, T2, T3, T4>::iterator::value_type;

template <typename T1, typename T2, typename T3, typename T4>
using t20 = typename std::multimap<T1, T2, T3, T4>::iterator::reference;

template <typename T1, typename T2, typename T3, typename T4>
using t21 = typename std::multimap<T1, T2, T3, T4>::iterator::pointer;

template <typename T1, typename T2, typename T3, typename T4>
using t22 = typename std::multimap<T1, T2, T3, T4>::iterator::iterator_category;

template <typename T1, typename T2, typename T3, typename T4>
using t23 = typename std::multimap<T1, T2, T3, T4>::iterator::difference_type;

template <typename T1, typename T2, typename T3, typename T4>
using t24 = typename std::multimap<T1, T2, T3, T4>::const_iterator::value_type;

template <typename T1, typename T2, typename T3, typename T4>
using t25 = typename std::multimap<T1, T2, T3, T4>::const_iterator::reference;

template <typename T1, typename T2, typename T3, typename T4>
using t26 = typename std::multimap<T1, T2, T3, T4>::const_iterator::pointer;

template <typename T1, typename T2, typename T3, typename T4>
using t27 =
    typename std::multimap<T1, T2, T3, T4>::const_iterator::iterator_category;

template <typename T1, typename T2, typename T3, typename T4>
using t28 =
    typename std::multimap<T1, T2, T3, T4>::const_iterator::difference_type;

template <typename T1, typename T2, typename T3, typename T4>
using t29 = typename std::multimap<T1, T2, T3, T4>::node_type::key_type;

template <typename T1, typename T2, typename T3, typename T4>
using t30 = typename std::multimap<T1, T2, T3, T4>::node_type::mapped_type;

template <typename T1, typename T2, typename T3, typename T4>
bool f81(const typename std::multimap<T1, T2, T3, T4>::value_compare &a0,
         const std::pair<const T1, T2> &a1, const std::pair<const T1, T2> &a2) {
  return a0.operator()(a1, a2);
}
