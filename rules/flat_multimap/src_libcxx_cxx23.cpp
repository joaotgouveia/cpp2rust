#include <cstddef>
#include <flat_map>
#include <initializer_list>
#include <ranges>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_multimap<T1, T2, T3, T4, T5>::size_type
f1(std::flat_multimap<T1, T2, T3, T4, T5> &a0, T6 a1) {
  return std::erase_if<T1, T2, T3, T4, T5, T6>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_multimap<T1, T2, T3, T4, T5> f2() {
  return std::flat_multimap<T1, T2, T3, T4, T5>();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_multimap<T1, T2, T3, T4, T5>
f3(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return std::flat_multimap<T1, T2, T3, T4, T5>(a0);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_multimap<T1, T2, T3, T4, T5>
f4(std::flat_multimap<T1, T2, T3, T4, T5> &&a0) {
  return std::flat_multimap<T1, T2, T3, T4, T5>(std::move(a0));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_multimap<T1, T2, T3, T4, T5> f5(T4 a0, T5 a1, const T3 &a2) {
  return std::flat_multimap<T1, T2, T3, T4, T5>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_multimap<T1, T2, T3, T4, T5> f6(std::sorted_equivalent_t a0, T4 a1,
                                          T5 a2, const T3 &a3) {
  return std::flat_multimap<T1, T2, T3, T4, T5>(std::move(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_multimap<T1, T2, T3, T4, T5> f7(const T3 &a0) {
  return std::flat_multimap<T1, T2, T3, T4, T5>(a0);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_multimap<T1, T2, T3, T4, T5>
f8(std::initializer_list<std::pair<T1, T2>> a0, const T3 &a1) {
  return std::flat_multimap<T1, T2, T3, T4, T5>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_multimap<T1, T2, T3, T4, T5>
f9(std::sorted_equivalent_t a0, std::initializer_list<std::pair<T1, T2>> a1,
   const T3 &a2) {
  return std::flat_multimap<T1, T2, T3, T4, T5>(std::move(a0), a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_multimap<T1, T2, T3, T4, T5> &
f10(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
    std::initializer_list<std::pair<T1, T2>> a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_multimap<T1, T2, T3, T4, T5> &
f11(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
    const std::flat_multimap<T1, T2, T3, T4, T5> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_multimap<T1, T2, T3, T4, T5> &
f12(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
    std::flat_multimap<T1, T2, T3, T4, T5> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator
f13(std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator
f14(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator
f15(std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator
f16(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::reverse_iterator
f17(std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::const_reverse_iterator
f18(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::reverse_iterator
f19(std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.rend();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::const_reverse_iterator
f20(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.rend();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator
f21(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator
f22(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.cend();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::const_reverse_iterator
f23(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.crbegin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::const_reverse_iterator
f24(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.crend();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
bool f25(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.empty();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::size_t f26(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.size();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::size_t f27(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator
f28(std::flat_multimap<T1, T2, T3, T4, T5> &a0, const std::pair<T1, T2> &a1) {
  return a0.insert(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator
f29(std::flat_multimap<T1, T2, T3, T4, T5> &a0, std::pair<T1, T2> &&a1) {
  return a0.insert(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator
f30(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
    typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator a1,
    const std::pair<T1, T2> &a2) {
  return a0.insert(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator
f31(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
    typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator a1,
    std::pair<T1, T2> &&a2) {
  return a0.insert(a1, std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f32(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
         std::initializer_list<std::pair<T1, T2>> a1) {
  return a0.insert(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f33(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
         std::sorted_equivalent_t a1,
         std::initializer_list<std::pair<T1, T2>> a2) {
  return a0.insert(std::move(a1), a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::containers
f34(std::flat_multimap<T1, T2, T3, T4, T5> &&a0) {
  return std::move(a0).extract();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f35(std::flat_multimap<T1, T2, T3, T4, T5> &a0, T4 &&a1, T5 &&a2) {
  return a0.replace(std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator
f36(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
    typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator
f37(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
    typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::size_t f38(std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator
f39(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
    typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator a1,
    typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator a2) {
  return a0.erase(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f40(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
         std::flat_multimap<T1, T2, T3, T4, T5> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f41(std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.clear();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
T3 f42(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.key_comp();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::value_compare
f43(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.value_comp();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
const T4 &f44(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.keys();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
const T5 &f45(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.values();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator
f46(std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator
f47(const std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::size_t f48(const std::flat_multimap<T1, T2, T3, T4, T5> &a0,
                const T1 &a1) {
  return a0.count(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
bool f49(const std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.contains(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator
f50(std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.lower_bound(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator
f51(const std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.lower_bound(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator
f52(std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.upper_bound(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator
f53(const std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.upper_bound(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::pair<typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator,
          typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator>
f54(std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::pair<typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator,
          typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator>
f55(const std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_multimap<T1, T2, T3, T4, T5>
f56(const std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return std::flat_multimap<T1, T2, T3, T4, T5>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_multimap<T1, T2, T3, T4, T5>
f57(std::flat_multimap<T1, T2, T3, T4, T5> &&a0, const T6 &a1) {
  return std::flat_multimap<T1, T2, T3, T4, T5>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_multimap<T1, T2, T3, T4, T5> f58(const T4 &a0, const T5 &a1,
                                           const T6 &a2) {
  return std::flat_multimap<T1, T2, T3, T4, T5>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_multimap<T1, T2, T3, T4, T5> f59(const T4 &a0, const T5 &a1,
                                           const T3 &a2, const T6 &a3) {
  return std::flat_multimap<T1, T2, T3, T4, T5>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_multimap<T1, T2, T3, T4, T5>
f60(std::sorted_equivalent_t a0, const T4 &a1, const T5 &a2, const T6 &a3) {
  return std::flat_multimap<T1, T2, T3, T4, T5>(std::move(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_multimap<T1, T2, T3, T4, T5> f61(std::sorted_equivalent_t a0,
                                           const T4 &a1, const T5 &a2,
                                           const T3 &a3, const T6 &a4) {
  return std::flat_multimap<T1, T2, T3, T4, T5>(std::move(a0), a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_multimap<T1, T2, T3, T4, T5> f62(const T3 &a0, const T6 &a1) {
  return std::flat_multimap<T1, T2, T3, T4, T5>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_multimap<T1, T2, T3, T4, T5> f63(const T6 &a0) {
  return std::flat_multimap<T1, T2, T3, T4, T5>(a0);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_multimap<T1, T2, T3, T4, T5> f64(T6 a0, T6 a1, const T3 &a2) {
  return std::flat_multimap<T1, T2, T3, T4, T5>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
std::flat_multimap<T1, T2, T3, T4, T5> f65(T6 a0, T6 a1, const T3 &a2,
                                           const T7 &a3) {
  return std::flat_multimap<T1, T2, T3, T4, T5>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
std::flat_multimap<T1, T2, T3, T4, T5> f66(T6 a0, T6 a1, const T7 &a2) {
  return std::flat_multimap<T1, T2, T3, T4, T5>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_multimap<T1, T2, T3, T4, T5> f67(std::from_range_t a0, T6 &&a1) {
  return std::flat_multimap<T1, T2, T3, T4, T5>(std::move(a0),
                                                std::forward<T6>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
std::flat_multimap<T1, T2, T3, T4, T5> f68(std::from_range_t a0, T6 &&a1,
                                           const T7 &a2) {
  return std::flat_multimap<T1, T2, T3, T4, T5>(std::move(a0),
                                                std::forward<T6>(a1), a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_multimap<T1, T2, T3, T4, T5> f69(std::from_range_t a0, T6 &&a1,
                                           const T3 &a2) {
  return std::flat_multimap<T1, T2, T3, T4, T5>(std::move(a0),
                                                std::forward<T6>(a1), a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
std::flat_multimap<T1, T2, T3, T4, T5> f70(std::from_range_t a0, T6 &&a1,
                                           const T3 &a2, const T7 &a3) {
  return std::flat_multimap<T1, T2, T3, T4, T5>(std::move(a0),
                                                std::forward<T6>(a1), a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_multimap<T1, T2, T3, T4, T5> f71(std::sorted_equivalent_t a0, T6 a1,
                                           T6 a2, const T3 &a3) {
  return std::flat_multimap<T1, T2, T3, T4, T5>(std::move(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
std::flat_multimap<T1, T2, T3, T4, T5> f72(std::sorted_equivalent_t a0, T6 a1,
                                           T6 a2, const T3 &a3, const T7 &a4) {
  return std::flat_multimap<T1, T2, T3, T4, T5>(std::move(a0), a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
std::flat_multimap<T1, T2, T3, T4, T5> f73(std::sorted_equivalent_t a0, T6 a1,
                                           T6 a2, const T7 &a3) {
  return std::flat_multimap<T1, T2, T3, T4, T5>(std::move(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_multimap<T1, T2, T3, T4, T5>
f74(std::initializer_list<std::pair<T1, T2>> a0, const T3 &a1, const T6 &a2) {
  return std::flat_multimap<T1, T2, T3, T4, T5>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_multimap<T1, T2, T3, T4, T5>
f75(std::initializer_list<std::pair<T1, T2>> a0, const T6 &a1) {
  return std::flat_multimap<T1, T2, T3, T4, T5>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_multimap<T1, T2, T3, T4, T5>
f76(std::sorted_equivalent_t a0, std::initializer_list<std::pair<T1, T2>> a1,
    const T3 &a2, const T6 &a3) {
  return std::flat_multimap<T1, T2, T3, T4, T5>(std::move(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_multimap<T1, T2, T3, T4, T5>
f77(std::sorted_equivalent_t a0, std::initializer_list<std::pair<T1, T2>> a1,
    const T6 &a2) {
  return std::flat_multimap<T1, T2, T3, T4, T5>(std::move(a0), a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator
f78(std::flat_multimap<T1, T2, T3, T4, T5> &a0, T6 &&...a1) {
  return a0.template emplace<T6...>(std::forward<T6>(a1)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator
f79(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
    typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator a1,
    T6 &&...a2) {
  return a0.template emplace_hint<T6...>(a1, std::forward<T6>(a2)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator
f80(std::flat_multimap<T1, T2, T3, T4, T5> &a0, T6 &&a1) {
  return a0.template insert<T6>(std::forward<T6>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator
f81(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
    typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator a1,
    T6 &&a2) {
  return a0.template insert<T6>(a1, std::forward<T6>(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
void f82(std::flat_multimap<T1, T2, T3, T4, T5> &a0, T6 a1, T6 a2) {
  return a0.template insert<T6>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
void f83(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
         std::sorted_equivalent_t a1, T6 a2, T6 a3) {
  return a0.template insert<T6>(std::move(a1), a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
void f84(std::flat_multimap<T1, T2, T3, T4, T5> &a0, T6 &&a1) {
  return a0.template insert_range<T6>(std::forward<T6>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
void f85(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
         std::sorted_equivalent_t a1, T6 &&a2) {
  return a0.template insert_range<T6>(std::move(a1), std::forward<T6>(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::size_t f86(std::flat_multimap<T1, T2, T3, T4, T5> &a0, T6 &&a1) {
  return a0.template erase<T6>(std::forward<T6>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator
f87(std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template find<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator
f88(const std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template find<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::size_t f89(const std::flat_multimap<T1, T2, T3, T4, T5> &a0,
                const T6 &a1) {
  return a0.template count<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
bool f90(const std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template contains<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator
f91(std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template lower_bound<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator
f92(const std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template lower_bound<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator
f93(std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template upper_bound<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator
f94(const std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template upper_bound<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::pair<typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator,
          typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator>
f95(std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template equal_range<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::pair<typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator,
          typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator>
f96(const std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template equal_range<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f97(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
         std::flat_multimap<T1, T2, T3, T4, T5> &a1) {
  return swap(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f98(const std::flat_multimap<T1, T2, T3, T4, T5> &a0,
         const std::flat_multimap<T1, T2, T3, T4, T5> &a1) {
  return operator<=>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
bool f99(const std::flat_multimap<T1, T2, T3, T4, T5> &a0,
         const std::flat_multimap<T1, T2, T3, T4, T5> &a1) {
  return operator==(a0, a1);
}
