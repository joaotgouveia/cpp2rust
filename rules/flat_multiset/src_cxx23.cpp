#include <algorithm>
#include <flat_map>
#include <flat_set>
#include <initializer_list>
#include <ranges>
#include <utility>

template <typename T1, typename T2, typename T3>
using t1 = std::flat_multiset<T1, T2, T3>;

template <typename T1, typename T2, typename T3>
using t2 = typename std::flat_multiset<T1, T2, T3>::key_type;

template <typename T1, typename T2, typename T3>
using t3 = typename std::flat_multiset<T1, T2, T3>::value_type;

template <typename T1, typename T2, typename T3>
using t4 = typename std::flat_multiset<T1, T2, T3>::key_compare;

template <typename T1, typename T2, typename T3>
using t5 = typename std::flat_multiset<T1, T2, T3>::reference;

template <typename T1, typename T2, typename T3>
using t6 = typename std::flat_multiset<T1, T2, T3>::const_reference;

template <typename T1, typename T2, typename T3>
using t7 = typename std::flat_multiset<T1, T2, T3>::size_type;

template <typename T1, typename T2, typename T3>
using t8 = typename std::flat_multiset<T1, T2, T3>::difference_type;

template <typename T1, typename T2, typename T3>
using t9 = typename std::flat_multiset<T1, T2, T3>::iterator;

template <typename T1, typename T2, typename T3>
using t10 = typename std::flat_multiset<T1, T2, T3>::const_iterator;

template <typename T1, typename T2, typename T3>
using t11 = typename std::flat_multiset<T1, T2, T3>::reverse_iterator;

template <typename T1, typename T2, typename T3>
using t12 = typename std::flat_multiset<T1, T2, T3>::const_reverse_iterator;

template <typename T1, typename T2, typename T3>
using t13 = typename std::flat_multiset<T1, T2, T3>::container_type;

template <typename T1, typename T2, typename T3>
using t14 = typename std::flat_multiset<T1, T2, T3>::value_compare;

template <typename T1, typename T2, typename T3>
typename std::flat_multiset<T1, T2, T3>::const_iterator
f1(const std::flat_multiset<T1, T2, T3> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3>
typename std::flat_multiset<T1, T2, T3>::const_iterator
f2(const std::flat_multiset<T1, T2, T3> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3>
typename std::flat_multiset<T1, T2, T3>::const_reverse_iterator
f3(const std::flat_multiset<T1, T2, T3> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2, typename T3>
typename std::flat_multiset<T1, T2, T3>::const_reverse_iterator
f4(const std::flat_multiset<T1, T2, T3> &a0) {
  return a0.rend();
}

template <typename T1, typename T2, typename T3>
typename std::flat_multiset<T1, T2, T3>::const_iterator
f5(const std::flat_multiset<T1, T2, T3> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2, typename T3>
typename std::flat_multiset<T1, T2, T3>::const_iterator
f6(const std::flat_multiset<T1, T2, T3> &a0) {
  return a0.cend();
}

template <typename T1, typename T2, typename T3>
typename std::flat_multiset<T1, T2, T3>::const_reverse_iterator
f7(const std::flat_multiset<T1, T2, T3> &a0) {
  return a0.crbegin();
}

template <typename T1, typename T2, typename T3>
typename std::flat_multiset<T1, T2, T3>::const_reverse_iterator
f8(const std::flat_multiset<T1, T2, T3> &a0) {
  return a0.crend();
}

template <typename T1, typename T2, typename T3>
bool f9(const std::flat_multiset<T1, T2, T3> &a0) {
  return a0.empty();
}

template <typename T1, typename T2, typename T3>
typename std::flat_multiset<T1, T2, T3>::size_type
f10(const std::flat_multiset<T1, T2, T3> &a0) {
  return a0.size();
}

template <typename T1, typename T2, typename T3>
typename std::flat_multiset<T1, T2, T3>::size_type
f11(const std::flat_multiset<T1, T2, T3> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2, typename T3>
typename std::flat_multiset<T1, T2, T3>::iterator
f12(std::flat_multiset<T1, T2, T3> &a0,
    typename std::flat_multiset<T1, T2, T3>::const_iterator a1, T1 &&a2) {
  return a0.insert(a1, std::move(a2));
}

template <typename T1, typename T2, typename T3>
void f13(std::flat_multiset<T1, T2, T3> &a0, std::initializer_list<T1> a1) {
  return a0.insert(a1);
}

template <typename T1, typename T2, typename T3>
void f14(std::flat_multiset<T1, T2, T3> &a0, std::sorted_equivalent_t a1,
         std::initializer_list<T1> a2) {
  return a0.insert(std::move(a1), a2);
}

template <typename T1, typename T2, typename T3>
T3 f15(std::flat_multiset<T1, T2, T3> &&a0) {
  return std::move(a0).extract();
}

template <typename T1, typename T2, typename T3>
void f16(std::flat_multiset<T1, T2, T3> &a0, T3 &&a1) {
  return a0.replace(std::move(a1));
}

template <typename T1, typename T2, typename T3>
typename std::flat_multiset<T1, T2, T3>::iterator
f17(std::flat_multiset<T1, T2, T3> &a0,
    typename std::flat_multiset<T1, T2, T3>::const_iterator a1,
    typename std::flat_multiset<T1, T2, T3>::const_iterator a2) {
  return a0.erase(a1, a2);
}

template <typename T1, typename T2, typename T3>
void f18(std::flat_multiset<T1, T2, T3> &a0,
         std::flat_multiset<T1, T2, T3> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2, typename T3>
void f19(std::flat_multiset<T1, T2, T3> &a0) {
  return a0.clear();
}

template <typename T1, typename T2, typename T3>
T2 f20(const std::flat_multiset<T1, T2, T3> &a0) {
  return a0.key_comp();
}

template <typename T1, typename T2, typename T3>
T2 f21(const std::flat_multiset<T1, T2, T3> &a0) {
  return a0.value_comp();
}
