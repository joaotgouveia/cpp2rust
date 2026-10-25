#include <ranges>
#include <utility>

template <typename T1, std::size_t T2>
std::ranges::elements_view<T1, T2> f53()
  requires(std::default_initializable<T1>) &&
          (std::ranges::view<T1> &&
           std::ranges::__has_tuple_element<std::ranges::range_value_t<T1>,
                                            T2> &&
           std::ranges::__has_tuple_element<
               std::remove_reference_t<std::ranges::range_reference_t<T1>>,
               T2> &&
           std::ranges::__returnable_element<std::ranges::range_reference_t<T1>,
                                             T2>)
{
  return std::ranges::elements_view<T1, T2>();
}

template <typename T1, std::size_t T2>
std::ranges::elements_view<T1, T2> f54(T1 a0)
  requires(
      std::ranges::view<T1> &&
      std::ranges::__has_tuple_element<std::ranges::range_value_t<T1>, T2> &&
      std::ranges::__has_tuple_element<
          std::remove_reference_t<std::ranges::range_reference_t<T1>>, T2> &&
      std::ranges::__returnable_element<std::ranges::range_reference_t<T1>, T2>)
{
  return std::ranges::elements_view<T1, T2>(std::move(a0));
}

template <typename T1, std::size_t T2>
T1 f55(const std::ranges::elements_view<T1, T2> &a0)
  requires(std::copy_constructible<T1>) &&
          (std::ranges::view<T1> &&
           std::ranges::__has_tuple_element<std::ranges::range_value_t<T1>,
                                            T2> &&
           std::ranges::__has_tuple_element<
               std::remove_reference_t<std::ranges::range_reference_t<T1>>,
               T2> &&
           std::ranges::__returnable_element<std::ranges::range_reference_t<T1>,
                                             T2>)
{
  return a0.base();
}

template <typename T1, std::size_t T2>
T1 f56(std::ranges::elements_view<T1, T2> &&a0)
  requires(
      std::ranges::view<T1> &&
      std::ranges::__has_tuple_element<std::ranges::range_value_t<T1>, T2> &&
      std::ranges::__has_tuple_element<
          std::remove_reference_t<std::ranges::range_reference_t<T1>>, T2> &&
      std::ranges::__returnable_element<std::ranges::range_reference_t<T1>, T2>)
{
  return std::move(a0).base();
}

template <typename T1, std::size_t T2>
auto f57(std::ranges::elements_view<T1, T2> &a0)
  requires((!std::ranges::__simple_view<T1>)) &&
          (std::ranges::view<T1> &&
           std::ranges::__has_tuple_element<std::ranges::range_value_t<T1>,
                                            T2> &&
           std::ranges::__has_tuple_element<
               std::remove_reference_t<std::ranges::range_reference_t<T1>>,
               T2> &&
           std::ranges::__returnable_element<std::ranges::range_reference_t<T1>,
                                             T2>)
{
  return a0.begin();
}

template <typename T1, std::size_t T2>
auto f58(const std::ranges::elements_view<T1, T2> &a0)
  requires(std::ranges::range<const T1>) &&
          (std::ranges::view<T1> &&
           std::ranges::__has_tuple_element<std::ranges::range_value_t<T1>,
                                            T2> &&
           std::ranges::__has_tuple_element<
               std::remove_reference_t<std::ranges::range_reference_t<T1>>,
               T2> &&
           std::ranges::__returnable_element<std::ranges::range_reference_t<T1>,
                                             T2>)
{
  return a0.begin();
}

template <typename T1, std::size_t T2>
auto f59(std::ranges::elements_view<T1, T2> &a0)
  requires((!std::ranges::__simple_view<T1> &&
            !std::ranges::common_range<T1>)) &&
          (std::ranges::view<T1> &&
           std::ranges::__has_tuple_element<std::ranges::range_value_t<T1>,
                                            T2> &&
           std::ranges::__has_tuple_element<
               std::remove_reference_t<std::ranges::range_reference_t<T1>>,
               T2> &&
           std::ranges::__returnable_element<std::ranges::range_reference_t<T1>,
                                             T2>)
{
  return a0.end();
}

