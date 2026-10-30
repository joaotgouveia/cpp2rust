#include <algorithm>
#include <flat_map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_map<T1, T2, T3, T4, T5>::size_type
f1(std::flat_map<T1, T2, T3, T4, T5> &a0, T6 a1) {
  return std::erase_if(a0, std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_multimap<T1, T2, T3, T4, T5>::size_type
f2(std::flat_multimap<T1, T2, T3, T4, T5> &a0, T6 a1) {
  return std::erase_if(a0, std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t1 = std::flat_map<T1, T2, T3, T4, T5>;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t2 = typename std::flat_map<T1, T2, T3, T4, T5>::key_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t3 = typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t4 = typename std::flat_map<T1, T2, T3, T4, T5>::value_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t5 = typename std::flat_map<T1, T2, T3, T4, T5>::key_compare;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t6 = typename std::flat_map<T1, T2, T3, T4, T5>::reference;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t7 = typename std::flat_map<T1, T2, T3, T4, T5>::const_reference;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t8 = typename std::flat_map<T1, T2, T3, T4, T5>::size_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t9 = typename std::flat_map<T1, T2, T3, T4, T5>::difference_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t10 = typename std::flat_map<T1, T2, T3, T4, T5>::iterator;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t11 = typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t12 = typename std::flat_map<T1, T2, T3, T4, T5>::reverse_iterator;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t13 = typename std::flat_map<T1, T2, T3, T4, T5>::const_reverse_iterator;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t14 = typename std::flat_map<T1, T2, T3, T4, T5>::key_container_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t15 = typename std::flat_map<T1, T2, T3, T4, T5>::mapped_container_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t16 = typename std::flat_map<T1, T2, T3, T4, T5>::value_compare;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t17 = typename std::flat_map<T1, T2, T3, T4, T5>::containers;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type &
f3(std::flat_map<T1, T2, T3, T4, T5> &a0,
   const typename std::flat_map<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type &
f4(std::flat_map<T1, T2, T3, T4, T5> &a0,
   typename std::flat_map<T1, T2, T3, T4, T5>::key_type &&a1) {
  return a0.operator[](std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type &
f5(std::flat_map<T1, T2, T3, T4, T5> &a0,
   const typename std::flat_map<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.at(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
const typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type &
f6(const std::flat_map<T1, T2, T3, T4, T5> &a0,
   const typename std::flat_map<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.at(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type &
f7(std::flat_map<T1, T2, T3, T4, T5> &a0, T6 &&a1)
  requires(!(std::is_same<T6, const typename std::flat_map<
                                  T1, T2, T3, T4, T5>::key_type &>::value) &&
           !(std::is_same<T6, typename std::flat_map<T1, T2, T3, T4,
                                                     T5>::key_type>::value))
{
  return a0.template operator[]<T6>(std::forward<T6>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type &
f8(std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(!(std::is_same<
             T6, typename std::flat_map<T1, T2, T3, T4, T5>::key_type>::value))
{
  return a0.template at<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
const typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type &
f9(const std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(!(std::is_same<
             T6, typename std::flat_map<T1, T2, T3, T4, T5>::key_type>::value))
{
  return a0.template at<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
std::pair<typename std::flat_map<T1, T2, T3, T4, T5>::iterator, bool>
f10(std::flat_map<T1, T2, T3, T4, T5> &a0,
    const typename std::flat_map<T1, T2, T3, T4, T5>::key_type &a1, T6 &&...a2)
  requires(std::is_constructible_v<
           typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type, T6...>)
{
  return a0.template try_emplace<T6...>(a1, std::forward<T6>(a2)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
std::pair<typename std::flat_map<T1, T2, T3, T4, T5>::iterator, bool>
f11(std::flat_map<T1, T2, T3, T4, T5> &a0,
    typename std::flat_map<T1, T2, T3, T4, T5>::key_type &&a1, T6 &&...a2)
  requires(std::is_constructible_v<
           typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type, T6...>)
{
  return a0.template try_emplace<T6...>(std::move(a1), std::forward<T6>(a2)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename... T7>
std::pair<typename std::flat_map<T1, T2, T3, T4, T5>::iterator, bool>
f12(std::flat_map<T1, T2, T3, T4, T5> &a0, T6 &&a1, T7 &&...a2)
  requires(
      std::is_constructible_v<
          typename std::flat_map<T1, T2, T3, T4, T5>::key_type, T6> &&
      std::is_constructible_v<
          typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type, T7...> &&
      (!std::is_convertible_v<
          T6 &&, typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator>) &&
      (!std::is_convertible_v<
          T6 &&, typename std::flat_map<T1, T2, T3, T4, T5>::iterator>))
{
  return a0.template try_emplace<T6, T7...>(std::forward<T6>(a1),
                                            std::forward<T7>(a2)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
typename std::flat_map<T1, T2, T3, T4, T5>::iterator
f13(std::flat_map<T1, T2, T3, T4, T5> &a0,
    typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator a1,
    const typename std::flat_map<T1, T2, T3, T4, T5>::key_type &a2, T6 &&...a3)
  requires(std::is_constructible_v<
           typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type, T6...>)
{
  return a0.template try_emplace<T6...>(
      std::forward<typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator>(
          a1),
      a2, std::forward<T6>(a3)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
typename std::flat_map<T1, T2, T3, T4, T5>::iterator
f14(std::flat_map<T1, T2, T3, T4, T5> &a0,
    typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator a1,
    typename std::flat_map<T1, T2, T3, T4, T5>::key_type &&a2, T6 &&...a3)
  requires(std::is_constructible_v<
           typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type, T6...>)
{
  return a0.template try_emplace<T6...>(
      std::forward<typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator>(
          a1),
      std::move(a2), std::forward<T6>(a3)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename... T7>
typename std::flat_map<T1, T2, T3, T4, T5>::iterator
f15(std::flat_map<T1, T2, T3, T4, T5> &a0,
    typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator a1, T6 &&a2,
    T7 &&...a3)
  requires(std::is_constructible_v<
               typename std::flat_map<T1, T2, T3, T4, T5>::key_type, T6> &&
           std::is_constructible_v<
               typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type, T7...>)
{
  return a0.template try_emplace<T6, T7...>(
      std::forward<typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator>(
          a1),
      std::forward<T6>(a2), std::forward<T7>(a3)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::pair<typename std::flat_map<T1, T2, T3, T4, T5>::iterator, bool>
f16(std::flat_map<T1, T2, T3, T4, T5> &a0,
    const typename std::flat_map<T1, T2, T3, T4, T5>::key_type &a1, T6 &&a2)
  requires(std::is_assignable_v<
               typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type &, T6> &&
           std::is_constructible_v<
               typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type, T6>)
{
  return a0.template insert_or_assign<T6>(a1, std::forward<T6>(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::pair<typename std::flat_map<T1, T2, T3, T4, T5>::iterator, bool>
f17(std::flat_map<T1, T2, T3, T4, T5> &a0,
    typename std::flat_map<T1, T2, T3, T4, T5>::key_type &&a1, T6 &&a2)
  requires(std::is_assignable_v<
               typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type &, T6> &&
           std::is_constructible_v<
               typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type, T6>)
{
  return a0.template insert_or_assign<T6>(std::move(a1), std::forward<T6>(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
std::pair<typename std::flat_map<T1, T2, T3, T4, T5>::iterator, bool>
f18(std::flat_map<T1, T2, T3, T4, T5> &a0, T6 &&a1, T7 &&a2)
  requires(std::is_constructible_v<
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
typename std::flat_map<T1, T2, T3, T4, T5>::iterator
f19(std::flat_map<T1, T2, T3, T4, T5> &a0,
    typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator a1,
    const typename std::flat_map<T1, T2, T3, T4, T5>::key_type &a2, T6 &&a3)
  requires(std::is_assignable_v<
               typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type &, T6> &&
           std::is_constructible_v<
               typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type, T6>)
{
  return a0.template insert_or_assign<T6>(
      std::forward<typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator>(
          a1),
      a2, std::forward<T6>(a3));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_map<T1, T2, T3, T4, T5>::iterator
f20(std::flat_map<T1, T2, T3, T4, T5> &a0,
    typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator a1,
    typename std::flat_map<T1, T2, T3, T4, T5>::key_type &&a2, T6 &&a3)
  requires(std::is_assignable_v<
               typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type &, T6> &&
           std::is_constructible_v<
               typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type, T6>)
{
  return a0.template insert_or_assign<T6>(
      std::forward<typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator>(
          a1),
      std::move(a2), std::forward<T6>(a3));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
typename std::flat_map<T1, T2, T3, T4, T5>::iterator
f21(std::flat_map<T1, T2, T3, T4, T5> &a0,
    typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator a1, T6 &&a2,
    T7 &&a3)
  requires(std::is_constructible_v<
               typename std::flat_map<T1, T2, T3, T4, T5>::key_type, T6> &&
           std::is_assignable_v<
               typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type &, T7> &&
           std::is_constructible_v<
               typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type, T7>)
{
  return a0.template insert_or_assign<T6, T7>(
      std::forward<typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator>(
          a1),
      std::forward<T6>(a2), std::forward<T7>(a3));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_map<T1, T2, T3, T4, T5> f22() {
  return std::flat_map<T1, T2, T3, T4, T5>();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_map<T1, T2, T3, T4, T5> f23(T3 const &a0) {
  return std::flat_map<T1, T2, T3, T4, T5>(a0);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_map<T1, T2, T3, T4, T5> f24(T4 a0, T5 a1, T3 const &a2) {
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0), std::move(a1), a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_map<T1, T2, T3, T4, T5> f25(std::sorted_unique_t a0, T4 a1, T5 a2,
                                      T3 const &a3) {
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0), std::move(a1),
                                           std::move(a2), a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_map<T1, T2, T3, T4, T5>
f26(std::initializer_list<std::pair<T1, T2>> a0, T3 const &a1) {
  return std::flat_map<T1, T2, T3, T4, T5>(
      std::forward<std::initializer_list<std::pair<T1, T2>>>(a0), a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_map<T1, T2, T3, T4, T5>
f27(std::sorted_unique_t a0, std::initializer_list<std::pair<T1, T2>> a1,
    T3 const &a2) {
  return std::flat_map<T1, T2, T3, T4, T5>(
      std::move(a0), std::forward<std::initializer_list<std::pair<T1, T2>>>(a1),
      a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::iterator
f28(std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator
f29(const std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::iterator
f30(std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator
f31(const std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::reverse_iterator
f32(std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::const_reverse_iterator
f33(const std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::reverse_iterator
f34(std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.rend();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::const_reverse_iterator
f35(const std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.rend();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator
f36(const std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator
f37(const std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.cend();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::const_reverse_iterator
f38(const std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.crbegin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::const_reverse_iterator
f39(const std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.crend();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
bool f40(const std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.empty();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::size_t f41(const std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.size();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::size_t f42(const std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f43(std::flat_map<T1, T2, T3, T4, T5> &a0, const std::pair<T1, T2> &a1) {
  return a0.insert(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f44(std::flat_map<T1, T2, T3, T4, T5> &a0, std::pair<T1, T2> &&a1) {
  return a0.insert(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::iterator
f45(std::flat_map<T1, T2, T3, T4, T5> &a0,
    typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator a1,
    const std::pair<T1, T2> &a2) {
  return a0.insert(
      std::forward<typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator>(
          a1),
      a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::iterator
f46(std::flat_map<T1, T2, T3, T4, T5> &a0,
    typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator a1,
    std::pair<T1, T2> &&a2) {
  return a0.insert(
      std::forward<typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator>(
          a1),
      std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f47(std::flat_map<T1, T2, T3, T4, T5> &a0,
         std::initializer_list<std::pair<T1, T2>> a1) {
  return a0.insert(std::forward<std::initializer_list<std::pair<T1, T2>>>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f48(std::flat_map<T1, T2, T3, T4, T5> &a0, std::sorted_unique_t a1,
         std::initializer_list<std::pair<T1, T2>> a2) {
  return a0.insert(std::move(a1),
                   std::forward<std::initializer_list<std::pair<T1, T2>>>(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::containers
f49(std::flat_map<T1, T2, T3, T4, T5> &&a0) {
  return std::move(a0).extract();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f50(std::flat_map<T1, T2, T3, T4, T5> &a0, T4 &&a1, T5 &&a2) {
  return a0.replace(std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::iterator
f51(std::flat_map<T1, T2, T3, T4, T5> &a0,
    typename std::flat_map<T1, T2, T3, T4, T5>::iterator a1) {
  return a0.erase(
      std::forward<typename std::flat_map<T1, T2, T3, T4, T5>::iterator>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::iterator
f52(std::flat_map<T1, T2, T3, T4, T5> &a0,
    typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator a1) {
  return a0.erase(
      std::forward<typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator>(
          a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::size_t f53(std::flat_map<T1, T2, T3, T4, T5> &a0, T1 const &a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::iterator
f54(std::flat_map<T1, T2, T3, T4, T5> &a0,
    typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator a1,
    typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator a2) {
  return a0.erase(
      std::forward<typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator>(
          a1),
      std::forward<typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator>(
          a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f55(std::flat_map<T1, T2, T3, T4, T5> &a0,
         std::flat_map<T1, T2, T3, T4, T5> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f56(std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.clear();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
T3 f57(const std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.key_comp();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::value_compare
f58(const std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.value_comp();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
T4 const &f59(const std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.keys();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
T5 const &f60(const std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.values();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::iterator
f61(std::flat_map<T1, T2, T3, T4, T5> &a0, T1 const &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator
f62(const std::flat_map<T1, T2, T3, T4, T5> &a0, T1 const &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::size_t f63(const std::flat_map<T1, T2, T3, T4, T5> &a0, T1 const &a1) {
  return a0.count(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
bool f64(const std::flat_map<T1, T2, T3, T4, T5> &a0, T1 const &a1) {
  return a0.contains(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::iterator
f65(std::flat_map<T1, T2, T3, T4, T5> &a0, T1 const &a1) {
  return a0.lower_bound(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator
f66(const std::flat_map<T1, T2, T3, T4, T5> &a0, T1 const &a1) {
  return a0.lower_bound(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::iterator
f67(std::flat_map<T1, T2, T3, T4, T5> &a0, T1 const &a1) {
  return a0.upper_bound(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator
f68(const std::flat_map<T1, T2, T3, T4, T5> &a0, T1 const &a1) {
  return a0.upper_bound(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::pair<typename std::flat_map<T1, T2, T3, T4, T5>::iterator,
          typename std::flat_map<T1, T2, T3, T4, T5>::iterator>
f69(std::flat_map<T1, T2, T3, T4, T5> &a0, T1 const &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::pair<typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator,
          typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator>
f70(const std::flat_map<T1, T2, T3, T4, T5> &a0, T1 const &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_map<T1, T2, T3, T4, T5> f71(T6 a0, T6 a1, T3 const &a2) {
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0), std::move(a1), a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_map<T1, T2, T3, T4, T5> f72(std::sorted_unique_t a0, T6 a1, T6 a2,
                                      T3 const &a3) {
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0), std::move(a1),
                                           std::move(a2), a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_map<T1, T2, T3, T4, T5> f73(std::from_range_t a0, T6 &&a1) {
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0), std::forward<T6>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_map<T1, T2, T3, T4, T5> f74(std::from_range_t a0, T6 &&a1,
                                      T3 const &a2) {
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0), std::forward<T6>(a1),
                                           a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_map<T1, T2, T3, T4, T5> f75(const T6 &a0) {
  return std::flat_map<T1, T2, T3, T4, T5>(a0);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_map<T1, T2, T3, T4, T5> f76(T3 const &a0, const T6 &a1) {
  return std::flat_map<T1, T2, T3, T4, T5>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_map<T1, T2, T3, T4, T5> f77(T4 const &a0, T5 const &a1,
                                      const T6 &a2) {
  return std::flat_map<T1, T2, T3, T4, T5>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_map<T1, T2, T3, T4, T5> f78(T4 const &a0, T5 const &a1, T3 const &a2,
                                      const T6 &a3) {
  return std::flat_map<T1, T2, T3, T4, T5>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_map<T1, T2, T3, T4, T5> f79(std::sorted_unique_t a0, T4 const &a1,
                                      T5 const &a2, const T6 &a3) {
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_map<T1, T2, T3, T4, T5> f80(std::sorted_unique_t a0, T4 const &a1,
                                      T5 const &a2, T3 const &a3,
                                      const T6 &a4) {
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0), a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_map<T1, T2, T3, T4, T5>
f81(const std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return std::flat_map<T1, T2, T3, T4, T5>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_map<T1, T2, T3, T4, T5> f82(std::flat_map<T1, T2, T3, T4, T5> &&a0,
                                      const T6 &a1) {
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
std::flat_map<T1, T2, T3, T4, T5> f83(T6 a0, T6 a1, const T7 &a2) {
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0), std::move(a1), a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
std::flat_map<T1, T2, T3, T4, T5> f84(T6 a0, T6 a1, T3 const &a2,
                                      const T7 &a3) {
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0), std::move(a1), a2,
                                           a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
std::flat_map<T1, T2, T3, T4, T5> f85(std::sorted_unique_t a0, T6 a1, T6 a2,
                                      const T7 &a3) {
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0), std::move(a1),
                                           std::move(a2), a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
std::flat_map<T1, T2, T3, T4, T5> f86(std::sorted_unique_t a0, T6 a1, T6 a2,
                                      T3 const &a3, const T7 &a4) {
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0), std::move(a1),
                                           std::move(a2), a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
std::flat_map<T1, T2, T3, T4, T5> f87(std::from_range_t a0, T6 &&a1,
                                      const T7 &a2) {
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0), std::forward<T6>(a1),
                                           a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
std::flat_map<T1, T2, T3, T4, T5> f88(std::from_range_t a0, T6 &&a1,
                                      T3 const &a2, const T7 &a3) {
  return std::flat_map<T1, T2, T3, T4, T5>(std::move(a0), std::forward<T6>(a1),
                                           a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_map<T1, T2, T3, T4, T5>
f89(std::initializer_list<std::pair<T1, T2>> a0, const T6 &a1) {
  return std::flat_map<T1, T2, T3, T4, T5>(
      std::forward<std::initializer_list<std::pair<T1, T2>>>(a0), a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_map<T1, T2, T3, T4, T5>
f90(std::initializer_list<std::pair<T1, T2>> a0, T3 const &a1, const T6 &a2) {
  return std::flat_map<T1, T2, T3, T4, T5>(
      std::forward<std::initializer_list<std::pair<T1, T2>>>(a0), a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_map<T1, T2, T3, T4, T5>
f91(std::sorted_unique_t a0, std::initializer_list<std::pair<T1, T2>> a1,
    const T6 &a2) {
  return std::flat_map<T1, T2, T3, T4, T5>(
      std::move(a0), std::forward<std::initializer_list<std::pair<T1, T2>>>(a1),
      a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::flat_map<T1, T2, T3, T4, T5>
f92(std::sorted_unique_t a0, std::initializer_list<std::pair<T1, T2>> a1,
    T3 const &a2, const T6 &a3) {
  return std::flat_map<T1, T2, T3, T4, T5>(
      std::move(a0), std::forward<std::initializer_list<std::pair<T1, T2>>>(a1),
      a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
auto f93(std::flat_map<T1, T2, T3, T4, T5> &a0, T6 &&...a1) {
  return a0.template emplace<T6...>(std::forward<T6>(a1)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
typename std::flat_map<T1, T2, T3, T4, T5>::iterator
f94(std::flat_map<T1, T2, T3, T4, T5> &a0,
    typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator a1, T6 &&...a2) {
  return a0.template emplace_hint<T6...>(
      std::forward<typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator>(
          a1),
      std::forward<T6>(a2)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
auto f95(std::flat_map<T1, T2, T3, T4, T5> &a0, T6 &&a1) {
  return a0.template insert<T6>(std::forward<T6>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_map<T1, T2, T3, T4, T5>::iterator
f96(std::flat_map<T1, T2, T3, T4, T5> &a0,
    typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator a1, T6 &&a2) {
  return a0.template insert<T6>(
      std::forward<typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator>(
          a1),
      std::forward<T6>(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
void f97(std::flat_map<T1, T2, T3, T4, T5> &a0, T6 a1, T6 a2) {
  return a0.template insert<T6>(std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
void f98(std::flat_map<T1, T2, T3, T4, T5> &a0, std::sorted_unique_t a1, T6 a2,
         T6 a3) {
  return a0.template insert<T6>(std::move(a1), std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
void f99(std::flat_map<T1, T2, T3, T4, T5> &a0, T6 &&a1) {
  return a0.template insert_range<T6>(std::forward<T6>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::size_t f100(std::flat_map<T1, T2, T3, T4, T5> &a0, T6 &&a1) {
  return a0.template erase<T6>(std::forward<T6>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_map<T1, T2, T3, T4, T5>::iterator
f101(std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template find<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator
f102(const std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template find<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::size_t f103(const std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template count<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
bool f104(const std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template contains<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_map<T1, T2, T3, T4, T5>::iterator
f105(std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template lower_bound<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator
f106(const std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template lower_bound<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_map<T1, T2, T3, T4, T5>::iterator
f107(std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template upper_bound<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator
f108(const std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template upper_bound<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::pair<typename std::flat_map<T1, T2, T3, T4, T5>::iterator,
          typename std::flat_map<T1, T2, T3, T4, T5>::iterator>
f109(std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template equal_range<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::pair<typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator,
          typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator>
f110(const std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template equal_range<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f111(std::flat_map<T1, T2, T3, T4, T5> &a0,
          std::flat_map<T1, T2, T3, T4, T5> &a1) {
  return swap(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
bool f112(const std::flat_map<T1, T2, T3, T4, T5> &a0,
          const std::flat_map<T1, T2, T3, T4, T5> &a1) {
  return operator==(a0, a1);
}
