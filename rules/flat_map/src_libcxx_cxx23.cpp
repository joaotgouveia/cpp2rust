#include <cstddef>
#include <flat_map>
#include <initializer_list>
#include <iterator>
#include <ranges>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_map<T1, T2, T3, T4, T5> f83() {
  return std::flat_map<T1, T2, T3, T4, T5>();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_map<T1, T2, T3, T4, T5>
f84(const std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return std::flat_map<T1, T2, T3, T4, T5>(a0);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_map<T1, T2, T3, T4, T5> f85(std::flat_map<T1, T2, T3, T4, T5> &&a0) {
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_map<T1, T2, T3, T4, T5> f86(T4 a0, T5 a1, const T3 &a2) {
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0), std::move(a1), a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_map<T1, T2, T3, T4, T5> f87(std::sorted_unique_t a0, T4 a1, T5 a2,
                                      const T3 &a3) {
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0), std::move(a1),
                                           std::move(a2), a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_map<T1, T2, T3, T4, T5> f88(const T3 &a0) {
  return std::flat_map<T1, T2, T3, T4, T5>(a0);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_map<T1, T2, T3, T4, T5>
f89(std::initializer_list<std::pair<T1, T2>> a0, const T3 &a1) {
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_map<T1, T2, T3, T4, T5>
f90(std::sorted_unique_t a0, std::initializer_list<std::pair<T1, T2>> a1,
    const T3 &a2) {
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0), std::move(a1), a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_map<T1, T2, T3, T4, T5> &
f91(std::flat_map<T1, T2, T3, T4, T5> &a0,
    std::initializer_list<std::pair<T1, T2>> a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_map<T1, T2, T3, T4, T5> &
f92(std::flat_map<T1, T2, T3, T4, T5> &a0,
    const std::flat_map<T1, T2, T3, T4, T5> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_map<T1, T2, T3, T4, T5> &f93(std::flat_map<T1, T2, T3, T4, T5> &a0,
                                       std::flat_map<T1, T2, T3, T4, T5> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, false>
f94(std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, true>
f95(const std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, false>
f96(std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, true>
f97(const std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::reverse_iterator<
    std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, false>>
f98(std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::reverse_iterator<
    std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, true>>
f99(const std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::reverse_iterator<
    std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, false>>
f100(std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.rend();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::reverse_iterator<
    std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, true>>
f101(const std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.rend();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, true>
f102(const std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, true>
f103(const std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.cend();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::reverse_iterator<
    std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, true>>
f104(const std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.crbegin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::reverse_iterator<
    std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, true>>
f105(const std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.crend();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
T2 &f106(std::flat_map<T1, T2, T3, T4, T5> &a0, const T1 &a1)
  requires(std::is_constructible_v<
           typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type>)
{
  return a0.operator[](a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
T2 &f107(std::flat_map<T1, T2, T3, T4, T5> &a0, T1 &&a1)
  requires(std::is_constructible_v<
           typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type>)
{
  return a0.operator[](std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
T2 &f108(std::flat_map<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.at(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
const T2 &f109(const std::flat_map<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.at(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::pair<
    std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, false>,
    bool>
f110(std::flat_map<T1, T2, T3, T4, T5> &a0, const std::pair<T1, T2> &a1) {
  return a0.insert(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::pair<
    std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, false>,
    bool>
f111(std::flat_map<T1, T2, T3, T4, T5> &a0, std::pair<T1, T2> &&a1) {
  return a0.insert(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, false>
f112(std::flat_map<T1, T2, T3, T4, T5> &a0,
     std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, true>
         a1,
     const std::pair<T1, T2> &a2) {
  return a0.insert(std::move(a1), a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, false>
f113(std::flat_map<T1, T2, T3, T4, T5> &a0,
     std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, true>
         a1,
     std::pair<T1, T2> &&a2) {
  return a0.insert(std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, false>
f114(std::flat_map<T1, T2, T3, T4, T5> &a0,
     std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, false>
         a1) {
  return a0.erase(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, false>
f115(std::flat_map<T1, T2, T3, T4, T5> &a0,
     std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, true>
         a1) {
  return a0.erase(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::size_t f116(std::flat_map<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, false>
f117(std::flat_map<T1, T2, T3, T4, T5> &a0,
     std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, true>
         a1,
     std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, true>
         a2) {
  return a0.erase(std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
const T4 &f118(const std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.keys();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
const T5 &f119(const std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.values();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, false>
f120(std::flat_map<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, true>
f121(const std::flat_map<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::size_t f122(const std::flat_map<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.count(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
bool f123(const std::flat_map<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.contains(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, false>
f124(std::flat_map<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.lower_bound(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, true>
f125(const std::flat_map<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.lower_bound(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, false>
f126(std::flat_map<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.upper_bound(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, true>
f127(const std::flat_map<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.upper_bound(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::pair<
    std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, false>,
    std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, false>>
f128(std::flat_map<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::pair<
    std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, true>,
    std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, true>>
f129(const std::flat_map<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_map<T1, T2, T3, T4, T5>
f130(const std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(std::flat_map<T1, T2, T3, T4,
                         T5>::template __allocator_ctor_constraint<T6>)
{
  return std::flat_map<T1, T2, T3, T4, T5>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_map<T1, T2, T3, T4, T5> f131(std::flat_map<T1, T2, T3, T4, T5> &&a0,
                                       const T6 &a1)
  requires(std::flat_map<T1, T2, T3, T4,
                         T5>::template __allocator_ctor_constraint<T6>)
{
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_map<T1, T2, T3, T4, T5> f132(const T4 &a0, const T5 &a1, const T6 &a2)
  requires(std::flat_map<T1, T2, T3, T4,
                         T5>::template __allocator_ctor_constraint<T6>)
{
  return std::flat_map<T1, T2, T3, T4, T5>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_map<T1, T2, T3, T4, T5> f133(const T4 &a0, const T5 &a1, const T3 &a2,
                                       const T6 &a3)
  requires(std::flat_map<T1, T2, T3, T4,
                         T5>::template __allocator_ctor_constraint<T6>)
{
  return std::flat_map<T1, T2, T3, T4, T5>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_map<T1, T2, T3, T4, T5> f134(std::sorted_unique_t a0, const T4 &a1,
                                       const T5 &a2, const T6 &a3)
  requires(std::flat_map<T1, T2, T3, T4,
                         T5>::template __allocator_ctor_constraint<T6>)
{
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_map<T1, T2, T3, T4, T5> f135(std::sorted_unique_t a0, const T4 &a1,
                                       const T5 &a2, const T3 &a3, const T6 &a4)
  requires(std::flat_map<T1, T2, T3, T4,
                         T5>::template __allocator_ctor_constraint<T6>)
{
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0), a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_map<T1, T2, T3, T4, T5> f136(const T3 &a0, const T6 &a1)
  requires(std::flat_map<T1, T2, T3, T4,
                         T5>::template __allocator_ctor_constraint<T6>)
{
  return std::flat_map<T1, T2, T3, T4, T5>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_map<T1, T2, T3, T4, T5> f137(const T6 &a0)
  requires(std::flat_map<T1, T2, T3, T4,
                         T5>::template __allocator_ctor_constraint<T6>) &&
          (!(std::is_same<T6, std::flat_map<T1, T2, T3, T4, T5>>::value) &&
           !(std::is_same<T6, typename std::flat_map<T1, T2, T3, T4,
                                                     T5>::key_compare>::value))
{
  return std::flat_map<T1, T2, T3, T4, T5>(a0);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_map<T1, T2, T3, T4, T5> f138(T6 a0, T6 a1, const T3 &a2)
  requires(std::__has_input_iterator_category<T6>::value)
{
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0), std::move(a1), a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
std::flat_map<T1, T2, T3, T4, T5> f139(T6 a0, T6 a1, const T3 &a2, const T7 &a3)
  requires((std::__has_input_iterator_category<T6>::value &&
            std::flat_map<T1, T2, T3, T4,
                          T5>::template __allocator_ctor_constraint<T7>))
{
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0), std::move(a1), a2,
                                           a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
std::flat_map<T1, T2, T3, T4, T5> f140(T6 a0, T6 a1, const T7 &a2)
  requires((std::__has_input_iterator_category<T6>::value &&
            std::flat_map<T1, T2, T3, T4,
                          T5>::template __allocator_ctor_constraint<T7>))
{
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0), std::move(a1), a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_map<T1, T2, T3, T4, T5> f141(std::from_range_t a0, T6 &&a1)
  requires(std::_ContainerCompatibleRange<
           T6, typename std::flat_map<T1, T2, T3, T4, T5>::value_type>)
{
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0), std::forward<T6>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
std::flat_map<T1, T2, T3, T4, T5> f142(std::from_range_t a0, T6 &&a1,
                                       const T7 &a2)
  requires(std::flat_map<T1, T2, T3, T4,
                         T5>::template __allocator_ctor_constraint<T7>) &&
          (std::_ContainerCompatibleRange<
              T6, typename std::flat_map<T1, T2, T3, T4, T5>::value_type>)
{
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0), std::forward<T6>(a1),
                                           a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_map<T1, T2, T3, T4, T5> f143(std::from_range_t a0, T6 &&a1,
                                       const T3 &a2)
  requires(std::_ContainerCompatibleRange<
           T6, typename std::flat_map<T1, T2, T3, T4, T5>::value_type>)
{
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0), std::forward<T6>(a1),
                                           a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
std::flat_map<T1, T2, T3, T4, T5> f144(std::from_range_t a0, T6 &&a1,
                                       const T3 &a2, const T7 &a3)
  requires(std::flat_map<T1, T2, T3, T4,
                         T5>::template __allocator_ctor_constraint<T7>) &&
          (std::_ContainerCompatibleRange<
              T6, typename std::flat_map<T1, T2, T3, T4, T5>::value_type>)
{
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0), std::forward<T6>(a1),
                                           a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_map<T1, T2, T3, T4, T5> f145(std::sorted_unique_t a0, T6 a1, T6 a2,
                                       const T3 &a3)
  requires(std::__has_input_iterator_category<T6>::value)
{
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0), std::move(a1),
                                           std::move(a2), a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
std::flat_map<T1, T2, T3, T4, T5> f146(std::sorted_unique_t a0, T6 a1, T6 a2,
                                       const T3 &a3, const T7 &a4)
  requires((std::__has_input_iterator_category<T6>::value &&
            std::flat_map<T1, T2, T3, T4,
                          T5>::template __allocator_ctor_constraint<T7>))
{
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0), std::move(a1),
                                           std::move(a2), a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
std::flat_map<T1, T2, T3, T4, T5> f147(std::sorted_unique_t a0, T6 a1, T6 a2,
                                       const T7 &a3)
  requires((std::__has_input_iterator_category<T6>::value &&
            std::flat_map<T1, T2, T3, T4,
                          T5>::template __allocator_ctor_constraint<T7>))
{
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0), std::move(a1),
                                           std::move(a2), a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_map<T1, T2, T3, T4, T5>
f148(std::initializer_list<std::pair<T1, T2>> a0, const T3 &a1, const T6 &a2)
  requires(std::flat_map<T1, T2, T3, T4,
                         T5>::template __allocator_ctor_constraint<T6>)
{
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0), a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_map<T1, T2, T3, T4, T5>
f149(std::initializer_list<std::pair<T1, T2>> a0, const T6 &a1)
  requires(std::flat_map<T1, T2, T3, T4,
                         T5>::template __allocator_ctor_constraint<T6>) &&
          (!(std::is_same<T6, typename std::flat_map<T1, T2, T3, T4,
                                                     T5>::key_compare>::value))
{
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_map<T1, T2, T3, T4, T5>
f150(std::sorted_unique_t a0, std::initializer_list<std::pair<T1, T2>> a1,
     const T3 &a2, const T6 &a3)
  requires(std::flat_map<T1, T2, T3, T4,
                         T5>::template __allocator_ctor_constraint<T6>)
{
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0), std::move(a1), a2,
                                           a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_map<T1, T2, T3, T4, T5>
f151(std::sorted_unique_t a0, std::initializer_list<std::pair<T1, T2>> a1,
     const T6 &a2)
  requires(std::flat_map<T1, T2, T3, T4,
                         T5>::template __allocator_ctor_constraint<T6>) &&
          (!(std::is_same<T6, typename std::flat_map<T1, T2, T3, T4,
                                                     T5>::key_compare>::value))
{
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0), std::move(a1), a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
T2 &f152(std::flat_map<T1, T2, T3, T4, T5> &a0, T6 &&a1)
  requires((std::flat_map<T1, T2, T3, T4, T5>::__is_compare_transparent &&
            std::is_constructible_v<
                typename std::flat_map<T1, T2, T3, T4, T5>::key_type, T6> &&
            std::is_constructible_v<
                typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type> &&
            !std::is_convertible_v<
                T6 &&,
                typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator> &&
            !std::is_convertible_v<
                T6 &&,
                typename std::flat_map<T1, T2, T3, T4, T5>::iterator>)) &&
          (!(std::is_same<T6, const typename std::flat_map<
                                  T1, T2, T3, T4, T5>::key_type &>::value) &&
           !(std::is_same<T6, typename std::flat_map<T1, T2, T3, T4,
                                                     T5>::key_type>::value))
{
  return a0.template operator[]<T6>(std::forward<T6>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
T2 &f153(std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(std::flat_map<T1, T2, T3, T4, T5>::__is_compare_transparent) &&
          (!(std::is_same<
              T6, typename std::flat_map<T1, T2, T3, T4, T5>::key_type>::value))
{
  return a0.template at<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
const T2 &f154(const std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(std::flat_map<T1, T2, T3, T4, T5>::__is_compare_transparent) &&
          (!(std::is_same<
              T6, typename std::flat_map<T1, T2, T3, T4, T5>::key_type>::value))
{
  return a0.template at<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
std::pair<
    std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, false>,
    bool>
f155(std::flat_map<T1, T2, T3, T4, T5> &a0, T6 &&...a1)
  requires(std::is_constructible_v<
           std::pair<typename std::flat_map<T1, T2, T3, T4, T5>::key_type,
                     typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type>,
           T6...>)
{
  return a0.template emplace<T6...>(std::forward<T6>(a1)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, false>
f156(std::flat_map<T1, T2, T3, T4, T5> &a0,
     std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, true>
         a1,
     T6 &&...a2)
  requires(std::is_constructible_v<
           std::pair<typename std::flat_map<T1, T2, T3, T4, T5>::key_type,
                     typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type>,
           T6...>)
{
  return a0.template emplace_hint<T6...>(std::move(a1),
                                         std::forward<T6>(a2)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::pair<
    std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, false>,
    bool>
f157(std::flat_map<T1, T2, T3, T4, T5> &a0, T6 &&a1)
  requires(std::is_constructible_v<
              std::pair<
                  typename std::flat_map<T1, T2, T3, T4, T5>::key_type,
                  typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type>,
              T6>) &&
          (!(std::is_same<T6, const typename std::flat_map<
                                  T1, T2, T3, T4, T5>::value_type &>::value) &&
           !(std::is_same<T6, typename std::flat_map<T1, T2, T3, T4,
                                                     T5>::value_type>::value) &&
           !(std::is_same<T6, std::initializer_list<typename std::flat_map<
                                  T1, T2, T3, T4, T5>::value_type>>::value))
{
  return a0.template insert<T6>(std::forward<T6>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, false>
f158(std::flat_map<T1, T2, T3, T4, T5> &a0,
     std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, true>
         a1,
     T6 &&a2)
  requires(std::is_constructible_v<
              std::pair<
                  typename std::flat_map<T1, T2, T3, T4, T5>::key_type,
                  typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type>,
              T6>) &&
          (!(std::is_same<T6, const typename std::flat_map<
                                  T1, T2, T3, T4, T5>::value_type &>::value) &&
           !(std::is_same<T6, typename std::flat_map<T1, T2, T3, T4,
                                                     T5>::value_type>::value))
{
  return a0.template insert<T6>(std::move(a1), std::forward<T6>(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
void f159(std::flat_map<T1, T2, T3, T4, T5> &a0, T6 a1, T6 a2)
  requires(std::__has_input_iterator_category<T6>::value)
{
  return a0.template insert<T6>(std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
void f160(std::flat_map<T1, T2, T3, T4, T5> &a0, std::sorted_unique_t a1, T6 a2,
          T6 a3)
  requires(std::__has_input_iterator_category<T6>::value)
{
  return a0.template insert<T6>(std::move(a1), std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
void f161(std::flat_map<T1, T2, T3, T4, T5> &a0, T6 &&a1)
  requires(std::_ContainerCompatibleRange<
           T6, typename std::flat_map<T1, T2, T3, T4, T5>::value_type>)
{
  return a0.template insert_range<T6>(std::forward<T6>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
void f162(std::flat_map<T1, T2, T3, T4, T5> &a0, std::sorted_unique_t a1,
          T6 &&a2)
  requires(std::_ContainerCompatibleRange<
           T6, typename std::flat_map<T1, T2, T3, T4, T5>::value_type>)
{
  return a0.template insert_range<T6>(std::move(a1), std::forward<T6>(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
std::pair<
    std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, false>,
    bool>
f163(std::flat_map<T1, T2, T3, T4, T5> &a0, const T1 &a1, T6 &&...a2)
  requires(std::is_constructible_v<
           typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type, T6...>)
{
  return a0.template try_emplace<T6...>(a1, std::forward<T6>(a2)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
std::pair<
    std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, false>,
    bool>
f164(std::flat_map<T1, T2, T3, T4, T5> &a0, T1 &&a1, T6 &&...a2)
  requires(std::is_constructible_v<
           typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type, T6...>)
{
  return a0.template try_emplace<T6...>(std::move(a1), std::forward<T6>(a2)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename... T7>
std::pair<
    std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, false>,
    bool>
f165(std::flat_map<T1, T2, T3, T4, T5> &a0, T6 &&a1, T7 &&...a2)
  requires(
      (std::flat_map<T1, T2, T3, T4, T5>::__is_compare_transparent &&
       std::is_constructible_v<
           typename std::flat_map<T1, T2, T3, T4, T5>::key_type, T6> &&
       std::is_constructible_v<
           typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type, T7...> &&
       !std::is_convertible_v<
           T6 &&, typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator> &&
       !std::is_convertible_v<
           T6 &&, typename std::flat_map<T1, T2, T3, T4, T5>::iterator>))
{
  return a0.template try_emplace<T6, T7...>(std::forward<T6>(a1),
                                            std::forward<T7>(a2)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, false>
f166(std::flat_map<T1, T2, T3, T4, T5> &a0,
     std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, true>
         a1,
     const T1 &a2, T6 &&...a3)
  requires(std::is_constructible_v<
           typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type, T6...>)
{
  return a0.template try_emplace<T6...>(std::move(a1), a2,
                                        std::forward<T6>(a3)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, false>
f167(std::flat_map<T1, T2, T3, T4, T5> &a0,
     std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, true>
         a1,
     T1 &&a2, T6 &&...a3)
  requires(std::is_constructible_v<
           typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type, T6...>)
{
  return a0.template try_emplace<T6...>(std::move(a1), std::move(a2),
                                        std::forward<T6>(a3)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename... T7>
std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, false>
f168(std::flat_map<T1, T2, T3, T4, T5> &a0,
     std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, true>
         a1,
     T6 &&a2, T7 &&...a3)
  requires(std::flat_map<T1, T2, T3, T4, T5>::__is_compare_transparent &&
           std::is_constructible_v<
               typename std::flat_map<T1, T2, T3, T4, T5>::key_type, T6> &&
           std::is_constructible_v<
               typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type, T7...>)
{
  return a0.template try_emplace<T6, T7...>(std::move(a1), std::forward<T6>(a2),
                                            std::forward<T7>(a3)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::pair<
    std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, false>,
    bool>
f169(std::flat_map<T1, T2, T3, T4, T5> &a0, const T1 &a1, T6 &&a2)
  requires(std::is_assignable_v<
               typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type &, T6> &&
           std::is_constructible_v<
               typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type, T6>)
{
  return a0.template insert_or_assign<T6>(a1, std::forward<T6>(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::pair<
    std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, false>,
    bool>
f170(std::flat_map<T1, T2, T3, T4, T5> &a0, T1 &&a1, T6 &&a2)
  requires(std::is_assignable_v<
               typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type &, T6> &&
           std::is_constructible_v<
               typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type, T6>)
{
  return a0.template insert_or_assign<T6>(std::move(a1), std::forward<T6>(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
std::pair<
    std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, false>,
    bool>
f171(std::flat_map<T1, T2, T3, T4, T5> &a0, T6 &&a1, T7 &&a2)
  requires(std::flat_map<T1, T2, T3, T4, T5>::__is_compare_transparent &&
           std::is_constructible_v<
               typename std::flat_map<T1, T2, T3, T4, T5>::key_type, T6> &&
           std::is_assignable_v<
               typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type &, T7> &&
           std::is_constructible_v<
               typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type, T7>)
{
  return a0.template insert_or_assign<T6, T7>(std::forward<T6>(a1),
                                              std::forward<T7>(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, false>
f172(std::flat_map<T1, T2, T3, T4, T5> &a0,
     std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, true>
         a1,
     const T1 &a2, T6 &&a3)
  requires(std::is_assignable_v<
               typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type &, T6> &&
           std::is_constructible_v<
               typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type, T6>)
{
  return a0.template insert_or_assign<T6>(std::move(a1), a2,
                                          std::forward<T6>(a3));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, false>
f173(std::flat_map<T1, T2, T3, T4, T5> &a0,
     std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, true>
         a1,
     T1 &&a2, T6 &&a3)
  requires(std::is_assignable_v<
               typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type &, T6> &&
           std::is_constructible_v<
               typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type, T6>)
{
  return a0.template insert_or_assign<T6>(std::move(a1), std::move(a2),
                                          std::forward<T6>(a3));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, false>
f174(std::flat_map<T1, T2, T3, T4, T5> &a0,
     std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, true>
         a1,
     T6 &&a2, T7 &&a3)
  requires(std::flat_map<T1, T2, T3, T4, T5>::__is_compare_transparent &&
           std::is_constructible_v<
               typename std::flat_map<T1, T2, T3, T4, T5>::key_type, T6> &&
           std::is_assignable_v<
               typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type &, T7> &&
           std::is_constructible_v<
               typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type, T7>)
{
  return a0.template insert_or_assign<T6, T7>(
      std::move(a1), std::forward<T6>(a2), std::forward<T7>(a3));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::size_t f175(std::flat_map<T1, T2, T3, T4, T5> &a0, T6 &&a1)
  requires((std::flat_map<T1, T2, T3, T4, T5>::__is_compare_transparent &&
            !std::is_convertible_v<
                T6 &&, typename std::flat_map<T1, T2, T3, T4, T5>::iterator> &&
            !std::is_convertible_v<
                T6 &&,
                typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator>)) &&
          (!(std::is_same<T6, typename std::flat_map<T1, T2, T3, T4,
                                                     T5>::iterator>::value) &&
           !(std::is_same<T6, typename std::flat_map<T1, T2, T3, T4, T5>::
                                  const_iterator>::value) &&
           !(std::is_same<T6, const typename std::flat_map<
                                  T1, T2, T3, T4, T5>::key_type &>::value))
{
  return a0.template erase<T6>(std::forward<T6>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, false>
f176(std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(std::flat_map<T1, T2, T3, T4, T5>::__is_compare_transparent) &&
          (!(std::is_same<
              T6, typename std::flat_map<T1, T2, T3, T4, T5>::key_type>::value))
{
  return a0.template find<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, true>
f177(const std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(std::flat_map<T1, T2, T3, T4, T5>::__is_compare_transparent) &&
          (!(std::is_same<
              T6, typename std::flat_map<T1, T2, T3, T4, T5>::key_type>::value))
{
  return a0.template find<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::size_t f178(const std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(std::flat_map<T1, T2, T3, T4, T5>::__is_compare_transparent) &&
          (!(std::is_same<
              T6, typename std::flat_map<T1, T2, T3, T4, T5>::key_type>::value))
{
  return a0.template count<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
bool f179(const std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(std::flat_map<T1, T2, T3, T4, T5>::__is_compare_transparent) &&
          (!(std::is_same<
              T6, typename std::flat_map<T1, T2, T3, T4, T5>::key_type>::value))
{
  return a0.template contains<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, false>
f180(std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(std::flat_map<T1, T2, T3, T4, T5>::__is_compare_transparent) &&
          (!(std::is_same<
              T6, typename std::flat_map<T1, T2, T3, T4, T5>::key_type>::value))
{
  return a0.template lower_bound<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, true>
f181(const std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(std::flat_map<T1, T2, T3, T4, T5>::__is_compare_transparent) &&
          (!(std::is_same<
              T6, typename std::flat_map<T1, T2, T3, T4, T5>::key_type>::value))
{
  return a0.template lower_bound<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, false>
f182(std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(std::flat_map<T1, T2, T3, T4, T5>::__is_compare_transparent) &&
          (!(std::is_same<
              T6, typename std::flat_map<T1, T2, T3, T4, T5>::key_type>::value))
{
  return a0.template upper_bound<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, true>
f183(const std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(std::flat_map<T1, T2, T3, T4, T5>::__is_compare_transparent) &&
          (!(std::is_same<
              T6, typename std::flat_map<T1, T2, T3, T4, T5>::key_type>::value))
{
  return a0.template upper_bound<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::pair<
    std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, false>,
    std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, false>>
f184(std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(std::flat_map<T1, T2, T3, T4, T5>::__is_compare_transparent) &&
          (!(std::is_same<
              T6, typename std::flat_map<T1, T2, T3, T4, T5>::key_type>::value))
{
  return a0.template equal_range<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::pair<
    std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, true>,
    std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, true>>
f185(const std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(std::flat_map<T1, T2, T3, T4, T5>::__is_compare_transparent) &&
          (!(std::is_same<
              T6, typename std::flat_map<T1, T2, T3, T4, T5>::key_type>::value))
{
  return a0.template equal_range<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f186(std::flat_map<T1, T2, T3, T4, T5> &a0,
          std::flat_map<T1, T2, T3, T4, T5> &a1) {
  return swap(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f187(const std::flat_map<T1, T2, T3, T4, T5> &a0,
          const std::flat_map<T1, T2, T3, T4, T5> &a1) {
  return operator<=>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
bool f188(const std::flat_map<T1, T2, T3, T4, T5> &a0,
          const std::flat_map<T1, T2, T3, T4, T5> &a1) {
  return operator==(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
bool f189(const typename std::flat_map<T1, T2, T3, T4, T5>::value_compare &a0,
          std::pair<const T1 &, const T2 &> a1,
          std::pair<const T1 &, const T2 &> a2) {
  return a0.operator()(std::move(a1), std::move(a2));
}