template <typename T1, std::size_t T2>
auto f60(std::ranges::elements_view<T1, T2> &a0)
  requires((!std::ranges::__simple_view<T1> &&
            std::ranges::common_range<T1>)) &&
          (std::ranges::view<T1> &&
           std::ranges::__has_tuple_element<std::ranges::range_value_t<T1>,
                                            T2> &&
           std::ranges::__has_tuple_element<
               std::remove_reference_t<std::ranges::range_reference_t<T1>>,
               T2> &&
           std::ranges::__returnable_element<std::ranges::range_reference_t<T1>,
                                             T2>)
{
  return a0.end();
}

template <typename T1, std::size_t T2>
auto f61(const std::ranges::elements_view<T1, T2> &a0)
  requires(std::ranges::range<const T1>) &&
          (std::ranges::view<T1> &&
           std::ranges::__has_tuple_element<std::ranges::range_value_t<T1>,
                                            T2> &&
           std::ranges::__has_tuple_element<
               std::remove_reference_t<std::ranges::range_reference_t<T1>>,
               T2> &&
           std::ranges::__returnable_element<std::ranges::range_reference_t<T1>,
                                             T2>)
{
  return a0.end();
}

template <typename T1, std::size_t T2>
auto f62(const std::ranges::elements_view<T1, T2> &a0)
  requires(std::ranges::common_range<const T1>) &&
          (std::ranges::view<T1> &&
           std::ranges::__has_tuple_element<std::ranges::range_value_t<T1>,
                                            T2> &&
           std::ranges::__has_tuple_element<
               std::remove_reference_t<std::ranges::range_reference_t<T1>>,
               T2> &&
           std::ranges::__returnable_element<std::ranges::range_reference_t<T1>,
                                             T2>)
{
  return a0.end();
}

template <typename T1, std::size_t T2>
auto f63(std::ranges::elements_view<T1, T2> &a0)
  requires(std::ranges::sized_range<T1>) &&
          (std::ranges::view<T1> &&
           std::ranges::__has_tuple_element<std::ranges::range_value_t<T1>,
                                            T2> &&
           std::ranges::__has_tuple_element<
               std::remove_reference_t<std::ranges::range_reference_t<T1>>,
               T2> &&
           std::ranges::__returnable_element<std::ranges::range_reference_t<T1>,
                                             T2>)
{
  return a0.size();
}

template <typename T1, std::size_t T2>
auto f64(const std::ranges::elements_view<T1, T2> &a0)
  requires(std::ranges::sized_range<const T1>) &&
          (std::ranges::view<T1> &&
           std::ranges::__has_tuple_element<std::ranges::range_value_t<T1>,
                                            T2> &&
           std::ranges::__has_tuple_element<
               std::remove_reference_t<std::ranges::range_reference_t<T1>>,
               T2> &&
           std::ranges::__returnable_element<std::ranges::range_reference_t<T1>,
                                             T2>)
{
  return a0.size();
}

template <typename T1, std::size_t T2,
          typename T3 = std::ranges::elements_view<T1, T2>>
bool f65(std::ranges::elements_view<T1, T2> &a0)
  requires(std::ranges::sized_range<std::ranges::elements_view<T1, T2>> ||
           std::ranges::forward_range<std::ranges::elements_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::elements_view<T1, T2>> &&
           std::same_as<
               std::ranges::elements_view<T1, T2>,
               std::remove_cv_t<std::ranges::elements_view<T1, T2>>>) &&
          (std::ranges::view<T1> &&
           std::ranges::__has_tuple_element<std::ranges::range_value_t<T1>,
                                            T2> &&
           std::ranges::__has_tuple_element<
               std::remove_reference_t<std::ranges::range_reference_t<T1>>,
               T2> &&
           std::ranges::__returnable_element<std::ranges::range_reference_t<T1>,
                                             T2>)
{
  return a0.empty();
}

