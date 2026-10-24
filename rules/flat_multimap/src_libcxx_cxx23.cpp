#include <cstddef>
#include <flat_map>
#include <initializer_list>
#include <iterator>
#include <ranges>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_multimap<T1, T2, T3, T4, T5>::size_type
f62(std::flat_multimap<T1, T2, T3, T4, T5> &a0, T6 a1) {
  return std::erase_if<T1, T2, T3, T4, T5, T6>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_multimap<T1, T2, T3, T4, T5> f63() {
  return std::flat_multimap<T1, T2, T3, T4, T5>();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_multimap<T1, T2, T3, T4, T5>
f64(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return std::flat_multimap<T1, T2, T3, T4, T5>(a0);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_multimap<T1, T2, T3, T4, T5>
f65(std::flat_multimap<T1, T2, T3, T4, T5> &&a0) {
  return std::flat_multimap<T1, T2, T3, T4, T5>(std::move(a0));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_multimap<T1, T2, T3, T4, T5> f66(T4 a0, T5 a1, const T3 &a2) {
  return std::flat_multimap<T1, T2, T3, T4, T5>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_multimap<T1, T2, T3, T4, T5> f67(std::sorted_equivalent_t a0, T4 a1,
                                           T5 a2, const T3 &a3) {
  return std::flat_multimap<T1, T2, T3, T4, T5>(std::move(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_multimap<T1, T2, T3, T4, T5> f68(const T3 &a0) {
  return std::flat_multimap<T1, T2, T3, T4, T5>(a0);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_multimap<T1, T2, T3, T4, T5>
f69(std::initializer_list<std::pair<T1, T2>> a0, const T3 &a1) {
  return std::flat_multimap<T1, T2, T3, T4, T5>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_multimap<T1, T2, T3, T4, T5>
f70(std::sorted_equivalent_t a0, std::initializer_list<std::pair<T1, T2>> a1,
    const T3 &a2) {
  return std::flat_multimap<T1, T2, T3, T4, T5>(std::move(a0), a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_multimap<T1, T2, T3, T4, T5> &
f71(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
    std::initializer_list<std::pair<T1, T2>> a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_multimap<T1, T2, T3, T4, T5> &
f72(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
    const std::flat_multimap<T1, T2, T3, T4, T5> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_multimap<T1, T2, T3, T4, T5> &
f73(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
    std::flat_multimap<T1, T2, T3, T4, T5> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::__key_value_iterator<std::flat_multimap<T1, T2, T3, T4, T5>, T4, T5, false>
f74(std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::__key_value_iterator<std::flat_multimap<T1, T2, T3, T4, T5>, T4, T5, true>
f75(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::__key_value_iterator<std::flat_multimap<T1, T2, T3, T4, T5>, T4, T5, false>
f76(std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::__key_value_iterator<std::flat_multimap<T1, T2, T3, T4, T5>, T4, T5, true>
f77(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::reverse_iterator<std::__key_value_iterator<
    std::flat_multimap<T1, T2, T3, T4, T5>, T4, T5, false>>
f78(std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::reverse_iterator<std::__key_value_iterator<
    std::flat_multimap<T1, T2, T3, T4, T5>, T4, T5, true>>
f79(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::reverse_iterator<std::__key_value_iterator<
    std::flat_multimap<T1, T2, T3, T4, T5>, T4, T5, false>>
f80(std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.rend();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::reverse_iterator<std::__key_value_iterator<
    std::flat_multimap<T1, T2, T3, T4, T5>, T4, T5, true>>
f81(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.rend();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::__key_value_iterator<std::flat_multimap<T1, T2, T3, T4, T5>, T4, T5, true>
f82(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::__key_value_iterator<std::flat_multimap<T1, T2, T3, T4, T5>, T4, T5, true>
f83(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.cend();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::reverse_iterator<std::__key_value_iterator<
    std::flat_multimap<T1, T2, T3, T4, T5>, T4, T5, true>>
f84(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.crbegin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::reverse_iterator<std::__key_value_iterator<
    std::flat_multimap<T1, T2, T3, T4, T5>, T4, T5, true>>
f85(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.crend();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::__key_value_iterator<std::flat_multimap<T1, T2, T3, T4, T5>, T4, T5, false>
f86(std::flat_multimap<T1, T2, T3, T4, T5> &a0, const std::pair<T1, T2> &a1) {
  return a0.insert(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::__key_value_iterator<std::flat_multimap<T1, T2, T3, T4, T5>, T4, T5, false>
f87(std::flat_multimap<T1, T2, T3, T4, T5> &a0, std::pair<T1, T2> &&a1) {
  return a0.insert(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::__key_value_iterator<std::flat_multimap<T1, T2, T3, T4, T5>, T4, T5, false>
f88(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
    std::__key_value_iterator<std::flat_multimap<T1, T2, T3, T4, T5>, T4, T5,
                              true>
        a1,
    const std::pair<T1, T2> &a2) {
  return a0.insert(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::__key_value_iterator<std::flat_multimap<T1, T2, T3, T4, T5>, T4, T5, false>
f89(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
    std::__key_value_iterator<std::flat_multimap<T1, T2, T3, T4, T5>, T4, T5,
                              true>
        a1,
    std::pair<T1, T2> &&a2) {
  return a0.insert(a1, std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::__key_value_iterator<std::flat_multimap<T1, T2, T3, T4, T5>, T4, T5, false>
f90(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
    std::__key_value_iterator<std::flat_multimap<T1, T2, T3, T4, T5>, T4, T5,
                              false>
        a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::__key_value_iterator<std::flat_multimap<T1, T2, T3, T4, T5>, T4, T5, false>
f91(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
    std::__key_value_iterator<std::flat_multimap<T1, T2, T3, T4, T5>, T4, T5,
                              true>
        a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::size_t f92(std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::__key_value_iterator<std::flat_multimap<T1, T2, T3, T4, T5>, T4, T5, false>
f93(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
    std::__key_value_iterator<std::flat_multimap<T1, T2, T3, T4, T5>, T4, T5,
                              true>
        a1,
    std::__key_value_iterator<std::flat_multimap<T1, T2, T3, T4, T5>, T4, T5,
                              true>
        a2) {
  return a0.erase(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
const T4 &f94(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.keys();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
const T5 &f95(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.values();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::__key_value_iterator<std::flat_multimap<T1, T2, T3, T4, T5>, T4, T5, false>
f96(std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::__key_value_iterator<std::flat_multimap<T1, T2, T3, T4, T5>, T4, T5, true>
f97(const std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::size_t f98(const std::flat_multimap<T1, T2, T3, T4, T5> &a0,
                const T1 &a1) {
  return a0.count(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
bool f99(const std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.contains(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::__key_value_iterator<std::flat_multimap<T1, T2, T3, T4, T5>, T4, T5, false>
f100(std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.lower_bound(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::__key_value_iterator<std::flat_multimap<T1, T2, T3, T4, T5>, T4, T5, true>
f101(const std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.lower_bound(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::__key_value_iterator<std::flat_multimap<T1, T2, T3, T4, T5>, T4, T5, false>
f102(std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.upper_bound(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::__key_value_iterator<std::flat_multimap<T1, T2, T3, T4, T5>, T4, T5, true>
f103(const std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.upper_bound(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::pair<std::__key_value_iterator<std::flat_multimap<T1, T2, T3, T4, T5>, T4,
                                    T5, false>,
          std::__key_value_iterator<std::flat_multimap<T1, T2, T3, T4, T5>, T4,
                                    T5, false>>
f104(std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::pair<std::__key_value_iterator<std::flat_multimap<T1, T2, T3, T4, T5>, T4,
                                    T5, true>,
          std::__key_value_iterator<std::flat_multimap<T1, T2, T3, T4, T5>, T4,
                                    T5, true>>
f105(const std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_multimap<T1, T2, T3, T4, T5>
f106(const std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(std::flat_multimap<T1, T2, T3, T4,
                              T5>::template __allocator_ctor_constraint<T6>)
{
  return std::flat_multimap<T1, T2, T3, T4, T5>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_multimap<T1, T2, T3, T4, T5>
f107(std::flat_multimap<T1, T2, T3, T4, T5> &&a0, const T6 &a1)
  requires(std::flat_multimap<T1, T2, T3, T4,
                              T5>::template __allocator_ctor_constraint<T6>)
{
  return std::flat_multimap<T1, T2, T3, T4, T5>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_multimap<T1, T2, T3, T4, T5> f108(const T4 &a0, const T5 &a1,
                                            const T6 &a2)
  requires(std::flat_multimap<T1, T2, T3, T4,
                              T5>::template __allocator_ctor_constraint<T6>)
{
  return std::flat_multimap<T1, T2, T3, T4, T5>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_multimap<T1, T2, T3, T4, T5> f109(const T4 &a0, const T5 &a1,
                                            const T3 &a2, const T6 &a3)
  requires(std::flat_multimap<T1, T2, T3, T4,
                              T5>::template __allocator_ctor_constraint<T6>)
{
  return std::flat_multimap<T1, T2, T3, T4, T5>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_multimap<T1, T2, T3, T4, T5>
f110(std::sorted_equivalent_t a0, const T4 &a1, const T5 &a2, const T6 &a3)
  requires(std::flat_multimap<T1, T2, T3, T4,
                              T5>::template __allocator_ctor_constraint<T6>)
{
  return std::flat_multimap<T1, T2, T3, T4, T5>(std::move(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_multimap<T1, T2, T3, T4, T5> f111(std::sorted_equivalent_t a0,
                                            const T4 &a1, const T5 &a2,
                                            const T3 &a3, const T6 &a4)
  requires(std::flat_multimap<T1, T2, T3, T4,
                              T5>::template __allocator_ctor_constraint<T6>)
{
  return std::flat_multimap<T1, T2, T3, T4, T5>(std::move(a0), a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_multimap<T1, T2, T3, T4, T5> f112(const T3 &a0, const T6 &a1)
  requires(std::flat_multimap<T1, T2, T3, T4,
                              T5>::template __allocator_ctor_constraint<T6>)
{
  return std::flat_multimap<T1, T2, T3, T4, T5>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_multimap<T1, T2, T3, T4, T5> f113(const T6 &a0)
  requires(std::flat_multimap<T1, T2, T3, T4,
                              T5>::template __allocator_ctor_constraint<T6>) &&
          (!(std::is_same<T6, std::flat_multimap<T1, T2, T3, T4, T5>>::value) &&
           !(std::is_same<T6, typename std::flat_multimap<
                                  T1, T2, T3, T4, T5>::key_compare>::value))
{
  return std::flat_multimap<T1, T2, T3, T4, T5>(a0);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_multimap<T1, T2, T3, T4, T5> f114(T6 a0, T6 a1, const T3 &a2)
  requires(std::__has_input_iterator_category<T6>::value)
{
  return std::flat_multimap<T1, T2, T3, T4, T5>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
std::flat_multimap<T1, T2, T3, T4, T5> f115(T6 a0, T6 a1, const T3 &a2,
                                            const T7 &a3)
  requires((std::__has_input_iterator_category<T6>::value &&
            std::flat_multimap<T1, T2, T3, T4,
                               T5>::template __allocator_ctor_constraint<T7>))
{
  return std::flat_multimap<T1, T2, T3, T4, T5>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
std::flat_multimap<T1, T2, T3, T4, T5> f116(T6 a0, T6 a1, const T7 &a2)
  requires((std::__has_input_iterator_category<T6>::value &&
            std::flat_multimap<T1, T2, T3, T4,
                               T5>::template __allocator_ctor_constraint<T7>))
{
  return std::flat_multimap<T1, T2, T3, T4, T5>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_multimap<T1, T2, T3, T4, T5> f117(std::from_range_t a0, T6 &&a1)
  requires(std::_ContainerCompatibleRange<
           T6, typename std::flat_multimap<T1, T2, T3, T4, T5>::value_type>)
{
  return std::flat_multimap<T1, T2, T3, T4, T5>(std::move(a0),
                                                std::forward<T6>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
std::flat_multimap<T1, T2, T3, T4, T5> f118(std::from_range_t a0, T6 &&a1,
                                            const T7 &a2)
  requires(std::flat_multimap<T1, T2, T3, T4,
                              T5>::template __allocator_ctor_constraint<T7>) &&
          (std::_ContainerCompatibleRange<
              T6, typename std::flat_multimap<T1, T2, T3, T4, T5>::value_type>)
{
  return std::flat_multimap<T1, T2, T3, T4, T5>(std::move(a0),
                                                std::forward<T6>(a1), a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_multimap<T1, T2, T3, T4, T5> f119(std::from_range_t a0, T6 &&a1,
                                            const T3 &a2)
  requires(std::_ContainerCompatibleRange<
           T6, typename std::flat_multimap<T1, T2, T3, T4, T5>::value_type>)
{
  return std::flat_multimap<T1, T2, T3, T4, T5>(std::move(a0),
                                                std::forward<T6>(a1), a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
std::flat_multimap<T1, T2, T3, T4, T5> f120(std::from_range_t a0, T6 &&a1,
                                            const T3 &a2, const T7 &a3)
  requires(std::flat_multimap<T1, T2, T3, T4,
                              T5>::template __allocator_ctor_constraint<T7>) &&
          (std::_ContainerCompatibleRange<
              T6, typename std::flat_multimap<T1, T2, T3, T4, T5>::value_type>)
{
  return std::flat_multimap<T1, T2, T3, T4, T5>(std::move(a0),
                                                std::forward<T6>(a1), a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_multimap<T1, T2, T3, T4, T5> f121(std::sorted_equivalent_t a0, T6 a1,
                                            T6 a2, const T3 &a3)
  requires(std::__has_input_iterator_category<T6>::value)
{
  return std::flat_multimap<T1, T2, T3, T4, T5>(std::move(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
std::flat_multimap<T1, T2, T3, T4, T5> f122(std::sorted_equivalent_t a0, T6 a1,
                                            T6 a2, const T3 &a3, const T7 &a4)
  requires((std::__has_input_iterator_category<T6>::value &&
            std::flat_multimap<T1, T2, T3, T4,
                               T5>::template __allocator_ctor_constraint<T7>))
{
  return std::flat_multimap<T1, T2, T3, T4, T5>(std::move(a0), a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
std::flat_multimap<T1, T2, T3, T4, T5> f123(std::sorted_equivalent_t a0, T6 a1,
                                            T6 a2, const T7 &a3)
  requires((std::__has_input_iterator_category<T6>::value &&
            std::flat_multimap<T1, T2, T3, T4,
                               T5>::template __allocator_ctor_constraint<T7>))
{
  return std::flat_multimap<T1, T2, T3, T4, T5>(std::move(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_multimap<T1, T2, T3, T4, T5>
f124(std::initializer_list<std::pair<T1, T2>> a0, const T3 &a1, const T6 &a2)
  requires(std::flat_multimap<T1, T2, T3, T4,
                              T5>::template __allocator_ctor_constraint<T6>)
{
  return std::flat_multimap<T1, T2, T3, T4, T5>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_multimap<T1, T2, T3, T4, T5>
f125(std::initializer_list<std::pair<T1, T2>> a0, const T6 &a1)
  requires(std::flat_multimap<T1, T2, T3, T4,
                              T5>::template __allocator_ctor_constraint<T6>) &&
          (!(std::is_same<T6, typename std::flat_multimap<
                                  T1, T2, T3, T4, T5>::key_compare>::value))
{
  return std::flat_multimap<T1, T2, T3, T4, T5>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_multimap<T1, T2, T3, T4, T5>
f126(std::sorted_equivalent_t a0, std::initializer_list<std::pair<T1, T2>> a1,
     const T3 &a2, const T6 &a3)
  requires(std::flat_multimap<T1, T2, T3, T4,
                              T5>::template __allocator_ctor_constraint<T6>)
{
  return std::flat_multimap<T1, T2, T3, T4, T5>(std::move(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_multimap<T1, T2, T3, T4, T5>
f127(std::sorted_equivalent_t a0, std::initializer_list<std::pair<T1, T2>> a1,
     const T6 &a2)
  requires(std::flat_multimap<T1, T2, T3, T4,
                              T5>::template __allocator_ctor_constraint<T6>) &&
          (!(std::is_same<T6, typename std::flat_multimap<
                                  T1, T2, T3, T4, T5>::key_compare>::value))
{
  return std::flat_multimap<T1, T2, T3, T4, T5>(std::move(a0), a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
std::__key_value_iterator<std::flat_multimap<T1, T2, T3, T4, T5>, T4, T5, false>
f128(std::flat_multimap<T1, T2, T3, T4, T5> &a0, T6 &&...a1)
  requires(
      std::is_constructible_v<
          std::pair<
              typename std::flat_multimap<T1, T2, T3, T4, T5>::key_type,
              typename std::flat_multimap<T1, T2, T3, T4, T5>::mapped_type>,
          T6...> &&
      std::is_move_constructible_v<
          typename std::flat_multimap<T1, T2, T3, T4, T5>::key_type> &&
      std::is_move_constructible_v<
          typename std::flat_multimap<T1, T2, T3, T4, T5>::mapped_type>)
{
  return a0.template emplace<T6...>(std::forward<T6>(a1)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
std::__key_value_iterator<std::flat_multimap<T1, T2, T3, T4, T5>, T4, T5, false>
f129(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
     std::__key_value_iterator<std::flat_multimap<T1, T2, T3, T4, T5>, T4, T5,
                               true>
         a1,
     T6 &&...a2)
  requires(std::is_constructible_v<
           std::pair<
               typename std::flat_multimap<T1, T2, T3, T4, T5>::key_type,
               typename std::flat_multimap<T1, T2, T3, T4, T5>::mapped_type>,
           T6...>)
{
  return a0.template emplace_hint<T6...>(a1, std::forward<T6>(a2)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::__key_value_iterator<std::flat_multimap<T1, T2, T3, T4, T5>, T4, T5, false>
f130(std::flat_multimap<T1, T2, T3, T4, T5> &a0, T6 &&a1)
  requires(std::is_constructible_v<
              std::pair<
                  typename std::flat_multimap<T1, T2, T3, T4, T5>::key_type,
                  typename std::flat_multimap<T1, T2, T3, T4, T5>::mapped_type>,
              T6>) &&
          (!(std::is_same<T6, const typename std::flat_multimap<
                                  T1, T2, T3, T4, T5>::value_type &>::value) &&
           !(std::is_same<T6, typename std::flat_multimap<
                                  T1, T2, T3, T4, T5>::value_type>::value) &&
           !(std::is_same<T6, std::initializer_list<typename std::flat_multimap<
                                  T1, T2, T3, T4, T5>::value_type>>::value))
{
  return a0.template insert<T6>(std::forward<T6>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::__key_value_iterator<std::flat_multimap<T1, T2, T3, T4, T5>, T4, T5, false>
f131(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
     std::__key_value_iterator<std::flat_multimap<T1, T2, T3, T4, T5>, T4, T5,
                               true>
         a1,
     T6 &&a2)
  requires(std::is_constructible_v<
              std::pair<
                  typename std::flat_multimap<T1, T2, T3, T4, T5>::key_type,
                  typename std::flat_multimap<T1, T2, T3, T4, T5>::mapped_type>,
              T6>) &&
          (!(std::is_same<T6, const typename std::flat_multimap<
                                  T1, T2, T3, T4, T5>::value_type &>::value) &&
           !(std::is_same<T6, typename std::flat_multimap<
                                  T1, T2, T3, T4, T5>::value_type>::value))
{
  return a0.template insert<T6>(a1, std::forward<T6>(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
void f132(std::flat_multimap<T1, T2, T3, T4, T5> &a0, T6 a1, T6 a2)
  requires(std::__has_input_iterator_category<T6>::value)
{
  return a0.template insert<T6>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
void f133(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
          std::sorted_equivalent_t a1, T6 a2, T6 a3)
  requires(std::__has_input_iterator_category<T6>::value)
{
  return a0.template insert<T6>(std::move(a1), a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
void f134(std::flat_multimap<T1, T2, T3, T4, T5> &a0, T6 &&a1)
  requires(std::_ContainerCompatibleRange<
           T6, typename std::flat_multimap<T1, T2, T3, T4, T5>::value_type>)
{
  return a0.template insert_range<T6>(std::forward<T6>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
void f135(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
          std::sorted_equivalent_t a1, T6 &&a2)
  requires(std::_ContainerCompatibleRange<
           T6, typename std::flat_multimap<T1, T2, T3, T4, T5>::value_type>)
{
  return a0.template insert_range<T6>(std::move(a1), std::forward<T6>(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::size_t f136(std::flat_multimap<T1, T2, T3, T4, T5> &a0, T6 &&a1)
  requires((std::flat_multimap<T1, T2, T3, T4, T5>::__is_compare_transparent &&
            !std::is_convertible_v<T6 &&, typename std::flat_multimap<
                                              T1, T2, T3, T4, T5>::iterator> &&
            !std::is_convertible_v<
                T6 &&, typename std::flat_multimap<T1, T2, T3, T4,
                                                   T5>::const_iterator>)) &&
          (!(std::is_same<T6, typename std::flat_multimap<
                                  T1, T2, T3, T4, T5>::iterator>::value) &&
           !(std::is_same<T6, typename std::flat_multimap<T1, T2, T3, T4, T5>::
                                  const_iterator>::value) &&
           !(std::is_same<T6, const typename std::flat_multimap<
                                  T1, T2, T3, T4, T5>::key_type &>::value))
{
  return a0.template erase<T6>(std::forward<T6>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::__key_value_iterator<std::flat_multimap<T1, T2, T3, T4, T5>, T4, T5, false>
f137(std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(std::flat_multimap<T1, T2, T3, T4, T5>::__is_compare_transparent) &&
          (!(std::is_same<T6, typename std::flat_multimap<
                                  T1, T2, T3, T4, T5>::key_type>::value))
{
  return a0.template find<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::__key_value_iterator<std::flat_multimap<T1, T2, T3, T4, T5>, T4, T5, true>
f138(const std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(std::flat_multimap<T1, T2, T3, T4, T5>::__is_compare_transparent) &&
          (!(std::is_same<T6, typename std::flat_multimap<
                                  T1, T2, T3, T4, T5>::key_type>::value))
{
  return a0.template find<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::size_t f139(const std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(std::flat_multimap<T1, T2, T3, T4, T5>::__is_compare_transparent) &&
          (!(std::is_same<T6, typename std::flat_multimap<
                                  T1, T2, T3, T4, T5>::key_type>::value))
{
  return a0.template count<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
bool f140(const std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(std::flat_multimap<T1, T2, T3, T4, T5>::__is_compare_transparent) &&
          (!(std::is_same<T6, typename std::flat_multimap<
                                  T1, T2, T3, T4, T5>::key_type>::value))
{
  return a0.template contains<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::__key_value_iterator<std::flat_multimap<T1, T2, T3, T4, T5>, T4, T5, false>
f141(std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(std::flat_multimap<T1, T2, T3, T4, T5>::__is_compare_transparent) &&
          (!(std::is_same<T6, typename std::flat_multimap<
                                  T1, T2, T3, T4, T5>::key_type>::value))
{
  return a0.template lower_bound<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::__key_value_iterator<std::flat_multimap<T1, T2, T3, T4, T5>, T4, T5, true>
f142(const std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(std::flat_multimap<T1, T2, T3, T4, T5>::__is_compare_transparent) &&
          (!(std::is_same<T6, typename std::flat_multimap<
                                  T1, T2, T3, T4, T5>::key_type>::value))
{
  return a0.template lower_bound<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::__key_value_iterator<std::flat_multimap<T1, T2, T3, T4, T5>, T4, T5, false>
f143(std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(std::flat_multimap<T1, T2, T3, T4, T5>::__is_compare_transparent) &&
          (!(std::is_same<T6, typename std::flat_multimap<
                                  T1, T2, T3, T4, T5>::key_type>::value))
{
  return a0.template upper_bound<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::__key_value_iterator<std::flat_multimap<T1, T2, T3, T4, T5>, T4, T5, true>
f144(const std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(std::flat_multimap<T1, T2, T3, T4, T5>::__is_compare_transparent) &&
          (!(std::is_same<T6, typename std::flat_multimap<
                                  T1, T2, T3, T4, T5>::key_type>::value))
{
  return a0.template upper_bound<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::pair<std::__key_value_iterator<std::flat_multimap<T1, T2, T3, T4, T5>, T4,
                                    T5, false>,
          std::__key_value_iterator<std::flat_multimap<T1, T2, T3, T4, T5>, T4,
                                    T5, false>>
f145(std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(std::flat_multimap<T1, T2, T3, T4, T5>::__is_compare_transparent) &&
          (!(std::is_same<T6, typename std::flat_multimap<
                                  T1, T2, T3, T4, T5>::key_type>::value))
{
  return a0.template equal_range<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::pair<std::__key_value_iterator<std::flat_multimap<T1, T2, T3, T4, T5>, T4,
                                    T5, true>,
          std::__key_value_iterator<std::flat_multimap<T1, T2, T3, T4, T5>, T4,
                                    T5, true>>
f146(const std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(std::flat_multimap<T1, T2, T3, T4, T5>::__is_compare_transparent) &&
          (!(std::is_same<T6, typename std::flat_multimap<
                                  T1, T2, T3, T4, T5>::key_type>::value))
{
  return a0.template equal_range<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f147(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
          std::flat_multimap<T1, T2, T3, T4, T5> &a1) {
  return swap(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f148(const std::flat_multimap<T1, T2, T3, T4, T5> &a0,
          const std::flat_multimap<T1, T2, T3, T4, T5> &a1) {
  return operator<=>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
bool f149(const std::flat_multimap<T1, T2, T3, T4, T5> &a0,
          const std::flat_multimap<T1, T2, T3, T4, T5> &a1) {
  return operator==(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
bool f150(
    const typename std::flat_multimap<T1, T2, T3, T4, T5>::value_compare &a0,
    std::pair<const T1 &, const T2 &> a1,
    std::pair<const T1 &, const T2 &> a2) {
  return a0.operator()(a1, a2);
}
