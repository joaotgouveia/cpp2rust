#include <algorithm>
#include <flat_map>
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
typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator
f1(std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator
f2(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator
f3(std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator
f4(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::reverse_iterator
f5(std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::const_reverse_iterator
f6(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::reverse_iterator
f7(std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.rend();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::const_reverse_iterator
f8(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.rend();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator
f9(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator
f10(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.cend();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::const_reverse_iterator
f11(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.crbegin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::const_reverse_iterator
f12(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.crend();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
bool f13(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.empty();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::size_t f14(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.size();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::size_t f15(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f16(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
         const std::pair<T1, T2> &a1) {
  return a0.insert(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f17(std::flat_multimap<T1, T2, T3, T4, T5> &a0, std::pair<T1, T2> &&a1) {
  return a0.insert(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator
f18(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
    typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator a1,
    const std::pair<T1, T2> &a2) {
  return a0.insert(
      std::forward<
          typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator>(a1),
      a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator
f19(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
    typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator a1,
    std::pair<T1, T2> &&a2) {
  return a0.insert(
      std::forward<
          typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator>(a1),
      std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f20(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
         std::initializer_list<std::pair<T1, T2>> a1) {
  return a0.insert(std::forward<std::initializer_list<std::pair<T1, T2>>>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f21(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
         std::sorted_equivalent_t a1,
         std::initializer_list<std::pair<T1, T2>> a2) {
  return a0.insert(std::move(a1),
                   std::forward<std::initializer_list<std::pair<T1, T2>>>(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::containers
f22(std::flat_multimap<T1, T2, T3, T4, T5> &&a0) {
  return std::move(a0).extract();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f23(std::flat_multimap<T1, T2, T3, T4, T5> &a0, T4 &&a1, T5 &&a2) {
  return a0.replace(std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator
f24(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
    typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator a1) {
  return a0.erase(
      std::forward<typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator>(
          a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator
f25(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
    typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator a1) {
  return a0.erase(
      std::forward<
          typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::size_t f26(std::flat_multimap<T1, T2, T3, T4, T5> &a0, T1 const &a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator
f27(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
    typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator a1,
    typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator a2) {
  return a0.erase(
      std::forward<
          typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator>(a1),
      std::forward<
          typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator>(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f28(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
         std::flat_multimap<T1, T2, T3, T4, T5> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f29(std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.clear();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
T3 f30(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.key_comp();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::value_compare
f31(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.value_comp();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
T4 const &f32(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.keys();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
T5 const &f33(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.values();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator
f34(std::flat_multimap<T1, T2, T3, T4, T5> &a0, T1 const &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator
f35(const std::flat_multimap<T1, T2, T3, T4, T5> &a0, T1 const &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::size_t f36(const std::flat_multimap<T1, T2, T3, T4, T5> &a0,
                T1 const &a1) {
  return a0.count(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
bool f37(const std::flat_multimap<T1, T2, T3, T4, T5> &a0, T1 const &a1) {
  return a0.contains(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator
f38(std::flat_multimap<T1, T2, T3, T4, T5> &a0, T1 const &a1) {
  return a0.lower_bound(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator
f39(const std::flat_multimap<T1, T2, T3, T4, T5> &a0, T1 const &a1) {
  return a0.lower_bound(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator
f40(std::flat_multimap<T1, T2, T3, T4, T5> &a0, T1 const &a1) {
  return a0.upper_bound(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator
f41(const std::flat_multimap<T1, T2, T3, T4, T5> &a0, T1 const &a1) {
  return a0.upper_bound(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::pair<typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator,
          typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator>
f42(std::flat_multimap<T1, T2, T3, T4, T5> &a0, T1 const &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::pair<typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator,
          typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator>
f43(const std::flat_multimap<T1, T2, T3, T4, T5> &a0, T1 const &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
auto f44(std::flat_multimap<T1, T2, T3, T4, T5> &a0, T6 &&...a1) {
  return a0.template emplace<T6...>(std::forward<T6>(a1)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator
f45(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
    typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator a1,
    T6 &&...a2) {
  return a0.template emplace_hint<T6...>(
      std::forward<
          typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator>(a1),
      std::forward<T6>(a2)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
auto f46(std::flat_multimap<T1, T2, T3, T4, T5> &a0, T6 &&a1) {
  return a0.template insert<T6>(std::forward<T6>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator
f47(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
    typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator a1,
    T6 &&a2) {
  return a0.template insert<T6>(
      std::forward<
          typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator>(a1),
      std::forward<T6>(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
void f48(std::flat_multimap<T1, T2, T3, T4, T5> &a0, T6 a1, T6 a2) {
  return a0.template insert<T6>(std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
void f49(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
         std::sorted_equivalent_t a1, T6 a2, T6 a3) {
  return a0.template insert<T6>(std::move(a1), std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
void f50(std::flat_multimap<T1, T2, T3, T4, T5> &a0, T6 &&a1) {
  return a0.template insert_range<T6>(std::forward<T6>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::size_t f51(std::flat_multimap<T1, T2, T3, T4, T5> &a0, T6 &&a1) {
  return a0.template erase<T6>(std::forward<T6>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator
f52(std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template find<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator
f53(const std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template find<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::size_t f54(const std::flat_multimap<T1, T2, T3, T4, T5> &a0,
                const T6 &a1) {
  return a0.template count<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
bool f55(const std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template contains<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator
f56(std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template lower_bound<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator
f57(const std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template lower_bound<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator
f58(std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template upper_bound<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator
f59(const std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template upper_bound<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::pair<typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator,
          typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator>
f60(std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template equal_range<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::pair<typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator,
          typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator>
f61(const std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template equal_range<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f62(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
         std::flat_multimap<T1, T2, T3, T4, T5> &a1) {
  return swap(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
bool f63(const std::flat_multimap<T1, T2, T3, T4, T5> &a0,
         const std::flat_multimap<T1, T2, T3, T4, T5> &a1) {
  return operator==(a0, a1);
}