template <typename T1, std::size_t T2,
          typename T3 = std::ranges::elements_view<T1, T2>>
bool f66(const std::ranges::elements_view<T1, T2> &a0)
  requires(std::ranges::sized_range<std::ranges::elements_view<T1, T2> const> ||
           std::ranges::forward_range<
               std::ranges::elements_view<T1, T2> const>) &&
          (std::is_class_v<std::ranges::elements_view<T1, T2>> &&
           std::same_as<
               std::ranges::elements_view<T1, T2>,
               std::remove_cv_t<std::ranges::elements_view<T1, T2>>>) &&
          (std::ranges::view<T1> &&
           std::ranges::__has_tuple_element<std::ranges::range_value_t<T1>,
                                            T2> &&
           std::ranges::__has_tuple_element<
               std::remove_reference_t<std::ranges::range_reference_t<T1>>,
               T2> &&
           std::ranges::__returnable_element<std::ranges::range_reference_t<T1>,
                                             T2>)
{
  return a0.empty();
}

template <typename T1, std::size_t T2,
          typename T3 = std::ranges::elements_view<T1, T2>>
bool f67(std::ranges::elements_view<T1, T2> &a0)
  requires(
      std::ranges::view<T1> &&
      std::ranges::__has_tuple_element<std::ranges::range_value_t<T1>, T2> &&
      std::ranges::__has_tuple_element<
          std::remove_reference_t<std::ranges::range_reference_t<T1>>, T2> &&
      std::ranges::__returnable_element<std::ranges::range_reference_t<T1>, T2>)
{
  return static_cast<bool>(a0);
}

template <typename T1, std::size_t T2,
          typename T3 = std::ranges::elements_view<T1, T2>>
bool f68(const std::ranges::elements_view<T1, T2> &a0)
  requires(
      std::ranges::view<T1> &&
      std::ranges::__has_tuple_element<std::ranges::range_value_t<T1>, T2> &&
      std::ranges::__has_tuple_element<
          std::remove_reference_t<std::ranges::range_reference_t<T1>>, T2> &&
      std::ranges::__returnable_element<std::ranges::range_reference_t<T1>, T2>)
{
  return static_cast<bool>(a0);
}

template <typename T1, std::size_t T2,
          typename T3 = std::ranges::elements_view<T1, T2>>
auto f69(std::ranges::elements_view<T1, T2> &a0)
  requires(std::contiguous_iterator<
              std::ranges::iterator_t<std::ranges::elements_view<T1, T2>>>) &&
          (std::is_class_v<std::ranges::elements_view<T1, T2>> &&
           std::same_as<
               std::ranges::elements_view<T1, T2>,
               std::remove_cv_t<std::ranges::elements_view<T1, T2>>>) &&
          (std::ranges::view<T1> &&
           std::ranges::__has_tuple_element<std::ranges::range_value_t<T1>,
                                            T2> &&
           std::ranges::__has_tuple_element<
               std::remove_reference_t<std::ranges::range_reference_t<T1>>,
               T2> &&
           std::ranges::__returnable_element<std::ranges::range_reference_t<T1>,
                                             T2>)
{
  return a0.data();
}

template <typename T1, std::size_t T2,
          typename T3 = std::ranges::elements_view<T1, T2>>
auto f70(const std::ranges::elements_view<T1, T2> &a0)
  requires(std::ranges::range<std::ranges::elements_view<T1, T2> const> &&
           std::contiguous_iterator<std::ranges::iterator_t<
               std::ranges::elements_view<T1, T2> const>>) &&
          (std::is_class_v<std::ranges::elements_view<T1, T2>> &&
           std::same_as<
               std::ranges::elements_view<T1, T2>,
               std::remove_cv_t<std::ranges::elements_view<T1, T2>>>) &&
          (std::ranges::view<T1> &&
           std::ranges::__has_tuple_element<std::ranges::range_value_t<T1>,
                                            T2> &&
           std::ranges::__has_tuple_element<
               std::remove_reference_t<std::ranges::range_reference_t<T1>>,
               T2> &&
           std::ranges::__returnable_element<std::ranges::range_reference_t<T1>,
                                             T2>)
{
  return a0.data();
}

