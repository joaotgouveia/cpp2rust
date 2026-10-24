#include <algorithm>
#include <cstddef>
#include <flat_map>
#include <initializer_list>
#include <iterator>
#include <ranges>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t1 = std::flat_multimap<T1, T2, T3, T4, T5>;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t2 = typename std::flat_multimap<T1, T2, T3, T4, T5>::key_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t3 = typename std::flat_multimap<T1, T2, T3, T4, T5>::mapped_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t4 = typename std::flat_multimap<T1, T2, T3, T4, T5>::value_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t5 = typename std::flat_multimap<T1, T2, T3, T4, T5>::key_compare;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t6 = typename std::flat_multimap<T1, T2, T3, T4, T5>::reference;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t7 = typename std::flat_multimap<T1, T2, T3, T4, T5>::const_reference;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t8 = typename std::flat_multimap<T1, T2, T3, T4, T5>::size_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t9 = typename std::flat_multimap<T1, T2, T3, T4, T5>::difference_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t10 = typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t11 = typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t12 = typename std::flat_multimap<T1, T2, T3, T4, T5>::reverse_iterator;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t13 =
    typename std::flat_multimap<T1, T2, T3, T4, T5>::const_reverse_iterator;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t14 = typename std::flat_multimap<T1, T2, T3, T4, T5>::key_container_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t15 =
    typename std::flat_multimap<T1, T2, T3, T4, T5>::mapped_container_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t16 = typename std::flat_multimap<T1, T2, T3, T4, T5>::value_compare;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t17 = typename std::flat_multimap<T1, T2, T3, T4, T5>::containers;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
bool f1(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.empty();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::size_t f2(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.size();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::size_t f3(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f4(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
        std::initializer_list<std::pair<T1, T2>> a1) {
  return a0.insert(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f5(std::flat_multimap<T1, T2, T3, T4, T5> &a0, std::sorted_equivalent_t a1,
        std::initializer_list<std::pair<T1, T2>> a2) {
  return a0.insert(std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::containers
f6(std::flat_multimap<T1, T2, T3, T4, T5> &&a0) {
  return std::move(a0).extract();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f7(std::flat_multimap<T1, T2, T3, T4, T5> &a0, T4 &&a1, T5 &&a2) {
  return a0.replace(std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f8(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
        std::flat_multimap<T1, T2, T3, T4, T5> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f9(std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.clear();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
T3 f10(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.key_comp();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::value_compare
f11(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.value_comp();
}
