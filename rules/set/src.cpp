#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <initializer_list>
#include <set>
#include <utility>

template <typename T1, typename T2, typename T3>
using t1 = std::set<T1, T2, T3>;

template <typename T1, typename T2, typename T3>
using t2 = typename std::set<T1, T2, T3>::key_type;

template <typename T1, typename T2, typename T3>
using t3 = typename std::set<T1, T2, T3>::value_type;

template <typename T1, typename T2, typename T3>
using t4 = typename std::set<T1, T2, T3>::key_compare;

template <typename T1, typename T2, typename T3>
using t5 = typename std::set<T1, T2, T3>::value_compare;

template <typename T1, typename T2, typename T3>
using t6 = typename std::set<T1, T2, T3>::allocator_type;

template <typename T1, typename T2, typename T3>
using t7 = typename std::set<T1, T2, T3>::pointer;

template <typename T1, typename T2, typename T3>
using t8 = typename std::set<T1, T2, T3>::const_pointer;

template <typename T1, typename T2, typename T3>
using t9 = typename std::set<T1, T2, T3>::reference;

template <typename T1, typename T2, typename T3>
using t10 = typename std::set<T1, T2, T3>::const_reference;

template <typename T1, typename T2, typename T3>
using t11 = typename std::set<T1, T2, T3>::size_type;

template <typename T1, typename T2, typename T3>
using t12 = typename std::set<T1, T2, T3>::difference_type;

template <typename T1, typename T2, typename T3> std::set<T1, T2, T3> f1() {
  return std::set<T1, T2, T3>();
}

