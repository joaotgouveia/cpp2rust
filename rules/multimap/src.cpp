#include "../common/init.h"
#include "../common/traits.h"
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
using t12 = typename std::multimap<T1, T2, T3, T4>::size_type;

template <typename T1, typename T2, typename T3, typename T4>
using t13 = typename std::multimap<T1, T2, T3, T4>::difference_type;

template <typename T1, typename T2, typename T3, typename T4>
std::multimap<T1, T2, T3, T4> f1() {
  return std::multimap<T1, T2, T3, T4>();
}

template <typename T1, typename T2, typename T3, typename T4>
std::multimap<T1, T2, T3, T4>
f2(const T3 &a0,
   const typename std::multimap<T1, T2, T3, T4>::allocator_type &a1) {
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
f5(std::initializer_list<typename std::multimap<T1, T2, T3, T4>::value_type> a0,
   const T3 &a1,
   const typename std::multimap<T1, T2, T3, T4>::allocator_type &a2) {
  return std::multimap<T1, T2, T3, T4>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::multimap<T1, T2, T3, T4>
f6(const typename std::multimap<T1, T2, T3, T4>::allocator_type &a0) {
  return std::multimap<T1, T2, T3, T4>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::multimap<T1, T2, T3, T4>
f7(const std::multimap<T1, T2, T3, T4> &a0,
   const typename std::multimap<T1, T2, T3, T4>::allocator_type &a1) {
  return std::multimap<T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::multimap<T1, T2, T3, T4>
f8(std::multimap<T1, T2, T3, T4> &&a0,
   const typename std::multimap<T1, T2, T3, T4>::allocator_type &a1) {
  return std::multimap<T1, T2, T3, T4>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::multimap<T1, T2, T3, T4>
f9(std::initializer_list<typename std::multimap<T1, T2, T3, T4>::value_type> a0,
   const typename std::multimap<T1, T2, T3, T4>::allocator_type &a1) {
  return std::multimap<T1, T2, T3, T4>(a0, a1);
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
    std::initializer_list<typename std::multimap<T1, T2, T3, T4>::value_type>
        a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::multimap<T1, T2, T3, T4>::allocator_type
f13(const std::multimap<T1, T2, T3, T4> &a0) {
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
f29(std::multimap<T1, T2, T3, T4> &a0,
    const typename std::multimap<T1, T2, T3, T4>::value_type &a1) {
  return a0.insert(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::multimap<T1, T2, T3, T4>::iterator
f30(std::multimap<T1, T2, T3, T4> &a0,
    typename std::multimap<T1, T2, T3, T4>::value_type &&a1) {
  return a0.insert(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::multimap<T1, T2, T3, T4>::iterator
f31(std::multimap<T1, T2, T3, T4> &a0,
    typename std::multimap<T1, T2, T3, T4>::const_iterator a1,
    const typename std::multimap<T1, T2, T3, T4>::value_type &a2) {
  return a0.insert(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::multimap<T1, T2, T3, T4>::iterator
f32(std::multimap<T1, T2, T3, T4> &a0,
    typename std::multimap<T1, T2, T3, T4>::const_iterator a1,
    typename std::multimap<T1, T2, T3, T4>::value_type &&a2) {
  return a0.insert(a1, std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4>
void f33(
    std::multimap<T1, T2, T3, T4> &a0,
    std::initializer_list<typename std::multimap<T1, T2, T3, T4>::value_type>
        a1) {
  return a0.insert(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::multimap<T1, T2, T3, T4>::iterator
f34(std::multimap<T1, T2, T3, T4> &a0,
    typename std::multimap<T1, T2, T3, T4>::const_iterator a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::multimap<T1, T2, T3, T4>::iterator
f35(std::multimap<T1, T2, T3, T4> &a0,
    typename std::multimap<T1, T2, T3, T4>::iterator a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::multimap<T1, T2, T3, T4>::size_type
f36(std::multimap<T1, T2, T3, T4> &a0,
    const typename std::multimap<T1, T2, T3, T4>::key_type &a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::multimap<T1, T2, T3, T4>::iterator
f37(std::multimap<T1, T2, T3, T4> &a0,
    typename std::multimap<T1, T2, T3, T4>::const_iterator a1,
    typename std::multimap<T1, T2, T3, T4>::const_iterator a2) {
  return a0.erase(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
void f38(std::multimap<T1, T2, T3, T4> &a0, std::multimap<T1, T2, T3, T4> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
void f39(std::multimap<T1, T2, T3, T4> &a0) {
  return a0.clear();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::multimap<T1, T2, T3, T4>::key_compare
f40(const std::multimap<T1, T2, T3, T4> &a0) {
  return a0.key_comp();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::multimap<T1, T2, T3, T4>::value_compare
f41(const std::multimap<T1, T2, T3, T4> &a0) {
  return a0.value_comp();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::multimap<T1, T2, T3, T4>::iterator
f42(std::multimap<T1, T2, T3, T4> &a0,
    const typename std::multimap<T1, T2, T3, T4>::key_type &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::multimap<T1, T2, T3, T4>::const_iterator
f43(const std::multimap<T1, T2, T3, T4> &a0,
    const typename std::multimap<T1, T2, T3, T4>::key_type &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::multimap<T1, T2, T3, T4>::size_type
f44(const std::multimap<T1, T2, T3, T4> &a0,
    const typename std::multimap<T1, T2, T3, T4>::key_type &a1) {
  return a0.count(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::multimap<T1, T2, T3, T4>::iterator
f45(std::multimap<T1, T2, T3, T4> &a0,
    const typename std::multimap<T1, T2, T3, T4>::key_type &a1) {
  return a0.lower_bound(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::multimap<T1, T2, T3, T4>::const_iterator
f46(const std::multimap<T1, T2, T3, T4> &a0,
    const typename std::multimap<T1, T2, T3, T4>::key_type &a1) {
  return a0.lower_bound(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::multimap<T1, T2, T3, T4>::iterator
f47(std::multimap<T1, T2, T3, T4> &a0,
    const typename std::multimap<T1, T2, T3, T4>::key_type &a1) {
  return a0.upper_bound(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::multimap<T1, T2, T3, T4>::const_iterator
f48(const std::multimap<T1, T2, T3, T4> &a0,
    const typename std::multimap<T1, T2, T3, T4>::key_type &a1) {
  return a0.upper_bound(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<typename std::multimap<T1, T2, T3, T4>::iterator,
          typename std::multimap<T1, T2, T3, T4>::iterator>
f49(std::multimap<T1, T2, T3, T4> &a0,
    const typename std::multimap<T1, T2, T3, T4>::key_type &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<typename std::multimap<T1, T2, T3, T4>::const_iterator,
          typename std::multimap<T1, T2, T3, T4>::const_iterator>
f50(const std::multimap<T1, T2, T3, T4> &a0,
    const typename std::multimap<T1, T2, T3, T4>::key_type &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::multimap<T1, T2, T3, T4>
f51(T5 a0, T5 a1,
    const typename std::multimap<T1, T2, T3, T4>::allocator_type &a2) {
  return std::multimap<T1, T2, T3, T4>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::multimap<T1, T2, T3, T4> f52(T5 a0, T5 a1) {
  return std::multimap<T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::multimap<T1, T2, T3, T4>
f53(T5 a0, T5 a1, const T3 &a2,
    const typename std::multimap<T1, T2, T3, T4>::allocator_type &a3) {
  return std::multimap<T1, T2, T3, T4>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename... T5>
typename std::multimap<T1, T2, T3, T4>::iterator
f54(std::multimap<T1, T2, T3, T4> &a0, T5 &&...a1) {
  return a0.template emplace<T5...>(std::forward<T5>(a1)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename... T5>
typename std::multimap<T1, T2, T3, T4>::iterator
f55(std::multimap<T1, T2, T3, T4> &a0,
    typename std::multimap<T1, T2, T3, T4>::const_iterator a1, T5 &&...a2) {
  return a0.template emplace_hint<T5...>(a1, std::forward<T5>(a2)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::multimap<T1, T2, T3, T4>::iterator
f56(std::multimap<T1, T2, T3, T4> &a0, T5 &&a1) {
  return a0.template insert<T5>(std::forward<T5>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::multimap<T1, T2, T3, T4>::iterator
f57(std::multimap<T1, T2, T3, T4> &a0,
    typename std::multimap<T1, T2, T3, T4>::const_iterator a1, T5 &&a2) {
  return a0.template insert<T5>(a1, std::forward<T5>(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f58(std::multimap<T1, T2, T3, T4> &a0, T5 a1, T5 a2) {
  return a0.template insert<T5>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
using t14 = typename std::multimap<T1, T2, T3, T4>::iterator::value_type;

template <typename T1, typename T2, typename T3, typename T4>
using t15 = typename std::multimap<T1, T2, T3, T4>::iterator::reference;

template <typename T1, typename T2, typename T3, typename T4>
using t16 = typename std::multimap<T1, T2, T3, T4>::iterator::pointer;

template <typename T1, typename T2, typename T3, typename T4>
using t17 = typename std::multimap<T1, T2, T3, T4>::iterator::iterator_category;

template <typename T1, typename T2, typename T3, typename T4>
using t18 = typename std::multimap<T1, T2, T3, T4>::iterator::difference_type;

template <typename T1, typename T2, typename T3, typename T4>
using t19 = typename std::multimap<T1, T2, T3, T4>::const_iterator::value_type;

template <typename T1, typename T2, typename T3, typename T4>
using t20 = typename std::multimap<T1, T2, T3, T4>::const_iterator::reference;

template <typename T1, typename T2, typename T3, typename T4>
using t21 = typename std::multimap<T1, T2, T3, T4>::const_iterator::pointer;

template <typename T1, typename T2, typename T3, typename T4>
using t22 =
    typename std::multimap<T1, T2, T3, T4>::const_iterator::iterator_category;

template <typename T1, typename T2, typename T3, typename T4>
using t23 =
    typename std::multimap<T1, T2, T3, T4>::const_iterator::difference_type;
