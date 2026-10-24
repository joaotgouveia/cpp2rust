#include <cstddef>
#include <flat_map>
#include <initializer_list>
#include <iterator>
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
std::flat_map<T1, T2, T3, T4, T5> f25(T4 a0, T5 a1, const T3 &a2) {
  return std::flat_map<T1, T2, T3, T4, T5>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_map<T1, T2, T3, T4, T5> f26(std::sorted_unique_t a0, T4 a1, T5 a2,
                                      const T3 &a3) {
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_map<T1, T2, T3, T4, T5> f27(const T3 &a0) {
  return std::flat_map<T1, T2, T3, T4, T5>(a0);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_map<T1, T2, T3, T4, T5>
f28(std::initializer_list<std::pair<T1, T2>> a0, const T3 &a1) {
  return std::flat_map<T1, T2, T3, T4, T5>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_map<T1, T2, T3, T4, T5>
f29(std::sorted_unique_t a0, std::initializer_list<std::pair<T1, T2>> a1,
    const T3 &a2) {
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0), a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_map<T1, T2, T3, T4, T5> &
f30(std::flat_map<T1, T2, T3, T4, T5> &a0,
    std::initializer_list<std::pair<T1, T2>> a1) {
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
std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, false>
f33(std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, true>
f34(const std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, false>
f35(std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, true>
f36(const std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::reverse_iterator<
    std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, false>>
f37(std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::reverse_iterator<
    std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, true>>
f38(const std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::reverse_iterator<
    std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, false>>
f39(std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.rend();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::reverse_iterator<
    std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, true>>
f40(const std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.rend();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, true>
f41(const std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, true>
f42(const std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.cend();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::reverse_iterator<
    std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, true>>
f43(const std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.crbegin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::reverse_iterator<
    std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, true>>
f44(const std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.crend();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
bool f45(const std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.empty();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::size_t f46(const std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.size();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::size_t f47(const std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
T2 &f48(std::flat_map<T1, T2, T3, T4, T5> &a0, const T1 &a1)
  requires(std::is_constructible_v<
           typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type>)
{
  return a0.operator[](a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
T2 &f49(std::flat_map<T1, T2, T3, T4, T5> &a0, T1 &&a1)
  requires(std::is_constructible_v<
           typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type>)
{
  return a0.operator[](std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
T2 &f50(std::flat_map<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.at(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
const T2 &f51(const std::flat_map<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.at(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::pair<
    std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, false>,
    bool>
f52(std::flat_map<T1, T2, T3, T4, T5> &a0, const std::pair<T1, T2> &a1) {
  return a0.insert(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::pair<
    std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, false>,
    bool>
f53(std::flat_map<T1, T2, T3, T4, T5> &a0, std::pair<T1, T2> &&a1) {
  return a0.insert(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, false>
f54(std::flat_map<T1, T2, T3, T4, T5> &a0,
    std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, true>
        a1,
    const std::pair<T1, T2> &a2) {
  return a0.insert(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, false>
f55(std::flat_map<T1, T2, T3, T4, T5> &a0,
    std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, true>
        a1,
    std::pair<T1, T2> &&a2) {
  return a0.insert(a1, std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f56(std::flat_map<T1, T2, T3, T4, T5> &a0,
         std::initializer_list<std::pair<T1, T2>> a1) {
  return a0.insert(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f57(std::flat_map<T1, T2, T3, T4, T5> &a0, std::sorted_unique_t a1,
         std::initializer_list<std::pair<T1, T2>> a2) {
  return a0.insert(std::move(a1), a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::containers
f58(std::flat_map<T1, T2, T3, T4, T5> &&a0) {
  return std::move(a0).extract();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f59(std::flat_map<T1, T2, T3, T4, T5> &a0, T4 &&a1, T5 &&a2) {
  return a0.replace(std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, false>
f60(std::flat_map<T1, T2, T3, T4, T5> &a0,
    std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, false>
        a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, false>
f61(std::flat_map<T1, T2, T3, T4, T5> &a0,
    std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, true>
        a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::size_t f62(std::flat_map<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, false>
f63(std::flat_map<T1, T2, T3, T4, T5> &a0,
    std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, true>
        a1,
    std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, true>
        a2) {
  return a0.erase(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f64(std::flat_map<T1, T2, T3, T4, T5> &a0,
         std::flat_map<T1, T2, T3, T4, T5> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f65(std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.clear();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
T3 f66(const std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.key_comp();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::value_compare
f67(const std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.value_comp();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
const T4 &f68(const std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.keys();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
const T5 &f69(const std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.values();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, false>
f70(std::flat_map<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, true>
f71(const std::flat_map<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::size_t f72(const std::flat_map<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.count(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
bool f73(const std::flat_map<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.contains(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, false>
f74(std::flat_map<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.lower_bound(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, true>
f75(const std::flat_map<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.lower_bound(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, false>
f76(std::flat_map<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.upper_bound(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, true>
f77(const std::flat_map<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.upper_bound(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::pair<
    std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, false>,
    std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, false>>
f78(std::flat_map<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::pair<
    std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, true>,
    std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, true>>
f79(const std::flat_map<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_map<T1, T2, T3, T4, T5>
f80(const std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(std::flat_map<T1, T2, T3, T4,
                         T5>::template __allocator_ctor_constraint<T6>)
{
  return std::flat_map<T1, T2, T3, T4, T5>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_map<T1, T2, T3, T4, T5> f81(std::flat_map<T1, T2, T3, T4, T5> &&a0,
                                      const T6 &a1)
  requires(std::flat_map<T1, T2, T3, T4,
                         T5>::template __allocator_ctor_constraint<T6>)
{
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_map<T1, T2, T3, T4, T5> f82(const T4 &a0, const T5 &a1, const T6 &a2)
  requires(std::flat_map<T1, T2, T3, T4,
                         T5>::template __allocator_ctor_constraint<T6>)
{
  return std::flat_map<T1, T2, T3, T4, T5>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_map<T1, T2, T3, T4, T5> f83(const T4 &a0, const T5 &a1, const T3 &a2,
                                      const T6 &a3)
  requires(std::flat_map<T1, T2, T3, T4,
                         T5>::template __allocator_ctor_constraint<T6>)
{
  return std::flat_map<T1, T2, T3, T4, T5>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_map<T1, T2, T3, T4, T5> f84(std::sorted_unique_t a0, const T4 &a1,
                                      const T5 &a2, const T6 &a3)
  requires(std::flat_map<T1, T2, T3, T4,
                         T5>::template __allocator_ctor_constraint<T6>)
{
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_map<T1, T2, T3, T4, T5> f85(std::sorted_unique_t a0, const T4 &a1,
                                      const T5 &a2, const T3 &a3, const T6 &a4)
  requires(std::flat_map<T1, T2, T3, T4,
                         T5>::template __allocator_ctor_constraint<T6>)
{
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0), a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_map<T1, T2, T3, T4, T5> f86(const T3 &a0, const T6 &a1)
  requires(std::flat_map<T1, T2, T3, T4,
                         T5>::template __allocator_ctor_constraint<T6>)
{
  return std::flat_map<T1, T2, T3, T4, T5>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_map<T1, T2, T3, T4, T5> f87(const T6 &a0)
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
std::flat_map<T1, T2, T3, T4, T5> f88(T6 a0, T6 a1, const T3 &a2)
  requires(std::__has_input_iterator_category<T6>::value)
{
  return std::flat_map<T1, T2, T3, T4, T5>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
std::flat_map<T1, T2, T3, T4, T5> f89(T6 a0, T6 a1, const T3 &a2, const T7 &a3)
  requires((std::__has_input_iterator_category<T6>::value &&
            std::flat_map<T1, T2, T3, T4,
                          T5>::template __allocator_ctor_constraint<T7>))
{
  return std::flat_map<T1, T2, T3, T4, T5>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
std::flat_map<T1, T2, T3, T4, T5> f90(T6 a0, T6 a1, const T7 &a2)
  requires((std::__has_input_iterator_category<T6>::value &&
            std::flat_map<T1, T2, T3, T4,
                          T5>::template __allocator_ctor_constraint<T7>))
{
  return std::flat_map<T1, T2, T3, T4, T5>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_map<T1, T2, T3, T4, T5> f91(std::from_range_t a0, T6 &&a1)
  requires(std::_ContainerCompatibleRange<
           T6, typename std::flat_map<T1, T2, T3, T4, T5>::value_type>)
{
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0), std::forward<T6>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
std::flat_map<T1, T2, T3, T4, T5> f92(std::from_range_t a0, T6 &&a1,
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
std::flat_map<T1, T2, T3, T4, T5> f93(std::from_range_t a0, T6 &&a1,
                                      const T3 &a2)
  requires(std::_ContainerCompatibleRange<
           T6, typename std::flat_map<T1, T2, T3, T4, T5>::value_type>)
{
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0), std::forward<T6>(a1),
                                           a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
std::flat_map<T1, T2, T3, T4, T5> f94(std::from_range_t a0, T6 &&a1,
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
std::flat_map<T1, T2, T3, T4, T5> f95(std::sorted_unique_t a0, T6 a1, T6 a2,
                                      const T3 &a3)
  requires(std::__has_input_iterator_category<T6>::value)
{
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
std::flat_map<T1, T2, T3, T4, T5> f96(std::sorted_unique_t a0, T6 a1, T6 a2,
                                      const T3 &a3, const T7 &a4)
  requires((std::__has_input_iterator_category<T6>::value &&
            std::flat_map<T1, T2, T3, T4,
                          T5>::template __allocator_ctor_constraint<T7>))
{
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0), a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
std::flat_map<T1, T2, T3, T4, T5> f97(std::sorted_unique_t a0, T6 a1, T6 a2,
                                      const T7 &a3)
  requires((std::__has_input_iterator_category<T6>::value &&
            std::flat_map<T1, T2, T3, T4,
                          T5>::template __allocator_ctor_constraint<T7>))
{
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_map<T1, T2, T3, T4, T5>
f98(std::initializer_list<std::pair<T1, T2>> a0, const T3 &a1, const T6 &a2)
  requires(std::flat_map<T1, T2, T3, T4,
                         T5>::template __allocator_ctor_constraint<T6>)
{
  return std::flat_map<T1, T2, T3, T4, T5>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_map<T1, T2, T3, T4, T5>
f99(std::initializer_list<std::pair<T1, T2>> a0, const T6 &a1)
  requires(std::flat_map<T1, T2, T3, T4,
                         T5>::template __allocator_ctor_constraint<T6>) &&
          (!(std::is_same<T6, typename std::flat_map<T1, T2, T3, T4,
                                                     T5>::key_compare>::value))
{
  return std::flat_map<T1, T2, T3, T4, T5>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_map<T1, T2, T3, T4, T5>
f100(std::sorted_unique_t a0, std::initializer_list<std::pair<T1, T2>> a1,
     const T3 &a2, const T6 &a3)
  requires(std::flat_map<T1, T2, T3, T4,
                         T5>::template __allocator_ctor_constraint<T6>)
{
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_map<T1, T2, T3, T4, T5>
f101(std::sorted_unique_t a0, std::initializer_list<std::pair<T1, T2>> a1,
     const T6 &a2)
  requires(std::flat_map<T1, T2, T3, T4,
                         T5>::template __allocator_ctor_constraint<T6>) &&
          (!(std::is_same<T6, typename std::flat_map<T1, T2, T3, T4,
                                                     T5>::key_compare>::value))
{
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0), a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
T2 &f102(std::flat_map<T1, T2, T3, T4, T5> &a0, T6 &&a1)
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
T2 &f103(std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(std::flat_map<T1, T2, T3, T4, T5>::__is_compare_transparent) &&
          (!(std::is_same<
              T6, typename std::flat_map<T1, T2, T3, T4, T5>::key_type>::value))
{
  return a0.template at<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
const T2 &f104(const std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
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
f105(std::flat_map<T1, T2, T3, T4, T5> &a0, T6 &&...a1)
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
f106(std::flat_map<T1, T2, T3, T4, T5> &a0,
     std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, true>
         a1,
     T6 &&...a2)
  requires(std::is_constructible_v<
           std::pair<typename std::flat_map<T1, T2, T3, T4, T5>::key_type,
                     typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type>,
           T6...>)
{
  return a0.template emplace_hint<T6...>(a1, std::forward<T6>(a2)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::pair<
    std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, false>,
    bool>
f107(std::flat_map<T1, T2, T3, T4, T5> &a0, T6 &&a1)
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
f108(std::flat_map<T1, T2, T3, T4, T5> &a0,
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
  return a0.template insert<T6>(a1, std::forward<T6>(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
void f109(std::flat_map<T1, T2, T3, T4, T5> &a0, T6 a1, T6 a2)
  requires(std::__has_input_iterator_category<T6>::value)
{
  return a0.template insert<T6>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
void f110(std::flat_map<T1, T2, T3, T4, T5> &a0, std::sorted_unique_t a1, T6 a2,
          T6 a3)
  requires(std::__has_input_iterator_category<T6>::value)
{
  return a0.template insert<T6>(std::move(a1), a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
void f111(std::flat_map<T1, T2, T3, T4, T5> &a0, T6 &&a1)
  requires(std::_ContainerCompatibleRange<
           T6, typename std::flat_map<T1, T2, T3, T4, T5>::value_type>)
{
  return a0.template insert_range<T6>(std::forward<T6>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
void f112(std::flat_map<T1, T2, T3, T4, T5> &a0, std::sorted_unique_t a1,
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
f113(std::flat_map<T1, T2, T3, T4, T5> &a0, const T1 &a1, T6 &&...a2)
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
f114(std::flat_map<T1, T2, T3, T4, T5> &a0, T1 &&a1, T6 &&...a2)
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
f115(std::flat_map<T1, T2, T3, T4, T5> &a0, T6 &&a1, T7 &&...a2)
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
f116(std::flat_map<T1, T2, T3, T4, T5> &a0,
     std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, true>
         a1,
     const T1 &a2, T6 &&...a3)
  requires(std::is_constructible_v<
           typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type, T6...>)
{
  return a0.template try_emplace<T6...>(a1, a2, std::forward<T6>(a3)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, false>
f117(std::flat_map<T1, T2, T3, T4, T5> &a0,
     std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, true>
         a1,
     T1 &&a2, T6 &&...a3)
  requires(std::is_constructible_v<
           typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type, T6...>)
{
  return a0.template try_emplace<T6...>(a1, std::move(a2),
                                        std::forward<T6>(a3)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename... T7>
std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, false>
f118(std::flat_map<T1, T2, T3, T4, T5> &a0,
     std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, true>
         a1,
     T6 &&a2, T7 &&...a3)
  requires(std::flat_map<T1, T2, T3, T4, T5>::__is_compare_transparent &&
           std::is_constructible_v<
               typename std::flat_map<T1, T2, T3, T4, T5>::key_type, T6> &&
           std::is_constructible_v<
               typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type, T7...>)
{
  return a0.template try_emplace<T6, T7...>(a1, std::forward<T6>(a2),
                                            std::forward<T7>(a3)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::pair<
    std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, false>,
    bool>
f119(std::flat_map<T1, T2, T3, T4, T5> &a0, const T1 &a1, T6 &&a2)
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
f120(std::flat_map<T1, T2, T3, T4, T5> &a0, T1 &&a1, T6 &&a2)
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
f121(std::flat_map<T1, T2, T3, T4, T5> &a0, T6 &&a1, T7 &&a2)
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
f122(std::flat_map<T1, T2, T3, T4, T5> &a0,
     std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, true>
         a1,
     const T1 &a2, T6 &&a3)
  requires(std::is_assignable_v<
               typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type &, T6> &&
           std::is_constructible_v<
               typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type, T6>)
{
  return a0.template insert_or_assign<T6>(a1, a2, std::forward<T6>(a3));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, false>
f123(std::flat_map<T1, T2, T3, T4, T5> &a0,
     std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, true>
         a1,
     T1 &&a2, T6 &&a3)
  requires(std::is_assignable_v<
               typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type &, T6> &&
           std::is_constructible_v<
               typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type, T6>)
{
  return a0.template insert_or_assign<T6>(a1, std::move(a2),
                                          std::forward<T6>(a3));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, false>
f124(std::flat_map<T1, T2, T3, T4, T5> &a0,
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
  return a0.template insert_or_assign<T6, T7>(a1, std::forward<T6>(a2),
                                              std::forward<T7>(a3));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::size_t f125(std::flat_map<T1, T2, T3, T4, T5> &a0, T6 &&a1)
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
f126(std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(std::flat_map<T1, T2, T3, T4, T5>::__is_compare_transparent) &&
          (!(std::is_same<
              T6, typename std::flat_map<T1, T2, T3, T4, T5>::key_type>::value))
{
  return a0.template find<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, true>
f127(const std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(std::flat_map<T1, T2, T3, T4, T5>::__is_compare_transparent) &&
          (!(std::is_same<
              T6, typename std::flat_map<T1, T2, T3, T4, T5>::key_type>::value))
{
  return a0.template find<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::size_t f128(const std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(std::flat_map<T1, T2, T3, T4, T5>::__is_compare_transparent) &&
          (!(std::is_same<
              T6, typename std::flat_map<T1, T2, T3, T4, T5>::key_type>::value))
{
  return a0.template count<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
bool f129(const std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(std::flat_map<T1, T2, T3, T4, T5>::__is_compare_transparent) &&
          (!(std::is_same<
              T6, typename std::flat_map<T1, T2, T3, T4, T5>::key_type>::value))
{
  return a0.template contains<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, false>
f130(std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(std::flat_map<T1, T2, T3, T4, T5>::__is_compare_transparent) &&
          (!(std::is_same<
              T6, typename std::flat_map<T1, T2, T3, T4, T5>::key_type>::value))
{
  return a0.template lower_bound<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, true>
f131(const std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(std::flat_map<T1, T2, T3, T4, T5>::__is_compare_transparent) &&
          (!(std::is_same<
              T6, typename std::flat_map<T1, T2, T3, T4, T5>::key_type>::value))
{
  return a0.template lower_bound<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, false>
f132(std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(std::flat_map<T1, T2, T3, T4, T5>::__is_compare_transparent) &&
          (!(std::is_same<
              T6, typename std::flat_map<T1, T2, T3, T4, T5>::key_type>::value))
{
  return a0.template upper_bound<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::__key_value_iterator<std::flat_map<T1, T2, T3, T4, T5>, T4, T5, true>
f133(const std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
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
f134(std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
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
f135(const std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(std::flat_map<T1, T2, T3, T4, T5>::__is_compare_transparent) &&
          (!(std::is_same<
              T6, typename std::flat_map<T1, T2, T3, T4, T5>::key_type>::value))
{
  return a0.template equal_range<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f136(std::flat_map<T1, T2, T3, T4, T5> &a0,
          std::flat_map<T1, T2, T3, T4, T5> &a1) {
  return swap(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f137(const std::flat_map<T1, T2, T3, T4, T5> &a0,
          const std::flat_map<T1, T2, T3, T4, T5> &a1) {
  return operator<=>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
bool f138(const std::flat_map<T1, T2, T3, T4, T5> &a0,
          const std::flat_map<T1, T2, T3, T4, T5> &a1) {
  return operator==(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
bool f139(const typename std::flat_map<T1, T2, T3, T4, T5>::value_compare &a0,
          std::pair<const T1 &, const T2 &> a1,
          std::pair<const T1 &, const T2 &> a2) {
  return a0.operator()(a1, a2);
}