template <typename T1, std::size_t T2,
          typename T3 = std::ranges::elements_view<T1, T2>>
decltype(auto) f71(std::ranges::elements_view<T1, T2> &a0)
  requires(std::ranges::forward_range<std::ranges::elements_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::elements_view<T1, T2>> &&
           std::same_as<
               std::ranges::elements_view<T1, T2>,
               std::remove_cv_t<std::ranges::elements_view<T1, T2>>>) &&
          (std::ranges::view<T1> &&
           std::ranges::__has_tuple_element<std::ranges::range_value_t<T1>,
                                            T2> &&
           std::ranges::__has_tuple_element<
               std::remove_reference_t<std::ranges::range_reference_t<T1>>,
               T2> &&
           std::ranges::__returnable_element<std::ranges::range_reference_t<T1>,
                                             T2>)
{
  return a0.front();
}

template <typename T1, std::size_t T2,
          typename T3 = std::ranges::elements_view<T1, T2>>
decltype(auto) f72(const std::ranges::elements_view<T1, T2> &a0)
  requires(std::ranges::forward_range<
              std::ranges::elements_view<T1, T2> const>) &&
          (std::is_class_v<std::ranges::elements_view<T1, T2>> &&
           std::same_as<
               std::ranges::elements_view<T1, T2>,
               std::remove_cv_t<std::ranges::elements_view<T1, T2>>>) &&
          (std::ranges::view<T1> &&
           std::ranges::__has_tuple_element<std::ranges::range_value_t<T1>,
                                            T2> &&
           std::ranges::__has_tuple_element<
               std::remove_reference_t<std::ranges::range_reference_t<T1>>,
               T2> &&
           std::ranges::__returnable_element<std::ranges::range_reference_t<T1>,
                                             T2>)
{
  return a0.front();
}

template <typename T1, std::size_t T2,
          typename T3 = std::ranges::elements_view<T1, T2>>
decltype(auto) f73(std::ranges::elements_view<T1, T2> &a0)
  requires(std::ranges::bidirectional_range<
               std::ranges::elements_view<T1, T2>> &&
           std::ranges::common_range<std::ranges::elements_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::elements_view<T1, T2>> &&
           std::same_as<
               std::ranges::elements_view<T1, T2>,
               std::remove_cv_t<std::ranges::elements_view<T1, T2>>>) &&
          (std::ranges::view<T1> &&
           std::ranges::__has_tuple_element<std::ranges::range_value_t<T1>,
                                            T2> &&
           std::ranges::__has_tuple_element<
               std::remove_reference_t<std::ranges::range_reference_t<T1>>,
               T2> &&
           std::ranges::__returnable_element<std::ranges::range_reference_t<T1>,
                                             T2>)
{
  return a0.back();
}

template <typename T1, std::size_t T2,
          typename T3 = std::ranges::elements_view<T1, T2>>
decltype(auto) f74(const std::ranges::elements_view<T1, T2> &a0)
  requires(std::ranges::bidirectional_range<
               std::ranges::elements_view<T1, T2> const> &&
           std::ranges::common_range<
               std::ranges::elements_view<T1, T2> const>) &&
          (std::is_class_v<std::ranges::elements_view<T1, T2>> &&
           std::same_as<
               std::ranges::elements_view<T1, T2>,
               std::remove_cv_t<std::ranges::elements_view<T1, T2>>>) &&
          (std::ranges::view<T1> &&
           std::ranges::__has_tuple_element<std::ranges::range_value_t<T1>,
                                            T2> &&
           std::ranges::__has_tuple_element<
               std::remove_reference_t<std::ranges::range_reference_t<T1>>,
               T2> &&
           std::ranges::__returnable_element<std::ranges::range_reference_t<T1>,
                                             T2>)
{
  return a0.back();
}

template <typename T1, std::size_t T2,
          typename T3 = std::ranges::elements_view<T1, T2>>
