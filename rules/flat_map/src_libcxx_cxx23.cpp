#include <flat_map>
#include <initializer_list>
#include <ranges>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_map<T1, T2, T3, T4, T5> f22() {
  return std::flat_map<T1, T2, T3, T4, T5>();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_map<T1, T2, T3, T4, T5>
f23(const std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return std::flat_map<T1, T2, T3, T4, T5>(a0);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_map<T1, T2, T3, T4, T5> f24(std::flat_map<T1, T2, T3, T4, T5> &&a0) {
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_map<T1, T2, T3, T4, T5>
f25(typename std::flat_map<T1, T2, T3, T4, T5>::key_container_type a0,
    typename std::flat_map<T1, T2, T3, T4, T5>::mapped_container_type a1,
    const typename std::flat_map<T1, T2, T3, T4, T5>::key_compare &a2) {
  return std::flat_map<T1, T2, T3, T4, T5>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_map<T1, T2, T3, T4, T5>
f26(std::sorted_unique_t a0,
    typename std::flat_map<T1, T2, T3, T4, T5>::key_container_type a1,
    typename std::flat_map<T1, T2, T3, T4, T5>::mapped_container_type a2,
    const typename std::flat_map<T1, T2, T3, T4, T5>::key_compare &a3) {
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_map<T1, T2, T3, T4, T5>
f27(const typename std::flat_map<T1, T2, T3, T4, T5>::key_compare &a0) {
  return std::flat_map<T1, T2, T3, T4, T5>(a0);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_map<T1, T2, T3, T4, T5>
f28(std::initializer_list<
        typename std::flat_map<T1, T2, T3, T4, T5>::value_type>
        a0,
    const typename std::flat_map<T1, T2, T3, T4, T5>::key_compare &a1) {
  return std::flat_map<T1, T2, T3, T4, T5>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_map<T1, T2, T3, T4, T5>
f29(std::sorted_unique_t a0,
    std::initializer_list<
        typename std::flat_map<T1, T2, T3, T4, T5>::value_type>
        a1,
    const typename std::flat_map<T1, T2, T3, T4, T5>::key_compare &a2) {
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0), a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_map<T1, T2, T3, T4, T5> &
f30(std::flat_map<T1, T2, T3, T4, T5> &a0,
    std::initializer_list<
        typename std::flat_map<T1, T2, T3, T4, T5>::value_type>
        a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_map<T1, T2, T3, T4, T5> &
f31(std::flat_map<T1, T2, T3, T4, T5> &a0,
    const std::flat_map<T1, T2, T3, T4, T5> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_map<T1, T2, T3, T4, T5> &f32(std::flat_map<T1, T2, T3, T4, T5> &a0,
                                       std::flat_map<T1, T2, T3, T4, T5> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::iterator
f33(std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator
f34(const std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::iterator
f35(std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator
f36(const std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::reverse_iterator
f37(std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::const_reverse_iterator
f38(const std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::reverse_iterator
f39(std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.rend();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::const_reverse_iterator
f40(const std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.rend();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator
f41(const std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator
f42(const std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.cend();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::const_reverse_iterator
f43(const std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.crbegin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::const_reverse_iterator
f44(const std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.crend();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
bool f45(const std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.empty();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::size_type
f46(const std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.size();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::size_type
f47(const std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::pair<typename std::flat_map<T1, T2, T3, T4, T5>::iterator, bool>
f48(std::flat_map<T1, T2, T3, T4, T5> &a0,
    const typename std::flat_map<T1, T2, T3, T4, T5>::value_type &a1) {
  return a0.insert(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::pair<typename std::flat_map<T1, T2, T3, T4, T5>::iterator, bool>
f49(std::flat_map<T1, T2, T3, T4, T5> &a0,
    typename std::flat_map<T1, T2, T3, T4, T5>::value_type &&a1) {
  return a0.insert(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::iterator
f50(std::flat_map<T1, T2, T3, T4, T5> &a0,
    typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator a1,
    const typename std::flat_map<T1, T2, T3, T4, T5>::value_type &a2) {
  return a0.insert(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::iterator
f51(std::flat_map<T1, T2, T3, T4, T5> &a0,
    typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator a1,
    typename std::flat_map<T1, T2, T3, T4, T5>::value_type &&a2) {
  return a0.insert(a1, std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f52(std::flat_map<T1, T2, T3, T4, T5> &a0,
         std::initializer_list<
             typename std::flat_map<T1, T2, T3, T4, T5>::value_type>
             a1) {
  return a0.insert(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f53(std::flat_map<T1, T2, T3, T4, T5> &a0, std::sorted_unique_t a1,
         std::initializer_list<
             typename std::flat_map<T1, T2, T3, T4, T5>::value_type>
             a2) {
  return a0.insert(std::move(a1), a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::containers
f54(std::flat_map<T1, T2, T3, T4, T5> &&a0) {
  return std::move(a0).extract();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f55(
    std::flat_map<T1, T2, T3, T4, T5> &a0,
    typename std::flat_map<T1, T2, T3, T4, T5>::key_container_type &&a1,
    typename std::flat_map<T1, T2, T3, T4, T5>::mapped_container_type &&a2) {
  return a0.replace(std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::iterator
f56(std::flat_map<T1, T2, T3, T4, T5> &a0,
    typename std::flat_map<T1, T2, T3, T4, T5>::iterator a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::iterator
f57(std::flat_map<T1, T2, T3, T4, T5> &a0,
    typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::size_type
f58(std::flat_map<T1, T2, T3, T4, T5> &a0,
    const typename std::flat_map<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::iterator
f59(std::flat_map<T1, T2, T3, T4, T5> &a0,
    typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator a1,
    typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator a2) {
  return a0.erase(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f60(std::flat_map<T1, T2, T3, T4, T5> &a0,
         std::flat_map<T1, T2, T3, T4, T5> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f61(std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.clear();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::key_compare
f62(const std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.key_comp();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::value_compare
f63(const std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.value_comp();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
const typename std::flat_map<T1, T2, T3, T4, T5>::key_container_type &
f64(const std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.keys();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
const typename std::flat_map<T1, T2, T3, T4, T5>::mapped_container_type &
f65(const std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.values();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::iterator
f66(std::flat_map<T1, T2, T3, T4, T5> &a0,
    const typename std::flat_map<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator
f67(const std::flat_map<T1, T2, T3, T4, T5> &a0,
    const typename std::flat_map<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::size_type
f68(const std::flat_map<T1, T2, T3, T4, T5> &a0,
    const typename std::flat_map<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.count(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
bool f69(const std::flat_map<T1, T2, T3, T4, T5> &a0,
         const typename std::flat_map<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.contains(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::iterator
f70(std::flat_map<T1, T2, T3, T4, T5> &a0,
    const typename std::flat_map<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.lower_bound(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator
f71(const std::flat_map<T1, T2, T3, T4, T5> &a0,
    const typename std::flat_map<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.lower_bound(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::iterator
f72(std::flat_map<T1, T2, T3, T4, T5> &a0,
    const typename std::flat_map<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.upper_bound(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator
f73(const std::flat_map<T1, T2, T3, T4, T5> &a0,
    const typename std::flat_map<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.upper_bound(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::pair<typename std::flat_map<T1, T2, T3, T4, T5>::iterator,
          typename std::flat_map<T1, T2, T3, T4, T5>::iterator>
f74(std::flat_map<T1, T2, T3, T4, T5> &a0,
    const typename std::flat_map<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::pair<typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator,
          typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator>
f75(const std::flat_map<T1, T2, T3, T4, T5> &a0,
    const typename std::flat_map<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_map<T1, T2, T3, T4, T5>
f76(const std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return std::flat_map<T1, T2, T3, T4, T5>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_map<T1, T2, T3, T4, T5> f77(std::flat_map<T1, T2, T3, T4, T5> &&a0,
                                      const T6 &a1) {
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_map<T1, T2, T3, T4, T5>
f78(const typename std::flat_map<T1, T2, T3, T4, T5>::key_container_type &a0,
    const typename std::flat_map<T1, T2, T3, T4, T5>::mapped_container_type &a1,
    const T6 &a2) {
  return std::flat_map<T1, T2, T3, T4, T5>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_map<T1, T2, T3, T4, T5>
f79(const typename std::flat_map<T1, T2, T3, T4, T5>::key_container_type &a0,
    const typename std::flat_map<T1, T2, T3, T4, T5>::mapped_container_type &a1,
    const typename std::flat_map<T1, T2, T3, T4, T5>::key_compare &a2,
    const T6 &a3) {
  return std::flat_map<T1, T2, T3, T4, T5>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_map<T1, T2, T3, T4, T5>
f80(std::sorted_unique_t a0,
    const typename std::flat_map<T1, T2, T3, T4, T5>::key_container_type &a1,
    const typename std::flat_map<T1, T2, T3, T4, T5>::mapped_container_type &a2,
    const T6 &a3) {
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_map<T1, T2, T3, T4, T5>
f81(std::sorted_unique_t a0,
    const typename std::flat_map<T1, T2, T3, T4, T5>::key_container_type &a1,
    const typename std::flat_map<T1, T2, T3, T4, T5>::mapped_container_type &a2,
    const typename std::flat_map<T1, T2, T3, T4, T5>::key_compare &a3,
    const T6 &a4) {
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0), a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_map<T1, T2, T3, T4, T5>
f82(const typename std::flat_map<T1, T2, T3, T4, T5>::key_compare &a0,
    const T6 &a1) {
  return std::flat_map<T1, T2, T3, T4, T5>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_map<T1, T2, T3, T4, T5> f83(const T6 &a0) {
  return std::flat_map<T1, T2, T3, T4, T5>(a0);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_map<T1, T2, T3, T4, T5>
f84(T6 a0, T6 a1,
    const typename std::flat_map<T1, T2, T3, T4, T5>::key_compare &a2) {
  return std::flat_map<T1, T2, T3, T4, T5>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
std::flat_map<T1, T2, T3, T4, T5>
f85(T6 a0, T6 a1,
    const typename std::flat_map<T1, T2, T3, T4, T5>::key_compare &a2,
    const T7 &a3) {
  return std::flat_map<T1, T2, T3, T4, T5>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
std::flat_map<T1, T2, T3, T4, T5> f86(T6 a0, T6 a1, const T7 &a2) {
  return std::flat_map<T1, T2, T3, T4, T5>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_map<T1, T2, T3, T4, T5> f87(std::from_range_t a0, T6 &&a1) {
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0), std::forward<T6>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
std::flat_map<T1, T2, T3, T4, T5> f88(std::from_range_t a0, T6 &&a1,
                                      const T7 &a2) {
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0), std::forward<T6>(a1),
                                           a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_map<T1, T2, T3, T4, T5>
f89(std::from_range_t a0, T6 &&a1,
    const typename std::flat_map<T1, T2, T3, T4, T5>::key_compare &a2) {
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0), std::forward<T6>(a1),
                                           a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
std::flat_map<T1, T2, T3, T4, T5>
f90(std::from_range_t a0, T6 &&a1,
    const typename std::flat_map<T1, T2, T3, T4, T5>::key_compare &a2,
    const T7 &a3) {
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0), std::forward<T6>(a1),
                                           a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_map<T1, T2, T3, T4, T5>
f91(std::sorted_unique_t a0, T6 a1, T6 a2,
    const typename std::flat_map<T1, T2, T3, T4, T5>::key_compare &a3) {
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
std::flat_map<T1, T2, T3, T4, T5>
f92(std::sorted_unique_t a0, T6 a1, T6 a2,
    const typename std::flat_map<T1, T2, T3, T4, T5>::key_compare &a3,
    const T7 &a4) {
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0), a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
std::flat_map<T1, T2, T3, T4, T5> f93(std::sorted_unique_t a0, T6 a1, T6 a2,
                                      const T7 &a3) {
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_map<T1, T2, T3, T4, T5>
f94(std::initializer_list<
        typename std::flat_map<T1, T2, T3, T4, T5>::value_type>
        a0,
    const typename std::flat_map<T1, T2, T3, T4, T5>::key_compare &a1,
    const T6 &a2) {
  return std::flat_map<T1, T2, T3, T4, T5>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_map<T1, T2, T3, T4, T5>
f95(std::initializer_list<
        typename std::flat_map<T1, T2, T3, T4, T5>::value_type>
        a0,
    const T6 &a1) {
  return std::flat_map<T1, T2, T3, T4, T5>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_map<T1, T2, T3, T4, T5>
f96(std::sorted_unique_t a0,
    std::initializer_list<
        typename std::flat_map<T1, T2, T3, T4, T5>::value_type>
        a1,
    const typename std::flat_map<T1, T2, T3, T4, T5>::key_compare &a2,
    const T6 &a3) {
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_map<T1, T2, T3, T4, T5>
f97(std::sorted_unique_t a0,
    std::initializer_list<
        typename std::flat_map<T1, T2, T3, T4, T5>::value_type>
        a1,
    const T6 &a2) {
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0), a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
std::pair<typename std::flat_map<T1, T2, T3, T4, T5>::iterator, bool>
f98(std::flat_map<T1, T2, T3, T4, T5> &a0, T6 &&...a1) {
  return a0.template emplace<T6...>(std::forward<T6>(a1)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
typename std::flat_map<T1, T2, T3, T4, T5>::iterator
f99(std::flat_map<T1, T2, T3, T4, T5> &a0,
    typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator a1, T6 &&...a2) {
  return a0.template emplace_hint<T6...>(a1, std::forward<T6>(a2)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::pair<typename std::flat_map<T1, T2, T3, T4, T5>::iterator, bool>
f100(std::flat_map<T1, T2, T3, T4, T5> &a0, T6 &&a1) {
  return a0.template insert<T6>(std::forward<T6>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_map<T1, T2, T3, T4, T5>::iterator
f101(std::flat_map<T1, T2, T3, T4, T5> &a0,
     typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator a1, T6 &&a2) {
  return a0.template insert<T6>(a1, std::forward<T6>(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
void f102(std::flat_map<T1, T2, T3, T4, T5> &a0, T6 a1, T6 a2) {
  return a0.template insert<T6>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
void f103(std::flat_map<T1, T2, T3, T4, T5> &a0, std::sorted_unique_t a1, T6 a2,
          T6 a3) {
  return a0.template insert<T6>(std::move(a1), a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
void f104(std::flat_map<T1, T2, T3, T4, T5> &a0, T6 &&a1) {
  return a0.template insert_range<T6>(std::forward<T6>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
void f105(std::flat_map<T1, T2, T3, T4, T5> &a0, std::sorted_unique_t a1,
          T6 &&a2) {
  return a0.template insert_range<T6>(std::move(a1), std::forward<T6>(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_map<T1, T2, T3, T4, T5>::size_type
f106(std::flat_map<T1, T2, T3, T4, T5> &a0, T6 &&a1) {
  return a0.template erase<T6>(std::forward<T6>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_map<T1, T2, T3, T4, T5>::iterator
f107(std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template find<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator
f108(const std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template find<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_map<T1, T2, T3, T4, T5>::size_type
f109(const std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template count<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
bool f110(const std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template contains<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_map<T1, T2, T3, T4, T5>::iterator
f111(std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template lower_bound<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator
f112(const std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template lower_bound<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_map<T1, T2, T3, T4, T5>::iterator
f113(std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template upper_bound<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator
f114(const std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template upper_bound<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::pair<typename std::flat_map<T1, T2, T3, T4, T5>::iterator,
          typename std::flat_map<T1, T2, T3, T4, T5>::iterator>
f115(std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template equal_range<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::pair<typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator,
          typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator>
f116(const std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template equal_range<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f117(std::flat_map<T1, T2, T3, T4, T5> &a0,
          std::flat_map<T1, T2, T3, T4, T5> &a1) {
  return swap(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f118(const std::flat_map<T1, T2, T3, T4, T5> &a0,
          const std::flat_map<T1, T2, T3, T4, T5> &a1) {
  return a0 <=> a1;
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
bool f119(const std::flat_map<T1, T2, T3, T4, T5> &a0,
          const std::flat_map<T1, T2, T3, T4, T5> &a1) {
  return a0 == a1;
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t18 = typename std::flat_map<T1, T2, T3, T4, T5>::value_type::first_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t19 = typename std::flat_map<T1, T2, T3, T4, T5>::value_type::second_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t20 = typename std::flat_map<T1, T2, T3, T4, T5>::reference::first_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t21 = typename std::flat_map<T1, T2, T3, T4, T5>::reference::second_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t22 =
    typename std::flat_map<T1, T2, T3, T4, T5>::const_reference::first_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t23 =
    typename std::flat_map<T1, T2, T3, T4, T5>::const_reference::second_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t24 = typename std::flat_map<T1, T2, T3, T4,
                                   T5>::reverse_iterator::iterator_category;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t25 =
    typename std::flat_map<T1, T2, T3, T4, T5>::reverse_iterator::pointer;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t26 = typename std::flat_map<T1, T2, T3, T4,
                                   T5>::reverse_iterator::iterator_concept;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t27 =
    typename std::flat_map<T1, T2, T3, T4, T5>::reverse_iterator::value_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t28 = typename std::flat_map<T1, T2, T3, T4,
                                   T5>::reverse_iterator::difference_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t29 =
    typename std::flat_map<T1, T2, T3, T4, T5>::reverse_iterator::reference;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t30 =
    typename std::flat_map<T1, T2, T3, T4,
                           T5>::const_reverse_iterator::iterator_category;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t31 =
    typename std::flat_map<T1, T2, T3, T4, T5>::const_reverse_iterator::pointer;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t32 =
    typename std::flat_map<T1, T2, T3, T4,
                           T5>::const_reverse_iterator::iterator_concept;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t33 = typename std::flat_map<T1, T2, T3, T4,
                                   T5>::const_reverse_iterator::value_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t34 = typename std::flat_map<T1, T2, T3, T4,
                                   T5>::const_reverse_iterator::difference_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t35 = typename std::flat_map<T1, T2, T3, T4,
                                   T5>::const_reverse_iterator::reference;
