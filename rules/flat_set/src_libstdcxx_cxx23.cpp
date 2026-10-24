#include <algorithm>
#include <flat_set>
#include <initializer_list>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
typename std::flat_multiset<T1, T2, T3>::size_type
f24(std::flat_multiset<T1, T2, T3> &a0, T4 a1) {
  return std::erase_if<T1, T2, T3, T4>(a0, std::move(a1));
}

template <typename T1, typename T2, typename T3>
auto f25(std::flat_set<T1, T2, T3> &a0, T1 const &a1) {
  return a0.insert(a1);
}

template <typename T1, typename T2, typename T3>
auto f26(std::flat_set<T1, T2, T3> &a0, T1 &&a1) {
  return a0.insert(std::move(a1));
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::iterator
f27(std::flat_set<T1, T2, T3> &a0,
    typename std::flat_set<T1, T2, T3>::const_iterator a1, T1 const &a2) {
  return a0.insert(std::move(a1), a2);
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::iterator
f28(std::flat_set<T1, T2, T3> &a0,
    typename std::flat_set<T1, T2, T3>::const_iterator a1) {
  return a0.erase(std::move(a1));
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::size_type f29(std::flat_set<T1, T2, T3> &a0,
                                                  T1 const &a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::iterator f30(std::flat_set<T1, T2, T3> &a0,
                                                 T1 const &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::const_iterator
f31(const std::flat_set<T1, T2, T3> &a0, T1 const &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::size_type
f32(const std::flat_set<T1, T2, T3> &a0, T1 const &a1) {
  return a0.count(a1);
}

template <typename T1, typename T2, typename T3>
bool f33(const std::flat_set<T1, T2, T3> &a0, T1 const &a1) {
  return a0.contains(a1);
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::iterator f34(std::flat_set<T1, T2, T3> &a0,
                                                 T1 const &a1) {
  return a0.lower_bound(a1);
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::const_iterator
f35(const std::flat_set<T1, T2, T3> &a0, T1 const &a1) {
  return a0.lower_bound(a1);
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::iterator f36(std::flat_set<T1, T2, T3> &a0,
                                                 T1 const &a1) {
  return a0.upper_bound(a1);
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::const_iterator
f37(const std::flat_set<T1, T2, T3> &a0, T1 const &a1) {
  return a0.upper_bound(a1);
}

template <typename T1, typename T2, typename T3>
std::pair<typename std::flat_set<T1, T2, T3>::iterator,
          typename std::flat_set<T1, T2, T3>::iterator>
f38(std::flat_set<T1, T2, T3> &a0, T1 const &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3>
std::pair<typename std::flat_set<T1, T2, T3>::const_iterator,
          typename std::flat_set<T1, T2, T3>::const_iterator>
f39(const std::flat_set<T1, T2, T3> &a0, T1 const &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename... T4>
auto f40(std::flat_set<T1, T2, T3> &a0, T4 &&...a1)
  requires(std::is_constructible_v<
           typename std::_Flat_set_impl<T1, T2, T3, false>::value_type, T4...>)
{
  return a0.template emplace<T4...>(std::forward<T4>(a1)...);
}

template <typename T1, typename T2, typename T3, typename T4>
auto f41(std::flat_set<T1, T2, T3> &a0, T4 &&a1)
  requires(std::is_constructible_v<
              typename std::_Flat_set_impl<T1, T2, T3, false>::value_type,
              T4>) &&
          (!(std::is_same<T4, const typename std::_Flat_set_impl<
                                  T1, T2, T3, false>::value_type &>::value) &&
           !(std::is_same<T4, typename std::_Flat_set_impl<
                                  T1, T2, T3, false>::value_type>::value) &&
           !(std::is_same<T4,
                          std::initializer_list<typename std::_Flat_set_impl<
                              T1, T2, T3, false>::value_type>>::value))
{
  return a0.template insert<T4>(std::forward<T4>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::flat_set<T1, T2, T3>::iterator
f42(std::flat_set<T1, T2, T3> &a0,
    typename std::flat_set<T1, T2, T3>::const_iterator a1, T4 &&a2)
  requires(std::is_constructible_v<
              typename std::_Flat_set_impl<T1, T2, T3, false>::value_type,
              T4>) &&
          (!(std::is_same<T4, const typename std::_Flat_set_impl<
                                  T1, T2, T3, false>::value_type &>::value) &&
           !(std::is_same<T4, typename std::_Flat_set_impl<
                                  T1, T2, T3, false>::value_type>::value))
{
  return a0.template insert<T4>(std::move(a1), std::forward<T4>(a2));
}

template <typename T1, typename T2, typename T3, typename T4>
void f43(std::flat_set<T1, T2, T3> &a0, T4 a1, T4 a2)
  requires(std::__has_input_iter_cat<T4>)
{
  return a0.template insert<T4>(std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4>
void f44(std::flat_set<T1, T2, T3> &a0, std::sorted_unique_t a1, T4 a2, T4 a3)
  requires(std::__has_input_iter_cat<T4>)
{
  return a0.template insert<T4>(std::move(a1), std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3, typename T4>
void f45(std::flat_set<T1, T2, T3> &a0, T4 &&a1)
  requires(std::__detail::__container_compatible_range<
           T4, typename std::_Flat_set_impl<T1, T2, T3, false>::value_type>)
{
  return a0.template insert_range<T4>(std::forward<T4>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::flat_set<T1, T2, T3>::size_type f46(std::flat_set<T1, T2, T3> &a0,
                                                  T4 &&a1)
  requires(std::same_as<std::remove_cvref_t<T4>, T1> ||
           (std::__transparent_comparator<T2> &&
            !std::is_convertible_v<T4, typename std::_Flat_set_impl<
                                           T1, T2, T3, false>::iterator> &&
            !std::is_convertible_v<
                T4, typename std::_Flat_set_impl<T1, T2, T3,
                                                 false>::const_iterator>)) &&
          (!(std::is_same<T4, typename std::_Flat_set_impl<
                                  T1, T2, T3, false>::const_iterator>::value) &&
           !(std::is_same<T4, const typename std::_Flat_set_impl<
                                  T1, T2, T3, false>::key_type &>::value))
{
  return a0.template erase<T4>(std::forward<T4>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::flat_set<T1, T2, T3>::iterator f47(std::flat_set<T1, T2, T3> &a0,
                                                 const T4 &a1)
  requires(std::same_as<T4, T1> || std::__transparent_comparator<T2>) &&
          (!(std::is_same<T4, typename std::_Flat_set_impl<
                                  T1, T2, T3, false>::key_type>::value))
{
  return a0.template find<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::flat_set<T1, T2, T3>::const_iterator
f48(const std::flat_set<T1, T2, T3> &a0, const T4 &a1)
  requires(std::same_as<T4, T1> || std::__transparent_comparator<T2>) &&
          (!(std::is_same<T4, typename std::_Flat_set_impl<
                                  T1, T2, T3, false>::key_type>::value))
{
  return a0.template find<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::flat_set<T1, T2, T3>::size_type
f49(const std::flat_set<T1, T2, T3> &a0, const T4 &a1)
  requires(std::same_as<T4, T1> || std::__transparent_comparator<T2>) &&
          (!(std::is_same<T4, typename std::_Flat_set_impl<
                                  T1, T2, T3, false>::key_type>::value))
{
  return a0.template count<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f50(const std::flat_set<T1, T2, T3> &a0, const T4 &a1)
  requires(std::same_as<T4, T1> || std::__transparent_comparator<T2>) &&
          (!(std::is_same<T4, typename std::_Flat_set_impl<
                                  T1, T2, T3, false>::key_type>::value))
{
  return a0.template contains<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::flat_set<T1, T2, T3>::iterator f51(std::flat_set<T1, T2, T3> &a0,
                                                 const T4 &a1)
  requires(std::same_as<T4, T1> || std::__transparent_comparator<T2>) &&
          (!(std::is_same<T4, typename std::_Flat_set_impl<
                                  T1, T2, T3, false>::key_type>::value))
{
  return a0.template lower_bound<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::flat_set<T1, T2, T3>::const_iterator
f52(const std::flat_set<T1, T2, T3> &a0, const T4 &a1)
  requires(std::same_as<T4, T1> || std::__transparent_comparator<T2>) &&
          (!(std::is_same<T4, typename std::_Flat_set_impl<
                                  T1, T2, T3, false>::key_type>::value))
{
  return a0.template lower_bound<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::flat_set<T1, T2, T3>::iterator f53(std::flat_set<T1, T2, T3> &a0,
                                                 const T4 &a1)
  requires(std::same_as<T4, T1> || std::__transparent_comparator<T2>) &&
          (!(std::is_same<T4, typename std::_Flat_set_impl<
                                  T1, T2, T3, false>::key_type>::value))
{
  return a0.template upper_bound<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::flat_set<T1, T2, T3>::const_iterator
f54(const std::flat_set<T1, T2, T3> &a0, const T4 &a1)
  requires(std::same_as<T4, T1> || std::__transparent_comparator<T2>) &&
          (!(std::is_same<T4, typename std::_Flat_set_impl<
                                  T1, T2, T3, false>::key_type>::value))
{
  return a0.template upper_bound<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<typename std::flat_set<T1, T2, T3>::iterator,
          typename std::flat_set<T1, T2, T3>::iterator>
f55(std::flat_set<T1, T2, T3> &a0, const T4 &a1)
  requires(std::same_as<T4, T1> || std::__transparent_comparator<T2>) &&
          (!(std::is_same<T4, typename std::_Flat_set_impl<
                                  T1, T2, T3, false>::key_type>::value))
{
  return a0.template equal_range<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<typename std::flat_set<T1, T2, T3>::const_iterator,
          typename std::flat_set<T1, T2, T3>::const_iterator>
f56(const std::flat_set<T1, T2, T3> &a0, const T4 &a1)
  requires(std::same_as<T4, T1> || std::__transparent_comparator<T2>) &&
          (!(std::is_same<T4, typename std::_Flat_set_impl<
                                  T1, T2, T3, false>::key_type>::value))
{
  return a0.template equal_range<T4>(a1);
}
