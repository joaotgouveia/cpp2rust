#include <algorithm>
#include <flat_map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator
f12(std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator
f13(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator
f14(std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator
f15(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::reverse_iterator
f16(std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::const_reverse_iterator
f17(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::reverse_iterator
f18(std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.rend();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::const_reverse_iterator
f19(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.rend();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator
f20(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator
f21(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.cend();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::const_reverse_iterator
f22(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.crbegin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::const_reverse_iterator
f23(const std::flat_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.crend();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f24(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
         const std::pair<T1, T2> &a1) {
  return a0.insert(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f25(std::flat_multimap<T1, T2, T3, T4, T5> &a0, std::pair<T1, T2> &&a1) {
  return a0.insert(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator
f26(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
    typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator a1,
    const std::pair<T1, T2> &a2) {
  return a0.insert(std::move(a1), a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator
f27(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
    typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator a1,
    std::pair<T1, T2> &&a2) {
  return a0.insert(std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator
f28(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
    typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator a1) {
  return a0.erase(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator
f29(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
    typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator a1) {
  return a0.erase(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::size_t f30(std::flat_multimap<T1, T2, T3, T4, T5> &a0, T1 const &a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator
f31(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
    typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator a1,
    typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator a2) {
  return a0.erase(std::move(a1), std::move(a2));
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
auto f44(std::flat_multimap<T1, T2, T3, T4, T5> &a0, T6 &&...a1)
  requires(std::is_constructible_v<
           typename std::_Flat_map_impl<T1, T2, T3, T4, T5, true>::value_type,
           T6...>)
{
  return a0.template emplace<T6...>(std::forward<T6>(a1)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator
f45(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
    typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator a1,
    T6 &&...a2) {
  return a0.template emplace_hint<T6...>(std::move(a1),
                                         std::forward<T6>(a2)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
auto f46(std::flat_multimap<T1, T2, T3, T4, T5> &a0, T6 &&a1)
  requires(std::is_constructible_v<typename std::_Flat_map_impl<
                                       T1, T2, T3, T4, T5, true>::value_type,
                                   T6>) &&
          (!(std::is_same<
               T6, const typename std::_Flat_map_impl<
                       T1, T2, T3, T4, T5, true>::value_type &>::value) &&
           !(std::is_same<
               T6, typename std::_Flat_map_impl<T1, T2, T3, T4, T5,
                                                true>::value_type>::value) &&
           !(std::is_same<T6,
                          std::initializer_list<typename std::_Flat_map_impl<
                              T1, T2, T3, T4, T5, true>::value_type>>::value))
{
  return a0.template insert<T6>(std::forward<T6>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator
f47(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
    typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator a1, T6 &&a2)
  requires(std::is_constructible_v<typename std::_Flat_map_impl<
                                       T1, T2, T3, T4, T5, true>::value_type,
                                   T6>) &&
          (!(std::is_same<
               T6, const typename std::_Flat_map_impl<
                       T1, T2, T3, T4, T5, true>::value_type &>::value) &&
           !(std::is_same<
               T6, typename std::_Flat_map_impl<T1, T2, T3, T4, T5,
                                                true>::value_type>::value))
{
  return a0.template insert<T6>(std::move(a1), std::forward<T6>(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
void f48(std::flat_multimap<T1, T2, T3, T4, T5> &a0, T6 a1, T6 a2)
  requires(std::__has_input_iter_cat<T6>)
{
  return a0.template insert<T6>(std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
void f49(std::flat_multimap<T1, T2, T3, T4, T5> &a0,
         std::sorted_equivalent_t a1, T6 a2, T6 a3)
  requires(std::__has_input_iter_cat<T6>)
{
  return a0.template insert<T6>(std::move(a1), std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
void f50(std::flat_multimap<T1, T2, T3, T4, T5> &a0, T6 &&a1)
  requires(std::__detail::__container_compatible_range<
           T6,
           typename std::_Flat_map_impl<T1, T2, T3, T4, T5, true>::value_type>)
{
  return a0.template insert_range<T6>(std::forward<T6>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::size_t f51(std::flat_multimap<T1, T2, T3, T4, T5> &a0, T6 &&a1)
  requires(std::same_as<std::remove_cvref_t<T6>, T1> ||
           (std::__transparent_comparator<T3> &&
            !std::is_convertible_v<
                T6, typename std::_Flat_map_impl<T1, T2, T3, T4, T5,
                                                 true>::iterator> &&
            !std::is_convertible_v<
                T6, typename std::_Flat_map_impl<T1, T2, T3, T4, T5,
                                                 true>::const_iterator>)) &&
          (!(std::is_same<
               T6, typename std::_Flat_map_impl<T1, T2, T3, T4, T5,
                                                true>::iterator>::value) &&
           !(std::is_same<
               T6, typename std::_Flat_map_impl<
                       T1, T2, T3, T4, T5, true>::const_iterator>::value) &&
           !(std::is_same<T6,
                          const typename std::_Flat_map_impl<
                              T1, T2, T3, T4, T5, true>::key_type &>::value))
{
  return a0.template erase<T6>(std::forward<T6>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator
f52(std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(std::same_as<T6, T1> || std::__transparent_comparator<T3>) &&
          (!(std::is_same<T6, typename std::_Flat_map_impl<
                                  T1, T2, T3, T4, T5, true>::key_type>::value))
{
  return a0.template find<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator
f53(const std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(std::same_as<T6, T1> || std::__transparent_comparator<T3>) &&
          (!(std::is_same<T6, typename std::_Flat_map_impl<
                                  T1, T2, T3, T4, T5, true>::key_type>::value))
{
  return a0.template find<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::size_t f54(const std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(std::same_as<T6, T1> || std::__transparent_comparator<T3>) &&
          (!(std::is_same<T6, typename std::_Flat_map_impl<
                                  T1, T2, T3, T4, T5, true>::key_type>::value))
{
  return a0.template count<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
bool f55(const std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(std::same_as<T6, T1> || std::__transparent_comparator<T3>) &&
          (!(std::is_same<T6, typename std::_Flat_map_impl<
                                  T1, T2, T3, T4, T5, true>::key_type>::value))
{
  return a0.template contains<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator
f56(std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(std::same_as<T6, T1> || std::__transparent_comparator<T3>) &&
          (!(std::is_same<T6, typename std::_Flat_map_impl<
                                  T1, T2, T3, T4, T5, true>::key_type>::value))
{
  return a0.template lower_bound<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator
f57(const std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(std::same_as<T6, T1> || std::__transparent_comparator<T3>) &&
          (!(std::is_same<T6, typename std::_Flat_map_impl<
                                  T1, T2, T3, T4, T5, true>::key_type>::value))
{
  return a0.template lower_bound<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator
f58(std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(std::same_as<T6, T1> || std::__transparent_comparator<T3>) &&
          (!(std::is_same<T6, typename std::_Flat_map_impl<
                                  T1, T2, T3, T4, T5, true>::key_type>::value))
{
  return a0.template upper_bound<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator
f59(const std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(std::same_as<T6, T1> || std::__transparent_comparator<T3>) &&
          (!(std::is_same<T6, typename std::_Flat_map_impl<
                                  T1, T2, T3, T4, T5, true>::key_type>::value))
{
  return a0.template upper_bound<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::pair<typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator,
          typename std::flat_multimap<T1, T2, T3, T4, T5>::iterator>
f60(std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(std::same_as<T6, T1> || std::__transparent_comparator<T3>) &&
          (!(std::is_same<T6, typename std::_Flat_map_impl<
                                  T1, T2, T3, T4, T5, true>::key_type>::value))
{
  return a0.template equal_range<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::pair<typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator,
          typename std::flat_multimap<T1, T2, T3, T4, T5>::const_iterator>
f61(const std::flat_multimap<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(std::same_as<T6, T1> || std::__transparent_comparator<T3>) &&
          (!(std::is_same<T6, typename std::_Flat_map_impl<
                                  T1, T2, T3, T4, T5, true>::key_type>::value))
{
  return a0.template equal_range<T6>(a1);
}
