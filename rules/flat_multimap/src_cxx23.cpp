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
std::flat_multimap<T1, T2, T3, T4, T5> f1() {
  return std::flat_multimap<T1, T2, T3, T4, T5>();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_multimap<T1, T2, T3, T4, T5> f2(T3 const &a0) {
  return std::flat_multimap<T1, T2, T3, T4, T5>(a0);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_multimap<T1, T2, T3, T4, T5> f3(T4 a0, T5 a1, T3 const &a2) {
  return std::flat_multimap<T1, T2, T3, T4, T5>(std::move(a0), std::move(a1),
                                                a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_multimap<T1, T2, T3, T4, T5> f4(std::sorted_equivalent_t a0, T4 a1,
                                          T5 a2, T3 const &a3) {
  return std::flat_multimap<T1, T2, T3, T4, T5>(std::move(a0), std::move(a1),
                                                std::move(a2), a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_multimap<T1, T2, T3, T4, T5>
f5(std::initializer_list<std::pair<T1, T2>> a0, T3 const &a1) {
  return std::flat_multimap<T1, T2, T3, T4, T5>(
      std::forward<std::initializer_list<std::pair<T1, T2>>>(a0), a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_multimap<T1, T2, T3, T4, T5>
f6(std::sorted_equivalent_t a0, std::initializer_list<std::pair<T1, T2>> a1,
   T3 const &a2) {
  return std::flat_multimap<T1, T2, T3, T4, T5>(
      std::move(a0), std::forward<std::initializer_list<std::pair<T1, T2>>>(a1),
      a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator
f7(std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator
f8(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator
f9(std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator
f10(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::reverse_iterator
f11(std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::const_reverse_iterator
f12(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::reverse_iterator
f13(std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.rend();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::const_reverse_iterator
f14(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.rend();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator
f15(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator
f16(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.cend();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::const_reverse_iterator
f17(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.crbegin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::const_reverse_iterator
f18(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.crend();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
bool f19(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.empty();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::size_t f20(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.size();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::size_t f21(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f22(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
         const std::pair<T1, T2> &a1) {
  return a0.insert(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f23(std::flat_multimap<T1, T2, T3, T4, T5> &a0, std::pair<T1, T2> &&a1) {
  return a0.insert(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator
f24(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
    typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator a1,
    const std::pair<T1, T2> &a2) {
  return a0.insert(
      std::forward<
          typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator>(a1),
      a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator
f25(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
    typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator a1,
    std::pair<T1, T2> &&a2) {
  return a0.insert(
      std::forward<
          typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator>(a1),
      std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f26(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
         std::initializer_list<std::pair<T1, T2>> a1) {
  return a0.insert(std::forward<std::initializer_list<std::pair<T1, T2>>>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f27(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
         std::sorted_equivalent_t a1,
         std::initializer_list<std::pair<T1, T2>> a2) {
  return a0.insert(std::move(a1),
                   std::forward<std::initializer_list<std::pair<T1, T2>>>(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::containers
f28(std::flat_multimap<T1, T2, T3, T4, T5> &&a0) {
  return std::move(a0).extract();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f29(std::flat_multimap<T1, T2, T3, T4, T5> &a0, T4 &&a1, T5 &&a2) {
  return a0.replace(std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator
f30(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
    typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator a1) {
  return a0.erase(
      std::forward<typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator>(
          a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator
f31(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
    typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator a1) {
  return a0.erase(
      std::forward<
          typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::size_t f32(std::flat_multimap<T1, T2, T3, T4, T5> &a0, T1 const &a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator
f33(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
    typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator a1,
    typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator a2) {
  return a0.erase(
      std::forward<
          typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator>(a1),
      std::forward<
          typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator>(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f34(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
         std::flat_multimap<T1, T2, T3, T4, T5> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f35(std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.clear();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
T3 f36(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.key_comp();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::value_compare
f37(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.value_comp();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
T4 const &f38(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.keys();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
T5 const &f39(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.values();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator
f40(std::flat_multimap<T1, T2, T3, T4, T5> &a0, T1 const &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator
f41(const std::flat_multimap<T1, T2, T3, T4, T5> &a0, T1 const &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::size_t f42(const std::flat_multimap<T1, T2, T3, T4, T5> &a0,
                T1 const &a1) {
  return a0.count(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
bool f43(const std::flat_multimap<T1, T2, T3, T4, T5> &a0, T1 const &a1) {
  return a0.contains(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator
f44(std::flat_multimap<T1, T2, T3, T4, T5> &a0, T1 const &a1) {
  return a0.lower_bound(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator
f45(const std::flat_multimap<T1, T2, T3, T4, T5> &a0, T1 const &a1) {
  return a0.lower_bound(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator
f46(std::flat_multimap<T1, T2, T3, T4, T5> &a0, T1 const &a1) {
  return a0.upper_bound(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator
f47(const std::flat_multimap<T1, T2, T3, T4, T5> &a0, T1 const &a1) {
  return a0.upper_bound(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::pair<typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator,
          typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator>
f48(std::flat_multimap<T1, T2, T3, T4, T5> &a0, T1 const &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::pair<typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator,
          typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator>
f49(const std::flat_multimap<T1, T2, T3, T4, T5> &a0, T1 const &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_multimap<T1, T2, T3, T4, T5> f50(T6 a0, T6 a1, T3 const &a2) {
  return std::flat_multimap<T1, T2, T3, T4, T5>(std::move(a0), std::move(a1),
                                                a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_multimap<T1, T2, T3, T4, T5> f51(std::sorted_equivalent_t a0, T6 a1,
                                           T6 a2, T3 const &a3) {
  return std::flat_multimap<T1, T2, T3, T4, T5>(std::move(a0), std::move(a1),
                                                std::move(a2), a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_multimap<T1, T2, T3, T4, T5> f52(std::from_range_t a0, T6 &&a1) {
  return std::flat_multimap<T1, T2, T3, T4, T5>(std::move(a0),
                                                std::forward<T6>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_multimap<T1, T2, T3, T4, T5> f53(std::from_range_t a0, T6 &&a1,
                                           T3 const &a2) {
  return std::flat_multimap<T1, T2, T3, T4, T5>(std::move(a0),
                                                std::forward<T6>(a1), a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_multimap<T1, T2, T3, T4, T5> f54(const T6 &a0) {
  return std::flat_multimap<T1, T2, T3, T4, T5>(a0);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_multimap<T1, T2, T3, T4, T5> f55(T3 const &a0, const T6 &a1) {
  return std::flat_multimap<T1, T2, T3, T4, T5>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_multimap<T1, T2, T3, T4, T5> f56(T4 const &a0, T5 const &a1,
                                           const T6 &a2) {
  return std::flat_multimap<T1, T2, T3, T4, T5>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_multimap<T1, T2, T3, T4, T5> f57(T4 const &a0, T5 const &a1,
                                           T3 const &a2, const T6 &a3) {
  return std::flat_multimap<T1, T2, T3, T4, T5>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_multimap<T1, T2, T3, T4, T5>
f58(std::sorted_equivalent_t a0, T4 const &a1, T5 const &a2, const T6 &a3) {
  return std::flat_multimap<T1, T2, T3, T4, T5>(std::move(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_multimap<T1, T2, T3, T4, T5> f59(std::sorted_equivalent_t a0,
                                           T4 const &a1, T5 const &a2,
                                           T3 const &a3, const T6 &a4) {
  return std::flat_multimap<T1, T2, T3, T4, T5>(std::move(a0), a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_multimap<T1, T2, T3, T4, T5>
f60(const std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return std::flat_multimap<T1, T2, T3, T4, T5>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_multimap<T1, T2, T3, T4, T5>
f61(std::flat_multimap<T1, T2, T3, T4, T5> &&a0, const T6 &a1) {
  return std::flat_multimap<T1, T2, T3, T4, T5>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
std::flat_multimap<T1, T2, T3, T4, T5> f62(T6 a0, T6 a1, const T7 &a2) {
  return std::flat_multimap<T1, T2, T3, T4, T5>(std::move(a0), std::move(a1),
                                                a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
std::flat_multimap<T1, T2, T3, T4, T5> f63(T6 a0, T6 a1, T3 const &a2,
                                           const T7 &a3) {
  return std::flat_multimap<T1, T2, T3, T4, T5>(std::move(a0), std::move(a1),
                                                a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
std::flat_multimap<T1, T2, T3, T4, T5> f64(std::sorted_equivalent_t a0, T6 a1,
                                           T6 a2, const T7 &a3) {
  return std::flat_multimap<T1, T2, T3, T4, T5>(std::move(a0), std::move(a1),
                                                std::move(a2), a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
std::flat_multimap<T1, T2, T3, T4, T5> f65(std::sorted_equivalent_t a0, T6 a1,
                                           T6 a2, T3 const &a3, const T7 &a4) {
  return std::flat_multimap<T1, T2, T3, T4, T5>(std::move(a0), std::move(a1),
                                                std::move(a2), a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
std::flat_multimap<T1, T2, T3, T4, T5> f66(std::from_range_t a0, T6 &&a1,
                                           const T7 &a2) {
  return std::flat_multimap<T1, T2, T3, T4, T5>(std::move(a0),
                                                std::forward<T6>(a1), a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
std::flat_multimap<T1, T2, T3, T4, T5> f67(std::from_range_t a0, T6 &&a1,
                                           T3 const &a2, const T7 &a3) {
  return std::flat_multimap<T1, T2, T3, T4, T5>(std::move(a0),
                                                std::forward<T6>(a1), a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_multimap<T1, T2, T3, T4, T5>
f68(std::initializer_list<std::pair<T1, T2>> a0, const T6 &a1) {
  return std::flat_multimap<T1, T2, T3, T4, T5>(
      std::forward<std::initializer_list<std::pair<T1, T2>>>(a0), a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_multimap<T1, T2, T3, T4, T5>
f69(std::initializer_list<std::pair<T1, T2>> a0, T3 const &a1, const T6 &a2) {
  return std::flat_multimap<T1, T2, T3, T4, T5>(
      std::forward<std::initializer_list<std::pair<T1, T2>>>(a0), a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_multimap<T1, T2, T3, T4, T5>
f70(std::sorted_equivalent_t a0, std::initializer_list<std::pair<T1, T2>> a1,
    const T6 &a2) {
  return std::flat_multimap<T1, T2, T3, T4, T5>(
      std::move(a0), std::forward<std::initializer_list<std::pair<T1, T2>>>(a1),
      a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_multimap<T1, T2, T3, T4, T5>
f71(std::sorted_equivalent_t a0, std::initializer_list<std::pair<T1, T2>> a1,
    T3 const &a2, const T6 &a3) {
  return std::flat_multimap<T1, T2, T3, T4, T5>(
      std::move(a0), std::forward<std::initializer_list<std::pair<T1, T2>>>(a1),
      a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
auto f72(std::flat_multimap<T1, T2, T3, T4, T5> &a0, T6 &&...a1) {
  return a0.template emplace<T6...>(std::forward<T6>(a1)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator
f73(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
    typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator a1,
    T6 &&...a2) {
  return a0.template emplace_hint<T6...>(
      std::forward<
          typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator>(a1),
      std::forward<T6>(a2)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
auto f74(std::flat_multimap<T1, T2, T3, T4, T5> &a0, T6 &&a1) {
  return a0.template insert<T6>(std::forward<T6>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator
f75(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
    typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator a1,
    T6 &&a2) {
  return a0.template insert<T6>(
      std::forward<
          typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator>(a1),
      std::forward<T6>(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
void f76(std::flat_multimap<T1, T2, T3, T4, T5> &a0, T6 a1, T6 a2) {
  return a0.template insert<T6>(std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
void f77(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
         std::sorted_equivalent_t a1, T6 a2, T6 a3) {
  return a0.template insert<T6>(std::move(a1), std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
void f78(std::flat_multimap<T1, T2, T3, T4, T5> &a0, T6 &&a1) {
  return a0.template insert_range<T6>(std::forward<T6>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::size_t f79(std::flat_multimap<T1, T2, T3, T4, T5> &a0, T6 &&a1) {
  return a0.template erase<T6>(std::forward<T6>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator
f80(std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template find<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator
f81(const std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template find<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::size_t f82(const std::flat_multimap<T1, T2, T3, T4, T5> &a0,
                const T6 &a1) {
  return a0.template count<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
bool f83(const std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template contains<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator
f84(std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template lower_bound<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator
f85(const std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template lower_bound<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator
f86(std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template upper_bound<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator
f87(const std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template upper_bound<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::pair<typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator,
          typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator>
f88(std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template equal_range<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::pair<typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator,
          typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator>
f89(const std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template equal_range<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f90(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
         std::flat_multimap<T1, T2, T3, T4, T5> &a1) {
  return swap(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
bool f91(const std::flat_multimap<T1, T2, T3, T4, T5> &a0,
         const std::flat_multimap<T1, T2, T3, T4, T5> &a1) {
  return operator==(a0, a1);
}
