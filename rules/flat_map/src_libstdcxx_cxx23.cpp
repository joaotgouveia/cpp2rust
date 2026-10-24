#include <algorithm>
#include <flat_map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_multimap<T1, T2, T3, T4, T5>::size_type
f13(std::flat_multimap<T1, T2, T3, T4, T5> &a0, T6 a1) {
  return std::erase_if<T1, T2, T3, T4, T5, T6>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type &
f14(std::flat_map<T1, T2, T3, T4, T5> &a0,
    const typename std::flat_map<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type &
f15(std::flat_map<T1, T2, T3, T4, T5> &a0,
    typename std::flat_map<T1, T2, T3, T4, T5>::key_type &&a1) {
  return a0.operator[](std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type &
f16(std::flat_map<T1, T2, T3, T4, T5> &a0,
    const typename std::flat_map<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.at(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
const typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type &
f17(const std::flat_map<T1, T2, T3, T4, T5> &a0,
    const typename std::flat_map<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.at(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type &
f18(std::flat_map<T1, T2, T3, T4, T5> &a0, T6 &&a1)
  requires(std::__transparent_comparator<T3>) &&
          (!(std::is_same<T6, const typename std::flat_map<
                                  T1, T2, T3, T4, T5>::key_type &>::value) &&
           !(std::is_same<T6, typename std::flat_map<T1, T2, T3, T4,
                                                     T5>::key_type>::value))
{
  return a0.template operator[]<T6>(std::forward<T6>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type &
f19(std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(std::same_as<T6, T1> || std::__transparent_comparator<T3>) &&
          (!(std::is_same<
              T6, typename std::flat_map<T1, T2, T3, T4, T5>::key_type>::value))
{
  return a0.template at<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
const typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type &
f20(const std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(std::same_as<T6, T1> || std::__transparent_comparator<T3>) &&
          (!(std::is_same<
              T6, typename std::flat_map<T1, T2, T3, T4, T5>::key_type>::value))
{
  return a0.template at<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
std::pair<typename std::flat_map<T1, T2, T3, T4, T5>::iterator, bool>
f21(std::flat_map<T1, T2, T3, T4, T5> &a0,
    const typename std::flat_map<T1, T2, T3, T4, T5>::key_type &a1, T6 &&...a2)
  requires(std::is_constructible_v<
           typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type, T6...>)
{
  return a0.template try_emplace<T6...>(a1, std::forward<T6>(a2)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
std::pair<typename std::flat_map<T1, T2, T3, T4, T5>::iterator, bool>
f22(std::flat_map<T1, T2, T3, T4, T5> &a0,
    typename std::flat_map<T1, T2, T3, T4, T5>::key_type &&a1, T6 &&...a2)
  requires(std::is_constructible_v<
           typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type, T6...>)
{
  return a0.template try_emplace<T6...>(std::move(a1), std::forward<T6>(a2)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename... T7>
std::pair<typename std::flat_map<T1, T2, T3, T4, T5>::iterator, bool>
f23(std::flat_map<T1, T2, T3, T4, T5> &a0, T6 &&a1, T7 &&...a2)
  requires(
      std::__transparent_comparator<T3> &&
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
f24(std::flat_map<T1, T2, T3, T4, T5> &a0,
    typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator a1,
    const typename std::flat_map<T1, T2, T3, T4, T5>::key_type &a2, T6 &&...a3)
  requires(std::is_constructible_v<
           typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type, T6...>)
{
  return a0.template try_emplace<T6...>(a1, a2, std::forward<T6>(a3)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
typename std::flat_map<T1, T2, T3, T4, T5>::iterator
f25(std::flat_map<T1, T2, T3, T4, T5> &a0,
    typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator a1,
    typename std::flat_map<T1, T2, T3, T4, T5>::key_type &&a2, T6 &&...a3)
  requires(std::is_constructible_v<
           typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type, T6...>)
{
  return a0.template try_emplace<T6...>(a1, std::move(a2),
                                        std::forward<T6>(a3)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename... T7>
typename std::flat_map<T1, T2, T3, T4, T5>::iterator
f26(std::flat_map<T1, T2, T3, T4, T5> &a0,
    typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator a1, T6 &&a2,
    T7 &&...a3)
  requires(std::__transparent_comparator<T3> &&
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
std::pair<typename std::flat_map<T1, T2, T3, T4, T5>::iterator, bool>
f27(std::flat_map<T1, T2, T3, T4, T5> &a0,
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
f28(std::flat_map<T1, T2, T3, T4, T5> &a0,
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
f29(std::flat_map<T1, T2, T3, T4, T5> &a0, T6 &&a1, T7 &&a2)
  requires((std::same_as<std::remove_cvref_t<T6>, T1> ||
            std::__transparent_comparator<T3>) &&
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
typename std::flat_map<T1, T2, T3, T4, T5>::iterator
f30(std::flat_map<T1, T2, T3, T4, T5> &a0,
    typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator a1,
    const typename std::flat_map<T1, T2, T3, T4, T5>::key_type &a2, T6 &&a3)
  requires(std::is_assignable_v<
               typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type &, T6> &&
           std::is_constructible_v<
               typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type, T6>)
{
  return a0.template insert_or_assign<T6>(a1, a2, std::forward<T6>(a3));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_map<T1, T2, T3, T4, T5>::iterator
f31(std::flat_map<T1, T2, T3, T4, T5> &a0,
    typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator a1,
    typename std::flat_map<T1, T2, T3, T4, T5>::key_type &&a2, T6 &&a3)
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
typename std::flat_map<T1, T2, T3, T4, T5>::iterator
f32(std::flat_map<T1, T2, T3, T4, T5> &a0,
    typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator a1, T6 &&a2,
    T7 &&a3)
  requires((std::same_as<std::remove_cvref_t<T6>, T1> ||
            std::__transparent_comparator<T3>) &&
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
auto f45(std::flat_map<T1, T2, T3, T4, T5> &a0, const std::pair<T1, T2> &a1) {
  return a0.insert(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f46(std::flat_map<T1, T2, T3, T4, T5> &a0, std::pair<T1, T2> &&a1) {
  return a0.insert(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::iterator
f47(std::flat_map<T1, T2, T3, T4, T5> &a0,
    typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator a1,
    const std::pair<T1, T2> &a2) {
  return a0.insert(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::iterator
f48(std::flat_map<T1, T2, T3, T4, T5> &a0,
    typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator a1,
    std::pair<T1, T2> &&a2) {
  return a0.insert(a1, std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::iterator
f49(std::flat_map<T1, T2, T3, T4, T5> &a0,
    typename std::flat_map<T1, T2, T3, T4, T5>::iterator a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::iterator
f50(std::flat_map<T1, T2, T3, T4, T5> &a0,
    typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::size_t f51(std::flat_map<T1, T2, T3, T4, T5> &a0, T1 const &a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::iterator
f52(std::flat_map<T1, T2, T3, T4, T5> &a0,
    typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator a1,
    typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator a2) {
  return a0.erase(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
T4 const &f53(const std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.keys();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
T5 const &f54(const std::flat_map<T1, T2, T3, T4, T5> &a0) {
  return a0.values();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::iterator
f55(std::flat_map<T1, T2, T3, T4, T5> &a0, T1 const &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator
f56(const std::flat_map<T1, T2, T3, T4, T5> &a0, T1 const &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::size_t f57(const std::flat_map<T1, T2, T3, T4, T5> &a0, T1 const &a1) {
  return a0.count(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
bool f58(const std::flat_map<T1, T2, T3, T4, T5> &a0, T1 const &a1) {
  return a0.contains(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::iterator
f59(std::flat_map<T1, T2, T3, T4, T5> &a0, T1 const &a1) {
  return a0.lower_bound(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator
f60(const std::flat_map<T1, T2, T3, T4, T5> &a0, T1 const &a1) {
  return a0.lower_bound(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::iterator
f61(std::flat_map<T1, T2, T3, T4, T5> &a0, T1 const &a1) {
  return a0.upper_bound(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator
f62(const std::flat_map<T1, T2, T3, T4, T5> &a0, T1 const &a1) {
  return a0.upper_bound(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::pair<typename std::flat_map<T1, T2, T3, T4, T5>::iterator,
          typename std::flat_map<T1, T2, T3, T4, T5>::iterator>
f63(std::flat_map<T1, T2, T3, T4, T5> &a0, T1 const &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::pair<typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator,
          typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator>
f64(const std::flat_map<T1, T2, T3, T4, T5> &a0, T1 const &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
auto f65(std::flat_map<T1, T2, T3, T4, T5> &a0, T6 &&...a1)
  requires(std::is_constructible_v<
           typename std::_Flat_map_impl<T1, T2, T3, T4, T5, false>::value_type,
           T6...>)
{
  return a0.template emplace<T6...>(std::forward<T6>(a1)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
typename std::flat_map<T1, T2, T3, T4, T5>::iterator
f66(std::flat_map<T1, T2, T3, T4, T5> &a0,
    typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator a1, T6 &&...a2) {
  return a0.template emplace_hint<T6...>(a1, std::forward<T6>(a2)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
auto f67(std::flat_map<T1, T2, T3, T4, T5> &a0, T6 &&a1)
  requires(std::is_constructible_v<typename std::_Flat_map_impl<
                                       T1, T2, T3, T4, T5, false>::value_type,
                                   T6>) &&
          (!(std::is_same<
               T6, const typename std::_Flat_map_impl<
                       T1, T2, T3, T4, T5, false>::value_type &>::value) &&
           !(std::is_same<
               T6, typename std::_Flat_map_impl<T1, T2, T3, T4, T5,
                                                false>::value_type>::value) &&
           !(std::is_same<T6,
                          std::initializer_list<typename std::_Flat_map_impl<
                              T1, T2, T3, T4, T5, false>::value_type>>::value))
{
  return a0.template insert<T6>(std::forward<T6>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_map<T1, T2, T3, T4, T5>::iterator
f68(std::flat_map<T1, T2, T3, T4, T5> &a0,
    typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator a1, T6 &&a2)
  requires(std::is_constructible_v<typename std::_Flat_map_impl<
                                       T1, T2, T3, T4, T5, false>::value_type,
                                   T6>) &&
          (!(std::is_same<
               T6, const typename std::_Flat_map_impl<
                       T1, T2, T3, T4, T5, false>::value_type &>::value) &&
           !(std::is_same<
               T6, typename std::_Flat_map_impl<T1, T2, T3, T4, T5,
                                                false>::value_type>::value))
{
  return a0.template insert<T6>(a1, std::forward<T6>(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
void f69(std::flat_map<T1, T2, T3, T4, T5> &a0, T6 a1, T6 a2)
  requires(std::__has_input_iter_cat<T6>)
{
  return a0.template insert<T6>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
void f70(std::flat_map<T1, T2, T3, T4, T5> &a0, std::sorted_unique_t a1, T6 a2,
         T6 a3)
  requires(std::__has_input_iter_cat<T6>)
{
  return a0.template insert<T6>(std::move(a1), a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
void f71(std::flat_map<T1, T2, T3, T4, T5> &a0, T6 &&a1)
  requires(std::__detail::__container_compatible_range<
           T6,
           typename std::_Flat_map_impl<T1, T2, T3, T4, T5, false>::value_type>)
{
  return a0.template insert_range<T6>(std::forward<T6>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::size_t f72(std::flat_map<T1, T2, T3, T4, T5> &a0, T6 &&a1)
  requires(std::same_as<std::remove_cvref_t<T6>, T1> ||
           (std::__transparent_comparator<T3> &&
            !std::is_convertible_v<
                T6, typename std::_Flat_map_impl<T1, T2, T3, T4, T5,
                                                 false>::iterator> &&
            !std::is_convertible_v<
                T6, typename std::_Flat_map_impl<T1, T2, T3, T4, T5,
                                                 false>::const_iterator>)) &&
          (!(std::is_same<
               T6, typename std::_Flat_map_impl<T1, T2, T3, T4, T5,
                                                false>::iterator>::value) &&
           !(std::is_same<
               T6, typename std::_Flat_map_impl<
                       T1, T2, T3, T4, T5, false>::const_iterator>::value) &&
           !(std::is_same<T6,
                          const typename std::_Flat_map_impl<
                              T1, T2, T3, T4, T5, false>::key_type &>::value))
{
  return a0.template erase<T6>(std::forward<T6>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_map<T1, T2, T3, T4, T5>::iterator
f73(std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(std::same_as<T6, T1> || std::__transparent_comparator<T3>) &&
          (!(std::is_same<T6, typename std::_Flat_map_impl<
                                  T1, T2, T3, T4, T5, false>::key_type>::value))
{
  return a0.template find<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator
f74(const std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(std::same_as<T6, T1> || std::__transparent_comparator<T3>) &&
          (!(std::is_same<T6, typename std::_Flat_map_impl<
                                  T1, T2, T3, T4, T5, false>::key_type>::value))
{
  return a0.template find<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::size_t f75(const std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(std::same_as<T6, T1> || std::__transparent_comparator<T3>) &&
          (!(std::is_same<T6, typename std::_Flat_map_impl<
                                  T1, T2, T3, T4, T5, false>::key_type>::value))
{
  return a0.template count<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
bool f76(const std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(std::same_as<T6, T1> || std::__transparent_comparator<T3>) &&
          (!(std::is_same<T6, typename std::_Flat_map_impl<
                                  T1, T2, T3, T4, T5, false>::key_type>::value))
{
  return a0.template contains<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_map<T1, T2, T3, T4, T5>::iterator
f77(std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(std::same_as<T6, T1> || std::__transparent_comparator<T3>) &&
          (!(std::is_same<T6, typename std::_Flat_map_impl<
                                  T1, T2, T3, T4, T5, false>::key_type>::value))
{
  return a0.template lower_bound<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator
f78(const std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(std::same_as<T6, T1> || std::__transparent_comparator<T3>) &&
          (!(std::is_same<T6, typename std::_Flat_map_impl<
                                  T1, T2, T3, T4, T5, false>::key_type>::value))
{
  return a0.template lower_bound<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_map<T1, T2, T3, T4, T5>::iterator
f79(std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(std::same_as<T6, T1> || std::__transparent_comparator<T3>) &&
          (!(std::is_same<T6, typename std::_Flat_map_impl<
                                  T1, T2, T3, T4, T5, false>::key_type>::value))
{
  return a0.template upper_bound<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator
f80(const std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(std::same_as<T6, T1> || std::__transparent_comparator<T3>) &&
          (!(std::is_same<T6, typename std::_Flat_map_impl<
                                  T1, T2, T3, T4, T5, false>::key_type>::value))
{
  return a0.template upper_bound<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::pair<typename std::flat_map<T1, T2, T3, T4, T5>::iterator,
          typename std::flat_map<T1, T2, T3, T4, T5>::iterator>
f81(std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(std::same_as<T6, T1> || std::__transparent_comparator<T3>) &&
          (!(std::is_same<T6, typename std::_Flat_map_impl<
                                  T1, T2, T3, T4, T5, false>::key_type>::value))
{
  return a0.template equal_range<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::pair<typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator,
          typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator>
f82(const std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(std::same_as<T6, T1> || std::__transparent_comparator<T3>) &&
          (!(std::is_same<T6, typename std::_Flat_map_impl<
                                  T1, T2, T3, T4, T5, false>::key_type>::value))
{
  return a0.template equal_range<T6>(a1);
}
