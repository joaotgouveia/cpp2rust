#include <algorithm>
#include <initializer_list>
#include <map>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2, typename T3, typename T4>
using t1 = std::map<T1, T2, T3, T4>;

template <typename T1, typename T2, typename T3, typename T4>
using t2 = typename std::map<T1, T2, T3, T4>::key_type;

template <typename T1, typename T2, typename T3, typename T4>
using t3 = typename std::map<T1, T2, T3, T4>::mapped_type;

template <typename T1, typename T2, typename T3, typename T4>
using t4 = typename std::map<T1, T2, T3, T4>::value_type;

template <typename T1, typename T2, typename T3, typename T4>
using t5 = typename std::map<T1, T2, T3, T4>::key_compare;

template <typename T1, typename T2, typename T3, typename T4>
using t6 = typename std::map<T1, T2, T3, T4>::allocator_type;

template <typename T1, typename T2, typename T3, typename T4>
using t7 = typename std::map<T1, T2, T3, T4>::value_compare;

template <typename T1, typename T2, typename T3, typename T4>
using t8 = typename std::map<T1, T2, T3, T4>::pointer;

template <typename T1, typename T2, typename T3, typename T4>
using t9 = typename std::map<T1, T2, T3, T4>::const_pointer;

template <typename T1, typename T2, typename T3, typename T4>
using t10 = typename std::map<T1, T2, T3, T4>::reference;

template <typename T1, typename T2, typename T3, typename T4>
using t11 = typename std::map<T1, T2, T3, T4>::const_reference;

template <typename T1, typename T2, typename T3, typename T4>
using t12 = typename std::map<T1, T2, T3, T4>::size_type;

template <typename T1, typename T2, typename T3, typename T4>
using t13 = typename std::map<T1, T2, T3, T4>::difference_type;

template <typename T1, typename T2, typename T3, typename T4>
std::map<T1, T2, T3, T4> f1() {
  return std::map<T1, T2, T3, T4>();
}