decltype(auto) f75(std::ranges::elements_view<T1, T2> &a0,
                   std::ranges::range_difference_t<T3> a1)
  requires(std::is_class_v<std::ranges::elements_view<T1, T2>> &&
           std::same_as<
               std::ranges::elements_view<T1, T2>,
               std::remove_cv_t<std::ranges::elements_view<T1, T2>>>) &&
          (std::ranges::random_access_range<T3>) &&
          (std::ranges::view<T1> &&
           std::ranges::__has_tuple_element<std::ranges::range_value_t<T1>,
                                            T2> &&
           std::ranges::__has_tuple_element<
               std::remove_reference_t<std::ranges::range_reference_t<T1>>,
               T2> &&
           std::ranges::__returnable_element<std::ranges::range_reference_t<T1>,
                                             T2>)
{
  return a0.template operator[]<T3>(
      std::forward<std::ranges::range_difference_t<T3>>(a1));
}

template <typename T1, std::size_t T2,
          typename T3 = const std::ranges::elements_view<T1, T2>>
decltype(auto) f76(const std::ranges::elements_view<T1, T2> &a0,
                   std::ranges::range_difference_t<T3> a1)
  requires(std::is_class_v<std::ranges::elements_view<T1, T2>> &&
           std::same_as<
               std::ranges::elements_view<T1, T2>,
               std::remove_cv_t<std::ranges::elements_view<T1, T2>>>) &&
          (std::ranges::random_access_range<T3>) &&
          (std::ranges::view<T1> &&
           std::ranges::__has_tuple_element<std::ranges::range_value_t<T1>,
                                            T2> &&
           std::ranges::__has_tuple_element<
               std::remove_reference_t<std::ranges::range_reference_t<T1>>,
               T2> &&
           std::ranges::__returnable_element<std::ranges::range_reference_t<T1>,
                                             T2>)
{
  return a0.template operator[]<T3>(
      std::forward<std::ranges::range_difference_t<T3>>(a1));
}

template <typename T1, std::size_t T2, bool T3>
using t7 = typename std::ranges::elements_view<T1, T2>::template __iterator<T3>;

template <typename T1, std::size_t T2, bool T3>
using t8 = typename std::ranges::elements_view<T1, T2>::template __iterator<
    T3>::iterator_concept;

template <typename T1, std::size_t T2, bool T3>
using t9 = typename std::ranges::elements_view<T1, T2>::template __iterator<
    T3>::value_type;

template <typename T1, std::size_t T2, bool T3>
using t10 = typename std::ranges::elements_view<T1, T2>::template __iterator<
    T3>::difference_type;

template <typename T1, std::size_t T2, bool T3>
typename std::ranges::elements_view<T1, T2>::template __iterator<T3> f77()
  requires(std::default_initializable<
           std::ranges::iterator_t<typename std::ranges::elements_view<
               T1, T2>::template __iterator<T3>::_Base>>)
{
  return typename std::ranges::elements_view<T1, T2>::template __iterator<T3>();
}

template <typename T1, std::size_t T2, bool T3>
typename std::ranges::elements_view<T1, T2>::template __iterator<T3>
f78(std::ranges::iterator_t<typename std::conditional<T3, const T1, T1>::type>
        a0) {
  return typename std::ranges::elements_view<T1, T2>::template __iterator<T3>(
      std::forward<std::ranges::iterator_t<
          typename std::conditional<T3, const T1, T1>::type>>(a0));
}

template <typename T1, std::size_t T2, bool T3>
typename std::ranges::elements_view<T1, T2>::template __iterator<T3>
f79(typename std::ranges::elements_view<T1, T2>::template __iterator<!T3> a0)
  requires(T3 &&
           std::convertible_to<
               std::ranges::iterator_t<T1>,
               std::ranges::iterator_t<typename std::ranges::elements_view<
                   T1, T2>::template __iterator<T3>::_Base>>)
{
  return typename std::ranges::elements_view<T1, T2>::template __iterator<T3>(
      std::forward<typename std::ranges::elements_view<
          T1, T2>::template __iterator<!T3>>(a0));
}