template <typename T1, typename T2, typename T3>
std::set<T1, T2, T3>
f2(const T2 &a0, const typename std::set<T1, T2, T3>::allocator_type &a1) {
  return std::set<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::set<T1, T2, T3> f3(const std::set<T1, T2, T3> &a0) {
  return std::set<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::set<T1, T2, T3> f4(std::set<T1, T2, T3> &&a0) {
  return std::set<T1, T2, T3>(std::move(a0));
}

template <typename T1, typename T2, typename T3>
std::set<T1, T2, T3>
f5(std::initializer_list<typename std::set<T1, T2, T3>::value_type> a0,
   const T2 &a1, const typename std::set<T1, T2, T3>::allocator_type &a2) {
  return std::set<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
std::set<T1, T2, T3>
f6(const typename std::set<T1, T2, T3>::allocator_type &a0) {
  return std::set<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::set<T1, T2, T3>
f7(const std::set<T1, T2, T3> &a0,
   const typename std::set<T1, T2, T3>::allocator_type &a1) {
  return std::set<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::set<T1, T2, T3>
f8(std::set<T1, T2, T3> &&a0,
   const typename std::set<T1, T2, T3>::allocator_type &a1) {
  return std::set<T1, T2, T3>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3>
std::set<T1, T2, T3>
f9(std::initializer_list<typename std::set<T1, T2, T3>::value_type> a0,
   const typename std::set<T1, T2, T3>::allocator_type &a1) {
  return std::set<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::set<T1, T2, T3> &f10(std::set<T1, T2, T3> &a0,
                          const std::set<T1, T2, T3> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3>
std::set<T1, T2, T3> &f11(std::set<T1, T2, T3> &a0, std::set<T1, T2, T3> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::set<T1, T2, T3> &
f12(std::set<T1, T2, T3> &a0,
    std::initializer_list<typename std::set<T1, T2, T3>::value_type> a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::key_compare f13(const std::set<T1, T2, T3> &a0) {
  return a0.key_comp();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::value_compare
f14(const std::set<T1, T2, T3> &a0) {
  return a0.value_comp();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::allocator_type
f15(const std::set<T1, T2, T3> &a0) {
  return a0.get_allocator();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::iterator f16(const std::set<T1, T2, T3> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::iterator f17(const std::set<T1, T2, T3> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::reverse_iterator
f18(const std::set<T1, T2, T3> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::reverse_iterator
f19(const std::set<T1, T2, T3> &a0) {
  return a0.rend();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::iterator f20(const std::set<T1, T2, T3> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::iterator f21(const std::set<T1, T2, T3> &a0) {
  return a0.cend();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::reverse_iterator
f22(const std::set<T1, T2, T3> &a0) {
  return a0.crbegin();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::reverse_iterator
f23(const std::set<T1, T2, T3> &a0) {
  return a0.crend();
}

template <typename T1, typename T2, typename T3>
bool f24(const std::set<T1, T2, T3> &a0) {
  return a0.empty();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::size_type f25(const std::set<T1, T2, T3> &a0) {
  return a0.size();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::size_type f26(const std::set<T1, T2, T3> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2, typename T3>
void f27(std::set<T1, T2, T3> &a0, std::set<T1, T2, T3> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2, typename T3>
std::pair<typename std::set<T1, T2, T3>::iterator, bool>
f28(std::set<T1, T2, T3> &a0,
    const typename std::set<T1, T2, T3>::value_type &a1) {
  return a0.insert(a1);
}

template <typename T1, typename T2, typename T3>
std::pair<typename std::set<T1, T2, T3>::iterator, bool>
f29(std::set<T1, T2, T3> &a0, typename std::set<T1, T2, T3>::value_type &&a1) {
  return a0.insert(std::move(a1));
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::iterator
f30(std::set<T1, T2, T3> &a0, typename std::set<T1, T2, T3>::const_iterator a1,
    const typename std::set<T1, T2, T3>::value_type &a2) {
  return a0.insert(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::iterator
f31(std::set<T1, T2, T3> &a0, typename std::set<T1, T2, T3>::const_iterator a1,
    typename std::set<T1, T2, T3>::value_type &&a2) {
  return a0.insert(a1, std::move(a2));
}

template <typename T1, typename T2, typename T3>
void f32(std::set<T1, T2, T3> &a0,
         std::initializer_list<typename std::set<T1, T2, T3>::value_type> a1) {
  return a0.insert(a1);
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::iterator
f33(std::set<T1, T2, T3> &a0,
    typename std::set<T1, T2, T3>::const_iterator a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::size_type
f34(std::set<T1, T2, T3> &a0,
    const typename std::set<T1, T2, T3>::key_type &a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::iterator
f35(std::set<T1, T2, T3> &a0, typename std::set<T1, T2, T3>::const_iterator a1,
    typename std::set<T1, T2, T3>::const_iterator a2) {
  return a0.erase(a1, a2);
}

template <typename T1, typename T2, typename T3>
void f36(std::set<T1, T2, T3> &a0) {
  return a0.clear();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::size_type
f37(const std::set<T1, T2, T3> &a0,
    const typename std::set<T1, T2, T3>::key_type &a1) {
  return a0.count(a1);
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::iterator
f38(std::set<T1, T2, T3> &a0,
    const typename std::set<T1, T2, T3>::key_type &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::const_iterator
f39(const std::set<T1, T2, T3> &a0,
    const typename std::set<T1, T2, T3>::key_type &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::iterator
f40(std::set<T1, T2, T3> &a0,
    const typename std::set<T1, T2, T3>::key_type &a1) {
  return a0.lower_bound(a1);
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::const_iterator
f41(const std::set<T1, T2, T3> &a0,
    const typename std::set<T1, T2, T3>::key_type &a1) {
  return a0.lower_bound(a1);
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::iterator
f42(std::set<T1, T2, T3> &a0,
    const typename std::set<T1, T2, T3>::key_type &a1) {
  return a0.upper_bound(a1);
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::const_iterator
f43(const std::set<T1, T2, T3> &a0,
    const typename std::set<T1, T2, T3>::key_type &a1) {
  return a0.upper_bound(a1);
}

template <typename T1, typename T2, typename T3>
std::pair<typename std::set<T1, T2, T3>::iterator,
          typename std::set<T1, T2, T3>::iterator>
f44(std::set<T1, T2, T3> &a0,
    const typename std::set<T1, T2, T3>::key_type &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3>
std::pair<typename std::set<T1, T2, T3>::const_iterator,
          typename std::set<T1, T2, T3>::const_iterator>
f45(const std::set<T1, T2, T3> &a0,
    const typename std::set<T1, T2, T3>::key_type &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::set<T1, T2, T3> f46(T4 a0, T4 a1) {
  return std::set<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::set<T1, T2, T3>
f47(T4 a0, T4 a1, const T2 &a2,
    const typename std::set<T1, T2, T3>::allocator_type &a3) {
  return std::set<T1, T2, T3>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
std::set<T1, T2, T3>
f48(T4 a0, T4 a1, const typename std::set<T1, T2, T3>::allocator_type &a2) {
  return std::set<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename... T4>
std::pair<typename std::set<T1, T2, T3>::iterator, bool>
f49(std::set<T1, T2, T3> &a0, T4 &&...a1) {
  return a0.template emplace<T4...>(std::forward<T4>(a1)...);
}

template <typename T1, typename T2, typename T3, typename... T4>
typename std::set<T1, T2, T3>::iterator
f50(std::set<T1, T2, T3> &a0, typename std::set<T1, T2, T3>::const_iterator a1,
    T4 &&...a2) {
  return a0.template emplace_hint<T4...>(a1, std::forward<T4>(a2)...);
}

template <typename T1, typename T2, typename T3, typename T4>
void f51(std::set<T1, T2, T3> &a0, T4 a1, T4 a2) {
  return a0.template insert<T4>(a1, a2);
}

template <typename T1, typename T2, typename T3>
using t13 = typename std::set<T1, T2, T3>::iterator::value_type;

template <typename T1, typename T2, typename T3>
using t14 = typename std::set<T1, T2, T3>::iterator::reference;

template <typename T1, typename T2, typename T3>
using t15 = typename std::set<T1, T2, T3>::iterator::pointer;

template <typename T1, typename T2, typename T3>
using t16 = typename std::set<T1, T2, T3>::iterator::iterator_category;

template <typename T1, typename T2, typename T3>
using t17 = typename std::set<T1, T2, T3>::iterator::difference_type;

template <typename T1, typename T2, typename T3>
using t18 = typename std::set<T1, T2, T3>::const_iterator::value_type;

template <typename T1, typename T2, typename T3>
using t19 = typename std::set<T1, T2, T3>::const_iterator::reference;

template <typename T1, typename T2, typename T3>
using t20 = typename std::set<T1, T2, T3>::const_iterator::pointer;

template <typename T1, typename T2, typename T3>
using t21 = typename std::set<T1, T2, T3>::const_iterator::iterator_category;

template <typename T1, typename T2, typename T3>
using t22 = typename std::set<T1, T2, T3>::const_iterator::difference_type;