template <typename T1, typename T2, typename T3, typename T4>
std::map<T1, T2, T3, T4> f2(const std::map<T1, T2, T3, T4> &a0) {
  return std::map<T1, T2, T3, T4>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::map<T1, T2, T3, T4> f3(std::map<T1, T2, T3, T4> &&a0) {
  return std::map<T1, T2, T3, T4>(std::move(a0));
}

template <typename T1, typename T2, typename T3, typename T4>
std::map<T1, T2, T3, T4>
f4(const typename std::map<T1, T2, T3, T4>::allocator_type &a0) {
  return std::map<T1, T2, T3, T4>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::map<T1, T2, T3, T4>
f5(const std::map<T1, T2, T3, T4> &a0,
   const typename std::map<T1, T2, T3, T4>::allocator_type &a1) {
  return std::map<T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::map<T1, T2, T3, T4>
f6(std::map<T1, T2, T3, T4> &&a0,
   const typename std::map<T1, T2, T3, T4>::allocator_type &a1) {
  return std::map<T1, T2, T3, T4>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::map<T1, T2, T3, T4>
f7(std::initializer_list<typename std::map<T1, T2, T3, T4>::value_type> a0,
   const typename std::map<T1, T2, T3, T4>::allocator_type &a1) {
  return std::map<T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::map<T1, T2, T3, T4> &f8(std::map<T1, T2, T3, T4> &a0,
                             const std::map<T1, T2, T3, T4> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::map<T1, T2, T3, T4> &f9(std::map<T1, T2, T3, T4> &a0,
                             std::map<T1, T2, T3, T4> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
std::map<T1, T2, T3, T4> &
f10(std::map<T1, T2, T3, T4> &a0,
    std::initializer_list<typename std::map<T1, T2, T3, T4>::value_type> a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::allocator_type
f11(const std::map<T1, T2, T3, T4> &a0) {
  return a0.get_allocator();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::iterator f12(std::map<T1, T2, T3, T4> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::const_iterator
f13(const std::map<T1, T2, T3, T4> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::iterator f14(std::map<T1, T2, T3, T4> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::const_iterator
f15(const std::map<T1, T2, T3, T4> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::reverse_iterator
f16(std::map<T1, T2, T3, T4> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::const_reverse_iterator
f17(const std::map<T1, T2, T3, T4> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::reverse_iterator
f18(std::map<T1, T2, T3, T4> &a0) {
  return a0.rend();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::const_reverse_iterator
f19(const std::map<T1, T2, T3, T4> &a0) {
  return a0.rend();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::const_iterator
f20(const std::map<T1, T2, T3, T4> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::const_iterator
f21(const std::map<T1, T2, T3, T4> &a0) {
  return a0.cend();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::const_reverse_iterator
f22(const std::map<T1, T2, T3, T4> &a0) {
  return a0.crbegin();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::const_reverse_iterator
f23(const std::map<T1, T2, T3, T4> &a0) {
  return a0.crend();
}

template <typename T1, typename T2, typename T3, typename T4>
bool f24(const std::map<T1, T2, T3, T4> &a0) {
  return a0.empty();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::size_type
f25(const std::map<T1, T2, T3, T4> &a0) {
  return a0.size();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::size_type
f26(const std::map<T1, T2, T3, T4> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::mapped_type &
f27(std::map<T1, T2, T3, T4> &a0,
    const typename std::map<T1, T2, T3, T4>::key_type &a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::mapped_type &
f28(std::map<T1, T2, T3, T4> &a0,
    typename std::map<T1, T2, T3, T4>::key_type &&a1) {
  return a0.operator[](std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::mapped_type &
f29(std::map<T1, T2, T3, T4> &a0,
    const typename std::map<T1, T2, T3, T4>::key_type &a1) {
  return a0.at(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
const typename std::map<T1, T2, T3, T4>::mapped_type &
f30(const std::map<T1, T2, T3, T4> &a0,
    const typename std::map<T1, T2, T3, T4>::key_type &a1) {
  return a0.at(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<typename std::map<T1, T2, T3, T4>::iterator, bool>
f31(std::map<T1, T2, T3, T4> &a0,
    const typename std::map<T1, T2, T3, T4>::value_type &a1) {
  return a0.insert(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<typename std::map<T1, T2, T3, T4>::iterator, bool>
f32(std::map<T1, T2, T3, T4> &a0,
    typename std::map<T1, T2, T3, T4>::value_type &&a1) {
  return a0.insert(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
void f33(
    std::map<T1, T2, T3, T4> &a0,
    std::initializer_list<typename std::map<T1, T2, T3, T4>::value_type> a1) {
  return a0.insert(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::iterator
f34(std::map<T1, T2, T3, T4> &a0,
    typename std::map<T1, T2, T3, T4>::const_iterator a1,
    const typename std::map<T1, T2, T3, T4>::value_type &a2) {
  return a0.insert(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::iterator
f35(std::map<T1, T2, T3, T4> &a0,
    typename std::map<T1, T2, T3, T4>::const_iterator a1,
    typename std::map<T1, T2, T3, T4>::value_type &&a2) {
  return a0.insert(a1, std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::iterator
f36(std::map<T1, T2, T3, T4> &a0,
    typename std::map<T1, T2, T3, T4>::const_iterator a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::iterator
f37(std::map<T1, T2, T3, T4> &a0,
    typename std::map<T1, T2, T3, T4>::iterator a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::size_type
f38(std::map<T1, T2, T3, T4> &a0,
    const typename std::map<T1, T2, T3, T4>::key_type &a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::iterator
f39(std::map<T1, T2, T3, T4> &a0,
    typename std::map<T1, T2, T3, T4>::const_iterator a1,
    typename std::map<T1, T2, T3, T4>::const_iterator a2) {
  return a0.erase(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
void f40(std::map<T1, T2, T3, T4> &a0, std::map<T1, T2, T3, T4> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
void f41(std::map<T1, T2, T3, T4> &a0) {
  return a0.clear();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::key_compare
f42(const std::map<T1, T2, T3, T4> &a0) {
  return a0.key_comp();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::value_compare
f43(const std::map<T1, T2, T3, T4> &a0) {
  return a0.value_comp();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::iterator
f44(std::map<T1, T2, T3, T4> &a0,
    const typename std::map<T1, T2, T3, T4>::key_type &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::const_iterator
f45(const std::map<T1, T2, T3, T4> &a0,
    const typename std::map<T1, T2, T3, T4>::key_type &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::size_type
f46(const std::map<T1, T2, T3, T4> &a0,
    const typename std::map<T1, T2, T3, T4>::key_type &a1) {
  return a0.count(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::iterator
f47(std::map<T1, T2, T3, T4> &a0,
    const typename std::map<T1, T2, T3, T4>::key_type &a1) {
  return a0.lower_bound(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::const_iterator
f48(const std::map<T1, T2, T3, T4> &a0,
    const typename std::map<T1, T2, T3, T4>::key_type &a1) {
  return a0.lower_bound(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::iterator
f49(std::map<T1, T2, T3, T4> &a0,
    const typename std::map<T1, T2, T3, T4>::key_type &a1) {
  return a0.upper_bound(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::const_iterator
f50(const std::map<T1, T2, T3, T4> &a0,
    const typename std::map<T1, T2, T3, T4>::key_type &a1) {
  return a0.upper_bound(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<typename std::map<T1, T2, T3, T4>::iterator,
          typename std::map<T1, T2, T3, T4>::iterator>
f51(std::map<T1, T2, T3, T4> &a0,
    const typename std::map<T1, T2, T3, T4>::key_type &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<typename std::map<T1, T2, T3, T4>::const_iterator,
          typename std::map<T1, T2, T3, T4>::const_iterator>
f52(const std::map<T1, T2, T3, T4> &a0,
    const typename std::map<T1, T2, T3, T4>::key_type &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::map<T1, T2, T3, T4>
f53(T5 a0, T5 a1, const typename std::map<T1, T2, T3, T4>::allocator_type &a2) {
  return std::map<T1, T2, T3, T4>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename... T5>
std::pair<typename std::map<T1, T2, T3, T4>::iterator, bool>
f54(std::map<T1, T2, T3, T4> &a0, T5 &&...a1) {
  return a0.template emplace<T5...>(std::forward<T5>(a1)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename... T5>
typename std::map<T1, T2, T3, T4>::iterator
f55(std::map<T1, T2, T3, T4> &a0,
    typename std::map<T1, T2, T3, T4>::const_iterator a1, T5 &&...a2) {
  return a0.template emplace_hint<T5...>(a1, std::forward<T5>(a2)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f56(std::map<T1, T2, T3, T4> &a0, T5 a1, T5 a2) {
  return a0.template insert<T5>(a1, a2);
}