template <typename T1, std::size_t T2, bool T3>
const std::ranges::iterator_t<
    typename std::conditional<T3, const T1, T1>::type> &
f80(const typename std::ranges::elements_view<T1, T2>::template __iterator<T3>
        &a0) {
  return a0.base();
}

template <typename T1, std::size_t T2, bool T3>
std::ranges::iterator_t<typename std::conditional<T3, const T1, T1>::type>
f81(typename std::ranges::elements_view<T1, T2>::template __iterator<T3> &&a0) {
  return std::move(a0).base();
}

template <typename T1, std::size_t T2, bool T3>
decltype(auto)
f82(const typename std::ranges::elements_view<T1, T2>::template __iterator<T3>
        &a0) {
  return a0.operator*();
}

template <typename T1, std::size_t T2, bool T3>
typename std::ranges::elements_view<T1, T2>::template __iterator<T3> &
f83(typename std::ranges::elements_view<T1, T2>::template __iterator<T3> &a0) {
  return a0.operator++();
}

template <typename T1, std::size_t T2, bool T3>
void f84(
    typename std::ranges::elements_view<T1, T2>::template __iterator<T3> &a0,
    int a1) {
  return a0.operator++(a1);
}

template <typename T1, std::size_t T2, bool T3>
typename std::ranges::elements_view<T1, T2>::template __iterator<T3>
f85(typename std::ranges::elements_view<T1, T2>::template __iterator<T3> &a0,
    int a1)
  requires(std::ranges::forward_range<typename std::ranges::elements_view<
               T1, T2>::template __iterator<T3>::_Base>)
{
  return a0.operator++(a1);
}

template <typename T1, std::size_t T2, bool T3>
typename std::ranges::elements_view<T1, T2>::template __iterator<T3> &
f86(typename std::ranges::elements_view<T1, T2>::template __iterator<T3> &a0)
  requires(std::ranges::bidirectional_range<typename std::ranges::elements_view<
               T1, T2>::template __iterator<T3>::_Base>)
{
  return a0.operator--();
}

template <typename T1, std::size_t T2, bool T3>
typename std::ranges::elements_view<T1, T2>::template __iterator<T3>
f87(typename std::ranges::elements_view<T1, T2>::template __iterator<T3> &a0,
    int a1)
  requires(std::ranges::bidirectional_range<typename std::ranges::elements_view<
               T1, T2>::template __iterator<T3>::_Base>)
{
  return a0.operator--(a1);
}

template <typename T1, std::size_t T2, bool T3>
typename std::ranges::elements_view<T1, T2>::template __iterator<T3> &
f88(typename std::ranges::elements_view<T1, T2>::template __iterator<T3> &a0,
    typename std::ranges::elements_view<T1, T2>::template __iterator<
        T3>::difference_type a1)
  requires(std::ranges::random_access_range<typename std::ranges::elements_view<
               T1, T2>::template __iterator<T3>::_Base>)
{
  return a0.operator+=(
      std::forward<typename std::ranges::elements_view<
          T1, T2>::template __iterator<T3>::difference_type>(a1));
}

template <typename T1, std::size_t T2, bool T3>
typename std::ranges::elements_view<T1, T2>::template __iterator<T3> &
f89(typename std::ranges::elements_view<T1, T2>::template __iterator<T3> &a0,
    typename std::ranges::elements_view<T1, T2>::template __iterator<
        T3>::difference_type a1)
  requires(std::ranges::random_access_range<typename std::ranges::elements_view<
               T1, T2>::template __iterator<T3>::_Base>)
{
  return a0.operator-=(
      std::forward<typename std::ranges::elements_view<
          T1, T2>::template __iterator<T3>::difference_type>(a1));
}

template <typename T1, std::size_t T2, bool T3>
decltype(auto)
f90(const typename std::ranges::elements_view<T1, T2>::template __iterator<T3>
        &a0,
    typename std::ranges::elements_view<T1, T2>::template __iterator<
        T3>::difference_type a1)
  requires(std::ranges::random_access_range<typename std::ranges::elements_view<
               T1, T2>::template __iterator<T3>::_Base>)
{
  return a0.operator[](
      std::forward<typename std::ranges::elements_view<
          T1, T2>::template __iterator<T3>::difference_type>(a1));
}

template <typename T1, std::size_t T2, bool T3>
auto f91(
    const typename std::ranges::elements_view<T1, T2>::template __iterator<T3>
        &a0,
    const typename std::ranges::elements_view<T1, T2>::template __iterator<T3>
        &a1) {
  return operator<=>(a0, a1);
}

template <typename T1, std::size_t T2, bool T3>
bool f92(
    const typename std::ranges::elements_view<T1, T2>::template __iterator<T3>
        &a0,
    const typename std::ranges::elements_view<T1, T2>::template __iterator<T3>
        &a1) {
  return operator>=(a0, a1);
}

template <typename T1, std::size_t T2, bool T3>
bool f93(
    const typename std::ranges::elements_view<T1, T2>::template __iterator<T3>
        &a0,
    const typename std::ranges::elements_view<T1, T2>::template __iterator<T3>
        &a1) {
  return operator<=(a0, a1);
}

template <typename T1, std::size_t T2, bool T3>
bool f94(
    const typename std::ranges::elements_view<T1, T2>::template __iterator<T3>
        &a0,
    const typename std::ranges::elements_view<T1, T2>::template __iterator<T3>
        &a1) {
  return operator>(a0, a1);
}

template <typename T1, std::size_t T2, bool T3>
bool f95(
    const typename std::ranges::elements_view<T1, T2>::template __iterator<T3>
        &a0,
    const typename std::ranges::elements_view<T1, T2>::template __iterator<T3>
        &a1) {
  return operator<(a0, a1);
}

template <typename T1, std::size_t T2, bool T3>
bool f96(
    const typename std::ranges::elements_view<T1, T2>::template __iterator<T3>
        &a0,
    const typename std::ranges::elements_view<T1, T2>::template __iterator<T3>
        &a1) {
  return operator==(a0, a1);
}

template <typename T1, std::size_t T2, bool T3>
using t11 =
    typename std::ranges::elements_view<T1, T2>::template __sentinel<T3>;

template <typename T1, std::size_t T2, bool T3>
typename std::ranges::elements_view<T1, T2>::template __sentinel<T3> f97() {
  return typename std::ranges::elements_view<T1, T2>::template __sentinel<T3>();
}

template <typename T1, std::size_t T2, bool T3>
typename std::ranges::elements_view<T1, T2>::template __sentinel<T3>
f98(std::ranges::sentinel_t<typename std::conditional<T3, const T1, T1>::type>
        a0) {
  return typename std::ranges::elements_view<T1, T2>::template __sentinel<T3>(
      std::forward<std::ranges::sentinel_t<
          typename std::conditional<T3, const T1, T1>::type>>(a0));
}

template <typename T1, std::size_t T2, bool T3>
typename std::ranges::elements_view<T1, T2>::template __sentinel<T3>
f99(typename std::ranges::elements_view<T1, T2>::template __sentinel<!T3> a0)
  requires(T3 &&
           std::convertible_to<
               std::ranges::sentinel_t<T1>,
               std::ranges::sentinel_t<typename std::ranges::elements_view<
                   T1, T2>::template __sentinel<T3>::_Base>>)
{
  return typename std::ranges::elements_view<T1, T2>::template __sentinel<T3>(
      std::forward<typename std::ranges::elements_view<
          T1, T2>::template __sentinel<!T3>>(a0));
}

template <typename T1, std::size_t T2, bool T3>
std::ranges::sentinel_t<typename std::conditional<T3, const T1, T1>::type>
f100(const typename std::ranges::elements_view<T1, T2>::template __sentinel<T3>
         &a0) {
  return a0.base();
}

template <typename T1> using t12 = std::ranges::keys_view<T1>;

template <typename T1> using t13 = std::ranges::values_view<T1>;
